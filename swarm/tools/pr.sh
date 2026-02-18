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
#   swarm/tools/pr.sh help
#   swarm/tools/pr.sh start   <issue> [--slug <slug>] [--title "<title>"] [--prefix codex] [--no-fetch-issue]
#   swarm/tools/pr.sh card    <issue> [input|output] [--slug <slug>] [--no-fetch-issue] [-f <input_card.md>] [--version <v0.2>]
#   swarm/tools/pr.sh output  <issue> [input|output] [--slug <slug>] [--no-fetch-issue] [-f <output_card.md>] [--version <v0.2>]
#   swarm/tools/pr.sh finish  <issue> --title "<title>" [-f <input_card.md>] [--output-card <output_card.md>] [--body "<extra body>"] [--paths "<p1,p2,...>"] [--no-checks] [--no-close] [--ready] [--allow-gitignore] [--no-open]
#   swarm/tools/pr.sh open
#   swarm/tools/pr.sh status
#
# Examples:
#   swarm/tools/pr.sh start 14 --slug b6-default-system
#   swarm/tools/pr.sh card  14 --version v0.2
#   swarm/tools/pr.sh card  14 input
#   swarm/tools/pr.sh card  14 output
#   swarm/tools/pr.sh output 14 --version v0.2
#   swarm/tools/pr.sh output 14 input
#   swarm/tools/pr.sh output 14 output
#   swarm/tools/pr.sh finish 14 --title "swarm: apply run.defaults.system fallback" -f /abs/cards_root/14/input_14.md --output-card /abs/cards_root/14/output_14.md
#   swarm/tools/pr.sh open
#
set -euo pipefail

CARD_PATHS_LIB="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/card_paths.sh"
# shellcheck disable=SC1090
source "$CARD_PATHS_LIB"

DEFAULT_VERSION="v0.3"
DEFAULT_NEW_LABELS="track:roadmap,version:v0.3,type:bug,area:tools,epic:v0.3-tooling-git"


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

run_tooling_sanity_checks() {
  local root
  root="$(repo_root)"
  note "Running tooling sanity checks (codex_pr/codexw)…"
  bash -n "$root/swarm/tools/codex_pr.sh"
  bash -n "$root/swarm/tools/codexw.sh"
  echo "Skipping codex_pr sanity check (no --paths configured)."
  bash "$root/swarm/tools/codexw.sh" --help >/dev/null 2>&1
  sh "$root/swarm/tools/codexw.sh" --help >/dev/null 2>&1
}

run_batched_checks() {
  local root
  root="$(repo_root)"
  note "Running batched checks via swarm/tools/batched_checks.sh…"
  bash "$root/swarm/tools/batched_checks.sh"
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
    die "finish: all provided --paths were empty or gitignored; pass non-ignored paths (e.g. --paths \"swarm/tools\")"
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

INPUT_TEMPLATE="swarm/templates/cards/input_card_template.md"
OUTPUT_TEMPLATE="swarm/templates/cards/output_card_template.md"
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

issue_version() {
  local issue="$1"
  local v
  v="$(gh issue view "$issue" --json labels -q '.labels[].name' 2>/dev/null | sed -n 's/^version://p' | head -n1 || true)"
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
  local issue="$1"
  card_input_path "$issue" || die "invalid issue number: $issue"
}

output_card_path() {
  local issue="$1"
  card_output_path "$issue" || die "invalid issue number: $issue"
}

resolve_input_card_path_abs() {
  local issue="$1" ver="$2"
  resolve_input_card_path "$issue" "$ver" || die "invalid issue number: $issue"
}

resolve_output_card_path_abs() {
  local issue="$1" ver="$2"
  resolve_output_card_path "$issue" "$ver" || die "invalid issue number: $issue"
}

sync_legacy_links_for_issue() {
  # Legacy compatibility links are intentionally disabled to keep cards
  # canonicalized under .adl/cards/<issue>/ only.
  :
}

render_template() {
  # Args: template_path
  local tpl="$1"
  [[ -f "$tpl" ]] || return 1
  cat "$tpl"
}

validate_card_header_count() {
  # Args: file_path header_line
  local path="$1" header="$2"
  local count
  count="$(grep -c -x -F "$header" "$path" || true)"
  [[ "$count" == "1" ]]
}

seed_input_card() {
  local path="$1" issue="$2" title="$3" branch="$4" ver="$5"
  local task_id run_id
  task_id="issue-$(card_issue_pad "$issue")"
  run_id="$task_id"
  local tpl tmp repo issue_url
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
    replace_first_line_re "$tmp" "^- Issue:[[:space:]]*$" "- Issue: $issue_url"
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
  validate_card_header_count "$tmp" "# ADL Output Card" || die "generated output card must contain exactly one '# ADL Output Card' header"
  ensure_nonempty_file "$tmp" || die "generated output card is empty: $tmp"
  mv "$tmp" "$path"
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
  # Args: issue close_line input_path output_path extra_body no_checks fingerprint
  local issue="$1" close_line="$2" input_path="$3" output_path="$4" extra_body="$5" no_checks="$6" fingerprint="$7"

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
    echo "- Idempotency-Key: $fingerprint"
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
    out_path="$(input_card_path "$issue")"
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
  seed_input_card "$out_path" "$issue" "$title" "$(current_branch)" "$version"
  if [[ "$out_path" == "$(input_card_path "$issue")" ]]; then
    sync_legacy_links_for_issue "$issue" "$version"
  fi
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
      quick_path="$(input_card_path "$issue")"
    else
      quick_path="$(output_card_path "$issue")"
    fi
    if ensure_nonempty_file "$quick_path"; then
      echo "$quick_path"
      return 0
    fi
  fi

  if [[ "$target_kind" == "input" ]]; then
    local input_target
    input_target="${out_path:-$(input_card_path "$issue")}"
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
    out_path="$(output_card_path "$issue")"
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
  if [[ "$out_path" == "$(output_card_path "$issue")" ]]; then
    sync_legacy_links_for_issue "$issue" "$version"
  fi
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

  local input_path output_path
  input_path="$(input_card_path "$issue")"
  output_path="$(output_card_path "$issue")"

  if ensure_nonempty_file "$input_path"; then
    note "Input card exists: $input_path"
  else
    note "Creating input card: $input_path"
    seed_input_card "$input_path" "$issue" "$title" "TBD (run pr.sh start $issue)" "$version"
  fi

  if ensure_nonempty_file "$output_path"; then
    note "Output card exists: $output_path"
  else
    note "Creating output card: $output_path"
    seed_output_card "$output_path" "$issue" "$title" "TBD (run pr.sh start $issue)" "$version"
  fi

  sync_legacy_links_for_issue "$issue" "$version"

  echo "READ  $input_path"
  echo "WRITE $output_path"
}

cmd_start() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_start
    return 0
  fi

  require_cmd git
  require_cmd gh

  local issue="${1:-}"; shift || true
  [[ -n "$issue" ]] || die_with_usage "start: missing <issue> number" usage_start
  issue="$(normalize_issue_or_die "$issue")"

  local prefix="codex"
  local slug=""
  local no_fetch_issue="0"
  local title=""
  local title_arg=""

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --prefix) prefix="$2"; shift 2 ;;
      --slug) slug="$2"; shift 2 ;;
      --title) title_arg="$2"; shift 2 ;;
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

  # Ensure we are on the target branch.
  if [[ "$(current_branch)" == "$branch" ]]; then
    note "Already on $branch"
  elif git show-ref --verify --quiet "refs/heads/$branch"; then
    note "Switching to existing local branch…"
    run_git_or_die "start: switch to existing branch '$branch'" git switch "$branch"
  elif git ls-remote --exit-code --heads origin "$branch" >/dev/null 2>&1; then
    note "Branch exists on origin; checking out and tracking…"
    run_git_or_die "start: switch to tracking branch 'origin/$branch'" git switch --track "origin/$branch"
  else
    # Otherwise create a new branch from origin/main without checking out local main.
    ensure_clean_worktree

    note "Fetching origin/main…"
    run_git_or_die "start: fetch origin main" git fetch origin main
    if ! git rev-parse --verify --quiet origin/main >/dev/null; then
      die "start: origin/main not found after fetch; verify remote setup and permissions"
    fi

    note "Creating branch from origin/main…"
    run_git_or_die "start: create branch '$branch' from origin/main" git checkout -b "$branch" origin/main
  fi

  local ver in_path out_path
  ver="$(issue_version "$issue")"
  in_path="$(input_card_path "$issue")"
  out_path="$(output_card_path "$issue")"
  ensure_adl_dirs
  if ! ensure_nonempty_file "$in_path"; then
    note "Creating input card: $in_path"
    seed_input_card "$in_path" "$issue" "$title" "$branch" "$ver"
  else
    note "Input card exists: $in_path"
  fi
  if ! ensure_nonempty_file "$out_path"; then
    note "Creating output card: $out_path"
    seed_output_card "$out_path" "$issue" "$title" "$branch" "$ver"
  else
    note "Output card exists: $out_path"
  fi
  sync_legacy_links_for_issue "$issue" "$ver"
  echo "• Agent:"
  echo "  READ   $in_path"
  echo "  WRITE  $out_path"
  echo "  OPEN   ./swarm/tools/open_artifact.sh card $issue output"
  note "Done."
}

cmd_new() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_new
    return 0
  fi

  require_cmd gh

  local title=""
  local slug=""
  local body=""
  local body_file=""
  local labels="$DEFAULT_NEW_LABELS"
  local version="$DEFAULT_VERSION"
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
  [[ -n "$version" ]] || die "new: --version must be non-empty"

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

  local labels_csv
  labels_csv="$labels"
  if [[ "$labels_csv" != *"version:"* ]]; then
    labels_csv="${labels_csv},version:${version}"
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

  if [[ "$no_start" == "1" ]]; then
    return 0
  fi

  cmd_start "$issue_num" --slug "$slug"
  echo "BRANCH=codex/${issue_num}-${slug}"
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
  local paths="swarm"
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
    # Stage selected paths (default: swarm). Use --paths "adl-spec,swarm" or --paths "." to stage everything.
    IFS=',' read -r -a path_arr <<< "$paths"
    if [[ ${#path_arr[@]} -eq 0 ]]; then
      die "finish: --paths resolved to empty; pass e.g. --paths \"swarm\""
    fi

    note "Staging changes (${paths})…"
    stage_selected_paths "${path_arr[@]}"

    if git diff --cached --quiet; then
      die "finish: nothing staged after 'git add' for paths '${paths}'. Your paths may be empty/ignored or there were no changes. Either change --paths or commit manually and re-run finish."
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
    note "PR updated:"
    echo "$pr_url"
  else
    note "Creating PR (draft)…"
    if ! pr_url="$(gh pr create $(gh_repo_flag "$repo") --base main --head "$branch" --title "$title" --body-file "$pr_body_file" --draft)"; then
      die "finish: failed to create PR"
    fi
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
  note "Open output card: ./swarm/tools/open_artifact.sh card $issue output"
  note "Open latest burst report: ./swarm/tools/open_artifact.sh burst latest"

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
  new     --title "<title>" [--slug <slug>] [--body "<text>" | --body-file <path>] [--labels <csv>] [--version <v>] [--no-start]
  start   <issue> [--slug <slug>] [--title "<title>"] [--prefix <pfx>] [--no-fetch-issue]
  card    <issue> [input|output] ... [--version <v0.2>] [-f <input_card.md>]
  output  <issue> [input|output] ... [--version <v0.2>] [-f <output_card.md>]
  cards   <issue> [--version <v0.2>] [--no-fetch-issue]
  finish  <issue> --title "<title>" ... [-f <input_card.md>] [--output-card <output_card.md>] [--no-open] [--merge]
  open
  status

Flags:
  (new)     --title "<title>"                 Required issue title for gh issue create.
  (new)     --body "<text>"                   Optional issue body text.
  (new)     --body-file <path|->              Optional issue body file path ('-' reads stdin).
  (new)     --labels <csv>                    Comma-separated labels (default: track:roadmap,version:v0.3,type:bug,area:tools,epic:v0.3-tooling-git).
  (new)     --version <v0.3>                  Default/fallback version label for new issue/card flow.
  (new)     --no-start                        Only create issue; do not invoke start.
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

Notes:
- PRs are created as DRAFT by default to preserve human review.
- Uses "Closes #N" by default so GitHub auto-closes issues when merged.
- Runs Rust checks in swarm/ by default (fmt, clippy -D warnings, test).
- finish stages swarm/ by default (reduces accidental commits).
- Templates are stored in swarm/templates/cards/ (legacy fallback: .adl/templates/).
- Cards are stored locally under cards_root and are not committed to git.
  cards_root resolves as: ADL_CARDS_ROOT (if set) else <primary-checkout>/.adl/cards.

Examples:
  swarm/tools/pr.sh help
  swarm/tools/pr.sh new --title "swarm: fix timeout handling" --slug timeout-fix
  swarm/tools/pr.sh start 17 --slug b6-default-system
  swarm/tools/pr.sh card  17 --help
  swarm/tools/pr.sh card  17 --version v0.2
  swarm/tools/pr.sh card  17 input
  swarm/tools/pr.sh card  17 output
  swarm/tools/pr.sh output 17 --version v0.2
  swarm/tools/pr.sh output 17 input
  swarm/tools/pr.sh output 17 output
  swarm/tools/pr.sh cards 17 --version v0.2
  swarm/tools/pr.sh finish 17 --title "swarm: apply run.defaults.system fallback" -f /abs/cards_root/17/input_17.md --output-card /abs/cards_root/17/output_17.md
EOF
}

usage_new() {
  cat <<'EOF'
Usage:
  swarm/tools/pr.sh new --title "<title>" [--slug <slug>] [--body "<text>" | --body-file <path>] [--labels <csv>] [--version <v>] [--no-start]
EOF
}

usage_start() {
  cat <<'EOF'
Usage:
  swarm/tools/pr.sh start <issue> [--slug <slug>] [--title "<title>"] [--prefix <pfx>] [--no-fetch-issue]
EOF
}

usage_card() {
  cat <<'EOF'
Usage:
  swarm/tools/pr.sh card <issue> [input|output] [--slug <slug>] [--no-fetch-issue] [--version <v>] [-f|--file <card.md>]

Notes:
- Default behavior (`card <issue>`) creates the input card if missing, then prints its path.
- Positional `input|output` opens/prints that card path and creates it if missing.
EOF
}

usage_output() {
  cat <<'EOF'
Usage:
  swarm/tools/pr.sh output <issue> [input|output] [--slug <slug>] [--no-fetch-issue] [--version <v>] [-f|--file <card.md>]

Notes:
- Default behavior (`output <issue>`) creates the output card if missing, then prints its path.
- Positional `input|output` opens/prints that card path and creates it if missing.
EOF
}

usage_cards() {
  cat <<'EOF'
Usage:
  swarm/tools/pr.sh cards <issue> [--version <v>] [--no-fetch-issue]
EOF
}

usage_finish() {
  cat <<'EOF'
Usage:
  swarm/tools/pr.sh finish <issue> --title "<title>" [--paths "<p1,p2,...>"] [-f|--file <input_card.md>] [--output-card <output_card.md>] [--no-checks] [--no-open] [--merge]
EOF
}

main() {
  local cmd="${1:-}"; shift || true
  case "$cmd" in
    help) usage ;;
    new) cmd_new "$@" ;;
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
