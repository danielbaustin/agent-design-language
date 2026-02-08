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
#   swarm/tools/pr.sh start   <issue> [--slug <slug>] [--prefix codex] [--no-fetch-issue]
#   swarm/tools/pr.sh card    <issue> [--slug <slug>] [--no-fetch-issue] [-f <input_card.md>] [--version <v0.2>]
#   swarm/tools/pr.sh output  <issue> [--slug <slug>] [--no-fetch-issue] [-f <output_card.md>] [--version <v0.2>]
#   swarm/tools/pr.sh finish  <issue> --title "<title>" [-f <input_card.md>] [--receipt <output_receipt.md>] [--body "<extra body>"] [--paths "<p1,p2,...>"] [--no-checks] [--no-close] [--ready] [--allow-gitignore] [--no-open]
#   swarm/tools/pr.sh open
#   swarm/tools/pr.sh status
#
# Examples:
#   swarm/tools/pr.sh start 14 --slug b6-default-system
#   swarm/tools/pr.sh card  14 --version v0.2
#   swarm/tools/pr.sh output 14 --version v0.2
#   swarm/tools/pr.sh finish 14 --title "swarm: apply run.defaults.system fallback" -f .adl/cards/issue-0014__input__v0.2.md --receipt .adl/cards/issue-0014__output__v0.2.md
#   swarm/tools/pr.sh open
#
set -euo pipefail


#
# ---------- helpers ----------
die() { echo "❌ $*" >&2; exit 1; }
note() { echo "• $*"; }
#
# Replace the first line that begins with "<Key>:" with "<Key>: <Value>".
# Portable (no GNU/BSD sed -i differences).
set_field_line() {
  local file="$1" key="$2" value="$3"
  local tmp
  tmp="$(mktemp -t prsh_field_XXXXXX)"
  awk -v k="$key" -v v="$value" '
    BEGIN { replaced = 0 }
    {
      if (!replaced && $0 ~ ("^" k ":")) {
        print k ": " v
        replaced = 1
        next
      }
      print $0
    }
  ' "$file" >"$tmp"
  mv "$tmp" "$file"
}

# Replace the first line that matches a regex pattern with a literal replacement line.
replace_first_line_re() {
  local file="$1" pattern="$2" replacement="$3"
  local tmp
  tmp="$(mktemp -t prsh_repl_XXXXXX)"
  awk -v p="$pattern" -v r="$replacement" '
    BEGIN { replaced = 0 }
    {
      if (!replaced && $0 ~ p) {
        print r
        replaced = 1
        next
      }
      print $0
    }
  ' "$file" >"$tmp"
  mv "$tmp" "$file"
}

print_next_steps() {
  cat <<'EOF'
Next steps (human review preserved):
- Open the PR in the browser and do a quick self-review.
- When satisfied, mark it Ready for review (or keep as draft if you want).
- Merge via GitHub UI (Squash and merge recommended).
EOF
}

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

# ----- pr/branch helpers -----
commits_ahead_of_main() {
  # Count commits on HEAD that are not on origin/main.
  # Returns 0 if origin/main isn't available yet.
  git rev-list --count origin/main..HEAD 2>/dev/null || echo 0
}

current_pr_url() {
  # Returns PR url for the current branch if one exists, else empty.
  local repo="$1"
  gh pr view $(gh_repo_flag "$repo") --json url -q .url 2>/dev/null || true
}

# ---------- cards + templates (templates tracked; cards local-only) ----------
ADL_DIR=".adl"
ADL_TEMPLATES_DIR="$ADL_DIR/templates"
ADL_CARDS_DIR="$ADL_DIR/cards"

INPUT_TEMPLATE="$ADL_TEMPLATES_DIR/input_card_template.md"
OUTPUT_TEMPLATE="$ADL_TEMPLATES_DIR/output_card_template.md"
LEGACY_OUTPUT_TEMPLATE="$ADL_TEMPLATES_DIR/output_receipt_card_template.md"

resolve_output_template() {
  # Prefer the new name; fall back to legacy for backwards compatibility.
  if [[ -f "$(repo_root)/$OUTPUT_TEMPLATE" ]]; then
    echo "$(repo_root)/$OUTPUT_TEMPLATE"
    return 0
  fi
  if [[ -f "$(repo_root)/$LEGACY_OUTPUT_TEMPLATE" ]]; then
    echo "$(repo_root)/$LEGACY_OUTPUT_TEMPLATE"
    return 0
  fi
  # Return the preferred path even if it doesn't exist (caller will handle).
  echo "$(repo_root)/$OUTPUT_TEMPLATE"
}

issue_pad() { printf '%04d' "$1"; }

issue_version() {
  local issue="$1"
  local v
  v="$(gh issue view "$issue" --json labels -q '.labels[].name' 2>/dev/null | sed -n 's/^version://p' | head -n1 || true)"
  if [[ -n "$v" ]]; then
    echo "$v"
  else
    echo "v0.2"
  fi
}

ensure_adl_dirs() {
  mkdir -p "$(repo_root)/$ADL_TEMPLATES_DIR" "$(repo_root)/$ADL_CARDS_DIR"
}

input_card_path() {
  local issue="$1" ver="$2"
  local pad
  pad="$(issue_pad "$issue")"
  echo "$(repo_root)/$ADL_CARDS_DIR/issue-${pad}__input__${ver}.md"
}

output_card_path() {
  local issue="$1" ver="$2"
  local pad
  pad="$(issue_pad "$issue")"
  echo "$(repo_root)/$ADL_CARDS_DIR/issue-${pad}__output__${ver}.md"
}

render_template() {
  # Args: template_path
  local tpl="$1"
  [[ -f "$tpl" ]] || return 1
  cat "$tpl"
}

seed_input_card() {
  local path="$1" issue="$2" title="$3" branch="$4" ver="$5"
  local task_id run_id
  task_id="issue-$(issue_pad "$issue")"
  run_id="$task_id"

  # Start from template if available, else fallback.
  if render_template "$(repo_root)/$INPUT_TEMPLATE" >"$path" 2>/dev/null; then
    :
  else
    cat >"$path" <<'EOF'
# ADL Input Card

Task ID:
Run ID:
Version:
Title:
Branch:

Context:
- Issue:
- PR:
- Docs:
- Other:

Execution:
- Agent:
- Provider:
- Tools allowed:
- Sandbox / approvals:

## Goal

## Acceptance Criteria

## Inputs
-

## Constraints / Policies
- Determinism requirements:
- Security / privacy requirements:
- Resource limits (time/CPU/memory/network):

## Non-goals / Out of scope

## Notes / Risks

## Instructions to the Agent
- Read this file.
- Do the work described above.
- Write results to the paired output card file.
EOF
  fi

  # Stamp fields (best-effort; keeps template generic and domain-agnostic).
  set_field_line "$path" "Task ID" "$task_id"
  set_field_line "$path" "Run ID" "$run_id"
  set_field_line "$path" "Version" "$ver"
  set_field_line "$path" "Title" "$title"
  set_field_line "$path" "Branch" "$branch"

  # If there is a Context Issue line, fill it with a URL.
  local repo
  repo="$(default_repo)"
  if [[ -n "$repo" ]]; then
    local issue_url
    issue_url="https://github.com/${repo}/issues/${issue}"
    replace_first_line_re "$path" "^- Issue:[[:space:]]*$" "- Issue: $issue_url"
  fi
}

seed_output_card() {
  local path="$1" issue="$2" title="$3" branch="$4" ver="$5"
  local task_id run_id
  task_id="issue-$(issue_pad "$issue")"
  run_id="$task_id"

  local out_tpl
  out_tpl="$(resolve_output_template)"

  if render_template "$out_tpl" >"$path" 2>/dev/null; then
    :
  else
    cat >"$path" <<'EOF'
# ADL Output Card

Task ID:
Run ID:
Version:
Title:
Branch:
Status: NOT_STARTED | IN_PROGRESS | DONE | FAILED

Execution:
- Actor:
- Model:
- Provider:
- Start Time:
- End Time:

## Summary

## Artifacts produced
-

## Actions taken
-

## Validation
- Tests / checks run:
- Results:

## Decisions / Deviations

## Follow-ups / Deferred work
EOF
  fi

  set_field_line "$path" "Task ID" "$task_id"
  set_field_line "$path" "Run ID" "$run_id"
  set_field_line "$path" "Version" "$ver"
  set_field_line "$path" "Title" "$title"
  set_field_line "$path" "Branch" "$branch"

  # Default Status if template left it blank.
  replace_first_line_re "$path" "^Status:[[:space:]]*$" "Status: NOT_STARTED | IN_PROGRESS | DONE | FAILED"
}

ensure_nonempty_file() {
  local path="$1"
  [[ -f "$path" ]] || return 1
  [[ -s "$path" ]] || return 1
  # Also reject files that are only whitespace
  if [[ -z "$(tr -d '[:space:]' <"$path")" ]]; then
    return 1
  fi
  return 0
}

render_pr_body_file() {
  # Renders a PR body into a temp file and echoes its path.
  # Args: issue close_line input_path output_path extra_body no_checks
  local issue="$1" close_line="$2" input_path="$3" output_path="$4" extra_body="$5" no_checks="$6"

  local tmp
  tmp="$(mktemp -t pr_body_XXXXXX.md)"

  {
    if [[ -n "$close_line" ]]; then
      echo "$close_line"
      echo
    fi

    echo "Local artifacts (not committed):"
    echo "- Input card:  $input_path"
    echo "- Output card: $output_path"
    echo

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

cmd_card() {
  require_cmd gh

  local issue="${1:-}"; shift || true
  [[ -n "$issue" ]] || die "card: missing <issue> number"

  local slug=""
  local no_fetch_issue="0"
  local out_path=""
  local version=""

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --slug) slug="$2"; shift 2 ;;
      --no-fetch-issue) no_fetch_issue="1"; shift ;;
      -f) out_path="$2"; shift 2 ;;
      --file) out_path="$2"; shift 2 ;;
      --version) version="$2"; shift 2 ;;
      *) die "card: unknown arg: $1" ;;
    esac
  done

  local repo
  repo="$(default_repo)"

  local title=""
  if [[ "$no_fetch_issue" != "1" ]]; then
    note "Fetching issue title via gh…"
    title="$(gh issue view "$issue" $(gh_repo_flag "$repo") --json title -q .title 2>/dev/null || true)"
  fi

  if [[ -z "$slug" ]]; then
    if [[ -n "$title" ]]; then
      slug="$(sanitize_slug "$title")"
    else
      die "card: --slug is required when --no-fetch-issue is set or issue title could not be fetched"
    fi
  fi

  if [[ -z "$title" ]]; then
    title="$slug"
  fi

  if [[ -z "$version" ]]; then
    version="$(issue_version "$issue")"
  fi
  if [[ -z "$out_path" ]]; then
    out_path="$(input_card_path "$issue" "$version")"
  fi
  if [[ -f "$out_path" ]]; then
    die "card: input card already exists: $out_path"
  fi
  note "Creating input card: $out_path"
  ensure_adl_dirs
  seed_input_card "$out_path" "$issue" "$title" "$(current_branch)" "$version"
  note "Done."
  echo "$out_path"
}

cmd_output() {
  require_cmd gh

  local issue="${1:-}"; shift || true
  [[ -n "$issue" ]] || die "output: missing <issue> number"

  local slug=""
  local no_fetch_issue="0"
  local out_path=""
  local version=""

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --slug) slug="$2"; shift 2 ;;
      --no-fetch-issue) no_fetch_issue="1"; shift ;;
      -f) out_path="$2"; shift 2 ;;
      --file) out_path="$2"; shift 2 ;;
      --version) version="$2"; shift 2 ;;
      *) die "output: unknown arg: $1" ;;
    esac
  done

  local repo
  repo="$(default_repo)"

  local title=""
  if [[ "$no_fetch_issue" != "1" ]]; then
    note "Fetching issue title via gh…"
    title="$(gh issue view "$issue" $(gh_repo_flag "$repo") --json title -q .title 2>/dev/null || true)"
  fi

  if [[ -z "$slug" ]]; then
    if [[ -n "$title" ]]; then
      slug="$(sanitize_slug "$title")"
    else
      die "output: --slug is required when --no-fetch-issue is set or issue title could not be fetched"
    fi
  fi

  if [[ -z "$title" ]]; then
    title="$slug"
  fi

  if [[ -z "$version" ]]; then
    version="$(issue_version "$issue")"
  fi
  if [[ -z "$out_path" ]]; then
    out_path="$(output_card_path "$issue" "$version")"
  fi
  if [[ -f "$out_path" ]]; then
    die "output: output card already exists: $out_path"
  fi
  note "Creating output card: $out_path"
  ensure_adl_dirs
  seed_output_card "$out_path" "$issue" "$title" "$(current_branch)" "$version"
  note "Done."
  echo "$out_path"
}

cmd_cards() {
  require_cmd gh

  local issue="${1:-}"; shift || true
  [[ -n "$issue" ]] || die "cards: missing <issue> number"

  local no_fetch_issue="0"
  local version=""

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --no-fetch-issue) no_fetch_issue="1"; shift ;;
      --version) version="$2"; shift 2 ;;
      *) die "cards: unknown arg: $1" ;;
    esac
  done

  local repo
  repo="$(default_repo)"

  local title=""
  if [[ "$no_fetch_issue" != "1" ]]; then
    note "Fetching issue title via gh…"
    title="$(gh issue view "$issue" $(gh_repo_flag "$repo") --json title -q .title 2>/dev/null || true)"
  fi

  if [[ -z "$title" ]]; then
    title="issue-${issue}"
  fi

  if [[ -z "$version" ]]; then
    version="$(issue_version "$issue")"
  fi
  [[ -n "$version" ]] || version="v0.2"

  ensure_adl_dirs

  local input_path output_path
  input_path="$(input_card_path "$issue" "$version")"
  output_path="$(output_card_path "$issue" "$version")"

  if [[ -f "$input_path" ]]; then
    note "Input card exists: $input_path"
  else
    note "Creating input card: $input_path"
    seed_input_card "$input_path" "$issue" "$title" "TBD (run pr.sh start $issue)" "$version"
  fi

  if [[ -f "$output_path" ]]; then
    note "Output card exists: $output_path"
  else
    note "Creating output card: $output_path"
    seed_output_card "$output_path" "$issue" "$title" "TBD (run pr.sh start $issue)" "$version"
  fi

  echo "READ  $input_path"
  echo "WRITE $output_path"
}

cmd_start() {
  require_cmd git
  require_cmd gh

  local issue="${1:-}"; shift || true
  [[ -n "$issue" ]] || die "start: missing <issue> number"

  local prefix="codex"
  local slug=""
  local no_fetch_issue="0"
  local title=""

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --prefix) prefix="$2"; shift 2 ;;
      --slug) slug="$2"; shift 2 ;;
      --no-fetch-issue) no_fetch_issue="1"; shift ;;
      *) die "start: unknown arg: $1" ;;
    esac
  done

  # Default slug: derive from issue title if possible.
  local repo
  repo="$(default_repo)"
  title=""
  if [[ -z "$slug" ]]; then
    if [[ "$no_fetch_issue" == "1" ]]; then
      die "start: --slug is required when --no-fetch-issue is set"
    fi
    note "Fetching issue title via gh…"
    title="$(gh issue view "$issue" $(gh_repo_flag "$repo") --json title -q .title 2>/dev/null || true)"
    [[ -n "$title" ]] || die "Could not fetch issue #$issue title. Pass --slug or check gh auth/repo."
    slug="$(sanitize_slug "$title")"
  fi
  if [[ -z "$title" ]]; then
    title="$slug"
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

  local ver in_path out_path
  ver="$(issue_version "$issue")"
  in_path="$(input_card_path "$issue" "$ver")"
  out_path="$(output_card_path "$issue" "$ver")"
  ensure_adl_dirs
  if [[ ! -f "$in_path" ]]; then
    note "Creating input card: $in_path"
    seed_input_card "$in_path" "$issue" "$title" "$branch" "$ver"
  fi
  if [[ ! -f "$out_path" ]]; then
    note "Creating output card: $out_path"
    seed_output_card "$out_path" "$issue" "$title" "$branch" "$ver"
  fi
  echo "• Agent:"
  echo "  READ   $in_path"
  echo "  WRITE  $out_path"
  note "Done."
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
  local input_path=""
  local output_path=""
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
      -f|--file|--input) input_path="$2"; shift 2 ;;
      --output|--receipt|--receipt-file) output_path="$2"; shift 2 ;;
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

  # Fetch origin so origin/main is up-to-date for ahead check.
  note "Fetching origin refs…"
  git fetch origin >/dev/null 2>&1 || true

  local ver
  ver="$(issue_version "$issue")"
  if [[ -z "$input_path" ]]; then
    input_path="$(input_card_path "$issue" "$ver")"
  fi
  if [[ -z "$output_path" ]]; then
    output_path="$(output_card_path "$issue" "$ver")"
  fi
  [[ -f "$input_path" ]] || die "finish: missing input card: $input_path"
  if ! ensure_nonempty_file "$output_path"; then
    if [[ ! -f "$output_path" ]]; then
      die "finish: missing output card: $output_path"
    fi
    die "finish: output card is empty: $output_path"
  fi

  # Safety: allow finish to proceed if there are commits ahead of origin/main
  # even when the user already committed manually (working tree clean).
  local ahead
  ahead="$(commits_ahead_of_main)"

  local has_uncommitted="0"
  if ! git diff --quiet || ! git diff --cached --quiet; then
    has_uncommitted="1"
  fi

  if [[ "$has_uncommitted" == "0" && "$ahead" -eq 0 ]]; then
    die "No changes detected and branch has no commits ahead of origin/main. Nothing to PR. If you already merged, switch branches."
  fi

  if [[ "$has_uncommitted" == "0" && "$ahead" -gt 0 ]]; then
    note "No uncommitted changes; will create/update PR using existing commits (ahead of origin/main by ${ahead})."
  fi

  if [[ "$no_checks" != "1" ]]; then
    run_swarm_checks
  else
    note "Skipping checks (--no-checks)"
  fi

  if [[ "$has_uncommitted" == "1" ]]; then
    # Stage selected paths (default: swarm). Use --paths "adl-spec,swarm" or --paths "." to stage everything.
    IFS=',' read -r -a path_arr <<< "$paths"
    if [[ ${#path_arr[@]} -eq 0 ]]; then
      die "finish: --paths resolved to empty; pass e.g. --paths \"swarm\""
    fi

    note "Staging changes (${paths})…"
    git add -A "${path_arr[@]}"

    if git diff --cached --quiet; then
      die "finish: nothing staged after 'git add' for paths '${paths}'. Either change --paths or commit manually and re-run finish."
    fi

    if [[ "$allow_gitignore" != "1" ]]; then
      if ! git diff --cached --quiet -- .gitignore swarm/.gitignore 2>/dev/null; then
        die "finish: .gitignore changes detected. Revert them or re-run with --allow-gitignore."
      fi
    fi
  fi

  local close_line=""
  if [[ "$no_close" != "1" ]]; then
    close_line="Closes #${issue}"
  fi

  local repo
  repo="$(default_repo)"

  local pr_body_file
  pr_body_file="$(render_pr_body_file "$issue" "$close_line" "$input_path" "$output_path" "$extra_body" "$no_checks")"
  trap 'rm -f "${pr_body_file:-}"' EXIT

  local commit_msg="$title"
  if [[ -n "$close_line" ]]; then
    commit_msg="${commit_msg} (${close_line})"
  fi

  if [[ "$has_uncommitted" == "1" ]]; then
    note "Committing…"
    git commit -m "$commit_msg"
  else
    note "Skipping commit (working tree clean; using existing commits)."
  fi

  local branch
  branch="$(current_branch)"

  note "Pushing…"
  if ! git push -u origin "$branch"; then
    die "Push failed (likely non-fast-forward due to remote divergence). Try: 'git fetch origin' then 'git push --force-with-lease origin $branch' (if you rebased) or 'git pull --rebase' (if you didn’t)."
  fi

  local pr_url
  pr_url="$(current_pr_url "$repo")"

  if [[ -n "$pr_url" ]]; then
    note "Updating existing PR…"
    gh pr edit $(gh_repo_flag "$repo") "$pr_url" --title "$title" --body-file "$pr_body_file" >/dev/null
    note "PR updated:"
    echo "$pr_url"
  else
    note "Creating PR (draft)…"
    pr_url="$(gh pr create $(gh_repo_flag "$repo") --base main --head "$branch" --title "$title" --body-file "$pr_body_file" --draft)"
    note "PR created:"
    echo "$pr_url"
  fi

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
  start   <issue> [--slug <slug>] [--prefix <pfx>] [--no-fetch-issue]
  card    <issue> ... [--version <v0.2>] [-f <input_card.md>]
  output  <issue> ... [--version <v0.2>] [-f <output_card.md>]
  cards   <issue> [--version <v0.2>] [--no-fetch-issue]
  finish  <issue> --title "<title>" ... [-f <input_card.md>] [--output <output_card.md>] [--no-open]
  open
  status

Flags:
  (card)    -f, --file <input_card.md>         Output path for the generated input card (default: .adl/cards/issue-####__input__vX.Y.md)
  (output)  -f, --file <output_card.md>        Output path for the generated output card (default: .adl/cards/issue-####__output__vX.Y.md)
  (cards)   --version <v0.2>                   Override detected version (otherwise inferred from issue labels version:vX.Y)
  (cards)   --no-fetch-issue                   Do not fetch issue title/labels (uses issue-<n> title)
  (card/output) --version <v0.2>               Override detected version (otherwise inferred from issue labels version:vX.Y)
  (finish) --receipt <output_card.md>          REQUIRED: output card path (must exist)
  (card/start) --slug <slug>                   Use an explicit slug instead of fetching the issue title.

Notes:
- PRs are created as DRAFT by default to preserve human review.
- Uses "Closes #N" by default so GitHub auto-closes issues when merged.
- Runs Rust checks in swarm/ by default (fmt, clippy -D warnings, test).
- finish stages swarm/ by default (reduces accidental commits).
- Templates are stored in .adl/templates/ (versioned): input_card_template.md and output_card_template.md (or legacy output_receipt_card_template.md) (output card template).
- Cards are stored locally under .adl/cards/ and are not committed to git.

Examples:
  swarm/tools/pr.sh start 17 --slug b6-default-system
  swarm/tools/pr.sh card  17 --version v0.2
  swarm/tools/pr.sh output 17 --version v0.2
  swarm/tools/pr.sh cards 17 --version v0.2
  swarm/tools/pr.sh finish 17 --title "swarm: apply run.defaults.system fallback" -f .adl/cards/issue-0017__input__v0.2.md --receipt .adl/cards/issue-0017__output__v0.2.md
EOF
}

main() {
  local cmd="${1:-}"; shift || true
  case "$cmd" in
    start) cmd_start "$@" ;;
    finish) cmd_finish "$@" ;;
    card) cmd_card "$@" ;;
    output) cmd_output "$@" ;;
    receipt) cmd_output "$@" ;;
    cards) cmd_cards "$@" ;;
    open) cmd_open ;;
    status) cmd_status ;;
    -h|--help|"") usage ;;
    *) die "Unknown command: $cmd (try --help)" ;;
  esac
}

main "$@"
