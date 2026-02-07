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
#   swarm/tools/pr.sh receipt <issue> [--slug <slug>] [--no-fetch-issue] [-f <output_receipt.md>] [--version <v0.2>]
#   swarm/tools/pr.sh finish  <issue> --title "<title>" [-f <input_card.md>] [--receipt <output_receipt.md>] [--body "<extra body>"] [--paths "<p1,p2,...>"] [--no-checks] [--no-close] [--ready] [--allow-gitignore] [--no-open]
#   swarm/tools/pr.sh open
#   swarm/tools/pr.sh status
#
# Examples:
#   swarm/tools/pr.sh start 14 --slug b6-default-system
#   swarm/tools/pr.sh card  14 --version v0.2
#   swarm/tools/pr.sh receipt 14 --version v0.2
#   swarm/tools/pr.sh finish 14 --title "swarm: apply run.defaults.system fallback" -f .adl/cards/issue-0014__input__v0.2.md --receipt .adl/cards/issue-0014__output__v0.2.md
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

# ---------- cards + templates (templates tracked; cards local-only) ----------
ADL_DIR=".adl"
ADL_TEMPLATES_DIR="$ADL_DIR/templates"
ADL_CARDS_DIR="$ADL_DIR/cards"

INPUT_TEMPLATE="$ADL_TEMPLATES_DIR/input_card_template.md"
OUTPUT_TEMPLATE="$ADL_TEMPLATES_DIR/output_receipt_card_template.md"

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

render_template_with_issue() {
  # Args: template_path issue title branch ver kind
  local tpl="$1" issue="$2" title="$3" branch="$4" ver="$5" kind="$6"
  [[ -f "$tpl" ]] || return 1

  local esc_title esc_branch
  esc_title="$(printf '%s' "$title" | sed -e 's/[\\&]/\\\\&/g')"
  esc_branch="$(printf '%s' "$branch" | sed -e 's/[\\&]/\\\\&/g')"

  sed -e "s/{{ISSUE}}/${issue}/g" \
      -e "s/{{TITLE}}/${esc_title}/g" \
      -e "s/{{BRANCH}}/${esc_branch}/g" \
      -e "s/{{VERSION}}/${ver}/g" \
      -e "s/{{KIND}}/${kind}/g" \
      "$tpl"
}

seed_input_card() {
  local path="$1" issue="$2" title="$3" branch="$4" ver="$5"
  if render_template_with_issue "$INPUT_TEMPLATE" "$issue" "$title" "$branch" "$ver" "input" >"$path" 2>/dev/null; then
    return 0
  fi
  cat >"$path" <<EOF
# ADL Input Card

Issue: #$issue
Version: $ver
Title: $title
Branch: $branch

## Goal

## Acceptance Criteria

## Context / Notes

## Codex Instructions
- Read this file.
- Do the work described above.
- Write the receipt to the paired output card file.
EOF
}

seed_output_card() {
  local path="$1" issue="$2" title="$3" branch="$4" ver="$5"
  if render_template_with_issue "$OUTPUT_TEMPLATE" "$issue" "$title" "$branch" "$ver" "output" >"$path" 2>/dev/null; then
    return 0
  fi
  cat >"$path" <<EOF
# ADL Output Receipt Card

Issue: #$issue
Version: $ver
Status: IN_PROGRESS

## Provenance
- Actor:
- Model:
- Branch: $branch
- Start Time:
- End Time:

## Commands Executed

## Files Changed

## Tests Run
- [ ] cargo fmt
- [ ] cargo clippy --all-targets -- -D warnings
- [ ] cargo test

## Result
- Pass / Fail:

## Decisions Made

## Follow-ups / Deferred Work
EOF
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

cmd_receipt() {
  require_cmd gh

  local issue="${1:-}"; shift || true
  [[ -n "$issue" ]] || die "receipt: missing <issue> number"

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
      *) die "receipt: unknown arg: $1" ;;
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
      die "receipt: --slug is required when --no-fetch-issue is set or issue title could not be fetched"
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
    die "receipt: output already exists: $out_path"
  fi
  note "Creating output receipt card: $out_path"
  ensure_adl_dirs
  seed_output_card "$out_path" "$issue" "$title" "$(current_branch)" "$version"
  note "Done."
  echo "$out_path"
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
    note "Creating output receipt card: $out_path"
    seed_output_card "$out_path" "$issue" "$title" "$branch" "$ver"
  fi
  echo "• Codex:"
  echo "  READ  $in_path"
  echo "  WRITE $out_path"
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
      die "finish: missing output receipt card: $output_path"
    fi
    die "finish: output receipt card is empty: $output_path"
  fi

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

  local pr_body_file
  pr_body_file="$(render_pr_body_file "$issue" "$close_line" "$input_path" "$output_path" "$extra_body" "$no_checks")"
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
  pr_url="$(gh pr create $(gh_repo_flag "$repo") --base main --head "$branch" --title "$title" --body-file "$pr_body_file" --draft)"

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
  start   <issue> [--slug <slug>] [--prefix <pfx>] [--no-fetch-issue]
  card    <issue> ... [--version <v0.2>] [-f <input_card.md>]
  receipt <issue> ... [--version <v0.2>] [-f <output_receipt.md>]
  finish  <issue> --title "<title>" ... [-f <input_card.md>] [--output <output_receipt.md>] [--no-open]
  open
  status

Flags:
  (card)    -f, --file <input_card.md>         Output path for the generated input card (default: .adl/cards/issue-####__input__vX.Y.md)
  (receipt) -f, --file <output_receipt.md>     Output path for the generated receipt card (default: .adl/cards/issue-####__output__vX.Y.md)
  (card/receipt) --version <v0.2>              Override detected version (otherwise inferred from issue labels version:vX.Y)
  (finish) --receipt <output_receipt.md>       REQUIRED: output receipt card path (must exist)
  (card/start) --slug <slug>                   Use an explicit slug instead of fetching the issue title.

Notes:
- PRs are created as DRAFT by default to preserve human review.
- Uses "Closes #N" by default so GitHub auto-closes issues when merged.
- Runs Rust checks in swarm/ by default (fmt, clippy -D warnings, test).
- finish stages swarm/ by default (reduces accidental commits).
- Templates are stored in .adl/templates/ (versioned): input_card_template.md and output_receipt_card_template.md.
- Cards are stored locally under .adl/cards/ and are not committed to git.

Examples:
  swarm/tools/pr.sh start 17 --slug b6-default-system
  swarm/tools/pr.sh card  17 --version v0.2
  swarm/tools/pr.sh receipt 17 --version v0.2
  swarm/tools/pr.sh finish 17 --title "swarm: apply run.defaults.system fallback" -f .adl/cards/issue-0017__input__v0.2.md --receipt .adl/cards/issue-0017__output__v0.2.md
EOF
}

main() {
  local cmd="${1:-}"; shift || true
  case "$cmd" in
    start) cmd_start "$@" ;;
    finish) cmd_finish "$@" ;;
    card) cmd_card "$@" ;;
    receipt) cmd_receipt "$@" ;;
    open) cmd_open ;;
    status) cmd_status ;;
    -h|--help|"") usage ;;
    *) die "Unknown command: $cmd (try --help)" ;;
  esac
}

main "$@"
