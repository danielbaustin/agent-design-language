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
# - Rust toolchain for `adl/` checks (fmt, clippy, test)
#
#   adl/tools/pr.sh help
#   adl/tools/pr.sh init    <issue> [--slug <slug>] [--title "<title>"] [--no-fetch-issue] [--version <v0.85>]
#   adl/tools/pr.sh start   <issue> [--slug <slug>] [--title "<title>"] [--prefix codex] [--no-fetch-issue] [--version <v0.85>]
#   adl/tools/pr.sh run     <adl.yaml> [--trace] [--print-plan] [--print-prompts] [--resume <run.json>] [--steer <steering.json>] [--overlay <overlay.json>] [--out <dir>] [--runs-root <dir>] [--quiet] [--open] [--allow-unsigned]
#   adl/tools/pr.sh card    <issue> [input|output] [--slug <slug>] [--no-fetch-issue] [-f <input_card.md>] [--version <v0.2>]
#   adl/tools/pr.sh output  <issue> [input|output] [--slug <slug>] [--no-fetch-issue] [-f <output_card.md>] [--version <v0.2>]
#   adl/tools/pr.sh finish  <issue> --title "<title>" [-f <input_card.md>] [--output-card <output_card.md>] [--body "<extra body>"] [--paths "<p1,p2,...>"] [--no-checks] [--no-close] [--ready] [--allow-gitignore] [--no-open]
#   adl/tools/pr.sh open
#   adl/tools/pr.sh status
#
# Examples:
#   adl/tools/pr.sh init  14 --slug b6-default-system --no-fetch-issue --version v0.85
#   adl/tools/pr.sh start 14 --slug b6-default-system
#   adl/tools/pr.sh run adl/examples/v0-4-demo-deterministic-replay.adl.yaml --trace --allow-unsigned
#   adl/tools/pr.sh card  14 --version v0.2
#   adl/tools/pr.sh card  14 input
#   adl/tools/pr.sh card  14 output
#   adl/tools/pr.sh output 14 --version v0.2
#   adl/tools/pr.sh output 14 input
#   adl/tools/pr.sh output 14 output
#   adl/tools/pr.sh finish 14 --title "adl: apply run.defaults.system fallback" -f /abs/cards_root/14/input_14.md --output-card /abs/cards_root/14/output_14.md
#   adl/tools/pr.sh open
#
set -euo pipefail

CARD_PATHS_LIB="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/card_paths.sh"
# shellcheck disable=SC1090
source "$CARD_PATHS_LIB"

DEFAULT_VERSION="v0.86"
DEFAULT_NEW_LABELS="track:roadmap,type:task,area:tools"


#
# ---------- helpers ----------
die() { echo "❌ $*" >&2; exit 1; }
note() { echo "• $*"; }

die_with_usage() {
  local msg="$1" usage_fn="$2"
  echo "❌ $msg" >&2
  "$usage_fn" >&2
  exit 1
}

die_index_lock() {
  local context="$1"
  die "$context failed due to .git/index.lock. Remediation: ensure no other git process is running, then remove stale lock with 'rm -f .git/index.lock', and rerun the same command."
}

run_git_or_die() {
  local context="$1"
  shift
  local out status
  set +e
  out="$("$@" 2>&1)"
  status=$?
  set -e
  if [[ "$status" -eq 0 ]]; then
    [[ -n "$out" ]] && echo "$out"
    return 0
  fi
  if [[ "$out" == *".git/index.lock"* ]]; then
    die_index_lock "$context"
  fi
  [[ -n "$out" ]] && echo "$out" >&2
  die "$context failed"
}
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

field_line_value() {
  local file="$1" key="$2"
  awk -v k="$key" '
    $0 ~ ("^" k ":") {
      sub(/^[^:]*:[[:space:]]*/, "", $0)
      print
      exit
    }
  ' "$file"
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

normalize_issue_or_die() {
  local raw="$1"
  local normalized
  normalized="$(card_issue_normalize "$raw" 2>/dev/null)" || die "invalid issue number: $raw"
  echo "$normalized"
}

repo_root() {
  git rev-parse --show-toplevel 2>/dev/null || die "Not in a git repo"
}

current_branch() {
  git rev-parse --abbrev-ref HEAD
}

path_relative_to_repo() {
  local path="$1"
  local root
  root="$(repo_root)"
  if [[ "$path" == "$root/"* ]]; then
    echo "${path#"$root/"}"
  else
    echo "$path"
  fi
}

issue_prompt_path_for_issue() {
  local issue="$1" scope="$2" slug="$3"
  local root
  root="$(repo_root)"
  echo "$root/.adl/issues/$scope/bodies/issue-${issue}-${slug}.md"
}

resolve_repo_relative_path() {
  local path="$1"
  local root
  root="$(repo_root)"
  if [[ "$path" == /* ]]; then
    echo "$path"
  else
    echo "$root/$path"
  fi
}

absolute_host_path_present() {
  local target="$1"
  rg -n -e '(^|[^A-Za-z])(\/Users\/|\/home\/|[A-Za-z]:\\)' "$target" >/dev/null 2>&1
}

extract_front_matter_to_file() {
  local src="$1" dest="$2"
  awk '
    NR == 1 && $0 == "---" { in_fm = 1; next }
    in_fm && $0 == "---" { exit }
    in_fm { print }
  ' "$src" >"$dest"
}

extract_markdown_body_to_file() {
  local src="$1" dest="$2"
  awk '
    NR == 1 && $0 == "---" { in_fm = 1; next }
    in_fm && $0 == "---" { in_fm = 0; next }
    !in_fm { print }
  ' "$src" >"$dest"
}

strip_yaml_scalar_quotes() {
  local v="$1"
  v="${v#\"}"
  v="${v%\"}"
  v="${v#\'}"
  v="${v%\'}"
  printf '%s\n' "$v"
}

stp_scalar_field() {
  local fm="$1" key="$2"
  awk -v k="$key" '
    $0 ~ ("^" k ":") {
      sub(/^[^:]*:[[:space:]]*/, "", $0)
      print
      exit
    }
  ' "$fm"
}

stp_array_items() {
  local fm="$1" key="$2"
  awk -v k="$key" '
    BEGIN { in_arr = 0 }
    $0 ~ ("^" k ":") { in_arr = 1; next }
    in_arr && $0 ~ /^[^[:space:]-]/ { exit }
    in_arr && $0 ~ /^[[:space:]]*-[[:space:]]*/ {
      sub(/^[[:space:]]*-[[:space:]]*/, "", $0)
      print
    }
  ' "$fm"
}

issue_card_reference() {
  local kind="$1" issue="$2"
  case "$kind" in
    input) echo ".adl/cards/${issue}/input_${issue}.md" ;;
    output) echo ".adl/cards/${issue}/output_${issue}.md" ;;
    *) die "invalid card reference kind: $kind" ;;
  esac
}

git_common_dir() {
  git rev-parse --git-common-dir 2>/dev/null || die "Not in a git repo"
}

acquire_repo_lock() {
  local name="$1"
  local lock_dir
  lock_dir="$(git_common_dir)/${name}.lock"
  local attempt max_attempts
  max_attempts=50
  for ((attempt=1; attempt<=max_attempts; attempt++)); do
    if mkdir "$lock_dir" 2>/dev/null; then
      printf '%s\n' "$lock_dir"
      return 0
    fi
    sleep 0.1
  done
  die "${name}: another pr.sh bootstrap operation appears to be running (lock: $lock_dir). Remediation: rerun the command serially after the current bootstrap completes."
}

release_repo_lock() {
  local lock_dir="${1:-}"
  [[ -n "$lock_dir" ]] || return 0
  rm -rf "$lock_dir"
}

ensure_clean_worktree() {
  if ! git diff --quiet || ! git diff --cached --quiet; then
    die "Working tree is dirty. Commit/stash your changes first."
  fi
}

primary_checkout_root() {
  card_primary_checkout_root
}

default_worktree_path_for_issue() {
  local issue="$1"
  local primary managed_root
  primary="$(primary_checkout_root)"
  managed_root="${ADL_WORKTREE_ROOT:-}"
  if [[ -z "$managed_root" ]]; then
    managed_root="$primary/.worktrees"
  fi
  echo "$managed_root/adl-wp-${issue}"
}

branch_checked_out_worktree_path() {
  local target_branch="$1"
  local wt="" br=""
  while IFS= read -r line; do
    case "$line" in
      worktree\ *)
        wt="${line#worktree }"
        br=""
        ;;
      branch\ refs/heads/*)
        br="${line#branch refs/heads/}"
        if [[ "$br" == "$target_branch" && -n "$wt" ]]; then
          echo "$wt"
          return 0
        fi
        ;;
    esac
  done < <(git worktree list --porcelain)
  return 1
}

warn_branch_upstream_not_origin_main() {
  local branch="$1"
  local upstream
  upstream="$(git for-each-ref --format='%(upstream:short)' "refs/heads/$branch" 2>/dev/null || true)"
  if [[ "$upstream" != "origin/main" ]]; then
    note "Warning: branch '$branch' upstream is '${upstream:-<none>}' (not origin/main)."
    note "Remediation (optional): git branch --set-upstream-to=origin/main \"$branch\""
  fi
}

ensure_primary_checkout_on_main() {
  local primary current
  primary="$(primary_checkout_root)"
  current="$(git -C "$primary" rev-parse --abbrev-ref HEAD 2>/dev/null || true)"
  [[ -n "$current" ]] || return 0
  if [[ "$current" == "main" ]]; then
    return 0
  fi

  if [[ -n "$(git -C "$primary" status --porcelain 2>/dev/null || true)" ]]; then
    die "start: primary checkout ($primary) is on '$current' with local changes. Remediation: commit/stash there, switch to main, then rerun."
  fi

  note "Switching primary checkout back to main: $primary"
  run_git_or_die "start: switch primary checkout to main" git -C "$primary" switch main
}

ensure_worktree_path_usable() {
  local path="$1" branch="$2"

  if [[ ! -e "$path" ]]; then
    return 0
  fi

  if [[ ! -d "$path" ]]; then
    die "start: worktree path exists but is not a directory: $path. Remediation: move/remove that path, then rerun."
  fi

  if ! git -C "$path" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    die "start: worktree path already exists and is not a git worktree: $path. Remediation: use an empty path or remove/move this directory."
  fi

  local shared wt_branch current_common
  shared="$(git -C "$path" rev-parse --git-common-dir 2>/dev/null || true)"
  current_common="$(git rev-parse --git-common-dir 2>/dev/null || true)"
  wt_branch="$(git -C "$path" rev-parse --abbrev-ref HEAD 2>/dev/null || true)"

  if [[ -z "$shared" || -z "$current_common" || "$shared" != "$current_common" ]]; then
    die "start: path '$path' belongs to a different repository. Remediation: choose a different worktree path."
  fi
  if [[ "$wt_branch" != "$branch" ]]; then
    die "start: worktree path '$path' is already on branch '$wt_branch' (expected '$branch'). Remediation: remove that worktree or use its current branch."
  fi
}

sanitize_slug() {
  # Lowercase, keep alnum+dash, collapse dashes.
  local s="$1"
  s="$(echo "$s" | tr '[:upper:]' '[:lower:]')"
  s="$(echo "$s" | sed -E 's/[^a-z0-9]+/-/g; s/^-+//; s/-+$//; s/-+/-/g')"
  echo "$s"
}

infer_wp_from_title() {
  local title="$1"
  if [[ "$title" =~ \[(WP-[0-9]+)\] ]]; then
    printf '%s\n' "${BASH_REMATCH[1]}"
    return 0
  fi
  printf 'unassigned\n'
}

version_from_title() {
  local title="$1"
  if [[ "$title" =~ \[(v[0-9]+\.[0-9]+)\] ]]; then
    printf '%s\n' "${BASH_REMATCH[1]}"
  fi
}

version_from_labels_csv() {
  local labels_csv="$1" label
  IFS=',' read -r -a label_arr <<< "$labels_csv"
  for label in "${label_arr[@]}"; do
    label="$(trim_ws "$label")"
    if [[ "$label" =~ ^version:(v[0-9]+\.[0-9]+)$ ]]; then
      printf '%s\n' "${BASH_REMATCH[1]}"
      return 0
    fi
  done
  return 1
}

infer_required_outcome_type() {
  local labels_csv="$1" title="$2"
  local lowered
  lowered="$(printf '%s %s' "$labels_csv" "$title" | tr '[:upper:]' '[:lower:]')"
  if [[ "$lowered" == *"type:docs"* || "$lowered" == *"area:docs"* || "$lowered" == *"[docs]"* || "$lowered" == *"type:design"* ]]; then
    printf 'docs\n'
    return 0
  fi
  if [[ "$lowered" == *"type:test"* || "$lowered" == *"area:tests"* || "$lowered" == *"[test]"* ]]; then
    printf 'tests\n'
    return 0
  fi
  if [[ "$lowered" == *"area:demo"* || "$lowered" == *"[demo]"* ]]; then
    printf 'demo\n'
    return 0
  fi
  printf 'code\n'
}

write_generated_issue_prompt() {
  local dest_path="$1" issue="$2" version="$3" slug="$4" title="$5" labels_csv="$6" issue_url="$7"
  local wp outcome_type
  wp="$(infer_wp_from_title "$title")"
  outcome_type="$(infer_required_outcome_type "$labels_csv" "$title")"
  mkdir -p "$(dirname "$dest_path")"
  cat >"$dest_path" <<EOF
---
issue_card_schema: adl.issue.v1
wp: "$wp"
slug: "$slug"
title: "$title"
labels:
$(IFS=',' read -r -a _labels <<< "$labels_csv"; for label in "${_labels[@]}"; do label="$(trim_ws "$label")"; [[ -n "$label" ]] || continue; printf '  - "%s"\n' "$label"; done)
issue_number: $issue
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "Pending sprint assignment"
required_outcome_type:
  - "$outcome_type"
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Bootstrap-generated from GitHub issue metadata because no canonical local issue prompt existed yet."
pr_start:
  enabled: true
  slug: "$slug"
---

# $title

## Summary

Bootstrap-generated local source prompt for issue #$issue.

## Goal

Translate the GitHub issue into the canonical local STP/task-bundle flow and refine this prompt before execution as needed.

## Required Outcome

This issue currently defaults to a required outcome type of \`$outcome_type\`. Refine this if the issue requires a different outcome or a combination.

## Deliverables

- one bounded, reviewable outcome matching the issue scope
- updated canonical docs/code/tests/demo artifacts as required by the issue

## Acceptance Criteria

- the issue title and labels are reflected in the local source prompt
- the task can proceed through \`pr init\`, \`pr start\`, and card editing without manual bootstrap repair

## Repo Inputs

- $issue_url

## Dependencies

- none recorded yet

## Demo Expectations

- No demo is required by default. Update this section if the issue requires a proof surface.

## Non-goals

- changing milestone scope without recording it explicitly
- ad-hoc local workflow drift outside the tracked issue flow

## Issue-Graph Notes

- This prompt was generated automatically because the canonical local issue prompt was missing.
- Review and refine it before substantive implementation work begins.

## Notes

- GitHub issue: $issue_url

## Tooling Notes

- Generated by \`pr.sh\` bootstrap fallback.
EOF
}

ensure_source_issue_prompt() {
  local issue="$1" version="$2" slug="$3" title="$4" labels_csv="${5:-}"
  local source_path issue_url
  source_path="$(issue_prompt_path_for_issue "$issue" "$version" "$slug")"
  if [[ -f "$source_path" ]]; then
    printf '%s\n' "$source_path"
    return 0
  fi

  if [[ -z "$labels_csv" ]]; then
    local repo
    repo="$(default_repo)"
    labels_csv="$(gh issue view "$issue" $(gh_repo_flag "$repo") --json labels -q '.labels[].name' 2>/dev/null | paste -sd, - || true)"
  fi
  issue_url="https://github.com/$(default_repo)/issues/${issue}"
  write_generated_issue_prompt "$source_path" "$issue" "$version" "$slug" "$title" "$labels_csv" "$issue_url"
  printf '%s\n' "$source_path"
}

default_repo() {
  # Derive "owner/repo" from git remote if possible; otherwise use the current repo
  # name under a deterministic local namespace so generated cards remain schema-valid
  # even in offline/minimal test repos.
  local url inferred root base
  url="$(git remote get-url origin 2>/dev/null || true)"
  if [[ "$url" =~ github.com[:/]+([^/]+/[^/.]+)(\.git)?$ ]]; then
    echo "${BASH_REMATCH[1]}"
    return 0
  fi

  inferred="$(gh repo view --json nameWithOwner --jq .nameWithOwner 2>/dev/null || true)"
  if [[ -n "$inferred" ]]; then
    echo "$inferred"
    return 0
  fi

  root="$(repo_root)"
  base="$(basename "$root")"
  echo "local/$base"
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
  note "Running checks in adl/ (fmt, clippy -D warnings, test)…"
  (
    cd "$(repo_root)/adl"
    cargo fmt
    cargo clippy --all-targets -- -D warnings
    cargo test
  )
}

run_tooling_sanity_checks() {
  local root
  root="$(repo_root)"
  note "Running tooling sanity checks (codex_pr/codexw)…"
  bash -n "$root/adl/tools/codex_pr.sh"
  bash -n "$root/adl/tools/codexw.sh"
  echo "Skipping codex_pr sanity check (no --paths configured)."
  bash "$root/adl/tools/codexw.sh" --help >/dev/null 2>&1
  sh "$root/adl/tools/codexw.sh" --help >/dev/null 2>&1
}

run_batched_checks() {
  local root
  root="$(repo_root)"
  note "Running batched checks via adl/tools/batched_checks.sh…"
  bash "$root/adl/tools/batched_checks.sh"
}

gh_repo_flag() {
  local r="$1"
  if [[ -n "$r" ]]; then
    echo "-R" "$r"
  else
    echo
  fi
}

# ----- staging helpers -----
trim_ws() {
  # Trim leading/trailing whitespace
  local s="$1"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  echo "$s"
}

is_ignored_path() {
  # Returns 0 if the path is ignored by git, 1 otherwise.
  local p="$1"
  git check-ignore -q -- "$p" >/dev/null 2>&1
}

stage_selected_paths() {
  # Stage a comma-split list of paths, skipping paths that are gitignored.
  # This avoids `git add` failing when an ignored path is explicitly passed.
  local -a arr=("$@")
  local staged_any="0"

  for p in "${arr[@]}"; do
    p="$(trim_ws "$p")"
    [[ -z "$p" ]] && continue

    if is_ignored_path "$p"; then
      note "Skipping ignored path: $p"
      continue
    fi

    # Stage the path. If the path doesn't exist but is listed in --paths, fail fast.
    if [[ ! -e "$p" ]]; then
      die "finish: path does not exist: $p"
    fi

    run_git_or_die "finish: git add for path '$p'" git add -A -- "$p"
    staged_any="1"
  done

  if [[ "$staged_any" != "1" ]]; then
    die "finish: all provided --paths were empty or gitignored; pass non-ignored paths (e.g. --paths \"adl/tools,docs/tooling\")"
  fi
}

# ----- pr/branch helpers -----
commits_ahead_of_main() {
  # Count commits on HEAD that are not on origin/main.
  # Returns 0 if origin/main isn't available yet.
  git rev-list --count origin/main..HEAD 2>/dev/null || echo 0
}

current_pr_url() {
  # Returns open PR url for a branch if one exists, else empty.
  local repo="$1" branch="$2"
  local url
  url="$(gh pr list $(gh_repo_flag "$repo") --head "$branch" --state open --json url -q '.[0].url' 2>/dev/null || true)"
  if [[ -n "$url" ]]; then
    echo "$url"
    return 0
  fi
  gh pr view $(gh_repo_flag "$repo") --json url -q .url 2>/dev/null || true
}

# ---------- cards + templates (templates tracked; cards local-only) ----------
ADL_DIR=".adl"

INPUT_TEMPLATE="adl/templates/cards/input_card_template.md"
OUTPUT_TEMPLATE="adl/templates/cards/output_card_template.md"
LEGACY_INPUT_TEMPLATE="$ADL_DIR/templates/input_card_template.md"
LEGACY_OUTPUT_TEMPLATE="$ADL_DIR/templates/output_card_template.md"

resolve_input_template() {
  if [[ -f "$(repo_root)/$INPUT_TEMPLATE" ]]; then
    echo "$(repo_root)/$INPUT_TEMPLATE"
    return 0
  fi
  if [[ -f "$(repo_root)/$LEGACY_INPUT_TEMPLATE" ]]; then
    echo "$(repo_root)/$LEGACY_INPUT_TEMPLATE"
    return 0
  fi
  # Return preferred path even if it doesn't exist (caller validates existence).
  echo "$(repo_root)/$INPUT_TEMPLATE"
}

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

resolve_structured_prompt_validator() {
  local validator
  validator="$(repo_root)/adl/tools/validate_structured_prompt.sh"
  [[ -x "$validator" ]] || die "start: missing executable structured prompt validator: $validator"
  echo "$validator"
}

issue_version() {
  local issue="$1"
  local v repo
  repo="$(default_repo)"
  v="$(gh issue view "$issue" $(gh_repo_flag "$repo") --json labels -q '.labels[].name' 2>/dev/null | sed -n 's/^version://p' | head -n1 || true)"
  if [[ -z "$v" ]]; then
    local title
    title="$(gh issue view "$issue" $(gh_repo_flag "$repo") --json title -q .title 2>/dev/null || true)"
    v="$(version_from_title "$title" || true)"
  fi
  if [[ -n "$v" ]]; then
    echo "$v"
  else
    echo "$DEFAULT_VERSION"
  fi
}

ensure_adl_dirs() {
  mkdir -p "$(cards_root_resolve)"
}

input_card_path() {
  local issue="$1" ver="${2:-}" slug="${3:-}"
  resolve_input_card_path "$issue" "$ver" "$slug" || die "invalid issue number: $issue"
}

output_card_path() {
  local issue="$1" ver="${2:-}" slug="${3:-}"
  resolve_output_card_path "$issue" "$ver" "$slug" || die "invalid issue number: $issue"
}

resolve_input_card_path_abs() {
  local issue="$1" ver="$2" slug="${3:-}"
  resolve_input_card_path "$issue" "$ver" "$slug" || die "invalid issue number: $issue"
}

resolve_output_card_path_abs() {
  local issue="$1" ver="$2" slug="${3:-}"
  resolve_output_card_path "$issue" "$ver" "$slug" || die "invalid issue number: $issue"
}

sync_legacy_links_for_issue() {
  local issue="$1" ver="$2" slug="${3:-}"
  local in_path out_path
  [[ -n "$slug" ]] || return 0
  in_path="$(resolve_input_card_path_abs "$issue" "$ver" "$slug")"
  out_path="$(resolve_output_card_path_abs "$issue" "$ver" "$slug")"
  ensure_legacy_card_compat_link input "$issue" "$in_path"
  ensure_legacy_card_compat_link output "$issue" "$out_path"
}

render_template() {
  # Args: template_path
  local tpl="$1"
  [[ -f "$tpl" ]] || return 1
  cat "$tpl"
}

join_by() {
  local delimiter="$1"
  shift || true
  local first=1 item
  for item in "$@"; do
    if [[ "$first" -eq 1 ]]; then
      printf '%s' "$item"
      first=0
    else
      printf '%s%s' "$delimiter" "$item"
    fi
  done
}

docs_context_value_for_issue_prompt() {
  local source_path="$1"
  [[ -f "$source_path" ]] || {
    printf 'none'
    return 0
  }

  local fm tmp item
  local -a docs=()
  fm="$(mktemp -t prsh_docs_context_fm_XXXXXX)"
  extract_front_matter_to_file "$source_path" "$fm"
  while IFS= read -r item; do
    item="$(strip_yaml_scalar_quotes "$item")"
    [[ -n "$item" ]] || continue
    if [[ "$item" == *.md || "$item" == docs/* || "$item" == .adl/docs/* ]]; then
      docs+=("$item")
    fi
  done < <(stp_array_items "$fm" "repo_inputs")
  rm -f "$fm"

  if [[ "${#docs[@]}" -eq 0 ]]; then
    printf 'none'
  else
    join_by '; ' "${docs[@]}"
  fi
}

validate_card_header_count() {
  # Args: file_path header_line
  local path="$1" header="$2"
  local count
  count="$(grep -c -x -F "$header" "$path" || true)"
  [[ "$count" == "1" ]]
}

seed_input_card() {
  local path="$1" issue="$2" title="$3" branch="$4" ver="$5" output_path_actual="${6:-}"
  local task_id run_id
  task_id="issue-$(card_issue_pad "$issue")"
  run_id="$task_id"
  local tpl tmp repo issue_url source_path docs_value source_slug
  tpl="$(resolve_input_template)"
  [[ -f "$tpl" ]] || die "missing input card template: $tpl"

  mkdir -p "$(dirname "$path")"
  tmp="$(mktemp -t prsh_input_card_XXXXXX)"
  render_template "$tpl" >"$tmp" || die "failed to render input card template: $tpl"
  ensure_nonempty_file "$tmp" || die "rendered input card is empty: $tmp"

  # Stamp fields (best-effort; keeps template generic and domain-agnostic).
  set_field_line "$tmp" "Task ID" "$task_id"
  set_field_line "$tmp" "Run ID" "$run_id"
  set_field_line "$tmp" "Version" "$ver"
  set_field_line "$tmp" "Title" "$title"
  set_field_line "$tmp" "Branch" "$branch"

  # If there is a Context Issue line, fill it with a URL.
  repo="$(default_repo)"
  if [[ -n "$repo" ]]; then
    issue_url="https://github.com/${repo}/issues/${issue}"
    replace_first_line_re "$tmp" "^- Issue:.*$" "- Issue: $issue_url"
  fi

  source_slug="$(sanitize_slug "$title")"
  source_path="$(issue_prompt_path_for_issue "$issue" "$ver" "$source_slug")"
  if [[ -f "$source_path" ]]; then
    replace_first_line_re "$tmp" "^- Source Issue Prompt:.*$" "- Source Issue Prompt: $(path_relative_to_repo "$source_path")"
  elif [[ -n "$issue_url" ]]; then
    replace_first_line_re "$tmp" "^- Source Issue Prompt:.*$" "- Source Issue Prompt: $issue_url"
  fi
  docs_value="$(docs_context_value_for_issue_prompt "$source_path")"
  replace_first_line_re "$tmp" "^- Docs:.*$" "- Docs: $docs_value"
  replace_first_line_re "$tmp" "^- Other:.*$" "- Other: none"

  if [[ -n "$output_path_actual" ]]; then
    output_path_actual="$(path_relative_to_repo "$output_path_actual")"
    replace_first_line_re "$tmp" "^- Write the output card to the paired .*" "- Write the output record to the paired local task bundle sor.md path."
    replace_first_line_re "$tmp" "^[[:space:]]*output_card: .*$" "  output_card: $output_path_actual"
  fi

  validate_card_header_count "$tmp" "# ADL Input Card" || die "generated input card must contain exactly one '# ADL Input Card' header"
  ensure_nonempty_file "$tmp" || die "generated input card is empty: $tmp"
  mv "$tmp" "$path"
}

seed_output_card() {
  local path="$1" issue="$2" title="$3" branch="$4" ver="$5"
  local task_id run_id
  task_id="issue-$(card_issue_pad "$issue")"
  run_id="$task_id"
  local out_tpl tmp
  out_tpl="$(resolve_output_template)"
  [[ -f "$out_tpl" ]] || die "missing output card template: $out_tpl"

  mkdir -p "$(dirname "$path")"
  tmp="$(mktemp -t prsh_output_card_XXXXXX)"
  render_template "$out_tpl" >"$tmp" || die "failed to render output card template: $out_tpl"
  ensure_nonempty_file "$tmp" || die "rendered output card is empty: $tmp"

  set_field_line "$tmp" "Task ID" "$task_id"
  set_field_line "$tmp" "Run ID" "$run_id"
  set_field_line "$tmp" "Version" "$ver"
  set_field_line "$tmp" "Title" "$title"
  set_field_line "$tmp" "Branch" "$branch"

  # Default Status if template left it blank.
  replace_first_line_re "$tmp" "^Status:[[:space:]]*$" "Status: NOT_STARTED | IN_PROGRESS | DONE | FAILED"
  replace_first_line_re "$tmp" "^- Integration state:.*$" "- Integration state: worktree_only"
  replace_first_line_re "$tmp" "^- Verification scope:.*$" "- Verification scope: worktree"
  validate_card_header_count "$tmp" "# ADL Output Card" || die "generated output card must contain exactly one '# ADL Output Card' header"
  ensure_nonempty_file "$tmp" || die "generated output card is empty: $tmp"
  mv "$tmp" "$path"
}

validate_bootstrap_cards() {
  local issue="$1" branch="$2" in_path="$3" out_path="$4"
  local validator expected task_id run_id in_branch out_branch
  validator="$(resolve_structured_prompt_validator)"

  "$validator" --type sip --phase bootstrap --input "$in_path" >/dev/null \
    || die "start: input card failed bootstrap validation: $in_path"
  "$validator" --type sor --phase bootstrap --input "$out_path" >/dev/null \
    || die "start: output card failed bootstrap validation: $out_path"

  expected="issue-$(card_issue_pad "$issue")"
  task_id="$(field_line_value "$in_path" "Task ID")"
  run_id="$(field_line_value "$in_path" "Run ID")"
  [[ "$task_id" == "$expected" ]] || die "start: input card Task ID mismatch (expected $expected, found ${task_id:-<empty>})"
  [[ "$run_id" == "$expected" ]] || die "start: input card Run ID mismatch (expected $expected, found ${run_id:-<empty>})"

  task_id="$(field_line_value "$out_path" "Task ID")"
  run_id="$(field_line_value "$out_path" "Run ID")"
  [[ "$task_id" == "$expected" ]] || die "start: output card Task ID mismatch (expected $expected, found ${task_id:-<empty>})"
  [[ "$run_id" == "$expected" ]] || die "start: output card Run ID mismatch (expected $expected, found ${run_id:-<empty>})"

  in_branch="$(field_line_value "$in_path" "Branch")"
  out_branch="$(field_line_value "$out_path" "Branch")"
  [[ "$in_branch" == "$branch" ]] || die "start: input card branch mismatch (expected $branch, found ${in_branch:-<empty>})"
  [[ "$out_branch" == "$branch" ]] || die "start: output card branch mismatch (expected $branch, found ${out_branch:-<empty>})"
}

validate_bootstrap_stp() {
  local path="$1"
  local validator
  validator="$(resolve_structured_prompt_validator)"
  "$validator" --type stp --input "$path" >/dev/null \
    || die "init: stp failed validation: $path"
}

seed_task_bundle_stp() {
  local source_path="$1" dest_path="$2"
  mkdir -p "$(dirname "$dest_path")"
  cp -f "$source_path" "$dest_path"
}

stp_issue_number_or_die() {
  local stp_path="$1" fm issue_num
  fm="$(mktemp -t prsh_stp_fm_XXXXXX)"
  extract_front_matter_to_file "$stp_path" "$fm"
  issue_num="$(strip_yaml_scalar_quotes "$(stp_scalar_field "$fm" "issue_number")")"
  rm -f "$fm"
  [[ "$issue_num" =~ ^[0-9]+$ ]] || die "create: STP issue_number must be an integer: $stp_path"
  printf '%s\n' "$issue_num"
}

reconcile_issue_from_stp() {
  local issue="$1" stp_path="$2" repo="$3"
  local validator fm body title
  local -a desired_labels=() current_labels=() add_labels=() remove_labels=()
  validator="$(resolve_structured_prompt_validator)"
  "$validator" --type stp --input "$stp_path" >/dev/null \
    || die "create: stp failed validation: $stp_path"

  local stp_issue
  stp_issue="$(stp_issue_number_or_die "$stp_path")"
  [[ "$stp_issue" == "$issue" ]] || die "create: STP issue_number ($stp_issue) does not match requested issue ($issue)"

  fm="$(mktemp -t prsh_create_fm_XXXXXX)"
  body="$(mktemp -t prsh_create_body_XXXXXX.md)"
  extract_front_matter_to_file "$stp_path" "$fm"
  extract_markdown_body_to_file "$stp_path" "$body"

  title="$(strip_yaml_scalar_quotes "$(stp_scalar_field "$fm" "title")")"
  [[ -n "$title" ]] || die "create: STP title is required: $stp_path"
  while IFS= read -r line; do
    desired_labels+=("$(strip_yaml_scalar_quotes "$line")")
  done < <(stp_array_items "$fm" "labels")
  while IFS= read -r line; do
    current_labels+=("$line")
  done < <(gh issue view "$issue" $(gh_repo_flag "$repo") --json labels -q '.labels[].name' 2>/dev/null || true)

  local existing desired found
  for desired in "${desired_labels[@]+"${desired_labels[@]}"}"; do
    [[ -n "$desired" ]] || continue
    found="0"
    for existing in "${current_labels[@]+"${current_labels[@]}"}"; do
      if [[ "$existing" == "$desired" ]]; then
        found="1"
        break
      fi
    done
    [[ "$found" == "1" ]] || add_labels+=("$desired")
  done

  for existing in "${current_labels[@]+"${current_labels[@]}"}"; do
    [[ -n "$existing" ]] || continue
    found="0"
    for desired in "${desired_labels[@]+"${desired_labels[@]}"}"; do
      if [[ "$desired" == "$existing" ]]; then
        found="1"
        break
      fi
    done
    [[ "$found" == "1" ]] || remove_labels+=("$existing")
  done

  gh issue edit "$issue" $(gh_repo_flag "$repo") --title "$title" --body-file "$body" >/dev/null
  for desired in "${add_labels[@]+"${add_labels[@]}"}"; do
    gh issue edit "$issue" $(gh_repo_flag "$repo") --add-label "$desired" >/dev/null
  done
  for existing in "${remove_labels[@]+"${remove_labels[@]}"}"; do
    gh issue edit "$issue" $(gh_repo_flag "$repo") --remove-label "$existing" >/dev/null
  done

  rm -f "$fm" "$body"
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

extract_markdown_section() {
  # Extract the body of a top-level markdown section (## Heading) from a file.
  local path="$1" heading="$2"
  awk -v heading="## ${heading}" '
    $0 == heading { in_section=1; next }
    in_section && /^## / { exit }
    in_section { print }
  ' "$path" | sed '/^[[:space:]]*$/{
    :a
    N
    /^\n*$/d
    ba
  }' | sed '${/^[[:space:]]*$/d;}'
}

extra_pr_body_looks_like_issue_template() {
  local body="${1:-}"
  grep -Eqi '(^|[[:space:]])(issue_card_schema:|wp:|pr_start:)([[:space:]]|$)|^## (Goal|Deliverables|Acceptance criteria)$|^---$' <<<"$body"
}

render_pr_body_file() {
  # Renders a PR body into a temp file and echoes its path.
  # Args: issue close_line input_path output_path extra_body no_checks fingerprint
  local issue="$1" close_line="$2" input_path="$3" output_path="$4" extra_body="$5" no_checks="$6" fingerprint="$7"

  local tmp
  tmp="$(mktemp -t pr_body_XXXXXX.md)"

  local input_ref output_ref summary_section artifacts_section validation_section
  input_ref="$(issue_card_reference input "$issue")"
  output_ref="$(issue_card_reference output "$issue")"
  summary_section="$(extract_markdown_section "$output_path" "Summary")"
  artifacts_section="$(extract_markdown_section "$output_path" "Artifacts produced")"
  validation_section="$(extract_markdown_section "$output_path" "Validation")"

  if [[ -n "$extra_body" ]] && extra_pr_body_looks_like_issue_template "$extra_body"; then
    die "finish: --body looks like issue-template/prompt text; use the output card as the PR summary source instead"
  fi

  {
    if [[ -n "$close_line" ]]; then
      echo "$close_line"
      echo
    fi

    if [[ -n "$summary_section" ]]; then
      echo "## Summary"
      echo "$summary_section"
      echo
    fi

    if [[ -n "$artifacts_section" ]]; then
      echo "## Artifacts"
      echo "$artifacts_section"
      echo
    fi

    if [[ -n "$validation_section" ]]; then
      echo "## Validation"
      echo "$validation_section"
      echo
    elif [[ "$no_checks" != "1" ]]; then
      echo "## Validation"
      echo "- cargo fmt"
      echo "- cargo clippy --all-targets -- -D warnings"
      echo "- cargo test"
      echo
    fi

    if [[ -n "$extra_body" ]]; then
      echo "## Notes"
      echo "$extra_body"
      echo
    fi

    echo "## Local Artifacts"
    echo "- Input card:  $input_ref"
    echo "- Output card: $output_ref"
    echo "- Idempotency-Key: $fingerprint"
  } >"$tmp"

  echo "$tmp"
}

pr_has_closing_linkage() {
  local repo="$1" pr_ref="$2" issue="$3"
  local linked body
  linked="$(gh pr view $(gh_repo_flag "$repo") "$pr_ref" --json closingIssuesReferences -q '.closingIssuesReferences[]?.number' 2>/dev/null || true)"
  if grep -Fxq "$issue" <<<"$linked"; then
    return 0
  fi

  body="$(gh pr view $(gh_repo_flag "$repo") "$pr_ref" --json body -q '.body' 2>/dev/null || true)"
  grep -Eiq "(^|[[:space:][:punct:]])Closes[[:space:]]+#${issue}([[:space:][:punct:]]|$)" <<<"$body"
}

ensure_pr_closing_linkage() {
  local repo="$1" pr_ref="$2" issue="$3" no_close="$4"
  [[ "$no_close" == "1" ]] && return 0
  if ! pr_has_closing_linkage "$repo" "$pr_ref" "$issue"; then
    die "finish: PR is missing GitHub closing linkage for issue #${issue}; ensure the PR body contains 'Closes #${issue}' and that the PR body update was applied"
  fi
}

extract_plan_value() {
  local label="$1" plan_output="$2"
  awk -v prefix="$label" '
    index($0, prefix) == 1 {
      print substr($0, length(prefix) + 1)
      exit
    }
  ' <<<"$plan_output"
}

resolve_runs_root_for_pr_run() {
  local requested="${1:-}"
  if [[ -n "$requested" ]]; then
    resolve_repo_relative_path "$requested"
    return 0
  fi
  if [[ -n "${ADL_RUNS_ROOT:-}" ]]; then
    resolve_repo_relative_path "$ADL_RUNS_ROOT"
    return 0
  fi
  echo "$(repo_root)/.adl/runs"
}

assert_run_artifact_contains() {
  local file="$1" needle="$2" context="$3"
  [[ -f "$file" ]] || die "run: missing $context artifact: $file"
  grep -Fq "$needle" "$file" || die "run: $context artifact missing expected content '$needle': $file"
}

verify_pr_run_artifacts() {
  local run_root="$1" run_id="$2" workflow_id="$3"
  local run_json run_status_json run_summary_json
  run_json="$run_root/run.json"
  run_status_json="$run_root/run_status.json"
  run_summary_json="$run_root/run_summary.json"

  [[ -f "$run_json" ]] || die "run: missing canonical run artifact: $run_json"
  [[ -f "$run_status_json" ]] || die "run: missing canonical run status artifact: $run_status_json"
  [[ -f "$run_summary_json" ]] || die "run: missing canonical run summary artifact: $run_summary_json"

  assert_run_artifact_contains "$run_json" "\"run_id\": \"$run_id\"" "run.json"
  assert_run_artifact_contains "$run_status_json" "\"run_id\": \"$run_id\"" "run_status.json"
  assert_run_artifact_contains "$run_status_json" "\"workflow_id\": \"$workflow_id\"" "run_status.json"
  assert_run_artifact_contains "$run_summary_json" "\"run_id\": \"$run_id\"" "run_summary.json"
  assert_run_artifact_contains "$run_summary_json" "\"workflow_id\": \"$workflow_id\"" "run_summary.json"
}

print_pr_run_summary() {
  local state="$1" adl_path="$2" run_id="$3" workflow_id="$4" runs_root="$5"
  local run_root run_json run_status_json run_summary_json
  run_root="$runs_root/$run_id"
  run_json="$(path_relative_to_repo "$run_root/run.json")"
  run_status_json="$(path_relative_to_repo "$run_root/run_status.json")"
  run_summary_json="$(path_relative_to_repo "$run_root/run_summary.json")"
  echo "PR RUN $state"
  echo "  adl_path=$(path_relative_to_repo "$(resolve_repo_relative_path "$adl_path")")"
  echo "  run_id=$run_id"
  echo "  workflow_id=$workflow_id"
  echo "  run_root=$(path_relative_to_repo "$run_root")"
  echo "  proof_run_json=$run_json"
  echo "  proof_run_status_json=$run_status_json"
  echo "  proof_run_summary_json=$run_summary_json"
}

cmd_run() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_run
    return 0
  fi

  local adl_path="${1:-}"
  [[ -n "$adl_path" ]] || die_with_usage "run: missing <adl.yaml>" usage_run
  shift || true

  local root adl_abs runs_root
  root="$(repo_root)"
  adl_abs="$(resolve_repo_relative_path "$adl_path")"
  [[ -f "$adl_abs" ]] || die "run: ADL file not found: $adl_path"

  local out_dir=""
  local runs_root_arg=""
  local overlay_path=""
  local resume_path=""
  local steer_path=""
  local -a forward_args
  forward_args=("$adl_abs")

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --print-plan|--print-prompts|--print-prompt|--trace|--quiet|--no-step-output|--open|--allow-unsigned)
        forward_args+=("$1")
        shift
        ;;
      --resume|--steer|--overlay|--out)
        [[ $# -ge 2 ]] || die_with_usage "run: $1 requires a value" usage_run
        if [[ "$1" == "--out" ]]; then
          out_dir="$(resolve_repo_relative_path "$2")"
        fi
        if [[ "$1" == "--overlay" ]]; then
          overlay_path="$(resolve_repo_relative_path "$2")"
        fi
        if [[ "$1" == "--resume" ]]; then
          resume_path="$(resolve_repo_relative_path "$2")"
        fi
        if [[ "$1" == "--steer" ]]; then
          steer_path="$(resolve_repo_relative_path "$2")"
        fi
        case "$1" in
          --out) forward_args+=("$1" "$out_dir") ;;
          --overlay) forward_args+=("$1" "$overlay_path") ;;
          --resume) forward_args+=("$1" "$resume_path") ;;
          --steer) forward_args+=("$1" "$steer_path") ;;
        esac
        shift 2
        ;;
      --runs-root)
        [[ $# -ge 2 ]] || die_with_usage "run: --runs-root requires a value" usage_run
        runs_root_arg="$2"
        shift 2
        ;;
      -h|--help)
        usage_run
        return 0
        ;;
      *)
        die_with_usage "run: unknown arg: $1" usage_run
        ;;
    esac
  done

  runs_root="$(resolve_runs_root_for_pr_run "$runs_root_arg")"
  mkdir -p "$runs_root"
  if [[ -n "$out_dir" ]]; then
    mkdir -p "$(resolve_repo_relative_path "$out_dir")"
  fi

  local -a plan_args
  plan_args=("$adl_abs")
  if [[ -n "$overlay_path" ]]; then
    plan_args+=("--overlay" "$overlay_path")
  fi

  local plan_output run_id workflow_id
  plan_output="$(
    cd "$root/adl" &&
      cargo run --quiet --bin adl -- "${plan_args[@]}" --print-plan
  )" || die "run: failed to resolve ADL execution plan for $adl_path"

  run_id="$(extract_plan_value "Resolved run: " "$plan_output")"
  workflow_id="$(extract_plan_value "Workflow:     " "$plan_output")"
  [[ -n "$run_id" ]] || die "run: failed to derive run_id from resolved plan"
  [[ -n "$workflow_id" ]] || die "run: failed to derive workflow_id from resolved plan"

  local -a exec_args
  exec_args=("${forward_args[@]}" "--run")

  local normalized_ollama_bin=""
  if [[ -n "${ADL_OLLAMA_BIN:-}" ]]; then
    normalized_ollama_bin="$(resolve_repo_relative_path "$ADL_OLLAMA_BIN")"
  fi

  local run_output run_status
  set +e
  if [[ -n "$normalized_ollama_bin" ]]; then
    run_output="$(
      cd "$root/adl" &&
        ADL_OLLAMA_BIN="$normalized_ollama_bin" ADL_RUNS_ROOT="$runs_root" cargo run --quiet --bin adl -- "${exec_args[@]}" 2>&1
    )"
    run_status=$?
  else
    run_output="$(
      cd "$root/adl" &&
        ADL_RUNS_ROOT="$runs_root" cargo run --quiet --bin adl -- "${exec_args[@]}" 2>&1
    )"
    run_status=$?
  fi
  set -e

  [[ -n "$run_output" ]] && printf '%s\n' "$run_output"

  local run_root
  run_root="$runs_root/$run_id"
  verify_pr_run_artifacts "$run_root" "$run_id" "$workflow_id"

  if [[ "$run_status" -eq 0 ]]; then
    print_pr_run_summary "ok" "$adl_path" "$run_id" "$workflow_id" "$runs_root"
    return 0
  fi

  print_pr_run_summary "failed" "$adl_path" "$run_id" "$workflow_id" "$runs_root" >&2
  return "$run_status"
}

sha256_file() {
  local path="$1"
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$path" | awk '{print $1}'
    return 0
  fi
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$path" | awk '{print $1}'
    return 0
  fi
  die "Missing hash command: need shasum or sha256sum"
}

finish_inputs_fingerprint() {
  local title="$1" paths="$2" input_path="$3" output_path="$4"
  local in_hash out_hash
  in_hash="$(sha256_file "$input_path")"
  out_hash="$(sha256_file "$output_path")"
  local payload
  payload="title=$title|paths=$paths|input=$in_hash|output=$out_hash"
  if command -v shasum >/dev/null 2>&1; then
    printf '%s' "$payload" | shasum -a 256 | awk '{print $1}'
    return 0
  fi
  printf '%s' "$payload" | sha256sum | awk '{print $1}'
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
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_card
    return 0
  fi

  local issue="${1:-}"; shift || true
  [[ -n "$issue" ]] || die_with_usage "card: missing <issue> number" usage_card
  issue="$(normalize_issue_or_die "$issue")"

  local slug=""
  local no_fetch_issue="0"
  local out_path=""
  local version=""
  local kind="create"
  local seen_kind="0"

  while [[ $# -gt 0 ]]; do
    case "$1" in
      input|output)
        if [[ "$seen_kind" == "1" ]]; then
          die_with_usage "card: duplicate positional card kind: $1" usage_card
        fi
        kind="$1"
        seen_kind="1"
        shift
        ;;
      --slug) slug="$2"; shift 2 ;;
      --no-fetch-issue) no_fetch_issue="1"; shift ;;
      -f) out_path="$2"; shift 2 ;;
      --file) out_path="$2"; shift 2 ;;
      --version) version="$2"; shift 2 ;;
      -h|--help) usage_card; return 0 ;;
      *) die_with_usage "card: unknown arg: $1" usage_card ;;
    esac
  done

  local target_kind
  target_kind="$kind"
  if [[ "$target_kind" == "create" ]]; then
    target_kind="input"
  fi

  if [[ "$kind" != "create" ]]; then
    local quick_path
    if [[ -n "$out_path" ]]; then
      quick_path="$out_path"
    elif [[ "$target_kind" == "output" ]]; then
      quick_path="$(output_card_path "$issue")"
    else
      quick_path="$(input_card_path "$issue")"
    fi
    if ensure_nonempty_file "$quick_path"; then
      echo "$quick_path"
      return 0
    fi
  fi

  if [[ "$target_kind" == "output" ]]; then
    local out_target
    out_target="${out_path:-$(output_card_path "$issue")}"
    if ! ensure_nonempty_file "$out_target"; then
      local -a create_args
      create_args=("$issue")
      if [[ -n "$slug" ]]; then
        create_args+=(--slug "$slug")
      fi
      if [[ "$no_fetch_issue" == "1" ]]; then
        create_args+=(--no-fetch-issue)
      fi
      if [[ -n "$version" ]]; then
        create_args+=(--version "$version")
      fi
      if [[ -n "$out_path" ]]; then
        create_args+=(--file "$out_path")
      fi
      cmd_output "${create_args[@]}" >/dev/null
    fi
    echo "$out_target"
    return 0
  fi

  local repo
  repo="$(default_repo)"

  if [[ "$no_fetch_issue" != "1" ]]; then
    require_cmd gh
  fi

  local title=""
  if [[ "$no_fetch_issue" != "1" ]]; then
    require_cmd gh
    note "Fetching issue title via gh…"
    title="$(gh issue view "$issue" $(gh_repo_flag "$repo") --json title -q .title 2>/dev/null || true)"
  fi

  if [[ -z "$slug" ]]; then
    if [[ -n "$title" ]]; then
      slug="$(sanitize_slug "$title")"
    elif [[ "$kind" != "create" ]]; then
      slug="issue-${issue}"
    else
      die "card: --slug is required when --no-fetch-issue is set or issue title could not be fetched"
    fi
  fi

  if [[ -z "$title" ]]; then
    title="$slug"
  fi

  if [[ -z "$version" ]]; then
    if [[ "$no_fetch_issue" == "1" ]]; then
      version="$DEFAULT_VERSION"
    else
      version="$(issue_version "$issue")"
    fi
  fi
  if [[ -z "$out_path" ]]; then
    out_path="$(input_card_path "$issue" "$version" "$slug")"
  fi
  if ensure_nonempty_file "$out_path"; then
    if [[ "$kind" == "input" ]]; then
      echo "$out_path"
      return 0
    fi
    die "card: input card already exists: $out_path"
  elif [[ -e "$out_path" ]]; then
    note "Input card exists but is empty; recreating: $out_path"
  fi
  note "Creating input card: $out_path"
  ensure_adl_dirs
  seed_input_card "$out_path" "$issue" "$title" "$(current_branch)" "$version" "$(output_card_path "$issue" "$version" "$slug")"
  sync_legacy_links_for_issue "$issue" "$version" "$slug"
  note "Done."
  echo "$out_path"
}

cmd_output() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_output
    return 0
  fi

  local issue="${1:-}"; shift || true
  [[ -n "$issue" ]] || die_with_usage "output: missing <issue> number" usage_output
  issue="$(normalize_issue_or_die "$issue")"

  local slug=""
  local no_fetch_issue="0"
  local out_path=""
  local version=""
  local kind="create"
  local seen_kind="0"

  while [[ $# -gt 0 ]]; do
    case "$1" in
      input|output)
        if [[ "$seen_kind" == "1" ]]; then
          die_with_usage "output: duplicate positional card kind: $1" usage_output
        fi
        kind="$1"
        seen_kind="1"
        shift
        ;;
      --slug) slug="$2"; shift 2 ;;
      --no-fetch-issue) no_fetch_issue="1"; shift ;;
      -f) out_path="$2"; shift 2 ;;
      --file) out_path="$2"; shift 2 ;;
      --version) version="$2"; shift 2 ;;
      -h|--help) usage_output; return 0 ;;
      *) die_with_usage "output: unknown arg: $1" usage_output ;;
    esac
  done

  local target_kind
  target_kind="$kind"
  if [[ "$target_kind" == "create" ]]; then
    target_kind="output"
  fi

  if [[ "$kind" != "create" ]]; then
    local quick_path
    if [[ -n "$out_path" ]]; then
      quick_path="$out_path"
    elif [[ "$target_kind" == "input" ]]; then
      quick_path="$(input_card_path "$issue" "${version:-}" "${slug:-}")"
    else
      quick_path="$(output_card_path "$issue" "${version:-}" "${slug:-}")"
    fi
    if ensure_nonempty_file "$quick_path"; then
      echo "$quick_path"
      return 0
    fi
  fi

  if [[ "$target_kind" == "input" ]]; then
    local input_target
    input_target="${out_path:-$(input_card_path "$issue" "${version:-$DEFAULT_VERSION}" "${slug:-issue-$issue}")}"
    if ! ensure_nonempty_file "$input_target"; then
      local -a create_args
      create_args=("$issue")
      if [[ -n "$slug" ]]; then
        create_args+=(--slug "$slug")
      fi
      if [[ "$no_fetch_issue" == "1" ]]; then
        create_args+=(--no-fetch-issue)
      fi
      if [[ -n "$version" ]]; then
        create_args+=(--version "$version")
      fi
      if [[ -n "$out_path" ]]; then
        create_args+=(--file "$out_path")
      fi
      cmd_card "${create_args[@]}" >/dev/null
    fi
    echo "$input_target"
    return 0
  fi

  local repo
  repo="$(default_repo)"

  if [[ "$no_fetch_issue" != "1" ]]; then
    require_cmd gh
  fi

  local title=""
  if [[ "$no_fetch_issue" != "1" ]]; then
    require_cmd gh
    note "Fetching issue title via gh…"
    title="$(gh issue view "$issue" $(gh_repo_flag "$repo") --json title -q .title 2>/dev/null || true)"
  fi

  if [[ -z "$slug" ]]; then
    if [[ -n "$title" ]]; then
      slug="$(sanitize_slug "$title")"
    elif [[ "$kind" != "create" ]]; then
      slug="issue-${issue}"
    else
      die "output: --slug is required when --no-fetch-issue is set or issue title could not be fetched"
    fi
  fi

  if [[ -z "$title" ]]; then
    title="$slug"
  fi

  if [[ -z "$version" ]]; then
    if [[ "$no_fetch_issue" == "1" ]]; then
      version="$DEFAULT_VERSION"
    else
      version="$(issue_version "$issue")"
    fi
  fi
  if [[ -z "$out_path" ]]; then
    out_path="$(output_card_path "$issue" "$version" "$slug")"
  fi
  if ensure_nonempty_file "$out_path"; then
    if [[ "$kind" == "output" ]]; then
      echo "$out_path"
      return 0
    fi
    die "output: output card already exists: $out_path"
  elif [[ -e "$out_path" ]]; then
    note "Output card exists but is empty; recreating: $out_path"
  fi
  note "Creating output card: $out_path"
  ensure_adl_dirs
  seed_output_card "$out_path" "$issue" "$title" "$(current_branch)" "$version"
  sync_legacy_links_for_issue "$issue" "$version" "$slug"
  note "Done."
  echo "$out_path"
}

cmd_cards() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_cards
    return 0
  fi

  local issue="${1:-}"; shift || true
  [[ -n "$issue" ]] || die_with_usage "cards: missing <issue> number" usage_cards
  issue="$(normalize_issue_or_die "$issue")"

  local no_fetch_issue="0"
  local version=""

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --no-fetch-issue) no_fetch_issue="1"; shift ;;
      --version) version="$2"; shift 2 ;;
      -h|--help) usage_cards; return 0 ;;
      *) die_with_usage "cards: unknown arg: $1" usage_cards ;;
    esac
  done

  local lock_dir=""
  lock_dir="$(acquire_repo_lock "pr-bootstrap")"
  trap "release_repo_lock '$lock_dir'" RETURN EXIT

  local repo
  repo="$(default_repo)"

  if [[ "$no_fetch_issue" != "1" ]]; then
    require_cmd gh
  fi

  local title=""
  if [[ "$no_fetch_issue" != "1" ]]; then
    require_cmd gh
    note "Fetching issue title via gh…"
    title="$(gh issue view "$issue" $(gh_repo_flag "$repo") --json title -q .title 2>/dev/null || true)"
  fi

  if [[ -z "$title" ]]; then
    title="issue-${issue}"
  fi

  if [[ -z "$version" ]]; then
    if [[ "$no_fetch_issue" == "1" ]]; then
      version="$DEFAULT_VERSION"
    else
      version="$(issue_version "$issue")"
    fi
  fi
  [[ -n "$version" ]] || version="v0.2"

  ensure_adl_dirs

  local input_path output_path cards_slug
  cards_slug="$(sanitize_slug "$title")"
  input_path="$(input_card_path "$issue" "$version" "$cards_slug")"
  output_path="$(output_card_path "$issue" "$version" "$cards_slug")"

  if ensure_nonempty_file "$input_path"; then
    note "Input card exists: $input_path"
  else
    note "Creating input card: $input_path"
    seed_input_card "$input_path" "$issue" "$title" "TBD (run pr.sh start $issue)" "$version" "$output_path"
  fi

  if ensure_nonempty_file "$output_path"; then
    note "Output card exists: $output_path"
  else
    note "Creating output card: $output_path"
    seed_output_card "$output_path" "$issue" "$title" "TBD (run pr.sh start $issue)" "$version"
  fi

  sync_legacy_links_for_issue "$issue" "$version" "$cards_slug"

  echo "READ  $input_path"
  echo "WRITE $output_path"
  echo "STATE=ISSUE_AND_CARDS_READY"
}

cmd_init() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_init
    return 0
  fi

  require_cmd git
  local lock_dir=""
  lock_dir="$(acquire_repo_lock "pr-bootstrap")"
  trap "release_repo_lock '$lock_dir'" RETURN EXIT
  local issue="${1:-}"; shift || true
  [[ -n "$issue" ]] || die_with_usage "init: missing <issue> number" usage_init
  issue="$(normalize_issue_or_die "$issue")"

  local slug=""
  local no_fetch_issue="0"
  local title=""
  local title_arg=""
  local version=""

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --slug) slug="$2"; shift 2 ;;
      --title) title_arg="$2"; shift 2 ;;
      --no-fetch-issue) no_fetch_issue="1"; shift ;;
      --version) version="$2"; shift 2 ;;
      -h|--help) usage_init; return 0 ;;
      *) die_with_usage "init: unknown arg: $1" usage_init ;;
    esac
  done

  local repo
  repo="$(default_repo)"
  if [[ -z "$slug" && -n "$title_arg" ]]; then
    slug="$(sanitize_slug "$title_arg")"
    [[ -n "$slug" ]] || die "init: --title produced empty slug after sanitization"
    title="$title_arg"
  fi
  if [[ -z "$title" && "$no_fetch_issue" != "1" ]]; then
    require_cmd gh
    note "Fetching issue title via gh…"
    title="$(gh issue view "$issue" $(gh_repo_flag "$repo") --json title -q .title 2>/dev/null || true)"
  fi
  if [[ -z "$slug" ]]; then
    if [[ "$no_fetch_issue" == "1" ]]; then
      die "init: --slug is required when --no-fetch-issue is set"
    fi
    [[ -n "$title" ]] || die "Could not fetch issue #$issue title. Pass --slug or check gh auth/repo."
    slug="$(sanitize_slug "$title")"
  fi
  if [[ -z "$title" ]]; then
    title="$slug"
  fi

  if [[ -z "$version" ]]; then
    if [[ "$no_fetch_issue" == "1" ]]; then
      version="$DEFAULT_VERSION"
    else
      version="$(issue_version "$issue")"
    fi
  fi
  [[ -n "$version" ]] || die "init: version must be non-empty"

  local source_path bundle_dir stp_path
  source_path="$(issue_prompt_path_for_issue "$issue" "$version" "$slug")"
  bundle_dir="$(task_bundle_dir_path "$issue" "$version" "$slug")"
  stp_path="$bundle_dir/stp.md"

  if [[ ! -f "$source_path" ]]; then
    note "Source issue prompt missing; generating canonical local issue prompt: $source_path"
    source_path="$(ensure_source_issue_prompt "$issue" "$version" "$slug" "$title")"
  fi
  validate_bootstrap_stp "$source_path"

  if ensure_nonempty_file "$stp_path"; then
    note "STP already exists: $stp_path"
    validate_bootstrap_stp "$stp_path"
  else
    note "Initializing task bundle: $bundle_dir"
    seed_task_bundle_stp "$source_path" "$stp_path"
    validate_bootstrap_stp "$stp_path"
  fi

  if [[ -e "$bundle_dir/sip.md" || -e "$bundle_dir/sor.md" ]]; then
    note "SIP/SOR already exist; pr init leaves them untouched."
  fi

  echo "• Initialized:"
  echo "  STP      $(path_relative_to_repo "$stp_path")"
  echo "  BUNDLE   $(path_relative_to_repo "$bundle_dir")"
  echo "  SOURCE   $(path_relative_to_repo "$source_path")"
  echo "  CONTRACT minimum v0.85 init = task-bundle directory + validated stp.md only"
  echo "  STATE    ISSUE_AND_STP_READY"
  note "Done."
}

cmd_create() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_create
    return 0
  fi

  if [[ "${1:-}" =~ ^[0-9]+$ ]]; then
    local issue="$1"
    shift || true
    issue="$(normalize_issue_or_die "$issue")"

    local stp_path=""
    while [[ $# -gt 0 ]]; do
      case "$1" in
        --stp) stp_path="$2"; shift 2 ;;
        -h|--help) usage_create; return 0 ;;
        *) die_with_usage "create: unknown arg: $1" usage_create ;;
      esac
    done

    [[ -n "$stp_path" ]] || die "create: --stp is required for reconcile mode"
    [[ -f "$stp_path" ]] || die "create: STP not found: $stp_path"
    require_cmd gh
    local repo
    repo="$(default_repo)"
    reconcile_issue_from_stp "$issue" "$stp_path" "$repo"
    echo "ISSUE_NUM=$issue"
    echo "STP_PATH=$(path_relative_to_repo "$stp_path")"
    echo "MODE=reconcile"
    return 0
  fi

  create_issue "$@"
}

cmd_start() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_start
    return 0
  fi

  require_cmd git
  local lock_dir=""
  lock_dir="$(acquire_repo_lock "pr-bootstrap")"
  trap "release_repo_lock '$lock_dir'" RETURN EXIT
  local issue="${1:-}"; shift || true
  [[ -n "$issue" ]] || die_with_usage "start: missing <issue> number" usage_start
  issue="$(normalize_issue_or_die "$issue")"

  local prefix="codex"
  local slug=""
  local no_fetch_issue="0"
  local title=""
  local title_arg=""
  local version=""
  local branch_preexisting="0"

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --prefix) prefix="$2"; shift 2 ;;
      --slug) slug="$2"; shift 2 ;;
      --title) title_arg="$2"; shift 2 ;;
      --version) version="$2"; shift 2 ;;
      --no-fetch-issue) no_fetch_issue="1"; shift ;;
      -h|--help) usage_start; return 0 ;;
      *) die_with_usage "start: unknown arg: $1" usage_start ;;
    esac
  done

  # Default slug: derive from issue title if possible.
  local repo
  repo="$(default_repo)"
  title=""
  if [[ -z "$slug" && -n "$title_arg" ]]; then
    # Accept --title on start for CLI symmetry with finish/new and derive a stable slug.
    slug="$(sanitize_slug "$title_arg")"
    [[ -n "$slug" ]] || die "start: --title produced empty slug after sanitization"
    title="$title_arg"
  fi
  if [[ -z "$title" && "$no_fetch_issue" != "1" ]]; then
    require_cmd gh
    note "Fetching issue title via gh…"
    title="$(gh issue view "$issue" $(gh_repo_flag "$repo") --json title -q .title 2>/dev/null || true)"
  fi
  if [[ -z "$slug" ]]; then
    if [[ "$no_fetch_issue" == "1" ]]; then
      die "start: --slug is required when --no-fetch-issue is set"
    fi
    [[ -n "$title" ]] || die "Could not fetch issue #$issue title. Pass --slug or check gh auth/repo."
    slug="$(sanitize_slug "$title")"
  fi
  if [[ -z "$title" ]]; then
    title="$slug"
  fi

  local branch
  branch="$(branch_for_issue "$prefix" "$issue" "$slug")"
  local worktree_path
  worktree_path="$(default_worktree_path_for_issue "$issue")"

  note "Target branch: $branch"
  note "Target worktree: $worktree_path"

  note "Fetching origin/main…"
  local fetch_out="" fetch_status=0
  set +e
  fetch_out="$(git fetch origin main 2>&1)"
  fetch_status=$?
  set -e
  if [[ "$fetch_status" -ne 0 ]]; then
    if [[ "$fetch_out" == *".git/index.lock"* ]]; then
      die_index_lock "start: fetch origin main"
    fi
    if git rev-parse --verify --quiet origin/main >/dev/null; then
      note "Warning: start: fetch origin main failed; reusing existing local origin/main"
      [[ -n "$fetch_out" ]] && note "$fetch_out"
    else
      [[ -n "$fetch_out" ]] && echo "$fetch_out" >&2
      die "start: fetch origin main failed and origin/main is unavailable locally"
    fi
  fi
  if ! git rev-parse --verify --quiet origin/main >/dev/null; then
    die "start: origin/main not available; verify remote setup and permissions"
  fi

  # Ensure local branch exists (without switching the caller to it).
  if git show-ref --verify --quiet "refs/heads/$branch"; then
    branch_preexisting="1"
    note "Local branch exists; reusing: $branch"
  elif git ls-remote --exit-code --heads origin "$branch" >/dev/null 2>&1; then
    note "Branch exists on origin; creating local tracking branch…"
    run_git_or_die "start: create local branch '$branch' from origin/$branch" git branch --track "$branch" "origin/$branch"
  else
    note "Creating local branch from origin/main…"
    run_git_or_die "start: create branch '$branch' from origin/main" git branch "$branch" origin/main
  fi

  if [[ "$branch_preexisting" == "1" ]]; then
    warn_branch_upstream_not_origin_main "$branch"
  fi

  local branch_worktree
  branch_worktree="$(branch_checked_out_worktree_path "$branch" || true)"
  if [[ -n "$branch_worktree" ]]; then
    if [[ "$branch_worktree" != "$worktree_path" ]]; then
      die "start: branch '$branch' is already checked out in worktree '$branch_worktree'. Remediation: run commands there or remove it with 'git worktree remove \"$branch_worktree\"'."
    fi
    note "Reusing existing worktree for branch: $worktree_path"
  else
    ensure_worktree_path_usable "$worktree_path" "$branch"
    if [[ -e "$worktree_path" ]]; then
      note "Reusing existing worktree path: $worktree_path"
    else
      note "Creating worktree: $worktree_path"
      mkdir -p "$(dirname "$worktree_path")"
      run_git_or_die "start: git worktree add '$worktree_path' '$branch'" git worktree add "$worktree_path" "$branch"
    fi
  fi

  ensure_primary_checkout_on_main

  local ver in_path out_path source_path
  if [[ -n "$version" ]]; then
    ver="$version"
  elif [[ "$no_fetch_issue" == "1" ]]; then
    ver="$DEFAULT_VERSION"
  else
    require_cmd gh
    ver="$(issue_version "$issue")"
  fi

  source_path="$(issue_prompt_path_for_issue "$issue" "$ver" "$slug")"
  if [[ ! -f "$source_path" ]]; then
    if [[ "$no_fetch_issue" == "1" ]]; then
      note "Source issue prompt missing; generating offline canonical local issue prompt: $source_path"
      write_generated_issue_prompt "$source_path" "$issue" "$ver" "$slug" "$title" "version:${ver}" "https://github.com/$(default_repo)/issues/${issue}"
    fi
    if [[ ! -f "$source_path" ]]; then
      note "Source issue prompt missing; generating canonical local issue prompt: $source_path"
      source_path="$(ensure_source_issue_prompt "$issue" "$ver" "$slug" "$title")"
    fi
  fi
  validate_bootstrap_stp "$source_path"

  local start_paths_file
  start_paths_file="$(mktemp -t prsh_start_paths_XXXXXX)"
  (
    cd "$worktree_path"
    local bundle_dir stp_path
    bundle_dir="$(task_bundle_dir_path "$issue" "$ver" "$slug")"
    stp_path="$bundle_dir/stp.md"
    mkdir -p "$bundle_dir"
    if ! ensure_nonempty_file "$stp_path"; then
      note "Creating task-bundle STP: $stp_path"
      cp "$source_path" "$stp_path"
    else
      note "Task-bundle STP exists: $stp_path"
    fi
    in_path="$(input_card_path "$issue" "$ver" "$slug")"
    out_path="$(output_card_path "$issue" "$ver" "$slug")"
    ensure_adl_dirs
    if ! ensure_nonempty_file "$in_path"; then
      note "Creating input card: $in_path"
      seed_input_card "$in_path" "$issue" "$title" "$branch" "$ver" "$out_path"
    else
      note "Input card exists: $in_path"
    fi
    if ! ensure_nonempty_file "$out_path"; then
      note "Creating output card: $out_path"
      seed_output_card "$out_path" "$issue" "$title" "$branch" "$ver"
    else
      note "Output card exists: $out_path"
    fi
    sync_legacy_links_for_issue "$issue" "$ver" "$slug"
    validate_bootstrap_stp "$stp_path"
    validate_bootstrap_cards "$issue" "$branch" "$in_path" "$out_path"
    printf '%s\n%s\n%s\n' "$stp_path" "$in_path" "$out_path" >"$start_paths_file"
  )
  local stp_path
  stp_path="$(sed -n '1p' "$start_paths_file")"
  in_path="$(sed -n '2p' "$start_paths_file")"
  out_path="$(sed -n '3p' "$start_paths_file")"
  rm -f "$start_paths_file"
  echo "• Agent:"
  echo "  STP    $stp_path"
  echo "  READ   $in_path"
  echo "  WRITE  $out_path"
  echo "  WORKTREE $worktree_path"
  echo "  BRANCH $branch"
  echo "  OPEN   ./adl/tools/open_artifact.sh card $issue output"
  echo "  STATE  FULLY_STARTED"
  note "Done."
}

create_issue() {
  require_cmd gh

  local title=""
  local slug=""
  local body=""
  local body_file=""
  local labels="$DEFAULT_NEW_LABELS"
  local version=""
  local no_start="0"

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --title) title="$2"; shift 2 ;;
      --slug) slug="$2"; shift 2 ;;
      --body) body="$2"; shift 2 ;;
      --body-file) body_file="$2"; shift 2 ;;
      --labels) labels="$2"; shift 2 ;;
      --version) version="$2"; shift 2 ;;
      --no-start) no_start="1"; shift ;;
      -h|--help) usage_new; return 0 ;;
      *) die_with_usage "new: unknown arg: $1" usage_new ;;
    esac
  done

  [[ -n "$title" ]] || die_with_usage "new: --title is required" usage_new

  if [[ -n "$body" && -n "$body_file" ]]; then
    die "new: pass only one of --body or --body-file"
  fi
  if [[ -n "$body_file" && "$body_file" != "-" && ! -f "$body_file" ]]; then
    die "new: --body-file not found: $body_file"
  fi

  if [[ -z "$slug" ]]; then
    slug="$(sanitize_slug "$title")"
  else
    slug="$(sanitize_slug "$slug")"
  fi
  [[ -n "$slug" ]] || die "new: slug is empty after sanitization"

  local issue_body
  if [[ -n "$body_file" ]]; then
    if [[ "$body_file" == "-" ]]; then
      issue_body="$(cat)"
    else
      issue_body="$(cat "$body_file")"
    fi
  elif [[ -n "$body" ]]; then
    issue_body="$body"
  else
    issue_body=$'## Goal\n-\n\n## Acceptance Criteria\n-'
  fi

  if absolute_host_path_present <(printf '%s\n' "$issue_body"); then
    die "new: issue body contains disallowed absolute host path"
  fi

  if [[ -z "$version" ]]; then
    version="$(version_from_labels_csv "$labels" || true)"
  fi
  if [[ -z "$version" ]]; then
    version="$(version_from_title "$title" || true)"
  fi
  if [[ -z "$version" ]]; then
    version="$DEFAULT_VERSION"
  fi
  [[ -n "$version" ]] || die "new: resolved version must be non-empty"

  local labels_csv normalized_labels label
  labels_csv="$labels"
  normalized_labels=""
  IFS=',' read -r -a label_arr <<< "$labels_csv"
  for label in "${label_arr[@]}"; do
    label="$(trim_ws "$label")"
    [[ -n "$label" ]] || continue
    [[ "$label" == version:* ]] && continue
    if [[ -z "$normalized_labels" ]]; then
      normalized_labels="$label"
    else
      normalized_labels="${normalized_labels},${label}"
    fi
  done
  labels_csv="$normalized_labels"
  if [[ -n "$version" ]]; then
    if [[ -n "$labels_csv" ]]; then
      labels_csv="${labels_csv},version:${version}"
    else
      labels_csv="version:${version}"
    fi
  fi

  local -a gh_args
  gh_args=(issue create --title "$title" --body "$issue_body")
  IFS=',' read -r -a label_arr <<< "$labels_csv"
  for label in "${label_arr[@]}"; do
    label="$(trim_ws "$label")"
    [[ -n "$label" ]] || continue
    gh_args+=(--label "$label")
  done

  local issue_url
  issue_url="$(gh "${gh_args[@]}")"
  [[ -n "$issue_url" ]] || die "new: gh issue create returned empty output"
  local issue_num
  issue_num="${issue_url##*/}"
  [[ "$issue_num" =~ ^[0-9]+$ ]] || die "new: failed to parse issue number from URL: $issue_url"

  echo "ISSUE_URL=$issue_url"
  echo "ISSUE_NUM=$issue_num"
  echo "STATE=ISSUE_CREATED"

  local source_path
  source_path="$(ensure_source_issue_prompt "$issue_num" "$version" "$slug" "$title" "$labels_csv")"
  echo "SOURCE_PATH=$(path_relative_to_repo "$source_path")"

  if [[ "$no_start" == "1" ]]; then
    echo "START_STATE=SKIPPED"
    return 0
  fi

  if ! cmd_start "$issue_num" --slug "$slug" --title "$title" --version "$version"; then
    echo "START_STATE=FAILED"
    die "create: issue created but start failed; issue #$issue_num exists and source prompt is at $(path_relative_to_repo "$source_path")"
  fi
  echo "START_STATE=STARTED"
  echo "BRANCH=codex/${issue_num}-${slug}"
}

cmd_new() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_new
    return 0
  fi
  die "new: retired; use 'adl/tools/pr.sh create --title ...' instead"
}

cmd_finish() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_finish
    return 0
  fi

  require_cmd git
  require_cmd gh

  local issue="${1:-}"; shift || true
  [[ -n "$issue" ]] || die_with_usage "finish: missing <issue> number" usage_finish
  issue="$(normalize_issue_or_die "$issue")"

  local title=""
  local extra_body=""
  local no_checks="0"
  local no_close="0"
  local ready="0"
  local allow_gitignore="0"
  local paths="."
  local input_path=""
  local output_path=""
  local no_open="0"
  local merge_mode="0"
  local idempotent="0"

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
      --output|--output-card|--output-card-file) output_path="$2"; shift 2 ;;
      --no-open) no_open="1"; shift ;;
      --open) no_open="0"; shift ;;
      --merge|--auto-merge) merge_mode="1"; shift ;;
      --idempotent) idempotent="1"; shift ;;
      -h|--help) usage_finish; return 0 ;;
      *) die_with_usage "finish: unknown arg: $1" usage_finish ;;
    esac
  done

  [[ -n "$title" ]] || die_with_usage "finish: --title is required" usage_finish
  if [[ "$merge_mode" == "1" && "$no_checks" == "1" ]]; then
    die "finish: --merge requires checks; remove --no-checks"
  fi

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
    input_path="$(resolve_input_card_path_abs "$issue" "$ver")"
  fi
  if [[ -z "$output_path" ]]; then
    output_path="$(resolve_output_card_path_abs "$issue" "$ver")"
  fi
  [[ -f "$input_path" ]] || die "finish: missing input card: $input_path"
  if ! ensure_nonempty_file "$output_path"; then
    if [[ ! -f "$output_path" ]]; then
      die "finish: missing output card: $output_path"
    fi
    die "finish: output card is empty: $output_path"
  fi

  local structured_prompt_validator
  structured_prompt_validator="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/validate_structured_prompt.sh"
  if ! bash "$structured_prompt_validator" --type sor --phase completed --input "$output_path" >/dev/null 2>&1; then
    die "finish: output card failed completed-phase validation: $output_path"
  fi

  # Safety: allow finish to proceed if there are commits ahead of origin/main
  # even when the user already committed manually (working tree clean).
  local ahead
  ahead="$(commits_ahead_of_main)"

  local has_uncommitted="0"
  if ! git diff --quiet || ! git diff --cached --quiet; then
    has_uncommitted="1"
  fi

  local branch
  branch="$(current_branch)"

  local repo
  repo="$(default_repo)"
  local fingerprint
  fingerprint="$(finish_inputs_fingerprint "$title" "$paths" "$input_path" "$output_path")"

  if [[ "$has_uncommitted" == "0" && "$ahead" -eq 0 ]]; then
    if [[ "$idempotent" == "1" ]]; then
      local pr_url pr_state pr_title pr_body
      pr_url="$(current_pr_url "$repo" "$branch")"
      if [[ -z "$pr_url" ]]; then
        die "finish: --idempotent requested but no PR exists for this branch"
      fi
      pr_state="$(gh pr view $(gh_repo_flag "$repo") "$pr_url" --json state -q .state 2>/dev/null || true)"
      pr_title="$(gh pr view $(gh_repo_flag "$repo") "$pr_url" --json title -q .title 2>/dev/null || true)"
      pr_body="$(gh pr view $(gh_repo_flag "$repo") "$pr_url" --json body -q .body 2>/dev/null || true)"
      [[ "$pr_state" == "MERGED" ]] || die "finish: --idempotent only skips for merged PRs; current state=$pr_state"
      [[ "$pr_title" == "$title" ]] || die "finish: --idempotent detected changed title; refusing skip"
      [[ "$pr_body" == *"Idempotency-Key: $fingerprint"* ]] || die "finish: --idempotent fingerprint mismatch; refusing skip"
      note "Idempotent skip: merged PR already matches current finish inputs."
      return 0
    fi
    die "No changes detected and branch has no commits ahead of origin/main. Nothing to PR. If you already merged, switch branches."
  fi

  if [[ "$has_uncommitted" == "0" && "$ahead" -gt 0 ]]; then
    note "No uncommitted changes; will create/update PR using existing commits (ahead of origin/main by ${ahead})."
  fi

  if [[ "$no_checks" != "1" ]]; then
    run_batched_checks
  else
    note "Skipping checks (--no-checks)"
  fi

  if [[ "$has_uncommitted" == "1" ]]; then
    # Stage selected paths (default: repo root). Use --paths "adl,docs" to narrow scope explicitly.
    IFS=',' read -r -a path_arr <<< "$paths"
    if [[ ${#path_arr[@]} -eq 0 ]]; then
      die "finish: --paths resolved to empty; pass e.g. --paths \"adl,docs\""
    fi

    note "Staging changes (${paths})…"
    stage_selected_paths "${path_arr[@]}"

    if git diff --cached --quiet; then
      die "finish: nothing staged after 'git add' for paths '${paths}'. Your paths may be empty/ignored or there were no changes. Either change --paths or commit manually and re-run finish."
    fi

    if [[ "$allow_gitignore" != "1" ]]; then
      if ! git diff --cached --quiet -- .gitignore adl/.gitignore 2>/dev/null; then
        die "finish: .gitignore changes detected. Revert them or re-run with --allow-gitignore."
      fi
    fi
  fi

  local close_line=""
  if [[ "$no_close" != "1" ]]; then
    close_line="Closes #${issue}"
  fi

  local pr_body_file
  pr_body_file="$(render_pr_body_file "$issue" "$close_line" "$input_path" "$output_path" "$extra_body" "$no_checks" "$fingerprint")"
  trap 'rm -f "${pr_body_file:-}"' EXIT

  local commit_msg="$title"
  if [[ -n "$close_line" ]]; then
    commit_msg="${commit_msg} (${close_line})"
  fi

  if [[ "$has_uncommitted" == "1" ]]; then
    note "Committing…"
    run_git_or_die "finish: git commit" git commit -m "$commit_msg"
  else
    note "Skipping commit (working tree clean; using existing commits)."
  fi

  note "Pushing…"
  if ! git push -u origin "$branch"; then
    die "Push failed (likely non-fast-forward due to remote divergence). Try: 'git fetch origin' then 'git push --force-with-lease origin $branch' (if you rebased) or 'git pull --rebase' (if you didn’t)."
  fi

  local pr_url
  pr_url="$(current_pr_url "$repo" "$branch")"

  if [[ -n "$pr_url" ]]; then
    note "Reusing existing open PR…"
    if ! gh pr edit $(gh_repo_flag "$repo") "$pr_url" --title "$title" --body-file "$pr_body_file" >/dev/null; then
      die "finish: failed to update existing PR"
    fi
    ensure_pr_closing_linkage "$repo" "$pr_url" "$issue" "$no_close"
    note "PR updated:"
    echo "$pr_url"
  else
    note "Creating PR (draft)…"
    if ! pr_url="$(gh pr create $(gh_repo_flag "$repo") --base main --head "$branch" --title "$title" --body-file "$pr_body_file" --draft)"; then
      die "finish: failed to create PR"
    fi
    ensure_pr_closing_linkage "$repo" "$pr_url" "$issue" "$no_close"
    note "PR created:"
    echo "$pr_url"
  fi

  note "finish mode: $( [[ "$merge_mode" == "1" ]] && echo "MERGE" || echo "SAFE" )"
  note "Operating on PR: $pr_url"

  if [[ "$no_open" != "1" ]]; then
    note "Opening PR in browser…"
    open_in_browser "$repo" "$pr_url" || true
  else
    note "Not opening PR (--no-open)"
  fi
  note "Open output card: ./adl/tools/open_artifact.sh card $issue output"
  note "Open latest burst report: ./adl/tools/open_artifact.sh burst latest"

  if [[ "$merge_mode" == "1" ]]; then
    if ! git diff --quiet || ! git diff --cached --quiet; then
      die "finish: --merge requires a clean working tree before merge"
    fi

    local is_draft="false"
    is_draft="$(gh pr view $(gh_repo_flag "$repo") "$pr_url" --json isDraft -q .isDraft 2>/dev/null || echo "false")"
    if [[ "$is_draft" == "true" ]]; then
      note "Running: gh pr ready $(gh_repo_flag "$repo") $pr_url"
      gh pr ready $(gh_repo_flag "$repo") "$pr_url"
    fi

    note "Running: gh pr merge $(gh_repo_flag "$repo") --squash --delete-branch $pr_url"
    if ! gh pr merge $(gh_repo_flag "$repo") --squash --delete-branch "$pr_url"; then
      local retry_cmd
      retry_cmd="gh pr ready $(gh_repo_flag "$repo") \"$pr_url\" && gh pr merge $(gh_repo_flag "$repo") --squash --delete-branch \"$pr_url\""
      echo "RETRY_COMMAND=$retry_cmd" >&2
      die "finish: merge failed"
    fi
    note "PR merged."
    return 0
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
  help
  init    <issue> [--slug <slug>] [--title "<title>"] [--no-fetch-issue] [--version <v>]
  create  <issue> --stp <path>
  create  --title "<title>" [--slug <slug>] [--body "<text>" | --body-file <path>] [--labels <csv>] [--version <v>] [--no-start]
  run     <adl.yaml> [--trace] [--print-plan] [--print-prompts] [--resume <run.json>] [--steer <steering.json>] [--overlay <overlay.json>] [--out <dir>] [--runs-root <dir>] [--quiet] [--open] [--allow-unsigned]
  start   <issue> [--slug <slug>] [--title "<title>"] [--prefix <pfx>] [--no-fetch-issue] [--version <v>]
  card    <issue> [input|output] ... [--version <v0.2>] [-f <input_card.md>]
  output  <issue> [input|output] ... [--version <v0.2>] [-f <output_card.md>]
  cards   <issue> [--version <v0.2>] [--no-fetch-issue]
  finish  <issue> --title "<title>" ... [-f <input_card.md>] [--output-card <output_card.md>] [--no-open] [--merge]
  open
  status

Flags:
  (create)  <issue> --stp <path>              Reconcile an existing GitHub issue from a canonical STP.
  (create)  --title "<title>"                 Required issue title for gh issue create.
  (create)  --body "<text>"                   Optional issue body text.
  (create)  --body-file <path|->              Optional issue body file path ('-' reads stdin).
  (create)  --labels <csv>                    Comma-separated labels (default: track:roadmap,type:task,area:tools).
  (create)  --version <v0.86>                 Default/fallback version label for issue/card flow.
  (create)  --no-start                        Only create issue; do not invoke start.
  (init)    --version <v0.85>                 Override detected version (otherwise inferred from issue labels version:vX.Y)
  (init)    --no-fetch-issue                  Do not fetch issue title/labels; requires --slug.
  (run)     --runs-root <dir>                 Override canonical run artifact root (default: <repo>/.adl/runs or ADL_RUNS_ROOT).
  (card)    -f, --file <input_card.md>         Output path for the generated input card (default: <cards_root>/<issue>/input_<issue>.md)
  (output)  -f, --file <output_card.md>        Output path for the generated output card (default: <cards_root>/<issue>/output_<issue>.md)
  (cards)   --version <v0.2>                   Override detected version (otherwise inferred from issue labels version:vX.Y)
  (cards)   --no-fetch-issue                   Do not fetch issue title/labels (uses issue-<n> title)
  (card/output) --version <v0.2>               Override detected version (otherwise inferred from issue labels version:vX.Y)
  (finish) --output-card <output_card.md>          REQUIRED: output card path (must exist)
  (finish) --merge                              Opt-in: ready + squash-merge + delete branch.
  (finish) --idempotent                         Safe no-op only when existing merged PR matches current finish inputs.
  (card/start) --slug <slug>                   Use an explicit slug instead of fetching the issue title.
  (start)   --title "<title>"                  Optional; accepted for UX symmetry and used to derive slug when --slug is omitted.
  (start)   --version <v0.85>                  Override detected version when the caller already knows the intended milestone band.

Notes:
- PRs are created as DRAFT by default to preserve human review.
- Uses "Closes #N" by default so GitHub auto-closes issues when merged.
- run is a bounded v0.85 wrapper over the Rust adl runtime; browser/editor direct invocation remains follow-on work.
- Runs Rust checks in adl/ by default (fmt, clippy -D warnings, test).
- finish stages repo-root changes by default (ignored paths remain skipped; use --paths to narrow scope).
- Templates are stored in adl/templates/cards/ (legacy fallback: .adl/templates/).
- Cards are stored locally under cards_root and are not committed to git.
  cards_root resolves as: ADL_CARDS_ROOT (if set) else <primary-checkout>/.adl/cards.

Examples:
  adl/tools/pr.sh help
  adl/tools/pr.sh init 17 --slug b6-default-system --no-fetch-issue --version v0.85
  adl/tools/pr.sh create 17 --stp .adl/issues/v0.85/bodies/issue-17-b6-default-system.md
  adl/tools/pr.sh create --title "adl: fix timeout handling" --slug timeout-fix
  adl/tools/pr.sh run adl/examples/v0-4-demo-deterministic-replay.adl.yaml --trace --allow-unsigned
  adl/tools/pr.sh start 17 --slug b6-default-system
  adl/tools/pr.sh card  17 --help
  adl/tools/pr.sh card  17 --version v0.2
  adl/tools/pr.sh card  17 input
  adl/tools/pr.sh card  17 output
  adl/tools/pr.sh output 17 --version v0.2
  adl/tools/pr.sh output 17 input
  adl/tools/pr.sh output 17 output
  adl/tools/pr.sh cards 17 --version v0.2
  adl/tools/pr.sh finish 17 --title "adl: apply run.defaults.system fallback" -f .adl/cards/17/input_17.md --output-card .adl/cards/17/output_17.md
EOF
}

usage_new() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh new ...

Notes:
- `pr new` is retired.
- Use `adl/tools/pr.sh create --title ...` instead.
EOF
}

usage_create() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh create <issue> --stp <path>
  adl/tools/pr.sh create --title "<title>" [--slug <slug>] [--body "<text>" | --body-file <path>] [--labels <csv>] [--version <v>] [--no-start]

Notes:
- Reconcile mode updates an existing GitHub issue from a canonical STP.
- Create mode is the canonical issue-creation path.
- `pr new` is retired and should not be used.
EOF
}

usage_init() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh init <issue> [--slug <slug>] [--title "<title>"] [--no-fetch-issue] [--version <v0.85>]

Notes:
- Initializes the canonical local task-bundle authoring surface for an existing issue-backed task.
- For v0.85, the minimum initialized artifact set is the task-bundle directory plus a validated stp.md copied from the canonical local issue prompt.
- Does not create sip.md or sor.md; those remain the responsibility of pr start.
EOF
}

usage_start() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh start <issue> [--slug <slug>] [--title "<title>"] [--prefix <pfx>] [--no-fetch-issue] [--version <v>]

Notes:
- Creates or reuses issue worktree at .worktrees/adl-wp-<issue> by default.
- Keeps the primary checkout on main.
- `--version` overrides inferred issue version when the caller already knows the intended milestone band.
EOF
}

usage_run() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh run <adl.yaml> [--trace] [--print-plan] [--print-prompts] [--resume <run.json>] [--steer <steering.json>] [--overlay <overlay.json>] [--out <dir>] [--runs-root <dir>] [--quiet] [--open] [--allow-unsigned]

Notes:
- This is a bounded v0.85 control-plane wrapper over `cargo run --bin adl -- ...`.
- The primary proof surface is the canonical run artifact set under `.adl/runs/<run_id>/`.
- Browser/editor direct invocation remains follow-on work; this command is the truthful supported run path today.
EOF
}

usage_card() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh card <issue> [input|output] [--slug <slug>] [--no-fetch-issue] [--version <v>] [-f|--file <card.md>]

Notes:
- Default behavior (`card <issue>`) creates the input card if missing, then prints its path.
- Positional `input|output` opens/prints that card path and creates it if missing.
EOF
}

usage_output() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh output <issue> [input|output] [--slug <slug>] [--no-fetch-issue] [--version <v>] [-f|--file <card.md>]

Notes:
- Default behavior (`output <issue>`) creates the output card if missing, then prints its path.
- Positional `input|output` opens/prints that card path and creates it if missing.
EOF
}

usage_cards() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh cards <issue> [--version <v>] [--no-fetch-issue]
EOF
}

usage_finish() {
  cat <<'EOF'
Usage:
  adl/tools/pr.sh finish <issue> --title "<title>" [--paths "<p1,p2,...>"] [-f|--file <input_card.md>] [--output-card <output_card.md>] [--no-checks] [--no-open] [--merge]

Notes:
- By default, finish stages repo-root changes (`.`), which keeps docs and code changes together unless you narrow with `--paths`.
EOF
}

main() {
  local cmd="${1:-}"; shift || true
  case "$cmd" in
    help) usage ;;
    init) cmd_init "$@" ;;
    create) cmd_create "$@" ;;
    new) cmd_new "$@" ;;
    run) cmd_run "$@" ;;
    start) cmd_start "$@" ;;
    finish) cmd_finish "$@" ;;
    card) cmd_card "$@" ;;
    output) cmd_output "$@" ;;
    output-card) cmd_output "$@" ;;
    cards) cmd_cards "$@" ;;
    open) cmd_open ;;
    status) cmd_status ;;
    -h|--help|"") usage ;;
    *) die "Unknown command: $cmd (try --help)" ;;
  esac
}

main "$@"
