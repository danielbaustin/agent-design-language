#!/usr/bin/env bash
# pr.sh — opinionated helper to reduce git/PR thrash while preserving human review.
#
# Design goals:
# - Automate the ceremony (branching, checks, commit, push, PR creation).
# - Make it hard to accidentally commit/push on main.
# - Always require human review: PRs are created as *draft* by default.
# - Always wire issues to PRs with "Closes #N" unless explicitly disabled.
#
# Requirements:
# - git
# - GitHub CLI (gh) authenticated with repo access
# - Rust toolchain for `swarm/` checks (fmt, clippy, test)
#
# Usage:
#   swarm/tools/pr.sh start <issue> [--slug <slug>] [--prefix codex] [--no-fetch-issue] [-f <input_card.md>]
#   swarm/tools/pr.sh finish <issue> --title "<title>" [--body "<extra body>"] [--paths "<p1,p2,...>"] [--no-checks] [--no-close] [--ready] [--allow-gitignore] [-f <input_card.md>] [--no-open]
#   swarm/tools/pr.sh open
#   swarm/tools/pr.sh status
#
# Examples:
#   swarm/tools/pr.sh start 14 --slug b6-default-system
#   swarm/tools/pr.sh start 14 -f .adl/input_cards/input_card_14_b6-default-system.md
#   swarm/tools/pr.sh finish 14 --title "swarm: apply run.defaults.system fallback" --body "Tests: fmt/clippy/test" -f .adl/input_cards/input_card_14_b6-default-system.md
#   swarm/tools/pr.sh open
#
set -euo pipefail

# ---------- helpers ----------
die() { echo "❌ $*" >&2; exit 1; }
note() { echo "• $*"; }

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || die "Missing required command: $1"
}

repo_root() {
  git rev-parse --show-toplevel 2>/dev/null || die "Not in a git repo"
}

current_branch() {
  git rev-parse --abbrev-ref HEAD
}

ensure_clean_worktree() {
  if ! git diff --quiet || ! git diff --cached --quiet; then
    die "Working tree is dirty. Commit/stash your changes first."
  fi
}

sanitize_slug() {
  # Lowercase, keep alnum+dash, collapse dashes.
  local s="$1"
  s="$(echo "$s" | tr '[:upper:]' '[:lower:]')"
  s="$(echo "$s" | sed -E 's/[^a-z0-9]+/-/g; s/^-+//; s/-+$//; s/-+/-/g')"
  echo "$s"
}

default_repo() {
  # Derive "owner/repo" from git remote if possible; fallback to current directory name.
  local url
  url="$(git remote get-url origin 2>/dev/null || true)"
  if [[ "$url" =~ github.com[:/]+([^/]+/[^/.]+)(\.git)?$ ]]; then
    echo "${BASH_REMATCH[1]}"
    return 0
  fi
  # Fallback: let gh infer from current repo.
  echo ""
}

branch_for_issue() {
  local prefix="$1" issue="$2" slug="$3"
  slug="$(sanitize_slug "$slug")"
  echo "${prefix}/${issue}-${slug}"
}

ensure_not_on_main() {
  local b
  b="$(current_branch)"
  if [[ "$b" == "main" ]]; then
    die "You are on main. Use 'start' to create/switch to a feature branch."
  fi
}

run_swarm_checks() {
  note "Running checks in swarm/ (fmt, clippy -D warnings, test)…"
  (
    cd "$(repo_root)/swarm"
    cargo fmt
    cargo clippy --all-targets -- -D warnings
    cargo test
  )
}

gh_repo_flag() {
  local r="$1"
  if [[ -n "$r" ]]; then
    echo "-R" "$r"
  else
    echo
  fi
}

print_next_steps() {
  cat <<'EOF'
Next steps (human review preserved):
- Open the PR in the browser and assign a reviewer (or self-review), then mark Ready for review when appropriate.
- After approval, merge via GitHub UI (Squash and merge recommended).
EOF
}

# ---------- input cards / PR body helpers ----------
input_cards_dir() {
  # Versionable location for archived input cards.
  echo "$(repo_root)/.adl/input_cards"
}

ensure_dir() {
  local d="$1"
  mkdir -p "$d"
}

unique_path() {
  # If path exists, append -2, -3, ... before extension.
  local path="$1"
  if [[ ! -e "$path" ]]; then
    echo "$path"
    return 0
  fi
  local base ext i
  base="${path%.*}"
  ext="${path##*.}"
  if [[ "$base" == "$path" ]]; then
    ext=""
  else
    ext=".${ext}"
  fi
  i=2
  while [[ -e "${base}-${i}${ext}" ]]; do
    i=$((i+1))
  done
  echo "${base}-${i}${ext}"
}

input_card_template() {
  # Consistent prompt-template style input card.
  # Args: issue title
  local issue="$1" title="$2"
  cat <<EOF
# Input Card — Issue #${issue}

## Purpose
(One sentence: what do we want to achieve?)

## Context
(Background, links, prior decisions, constraints.)

## Requirements
- [ ]

## Non-goals
- 

## Acceptance Criteria
- [ ]

## Test / Verification Plan
- 

## Notes
- 

---

## Prompt Template (for Codex / agents)
You are working on Issue #${issue}: ${title}

### Task
(Describe exactly what to change.)

### Constraints
- Keep changes minimal and focused.
- Preserve existing behavior unless explicitly requested.
- Add/adjust tests as needed.

### Deliverables
- Code changes
- Tests
- Short explanation of what changed

### Review Checklist
- [ ] Build/tests pass
- [ ] No unrelated formatting churn
- [ ] Clear commit + PR description
EOF
}

archive_input_card() {
  # Copies the card into .adl/input_cards/ if it's not already there.
  local src="$1" issue="$2" slug="$3"
  [[ -n "$src" ]] || return 0
  [[ -f "$src" ]] || die "Input card not found: $src"

  local dir dest name
  dir="$(input_cards_dir)"
  ensure_dir "$dir"

  slug="$(sanitize_slug "$slug")"
  name="input_card_${issue}_${slug}.md"
  dest="$dir/$name"

  # If src is already within the archive dir, don't copy.
  local abs_src abs_dir
  abs_src="$(cd "$(dirname "$src")" && pwd)/$(basename "$src")"
  abs_dir="$(cd "$dir" && pwd)"
  if [[ "$abs_src" == "$abs_dir"/* ]]; then
    echo "$abs_src"
    return 0
  fi

  dest="$(unique_path "$dest")"
  cp -f "$src" "$dest"
  echo "$dest"
}

render_pr_body_file() {
  # Renders a PR body into a temp file and echoes its path.
  # Args: issue close_line card_path extra_body no_checks
  local issue="$1" close_line="$2" card_path="$3" extra_body="$4" no_checks="$5"

  local tmp
  tmp="$(mktemp -t pr_body_XXXXXX.md)"

  {
    if [[ -n "$close_line" ]]; then
      echo "$close_line"
      echo
    fi

    if [[ -n "$card_path" ]]; then
      if [[ -f "$card_path" ]]; then
        cat "$card_path"
        echo
      else
        die "Input card not found: $card_path"
      fi
    fi

    if [[ -n "$extra_body" ]]; then
      echo "$extra_body"
      echo
    fi

    if [[ "$no_checks" != "1" ]]; then
      echo "Tests:"
      echo "- cargo fmt"
      echo "- cargo clippy --all-targets -- -D warnings"
      echo "- cargo test"
    fi
  } >"$tmp"

  echo "$tmp"
}

open_in_browser() {
  # Opens the PR in a browser using gh (preferred) or the OS 'open' command.
  local repo="$1" pr_ref="$2"
  if gh pr view $(gh_repo_flag "$repo") "$pr_ref" --web >/dev/null 2>&1; then
    return 0
  fi
  if command -v open >/dev/null 2>&1; then
    open "$pr_ref" >/dev/null 2>&1 || true
  fi
}

# ---------- commands ----------
cmd_start() {
  require_cmd git
  require_cmd gh

  local issue="${1:-}"; shift || true
  [[ -n "$issue" ]] || die "start: missing <issue> number"

  local prefix="codex"
  local slug=""
  local no_fetch_issue="0"
  local card_path=""

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --prefix) prefix="$2"; shift 2 ;;
      --slug) slug="$2"; shift 2 ;;
      --no-fetch-issue) no_fetch_issue="1"; shift ;;
      -f) card_path="$2"; shift 2 ;;
      --file) card_path="$2"; shift 2 ;;
      *) die "start: unknown arg: $1" ;;
    esac
  done

  # Default slug: derive from issue title if possible.
  local repo
  repo="$(default_repo)"
  if [[ -z "$slug" ]]; then
    if [[ "$no_fetch_issue" == "1" ]]; then
      die "start: --slug is required when --no-fetch-issue is set"
    fi
    note "Fetching issue title via gh…"
    local title
    title="$(gh issue view "$issue" $(gh_repo_flag "$repo") --json title -q .title 2>/dev/null || true)"
    [[ -n "$title" ]] || die "Could not fetch issue #$issue title. Pass --slug or check gh auth/repo."
    slug="$(sanitize_slug "$title")"

    # If requested, create a consistent input card template (project artifact) if it doesn't exist yet.
    if [[ -n "$card_path" && ! -f "$card_path" ]]; then
      note "Creating input card: $card_path"
      mkdir -p "$(dirname "$card_path")"
      input_card_template "$issue" "$title" >"$card_path"
    fi
  fi

  local branch
  branch="$(branch_for_issue "$prefix" "$issue" "$slug")"

  note "Target branch: $branch"

  # If already on the branch, do nothing.
  if [[ "$(current_branch)" == "$branch" ]]; then
    note "Already on $branch"
    return 0
  fi

  # If branch exists locally, switch.
  if git show-ref --verify --quiet "refs/heads/$branch"; then
    note "Switching to existing local branch…"
    git switch "$branch"
    return 0
  fi

  # If branch exists on origin, switch and track.
  if git ls-remote --exit-code --heads origin "$branch" >/dev/null 2>&1; then
    note "Branch exists on origin; checking out and tracking…"
    git switch --track "origin/$branch"
    return 0
  fi

  # Otherwise create new branch from main, ensuring main is up to date.
  ensure_clean_worktree

  note "Updating main…"
  git switch main >/dev/null 2>&1 || true
  git pull --ff-only

  note "Creating branch…"
  git switch -c "$branch"

  note "Done."
  note "Tip: paste acceptance criteria + commands into swarm/.local/codex_inbox.md for Codex."
  if [[ -n "$card_path" ]]; then
    note "Tip: keep the input card archived (recommended: $(input_cards_dir))."
  fi
}

cmd_finish() {
  require_cmd git
  require_cmd gh

  local issue="${1:-}"; shift || true
  [[ -n "$issue" ]] || die "finish: missing <issue> number"

  local title=""
  local extra_body=""
  local no_checks="0"
  local no_close="0"
  local ready="0"
  local allow_gitignore="0"
  local paths="swarm"
  local card_path=""
  local no_open="0"

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --title) title="$2"; shift 2 ;;
      --body) extra_body="$2"; shift 2 ;;
      --paths) paths="$2"; shift 2 ;;
      --no-checks) no_checks="1"; shift ;;
      --no-close) no_close="1"; shift ;;
      --ready) ready="1"; shift ;;
      --allow-gitignore) allow_gitignore="1"; shift ;;
      -f) card_path="$2"; shift 2 ;;
      --file) card_path="$2"; shift 2 ;;
      --no-open) no_open="1"; shift ;;
      --open) no_open="0"; shift ;;
      *) die "finish: unknown arg: $1" ;;
    esac
  done

  [[ -n "$title" ]] || die "finish: --title is required"

  ensure_not_on_main

  local branch
  branch="$(current_branch)"
  if [[ "$branch" != */"${issue}"-* ]]; then
    die "finish: current branch '$branch' does not look like it matches issue #$issue (expected */${issue}-<slug>). Switch branches or pass the correct issue number."
  fi

  # Derive a stable slug from the branch name for archiving.
  local slug_for_card
  slug_for_card="${branch##*/}"
  slug_for_card="${slug_for_card#${issue}-}"
  slug_for_card="$(sanitize_slug "$slug_for_card")"

  # Basic safety: ensure there are changes to commit.
  if git diff --cached --quiet && git diff --quiet; then
    die "No changes detected. Nothing to commit/PR."
  fi

  if [[ "$no_checks" != "1" ]]; then
    run_swarm_checks
  else
    note "Skipping checks (--no-checks)"
  fi

  # Stage selected paths (default: swarm). Use --paths "adl-spec,swarm" or --paths "." to stage everything.
  IFS=',' read -r -a path_arr <<< "$paths"
  if [[ ${#path_arr[@]} -eq 0 ]]; then
    die "finish: --paths resolved to empty; pass e.g. --paths \"swarm\""
  fi

  note "Staging changes (${paths})…"
  git add -A "${path_arr[@]}"

  if [[ "$allow_gitignore" != "1" ]]; then
    if ! git diff --cached --quiet -- .gitignore swarm/.gitignore 2>/dev/null; then
      die "finish: .gitignore changes detected. Revert them or re-run with --allow-gitignore."
    fi
  fi

  local close_line=""
  if [[ "$no_close" != "1" ]]; then
    close_line="Closes #${issue}"
  fi

  local repo
  repo="$(default_repo)"

  # If an input card is provided, archive it (versionable) and use it in the PR body.
  local archived_card=""
  if [[ -n "$card_path" ]]; then
    archived_card="$(archive_input_card "$card_path" "$issue" "$slug_for_card")"
    note "Archived input card: $archived_card"
  fi

  local pr_body_file
  pr_body_file="$(render_pr_body_file "$issue" "$close_line" "$archived_card" "$extra_body" "$no_checks")"
  trap 'rm -f "$pr_body_file"' EXIT

  local commit_msg="$title"
  if [[ -n "$close_line" ]]; then
    commit_msg="${commit_msg} (${close_line})"
  fi

  note "Committing…"
  git commit -m "$commit_msg"

  local branch
  branch="$(current_branch)"

  note "Pushing…"
  git push -u origin "$branch"

  note "Creating PR (draft)…"
  local pr_url
  pr_url="$(gh pr create $(gh_repo_flag "$repo") --base main --head "$branch" --title "$title" --body-file "$pr_body_file" --draft --json url -q .url)"

  note "PR created:"
  echo "$pr_url"

  if [[ "$no_open" != "1" ]]; then
    note "Opening PR in browser…"
    open_in_browser "$repo" "$pr_url" || true
  else
    note "Not opening PR (--no-open)"
  fi

  if [[ "$ready" == "1" ]]; then
    note "Marking PR ready for review…"
    gh pr ready $(gh_repo_flag "$repo") "$pr_url" >/dev/null
    note "PR is ready for review."
  else
    note "PR is a draft (by design). When you've reviewed, mark it Ready for review in GitHub."
  fi

  print_next_steps
}

cmd_status() {
  require_cmd git
  note "Branch: $(current_branch)"
  git status -sb
}

cmd_open() {
  require_cmd gh
  local repo
  repo="$(default_repo)"
  note "Opening PR for current branch in browser…"
  gh pr view $(gh_repo_flag "$repo") --web >/dev/null
}

usage() {
  cat <<'EOF'
pr.sh — reduce git/PR thrash while preserving human review

Commands:
  start  <issue> [--slug <slug>] [--prefix <pfx>] [--no-fetch-issue] [-f <input_card.md>]
  finish <issue> --title "<title>" [--body "<extra body>"] [--paths "<p1,p2,...>"] [--no-checks] [--no-close] [--ready] [--allow-gitignore] [-f <input_card.md>] [--no-open]
  open
  status

Flags:
  -f, --file <input_card.md>   Use an input card file (created on start if missing). On finish, the card is archived into .adl/input_cards/ and included in the PR body.
  --no-open                    Do not open the PR in a browser after creation.
  --open                       Explicitly enable browser open (default).

Notes:
- PRs are created as DRAFT by default to preserve human review.
- Uses "Closes #N" by default so GitHub auto-closes issues when merged.
- Runs Rust checks in swarm/ by default (fmt, clippy -D warnings, test).
- finish stages swarm/ by default (reduces accidental commits).
- Input cards are archived under .adl/input_cards/ so they can be versioned in git.

Examples:
  swarm/tools/pr.sh start 17 --slug b6-default-system
  swarm/tools/pr.sh start 17 -f .adl/input_cards/input_card_17_b6-default-system.md
  swarm/tools/pr.sh finish 17 --title "swarm: apply run.defaults.system fallback" -f .adl/input_cards/input_card_17_b6-default-system.md
  swarm/tools/pr.sh open
EOF
}

main() {
  local cmd="${1:-}"; shift || true
  case "$cmd" in
    start) cmd_start "$@" ;;
    finish) cmd_finish "$@" ;;
    open) cmd_open ;;
    status) cmd_status ;;
    -h|--help|"") usage ;;
    *) die "Unknown command: $cmd (try --help)" ;;
  esac
}

main "$@"
