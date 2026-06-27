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
# - GitHub token for Rust octocrab-backed GitHub operations. Supported shared
#   sources are GITHUB_TOKEN, GH_TOKEN, ADL_GITHUB_TOKEN_FILE, or
#   ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE. Do not use direct `gh` commands as a
#   fallback.
# - Rust toolchain for `adl/` checks (fmt, clippy, test)
#
#   adl/tools/pr.sh help
#   adl/tools/pr.sh create  --title "<title>" [--slug <slug>] [--body "<markdown>" | --body-file <path>] [--labels <csv>] [--version <v0.85|v0.87.1>]
#   adl/tools/pr.sh init    <issue> [--slug <slug>] [--title "<title>"] [--no-fetch-issue] [--version <v0.85|v0.87.1>]
#   adl/tools/pr.sh repair-issue-body <issue> [--slug <slug>] [--title "<title>"] [--body "<markdown>" | --body-file <path>] [--labels <csv>] [--version <v0.85|v0.87.1>] [--force]
#   adl/tools/pr.sh run     <issue> [--slug <slug>] [--title "<title>"] [--prefix codex] [--no-fetch-issue] [--version <v0.85|v0.87.1>] [--allow-open-pr-wave]
#   adl/tools/pr.sh run     <adl.yaml> [--trace] [--print-plan] [--print-prompts] [--resume <run.json>] [--steer <steering.json>] [--overlay <overlay.json>] [--out <dir>] [--runs-root <dir>] [--quiet] [--open] [--allow-unsigned]
#   adl/tools/pr.sh card    <issue> [input|output] [--slug <slug>] [--no-fetch-issue] [-f <input_card.md>] [--version <v0.2>]
#   adl/tools/pr.sh output  <issue> [input|output] [--slug <slug>] [--no-fetch-issue] [-f <output_card.md>] [--version <v0.2>]
#   adl/tools/pr.sh doctor  <issue> [--slug <slug>] [--no-fetch-issue] [--version <v0.2>] [--mode full|ready|preflight] [--allow-open-pr-wave] [--json]
#   adl/tools/pr.sh preflight <issue> [--slug <slug>] [--no-fetch-issue] [--version <v0.2>] [--allow-open-pr-wave] [--json]
#   adl/tools/pr.sh finish  <issue> --title "<title>" [-f <input_card.md>] [--output-card <output_card.md>] [--body "<extra body>"] [--paths "<p1,p2,...>"] [--no-checks] [--no-close] [--ready] [--allow-gitignore] [--no-open]
#   adl/tools/pr.sh closing-linkage [--event-name <event>] [--event-path <path>] [--head-ref <branch>] [-R owner/repo]
#   adl/tools/pr.sh open
#   adl/tools/pr.sh status
#
# Examples:
#   adl/tools/pr.sh create --title "[v0.86][tools] Example task" --labels track:roadmap,type:task,area:tools --version v0.86
#   adl/tools/pr.sh init  14 --slug b6-default-system --no-fetch-issue --version v0.85
#   adl/tools/pr.sh run 14 --slug b6-default-system --version v0.85
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
OBSERVABILITY_LIB="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/observability.sh"
if [[ -f "$OBSERVABILITY_LIB" ]]; then
  # shellcheck disable=SC1090
  source "$OBSERVABILITY_LIB"
else
  # Some compatibility tests copy pr.sh into a minimal fake repo. Observability
  # must never make those compatibility surfaces fail before their assertions.
  adl_obs_event() { :; }
  adl_obs_heartbeat_interval_ms() { printf '5000\n'; }
  adl_obs_sleep_ms() {
    python3 - "${1:-0}" <<'PY'
import sys
import time

millis = int(sys.argv[1])
time.sleep(max(millis, 0) / 1000.0)
PY
  }
fi

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

section_has_authored_content() {
  local file="$1" header="$2"
  awk -v header="$header" '
    BEGIN { in_section = 0; found = 0 }
    {
      line = $0
      trimmed = line
      sub(/^[[:space:]]+/, "", trimmed)
      sub(/[[:space:]]+$/, "", trimmed)
      if (trimmed == header) {
        in_section = 1
        next
      }
      if (in_section && trimmed ~ /^##[[:space:]]+/) {
        in_section = 0
      }
      if (in_section && trimmed != "" && trimmed != "-" && trimmed != "none") {
        found = 1
        exit
      }
    }
    END { exit(found ? 0 : 1) }
  ' "$file"
}

input_card_is_bootstrap_stub() {
  local file="$1"
  [[ -f "$file" ]] || return 1
  if ! section_has_authored_content "$file" "## Goal"; then
    return 0
  fi
  if ! section_has_authored_content "$file" "## Acceptance Criteria"; then
    return 0
  fi
  local marker
  while IFS= read -r marker; do
    [[ -n "$marker" ]] || continue
    if grep -Fqx -- "$marker" "$file"; then
      return 0
    fi
  done <<'EOF'
- State whether this issue must ship code, docs, tests, demo artifacts, or a combination.
- Likely files, modules, docs, commands, schemas, or artifacts to modify or validate
- Required commands:
- Required tests:
- Required artifacts / traces:
- Required reviewer or demo checks:
- Required demo(s):
- Required proof surface(s):
- If no demo is required, say why:
- Determinism requirements:
- Security / privacy requirements:
- Resource limits (time/CPU/memory/network):
EOF
  return 1
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


PR_TOOLS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

source_pr_helper() {
  local helper="$1"
  local path="$PR_TOOLS_DIR/$helper"
  if [[ ! -f "$path" ]]; then
    adl_obs_event "pr.sh" "source-helper" "missing" "helper" "$helper" "path" "$path"
    die "missing pr.sh helper: $helper"
  fi
  # shellcheck disable=SC1090
  source "$path"
  adl_obs_event "pr.sh" "source-helper" "ok" "helper" "$helper"
}

source_pr_helper "pr_delegate.sh"

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



repo_lock_root() {
  local root
  root="$(primary_checkout_root)"
  echo "$root/.adl/locks"
}

issue_bootstrap_lock_name() {
  local issue="$1"
  printf 'pr-bootstrap-issue-%s\n' "$issue"
}

acquire_repo_lock_into() {
  local name="$1" outvar="$2"
  local lock_dir
  lock_dir="$(repo_lock_root)/${name}.lock"
  mkdir -p "$(dirname "$lock_dir")"
  local attempt max_attempts pid_file owner_pid stale_marker
  max_attempts=50
  for ((attempt=1; attempt<=max_attempts; attempt++)); do
    if mkdir "$lock_dir" 2>/dev/null; then
      if ! printf '%s\n' "$$" >"$lock_dir/pid"; then
        rm -rf "$lock_dir"
        die "${name}: acquired bootstrap lock but failed to record owner pid at $lock_dir/pid"
      fi
      printf -v "$outvar" '%s' "$lock_dir"
      return 0
    fi
    pid_file="$lock_dir/pid"
    if [[ -f "$pid_file" ]]; then
      owner_pid="$(tr -d '[:space:]' <"$pid_file" 2>/dev/null || true)"
      if [[ "$owner_pid" =~ ^[0-9]+$ ]] && ! kill -0 "$owner_pid" 2>/dev/null; then
        rm -rf "$lock_dir"
        continue
      fi
    else
      stale_marker="$(find "$lock_dir" -prune -mmin +1 -print -quit 2>/dev/null || true)"
      if [[ -n "$stale_marker" ]]; then
        rm -rf "$lock_dir"
        continue
      fi
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


primary_checkout_root() {
  card_primary_checkout_root
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
  if [[ "$title" =~ \[(v[0-9]+\.[0-9]+(\.[0-9]+)*)\] ]]; then
    printf '%s\n' "${BASH_REMATCH[1]}"
  fi
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
  local lowered_title use_workflow_skill_template
  lowered_title="$(printf '%s' "$title" | tr '[:upper:]' '[:lower:]')"
  use_workflow_skill_template=false
  if [[ "$lowered_title" == *"[tools]"* && ( "$lowered_title" == *" skill "* || "$lowered_title" == skill\ * || "$lowered_title" == *" workflow "* || "$lowered_title" == workflow\ * ) ]]; then
    use_workflow_skill_template=true
  fi
  if [[ "$use_workflow_skill_template" == true ]]; then
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

Bootstrap-generated workflow-skill issue body created from the requested title and labels so the issue starts with a concrete first draft instead of a generic bootstrap stub.

## Goal

Define one bounded workflow-skill or tooling-surface change in the tracked PR workflow substrate and make the resulting source prompt/STP concrete enough for qualitative review before execution.

## Required Outcome

The default required outcome type for this issue is \`$outcome_type\` based on the current title and labels. Workflow-skill issues should also name the tracked skill, contract, docs, and validation surfaces that need to move together.

## Deliverables

- the targeted workflow-skill or tooling-surface change under \`adl/tools/skills\` or the owning control-plane code
- matching schema or operator-doc updates when invocation, lifecycle behavior, or closeout guidance changes
- focused validation covering the changed workflow-skill surface

## Acceptance Criteria

- the generated prompt identifies this as a workflow-skill/tooling issue rather than a generic bootstrap task
- the generated first draft names likely tracked surfaces, expected validation, and lifecycle boundaries concretely enough that only bounded refinement is normally needed before readiness review
- bootstrap output remains deterministic, reviewable, and free of placeholder drift

## Repo Inputs

- $issue_url
- adl/tools/skills
- adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md

## Dependencies

- none recorded yet

## Demo Expectations

- No demo is required by default. Update this section only if the workflow-skill change needs a proof surface.

## Non-goals

- silently widening the issue into a broad workflow redesign
- introducing ad-hoc card or lifecycle drift outside the tracked skill flow

## Issue-Graph Notes

- This issue body was generated automatically because no canonical local issue prompt existed yet.
- The workflow-skill bootstrap template should still be refined if the issue needs more specific acceptance criteria, but the starting draft should already be reviewable.

## Notes

- Generated by the ADL PR control plane from issue metadata using the workflow-skill bootstrap template.

## Tooling Notes

- This body should be concrete enough that \`GitHub issue view\` is useful immediately after creation.
- Default next steps should follow \`pr-ready\`, the editor skills, and \`pr-run\`, not the older \`pr start\` path.
EOF
    return 0
  fi
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
- the task can proceed through \`pr init\`, issue-mode \`pr run\`, and card editing without manual bootstrap repair

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

  root="$(repo_root)"
  base="$(basename "$root")"
  echo "local/$base"
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



# ----- pr/branch helpers -----


# ---------- cards + templates (templates tracked; cards local-only) ----------
ADL_DIR=".adl"

PROMPT_TEMPLATE_ROOT="docs/templates/prompts/1.0.0"
INPUT_TEMPLATE="$PROMPT_TEMPLATE_ROOT/sip.md"
STP_TEMPLATE="$PROMPT_TEMPLATE_ROOT/stp.md"
SPP_TEMPLATE="$PROMPT_TEMPLATE_ROOT/spp.md"
VPP_TEMPLATE="$PROMPT_TEMPLATE_ROOT/vpp.md"
SRP_TEMPLATE="$PROMPT_TEMPLATE_ROOT/srp.md"
OUTPUT_TEMPLATE="$PROMPT_TEMPLATE_ROOT/sor.md"
COMPAT_INPUT_TEMPLATE="adl/templates/cards/input_card_template.md"
COMPAT_OUTPUT_TEMPLATE="adl/templates/cards/output_card_template.md"
LEGACY_INPUT_TEMPLATE="$ADL_DIR/templates/input_card_template.md"
LEGACY_OUTPUT_TEMPLATE="$ADL_DIR/templates/output_card_template.md"

resolve_prompt_template() {
  local kind="$1" primary="" compat="" legacy=""
  local registry_path registry_template
  case "$kind" in
    sip) primary="$INPUT_TEMPLATE"; compat="$COMPAT_INPUT_TEMPLATE"; legacy="$LEGACY_INPUT_TEMPLATE" ;;
    stp) primary="$STP_TEMPLATE" ;;
    spp) primary="$SPP_TEMPLATE" ;;
    vpp) primary="$VPP_TEMPLATE" ;;
    srp) primary="$SRP_TEMPLATE" ;;
    sor) primary="$OUTPUT_TEMPLATE"; compat="$COMPAT_OUTPUT_TEMPLATE"; legacy="$LEGACY_OUTPUT_TEMPLATE" ;;
    *) die "unknown prompt template kind: $kind" ;;
  esac
  registry_path="$(repo_root)/docs/templates/prompts/current.json"
  if [[ -f "$registry_path" ]]; then
    registry_template="$(
      python3 - "$registry_path" "$kind" <<'PY' 2>/dev/null || true
import json
import sys
from pathlib import Path

registry = json.loads(Path(sys.argv[1]).read_text())
entry = registry.get("templates", {}).get(sys.argv[2], {})
print(entry.get("path", ""))
PY
    )"
    if [[ -n "$registry_template" && -f "$(repo_root)/$registry_template" ]]; then
      echo "$(repo_root)/$registry_template"
      return 0
    fi
  fi
  if [[ -n "$primary" && -f "$(repo_root)/$primary" ]]; then
    echo "$(repo_root)/$primary"
    return 0
  fi
  if [[ -n "$compat" && -f "$(repo_root)/$compat" ]]; then
    echo "$(repo_root)/$compat"
    return 0
  fi
  if [[ -n "$legacy" && -f "$(repo_root)/$legacy" ]]; then
    echo "$(repo_root)/$legacy"
    return 0
  fi
  echo "$(repo_root)/$primary"
}

resolve_input_template() {
  resolve_prompt_template sip
}

resolve_output_template() {
  resolve_prompt_template sor
}

resolve_structured_prompt_validator() {
  local validator
  validator="$(repo_root)/adl/tools/validate_structured_prompt.sh"
  [[ -x "$validator" ]] || die "start: missing executable structured prompt validator: $validator"
  echo "$validator"
}

issue_version() {
  local issue="$1"
  die "issue_version: GitHub-backed version inference for issue #$issue is Rust-owned; use create/init/run/start/doctor/finish/closeout or pass --version with legacy card commands"
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

apply_prompt_template_values() {
  local file="$1"
  shift
  python3 - "$file" "$@" <<'PY'
import sys

path = sys.argv[1]
pairs = sys.argv[2:]
if len(pairs) % 2:
    raise SystemExit("template replacement requires token/value pairs")
with open(path, "r", encoding="utf-8") as handle:
    text = handle.read()
for idx in range(0, len(pairs), 2):
    text = text.replace(pairs[idx], pairs[idx + 1])
with open(path, "w", encoding="utf-8") as handle:
    handle.write(text)
PY
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

replace_first_markdown_h1() {
  local file="$1" title="$2"
  replace_first_line_re "$file" '^# .*$' "# $title"
}

branch_indicates_unbound_state() {
  local branch="${1:-}"
  [[ -z "$branch" || "$branch" == "not bound yet" || "$branch" == TBD\ \(run\ pr.sh\ start\ * || "$branch" == TBD\ \(run\ pr.sh\ run\ * ]]
}

remove_exact_line() {
  local file="$1" target="$2"
  local tmp
  tmp="$(mktemp -t prsh_remove_line_XXXXXX)"
  grep -Fvx -- "$target" "$file" >"$tmp" || true
  mv "$tmp" "$file"
}

deduplicate_exact_line() {
  local file="$1" target="$2"
  local tmp
  tmp="$(mktemp -t prsh_dedupe_line_XXXXXX)"
  awk -v target="$target" '
    $0 == target {
      if (seen) {
        next
      }
      seen = 1
    }
    { print }
  ' "$file" >"$tmp"
  mv "$tmp" "$file"
}

apply_input_card_lifecycle() {
  local file="$1" branch="$2"
  branch_indicates_unbound_state "$branch" && return 0

  replace_first_line_re "$file" '^- This issue is not started yet; do not assume a branch or worktree already exists\.$' '- Do not run `pr start`; the branch and worktree already exist.'
  replace_first_line_re "$file" '^- Do not run `pr start`; use the current issue-mode `pr run` flow only if execution later becomes necessary\.$' '- Do not delete or recreate cards.'
  deduplicate_exact_line "$file" '- Do not delete or recreate cards.'
  replace_first_line_re "$file" '^Prepare the linked issue prompt and review surfaces for truthful pre-run review before execution is bound\.$' 'Execute the linked issue prompt in this started worktree without rerunning bootstrap commands.'
  replace_first_line_re "$file" '^- Keep the linked issue prompt, input card, and output record aligned for review\.$' '- Ship the required outcome type recorded in the linked source issue prompt.'
  replace_first_line_re "$file" '^- Preserve truthful lifecycle state until `pr run` binds the branch and worktree\.$' '- Keep the linked issue prompt, repository changes, and output record aligned.'
  replace_first_line_re "$file" '^- The linked source issue prompt is reviewable and structurally valid\.$' '- The implementation satisfies the linked source issue prompt.'
  replace_first_line_re "$file" '^- The card bundle does not imply a branch or worktree exists before `pr run`\.$' '- Validation and proof surfaces named below are completed or explicitly marked not applicable.'
  remove_exact_line "$file" '- Validation and proof expectations are recorded or explicitly marked not applicable.'
  replace_first_line_re "$file" '^- root task bundle cards$' '- root and worktree task bundle cards'
  replace_first_line_re "$file" '^- current repository state before execution binding$' '- current repository state for this branch'
  replace_first_line_re "$file" '^- files, docs, tests, commands, schemas, and artifacts named by the linked source issue prompt, once execution is bound$' '- files, docs, tests, commands, schemas, and artifacts named by the linked source issue prompt'
  replace_first_line_re "$file" '^- Commands to run before execution: structured prompt/card validation only, unless the source issue prompt explicitly requires a pre-run proof\.$' '- Commands to run: derive the exact command set from the linked issue prompt and repo state; record what actually ran in the output card.'
  replace_first_line_re "$file" '^- Commands to run during execution: derive the exact command set from the linked issue prompt and repo state after `pr run` binds the worktree\.$' '- Tests to run: execute the smallest proving test set for the required outcome.'
  replace_first_line_re "$file" '^- Tests to run: execute the smallest proving test set for the required outcome during execution\.$' '- Artifacts or traces: produce or update the proof surfaces required by the linked issue prompt.'
  replace_first_line_re "$file" '^- Artifacts or traces: produce or update the proof surfaces required by the linked issue prompt during execution\.$' '- Reviewer checks: capture any manual review or demo checks in the output card.'
  remove_exact_line "$file" '- Reviewer checks: capture any manual review or demo checks in the output card after execution.'
  replace_first_line_re "$file" '^- Proof surfaces: use the proof surfaces named by the linked issue prompt and output card once execution is bound\.$' '- Proof surfaces: use the proof surfaces named by the linked issue prompt and output card.'
  replace_first_line_re "$file" '^- No-demo rationale: if no demo is required, explain why in the output card during execution\.$' '- No-demo rationale: if no demo is required, explain why in the output card.'
  replace_first_line_re "$file" '^- Refine this card if the linked source issue prompt changes materially before execution begins\.$' '- Refine this card if the linked source issue prompt changes materially before implementation begins.'
  remove_exact_line "$file" '- Do not create a branch or worktree from this card alone.'
  replace_first_line_re "$file" '^- When execution is approved, run the repo-native issue-mode `pr run` flow and then perform the work described above\.$' '- Do the work described above.'
  replace_first_line_re "$file" '^- Write results to the paired output card file during execution\.$' '- Write results to the paired output card file.'
}

output_card_title_matches_slug() {
  local path="$1" slug="$2"
  validate_card_header_count "$path" "# $slug"
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

  repo="$(default_repo)"
  source_slug="$(sanitize_slug "$title")"
  source_path="$(issue_prompt_path_for_issue "$issue" "$ver" "$source_slug")"
  issue_url="https://github.com/${repo}/issues/${issue}"
  docs_value="$(docs_context_value_for_issue_prompt "$source_path")"
  output_path_actual="${output_path_actual:-$(output_card_path "$issue" "$ver" "$source_slug")}"
  output_path_actual="$(path_relative_to_repo "$output_path_actual")"
  apply_prompt_template_values "$tmp" \
    "<issue>" "$issue" \
    "<issue_padded>" "$(card_issue_pad "$issue")" \
    "<task_id>" "$task_id" \
    "<run_id>" "$run_id" \
    "<version>" "$ver" \
    "<slug>" "$source_slug" \
    "<title>" "$title" \
    "<branch>" "$branch" \
    "<card_status>" "ready" \
    "<timestamp>" "$(date -u +"%Y-%m-%dT%H:%M:%SZ")" \
    "<issue_url>" "$issue_url" \
    "<source_issue_prompt>" "$(path_relative_to_repo "$source_path")" \
    "<docs_context>" "$docs_value" \
    "<output_card>" "$output_path_actual" \
    "<required_outcome_type>" "combination" \
    "<demo_required>" "false" \
    "<goal>" "Execute the linked issue prompt in the bound issue worktree." \
    "<required_outcome>" "Ship the required outcome described by the linked source issue prompt." \
    "<acceptance_criteria>" "Satisfy the acceptance criteria in the linked source issue prompt and record focused proof in SOR." \
    "<inputs>" "Linked source issue prompt; root task bundle cards; current repository state." \
    "<target_files_surfaces>" "Files, docs, tests, commands, schemas, and artifacts named by the linked source issue prompt." \
    "<validation_plan>" "Run the smallest proving validation for the touched surface and record exact commands in SOR." \
    "<demo_proof_requirements>" "Follow demo and proof requirements from the linked source issue prompt." \
    "<non_goals>" "Do not widen scope beyond the linked source issue prompt." \
    "<notes_risks>" "Refine this card if the linked source issue prompt changes materially before execution begins."

  # Stamp fields (best-effort; keeps template generic and domain-agnostic).
  set_field_line "$tmp" "Task ID" "$task_id"
  set_field_line "$tmp" "Run ID" "$run_id"
  set_field_line "$tmp" "Version" "$ver"
  set_field_line "$tmp" "Title" "$title"
  set_field_line "$tmp" "Branch" "$branch"

  # If there is a Context Issue line, fill it with a URL.
  if [[ -n "$repo" ]]; then
    replace_first_line_re "$tmp" "^- Issue:.*$" "- Issue: $issue_url"
  fi

  if [[ -f "$source_path" ]]; then
    replace_first_line_re "$tmp" "^- Source Issue Prompt:.*$" "- Source Issue Prompt: $(path_relative_to_repo "$source_path")"
  elif [[ -n "$issue_url" ]]; then
    replace_first_line_re "$tmp" "^- Source Issue Prompt:.*$" "- Source Issue Prompt: $issue_url"
  fi
  replace_first_line_re "$tmp" "^- Docs:.*$" "- Docs: $docs_value"
  replace_first_line_re "$tmp" "^- Other:.*$" "- Other: none"

  if [[ -n "$output_path_actual" ]]; then
    replace_first_line_re "$tmp" "^- Write the output card to the paired .*" "- Write the output record to the paired local task bundle sor.md path."
    replace_first_line_re "$tmp" "^[[:space:]]*output_card: .*$" "  output_card: $output_path_actual"
  fi
  apply_input_card_lifecycle "$tmp" "$branch"

  validate_card_header_count "$tmp" "# ADL Input Card" || die "generated input card must contain exactly one '# ADL Input Card' header"
  ensure_nonempty_file "$tmp" || die "generated input card is empty: $tmp"
  mv "$tmp" "$path"
}

seed_output_card() {
  local path="$1" issue="$2" title="$3" branch="$4" ver="$5"
  local task_id run_id issue_slug
  task_id="issue-$(card_issue_pad "$issue")"
  run_id="$task_id"
  issue_slug="$(sanitize_slug "$title")"
  local out_tpl tmp
  out_tpl="$(resolve_output_template)"
  [[ -f "$out_tpl" ]] || die "missing output card template: $out_tpl"

  mkdir -p "$(dirname "$path")"
  tmp="$(mktemp -t prsh_output_card_XXXXXX)"
  render_template "$out_tpl" >"$tmp" || die "failed to render output card template: $out_tpl"
  ensure_nonempty_file "$tmp" || die "rendered output card is empty: $tmp"

  local timestamp status branch_action output_rel
  timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
  if branch_indicates_unbound_state "$branch"; then
    status="NOT_STARTED"
    branch_action="Preserved pre-run branch truth; no execution branch or worktree is bound yet."
  else
    status="IN_PROGRESS"
    branch_action="Reserved the execution branch for later implementation."
  fi
  output_rel="$(path_relative_to_repo "$path")"
  apply_prompt_template_values "$tmp" \
    "<issue>" "$issue" \
    "<issue_padded>" "$(card_issue_pad "$issue")" \
    "<task_id>" "$task_id" \
    "<run_id>" "$run_id" \
    "<version>" "$ver" \
    "<slug>" "$issue_slug" \
    "<title>" "$title" \
    "<branch>" "$branch" \
    "<card_status>" "draft" \
    "<status>" "$status" \
    "<timestamp>" "$timestamp" \
    "<output_card>" "$output_rel" \
    "<branch_action>" "$branch_action" \
    "<initial_pvf_lane>" "needs_planning_lane_assignment" \
    "<planned_pvf_lane>" "needs_planning_lane_assignment" \
    "<final_pvf_lane>" "not_recorded_yet" \
    "<lane_change_reason>" "not_recorded_yet" \
    "<estimated_elapsed_seconds>" "unknown" \
    "<actual_elapsed_seconds>" "unknown" \
    "<estimated_total_tokens>" "unknown" \
    "<actual_total_tokens>" "unknown" \
    "<estimated_validation_seconds>" "unknown" \
    "<actual_validation_seconds>" "unknown" \
    "<actual_metrics_data_source>" "unavailable_before_issue_execution" \
    "<actual_metrics_source_ref>" "not_recorded_yet" \
    "<actual_metrics_confidence>" "unknown" \
    "<estimate_error_percent>" "not_recorded_yet" \
    "<issue_goal_ref>" "issue-${issue}" \
    "<sprint_goal_ref>" "unknown" \
    "<goal_metrics_rollup_ref>" "unknown" \
    "<vpp_card>" "$(path_relative_to_repo "$(task_bundle_dir_path "$issue" "$ver" "$issue_slug")/vpp.md")" \
    "<variance_analysis_required>" "not_recorded_yet" \
    "<variance_analysis_completed>" "not_recorded_yet" \
    "<variance_category>" "not_recorded_yet" \
    "<variance_note>" "No issue execution metrics have been recorded yet."

  set_field_line "$tmp" "Task ID" "$task_id"
  set_field_line "$tmp" "Run ID" "$run_id"
  set_field_line "$tmp" "Version" "$ver"
  set_field_line "$tmp" "Title" "$title"
  set_field_line "$tmp" "Branch" "$branch"
  replace_first_markdown_h1 "$tmp" "$issue_slug"

  # Default Status if template left it blank.
  replace_first_line_re "$tmp" "^Status:[[:space:]]*$" "Status: $status"
  replace_first_line_re "$tmp" "^- Integration state:.*$" "- Integration state: worktree_only"
  replace_first_line_re "$tmp" "^- Verification scope:.*$" "- Verification scope: worktree"
  validate_card_header_count "$tmp" "# $issue_slug" || die "generated output card must contain exactly one '# $issue_slug' header"
  ensure_nonempty_file "$tmp" || die "generated output card is empty: $tmp"
  mv "$tmp" "$path"
}

validate_bootstrap_cards() {
  local issue="$1" slug="$2" branch="$3" in_path="$4" out_path="$5"
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
  output_card_title_matches_slug "$out_path" "$slug" || die "start: output card title mismatch (expected '# $slug')"
}

validate_bootstrap_stp() {
  local path="$1"
  local validator
  validator="$(resolve_structured_prompt_validator)"
  "$validator" --type stp --input "$path" >/dev/null \
    || die "init: stp failed validation: $path"
}

validate_structured_card() {
  local kind="$1" path="$2"
  local validator
  validator="$(resolve_structured_prompt_validator)"
  "$validator" --type "$kind" --phase bootstrap --input "$path" >/dev/null \
    || die "init: $kind failed validation: $path"
}

source_section_one_line() {
  local path="$1" heading="$2" fallback="$3"
  local value
  if [[ -f "$path" ]]; then
    value="$(
      awk -v heading="## ${heading}" '
        $0 == heading { in_section=1; next }
        in_section && /^## / { exit }
        in_section {
          line=$0
          sub(/^[[:space:]]*[-*][[:space:]]+/, "", line)
          sub(/^[[:space:]]+/, "", line)
          sub(/[[:space:]]+$/, "", line)
          if (line != "" && line != "-" && line != "none") {
            if (out != "") { out = out "; " }
            out = out line
          }
        }
        END { print out }
      ' "$path"
    )"
    value="$(trim_ws "$value")"
  fi
  if [[ -n "${value:-}" ]]; then
    printf '%s\n' "$value"
  else
    printf '%s\n' "$fallback"
  fi
}

seed_prompt_template_card() {
  local kind="$1" path="$2" issue="$3" title="$4" branch="$5" ver="$6" source_path="$7" slug="$8"
  local tpl tmp repo issue_url bundle_dir stp_rel sip_rel spp_rel vpp_rel srp_rel sor_rel source_rel target_inline
  local timestamp template_source_note
  local source_summary source_goal source_required_outcome source_deliverables source_acceptance source_repo_inputs
  local source_dependencies source_validation source_demo source_non_goals source_issue_graph source_notes
  tpl="$(resolve_prompt_template "$kind")"
  mkdir -p "$(dirname "$path")"
  tmp="$(mktemp -t prsh_${kind}_card_XXXXXX)"

  repo="$(default_repo)"
  issue_url="https://github.com/${repo}/issues/${issue}"
  timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
  bundle_dir="$(task_bundle_dir_path "$issue" "$ver" "$slug")"
  source_rel="$(path_relative_to_repo "$source_path")"
  stp_rel="$(path_relative_to_repo "$bundle_dir/stp.md")"
  sip_rel="$(path_relative_to_repo "$bundle_dir/sip.md")"
  spp_rel="$(path_relative_to_repo "$bundle_dir/spp.md")"
  vpp_rel="$(path_relative_to_repo "$bundle_dir/vpp.md")"
  srp_rel="$(path_relative_to_repo "$bundle_dir/srp.md")"
  sor_rel="$(path_relative_to_repo "$bundle_dir/sor.md")"
  source_summary="$(source_section_one_line "$source_path" "Summary" "Issue-local task surface for $title.")"
  source_goal="$(source_section_one_line "$source_path" "Goal" "Refine the linked source issue prompt goal.")"
  source_required_outcome="$(source_section_one_line "$source_path" "Required Outcome" "Refine the linked source issue prompt required outcome.")"
  source_deliverables="$(source_section_one_line "$source_path" "Deliverables" "Refine source issue deliverables before execution.")"
  source_acceptance="$(source_section_one_line "$source_path" "Acceptance Criteria" "Refine source issue acceptance criteria before execution.")"
  source_repo_inputs="$(source_section_one_line "$source_path" "Repo Inputs" "$source_rel")"
  source_dependencies="$(source_section_one_line "$source_path" "Dependencies" "none recorded in source issue prompt")"
  source_validation="$(source_section_one_line "$source_path" "Validation Plan" "$(source_section_one_line "$source_path" "Tooling Notes" "Run the smallest proving validation for the touched surface and record it in SOR.")")"
  source_demo="$(source_section_one_line "$source_path" "Demo Expectations" "No demo required unless the source issue says otherwise.")"
  source_non_goals="$(source_section_one_line "$source_path" "Non-goals" "Do not widen scope beyond the linked source issue prompt.")"
  source_issue_graph="$(source_section_one_line "$source_path" "Issue-Graph Notes" "Preserve issue graph truth from the linked source issue prompt.")"
  source_notes="$(source_section_one_line "$source_path" "Notes" "Update this card if execution reality diverges.")"
  target_inline="$source_repo_inputs"

  if [[ -f "$tpl" ]]; then
    render_template "$tpl" >"$tmp" || die "failed to render $kind prompt template: $tpl"
  elif [[ "$kind" == "spp" ]]; then
    cat >"$tmp" <<'EOF'
---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "<slug>-execution-plan"
issue: <issue>
task_id: "issue-<issue_padded>"
run_id: "issue-<issue_padded>"
version: "<version>"
title: "<title>"
branch: "<branch>"
status: "draft"
activation_state: "draft"
plan_revision: 1
source_refs:
  - kind: "issue"
    ref: "<issue_url>"
scope:
  files:
    - "<target_files_surfaces_inline>"
  components:
    - "<slug>"
  out_of_scope:
    - "<non_goals_inline>"
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
confidence: "medium"
plan_summary: "<plan_summary>"
assumptions:
  - "The linked source issue prompt remains canonical."
proposed_steps:
  - id: "step-1"
    description: "Implement only the bounded deliverables: <deliverables_inline>"
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
codex_plan:
  - step: "Implement the bounded deliverables only."
    status: "pending"
affected_areas:
  - "<slug>"
invariants_to_preserve:
  - "Keep SPP issue-local."
risks_and_edge_cases:
  - "<risks_inline>"
test_strategy:
  - "<validation_plan_inline>"
execution_handoff: "Use this SPP as the design-time plan-of-record."
required_permissions:
  - "workspace-write after execution approval"
stop_conditions:
  - "Stop and re-plan if scope changes."
alternatives_considered:
  - description: "Rely only on transient chat planning."
    reason_not_chosen: "Chat-only planning is not durable."
review_hooks:
  - "Check scope truthfulness and validation sufficiency."
notes: "<notes_risks_inline>"
---

# Structured Plan Prompt

## Plan Summary

<plan_summary>

## Codex Plan

1. [pending] Implement the bounded deliverables only.

## Assumptions

- The linked source issue prompt remains canonical.

## Proposed Steps

1. Implement only the bounded deliverables: <deliverables_inline>

## Affected Areas

- <slug>

## Invariants To Preserve

- Keep SPP issue-local.

## Risks And Edge Cases

- <risks_inline>

## Test Strategy

- <validation_plan_inline>

## Execution Handoff

Use this SPP as the design-time plan-of-record.

## Stop Conditions

- Stop and re-plan if scope changes.

## Notes

<notes_risks_inline>
EOF
  elif [[ "$kind" == "srp" ]]; then
    cat >"$tmp" <<'EOF'
---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "<slug>-review-prompt"
issue: <issue>
task_id: "issue-<issue_padded>"
version: "<version>"
title: "<title>"
branch: "<branch>"
status: "draft"
source_refs:
  - kind: "issue"
    ref: "<issue_url>"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
scope_basis:
  - "<stp_card>"
  - "<sip_card>"
in_scope_surfaces:
  - "tracked changes for this issue branch"
evidence_policy:
  - "Use repository evidence and targeted validation output only."
validation_inputs:
  - "Issue-local proofs recorded in the SOR."
allowed_dispositions:
  - "PASS"
  - "BLOCK"
  - "NEEDS_FOLLOWUP"
reviewer_constraints:
  - "Do not widen issue scope."
refusal_policy:
  - "Refuse claims that are unsupported by repository evidence."
follow_up_routing:
  - "Route actionable defects back to the issue branch."
non_claims:
  - "This prompt does not claim review has already run."
policy_refs:
  - "<stp_card>"
notes: "Structured Review Prompt prepared before execution."
---

# Structured Review Prompt

## Review Summary

Use this prompt to govern the independent pre-PR review for this issue.

## Scope Basis

- <stp_card>
- <sip_card>

## In-Scope Surfaces

- tracked changes for this issue branch

## Evidence Rules

- Use repository evidence and targeted validation output only.

## Validation Inputs

- Issue-local proofs recorded in the SOR.

## Allowed Dispositions

- PASS
- BLOCK
- NEEDS_FOLLOWUP

## Reviewer Constraints

- Do not widen issue scope.

## Refusal Policy

- Refuse claims that are unsupported by repository evidence.

## Follow-up Routing

- Route actionable defects back to the issue branch.

## Non-Claims

- This prompt does not claim review has already run.

## Review Results

### Findings

- Not run yet; implementation has not been bound.

### Dispositions

- Not applicable until review runs.

### Recommended Outcome

- Not run yet.

## Notes

Structured Review Prompt prepared before execution.
EOF
  else
    die "missing $kind prompt template: $tpl"
  fi
  ensure_nonempty_file "$tmp" || die "rendered $kind card is empty: $tmp"
  template_source_note="Generated from $(path_relative_to_repo "$tpl")."

  apply_prompt_template_values "$tmp" \
    "<issue>" "$issue" \
    "<issue_padded>" "$(card_issue_pad "$issue")" \
    "<task_id>" "issue-$(card_issue_pad "$issue")" \
    "<run_id>" "issue-$(card_issue_pad "$issue")" \
    "<version>" "$ver" \
    "<slug>" "$slug" \
    "<title>" "$title" \
    "<branch>" "$branch" \
    "<timestamp>" "$timestamp" \
    "<card_status>" "ready" \
    "<status>" "ready" \
    "<activation_state>" "ready" \
    "<issue_url>" "$issue_url" \
    "<source_issue_prompt>" "$source_rel" \
    "<stp_card>" "$stp_rel" \
    "<sip_card>" "$sip_rel" \
    "<spp_card>" "$spp_rel" \
    "<vpp_card>" "$vpp_rel" \
    "<srp_card>" "$srp_rel" \
    "<sor_card>" "$sor_rel" \
    "<output_card>" "$sor_rel" \
    "<wp>" "process" \
    "<required_outcome_type>" "code" \
    "<demo_required>" "false" \
    "<issue_graph_note>" "Versioned C-SDLC prompt template applied; source issue prompt remains the design-time intent source." \
    "<summary>" "$source_summary" \
    "<goal>" "$source_goal" \
    "<required_outcome>" "$source_required_outcome" \
    "<deliverables>" "$source_deliverables" \
    "<acceptance_criteria>" "$source_acceptance" \
    "<repo_inputs>" "$source_repo_inputs" \
    "<dependencies>" "$source_dependencies" \
    "<target_files_surfaces>" "$target_inline" \
    "<validation_plan>" "$source_validation" \
    "<demo_proof_requirements>" "$source_demo" \
    "<non_goals>" "$source_non_goals" \
    "<issue_graph_notes>" "$source_issue_graph" \
    "<notes_risks>" "$source_notes" \
    "<tooling_notes>" "$template_source_note" \
    "<target_files_surfaces_inline>" "$target_inline" \
    "<non_goals_inline>" "$source_non_goals" \
    "<plan_summary>" "Issue-local execution plan for $title." \
    "<dependencies_inline>" "$source_dependencies" \
    "<repo_inputs_inline>" "$source_repo_inputs" \
    "<deliverables_inline>" "$source_deliverables" \
    "<acceptance_criteria_inline>" "$source_acceptance" \
    "<risks_inline>" "Generated card may need editor tightening if the source issue prompt is underspecified." \
    "<validation_plan_inline>" "$source_validation" \
    "<notes_risks_inline>" "$template_source_note Update before continuing if execution diverges." \
    "<initial_pvf_lane>" "needs_planning_lane_assignment" \
    "<planned_pvf_lane>" "needs_planning_lane_assignment" \
    "<planned_pvf_lane_source>" "bootstrap_default_fail_closed" \
    "<estimated_elapsed_seconds>" "unknown" \
    "<estimated_total_tokens>" "unknown" \
    "<estimated_validation_seconds>" "unknown" \
    "<estimate_confidence>" "unknown" \
    "<estimate_data_source>" "unknown" \
    "<estimate_source_ref>" "not_recorded_yet" \
    "<issue_goal_ref>" "issue-${issue}" \
    "<sprint_goal_ref>" "unknown" \
    "<goal_metrics_rollup_ref>" "unknown" \
    "<findings_status>" "not_run" \
    "<recommended_outcome>" "not_applicable"

  if [[ "$kind" == "vpp" ]]; then
    apply_prompt_template_values "$tmp" \
      "<card_status>" "ready" \
      "<status>" "ready" \
      "<initial_pvf_lane>" "needs_planning_lane_assignment" \
      "<planned_pvf_lane>" "needs_planning_lane_assignment" \
      "<lane_registry_path>" "docs/validation/pvf_lanes.json" \
      "<lane_registry_template_set>" "vpp.lane.v1" \
      "<validation_runtime_class>" "unknown" \
      "<validation_resource_profile>" "unknown" \
      "<expected_proof_cost>" "unknown" \
      "<planned_validation_seconds>" "unknown" \
      "<planned_validation_tokens>" "unknown" \
      "<issue_goal_ref>" "issue-${issue}" \
      "<sprint_goal_ref>" "unknown" \
      "<goal_metrics_rollup_ref>" "unknown" \
      "<selected_lanes_inline>" "needs_planning_lane_assignment" \
      "<parallel_groups_inline>" "unknown" \
      "<validation_commands_inline>" "needs_planning_lane_assignment" \
      "<failure_policy>" "fail_closed_until_validation_lane_is_selected"
  fi

  mv "$tmp" "$path"
}

seed_task_bundle_stp() {
  local source_path="$1" dest_path="$2" issue="$3" title="$4" branch="$5" version="$6" slug="$7"
  mkdir -p "$(dirname "$dest_path")"
  if [[ -f "$(repo_root)/$STP_TEMPLATE" ]]; then
    seed_prompt_template_card stp "$dest_path" "$issue" "$title" "$branch" "$version" "$source_path" "$slug"
  else
    cp -f "$source_path" "$dest_path"
  fi
}

seed_bootstrap_surfaces() {
  local issue="$1" version="$2" slug="$3" title="$4" branch="$5" source_path="$6"
  local bundle_dir stp_path in_path out_path
  bundle_dir="$(task_bundle_dir_path "$issue" "$version" "$slug")"
  stp_path="$bundle_dir/stp.md"
  mkdir -p "$bundle_dir"
  if ! ensure_nonempty_file "$stp_path"; then
    note "Creating task-bundle STP: $stp_path" >&2
    seed_task_bundle_stp "$source_path" "$stp_path" "$issue" "$title" "$branch" "$version" "$slug"
  else
    note "Task-bundle STP exists: $stp_path" >&2
  fi

  in_path="$(input_card_path "$issue" "$version" "$slug")"
  out_path="$(output_card_path "$issue" "$version" "$slug")"
  local spp_path vpp_path srp_path
  spp_path="$bundle_dir/spp.md"
  vpp_path="$bundle_dir/vpp.md"
  srp_path="$bundle_dir/srp.md"
  ensure_adl_dirs
  if ! ensure_nonempty_file "$in_path" || input_card_is_bootstrap_stub "$in_path"; then
    note "Creating input card: $in_path" >&2
    seed_input_card "$in_path" "$issue" "$title" "$branch" "$version" "$out_path"
  else
    note "Input card exists: $in_path" >&2
  fi
  if ! ensure_nonempty_file "$out_path" || ! output_card_title_matches_slug "$out_path" "$slug"; then
    note "Creating output card: $out_path" >&2
    seed_output_card "$out_path" "$issue" "$title" "$branch" "$version"
  else
    note "Output card exists: $out_path" >&2
  fi
  if ! ensure_nonempty_file "$spp_path"; then
    note "Creating SPP card: $spp_path" >&2
    seed_prompt_template_card spp "$spp_path" "$issue" "$title" "$branch" "$version" "$source_path" "$slug"
  else
    note "SPP card exists: $spp_path" >&2
  fi
  if ! ensure_nonempty_file "$vpp_path"; then
    note "Creating VPP card: $vpp_path" >&2
    seed_prompt_template_card vpp "$vpp_path" "$issue" "$title" "$branch" "$version" "$source_path" "$slug"
  else
    note "VPP card exists: $vpp_path" >&2
  fi
  if ! ensure_nonempty_file "$srp_path"; then
    note "Creating SRP card: $srp_path" >&2
    seed_prompt_template_card srp "$srp_path" "$issue" "$title" "$branch" "$version" "$source_path" "$slug"
  else
    note "SRP card exists: $srp_path" >&2
  fi
  sync_legacy_links_for_issue "$issue" "$version" "$slug"
  validate_bootstrap_stp "$stp_path"
  validate_structured_card spp "$spp_path"
  validate_structured_card vpp "$vpp_path"
  validate_structured_card srp "$srp_path"
  validate_bootstrap_cards "$issue" "$slug" "$branch" "$in_path" "$out_path"
  printf '%s\n%s\n%s\n%s\n%s\n%s\n' "$stp_path" "$in_path" "$out_path" "$spp_path" "$vpp_path" "$srp_path"
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

pr_run_flag_family() {
  case "$1" in
    --trace|--print-plan|--print-prompts|--print-prompt|--resume|--steer|--overlay|--out|--runs-root|--quiet|--no-step-output|--open|--allow-unsigned)
      printf 'runtime\n'
      ;;
    --prefix|--slug|--title|--no-fetch-issue|--version|--allow-open-pr-wave)
      printf 'issue\n'
      ;;
    *)
      printf 'unknown\n'
      ;;
  esac
}

ensure_pr_run_issue_args_are_issue_only() {
  local issue="$1"
  shift || true
  while [[ $# -gt 0 ]]; do
    local flag="$1"
    local family
    family="$(pr_run_flag_family "$flag")"
    if [[ "$family" == "runtime" ]]; then
      die "run: ambiguous operand '$issue': issue-mode run cannot accept runtime flag '$flag'. Use 'adl/tools/pr.sh run <adl.yaml> ...' for runtime workflows or 'adl/tools/pr.sh run <issue> ...' with issue flags only."
    fi
    case "$flag" in
      --prefix|--slug|--title|--version)
        [[ $# -ge 2 ]] || die_with_usage "run: $flag requires a value" usage_start
        shift 2
        ;;
      --no-fetch-issue|--allow-open-pr-wave)
        shift
        ;;
      *)
        # Let the Rust start parser preserve the exact compatibility behavior
        # for future issue-mode flags that are not classified here.
        shift
        ;;
    esac
  done
}

ensure_pr_run_runtime_args_are_runtime_only() {
  local adl_path="$1"
  shift || true
  while [[ $# -gt 0 ]]; do
    local flag="$1"
    local family
    family="$(pr_run_flag_family "$flag")"
    if [[ "$family" == "issue" ]]; then
      die "run: ambiguous operand '$adl_path': runtime workflow run cannot accept issue flag '$flag'. Use 'adl/tools/pr.sh run <issue> ...' for C-SDLC issue execution or 'adl/tools/pr.sh run <adl.yaml> ...' with runtime flags only."
    fi
    case "$flag" in
      --resume|--steer|--overlay|--out|--runs-root)
        [[ $# -ge 2 ]] || die_with_usage "run: $flag requires a value" usage_run
        shift 2
        ;;
      --print-plan|--print-prompts|--print-prompt|--trace|--quiet|--no-step-output|--open|--allow-unsigned|-h|--help)
        shift
        ;;
      *)
        # The normal runtime parser below owns unknown-runtime-flag diagnostics.
        shift
        ;;
    esac
  done
}

cmd_run() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_run
    return 0
  fi
  adl_obs_event "pr.sh" "run_dispatch" "started" "arg0" "${1:-}"

  if [[ "${1:-}" =~ ^[0-9]+$ ]]; then
    ensure_pr_run_issue_args_are_issue_only "$@"
    require_rust_pr_delegate start
    adl_obs_event "pr.sh" "issue_bind" "started" "issue" "$1"
    note "Issue-mode run: binding execution context for issue $1"
    note "Goal guardrail: call create_goal for issue $1 after bind succeeds and before implementation starts; treat update_goal as a truthful session-terminal record only."
    ADL_PR_SUPPRESS_START_COMPAT_NOTE=1 delegate_pr_command_to_rust start "$@"
    return 0
  fi

  local adl_path="${1:-}"
  [[ -n "$adl_path" ]] || die_with_usage "run: missing <adl.yaml>" usage_run
  ensure_pr_run_runtime_args_are_runtime_only "$@"
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





cmd_init() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_init
    return 0
  fi
  require_rust_pr_delegate init
  delegate_pr_command_to_rust init "$@"
}

cmd_create() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_create
    return 0
  fi
  require_rust_pr_delegate create
  delegate_pr_command_to_rust create "$@"
}

cmd_repair_issue_body() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_repair_issue_body
    return 0
  fi
  require_rust_pr_delegate repair-issue-body
  delegate_pr_command_to_rust repair-issue-body "$@"
}

cmd_start() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_start
    return 0
  fi
  adl_obs_event "pr.sh" "issue_bind" "started" "issue" "${1:-}"
  require_rust_pr_delegate start
  note "Deprecated compatibility path: prefer 'adl/tools/pr.sh run <issue> ...' for execution-context binding."
  ADL_PR_SUPPRESS_START_COMPAT_NOTE=1 delegate_pr_command_to_rust start "$@"
}


cmd_finish() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_finish
    return 0
  fi
  local finish_arg previous_arg title_value_seen
  previous_arg=""
  title_value_seen=0
  for finish_arg in "$@"; do
    if [[ "$previous_arg" == "--title" ]]; then
      if [[ -n "$finish_arg" && "$finish_arg" != --* ]]; then
        title_value_seen=1
      fi
      break
    fi
    case "$finish_arg" in
      --title=*)
        if [[ -n "${finish_arg#--title=}" && "${finish_arg#--title=}" != --* ]]; then
          title_value_seen=1
        fi
        break
        ;;
    esac
    previous_arg="$finish_arg"
  done
  if [[ "$title_value_seen" != "1" ]]; then
    die_with_usage "finish: --title is required" usage_finish
  fi
  adl_obs_event "pr.sh" "finish" "started" "issue" "${1:-}"
  require_rust_pr_delegate finish
  delegate_pr_command_to_rust finish "$@"
}

cmd_validation() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_validation
    return 0
  fi
  adl_obs_event "pr.sh" "validation" "started" "pr" "${1:-}"
  require_rust_pr_delegate validation
  delegate_pr_command_to_rust validation "$@"
}

cmd_watch() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_watch
    return 0
  fi
  adl_obs_event "pr.sh" "watch" "started" "issue" "${1:-}"
  require_rust_pr_delegate watch
  delegate_pr_command_to_rust watch "$@"
}

cmd_closing_linkage() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    note "Usage: adl/tools/pr.sh closing-linkage [--event-name <event>] [--event-path <path>] [--head-ref <branch>] [-R owner/repo]"
    note ""
    note "Runs the Rust-owned PR closing-linkage guard against live PR metadata when token context is present, with event-payload fallback only when live metadata is unavailable."
    return 0
  fi
  adl_obs_event "pr.sh" "closing_linkage" "started"
  require_rust_pr_delegate closing-linkage
  delegate_pr_command_to_rust closing-linkage "$@"
}

cmd_issue() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_issue
    return 0
  fi
  adl_obs_event "pr.sh" "issue" "started" "issue_query" "${1:-}"
  require_rust_pr_delegate issue
  delegate_pr_command_to_rust issue "$@"
}

cmd_projection_map() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    note "Usage: adl/tools/pr.sh projection-map [--json]"
    note ""
    note "Reports the GitHub/C-SDLC projection ownership map for issue, PR, and card surfaces."
    return 0
  fi
  adl_obs_event "pr.sh" "projection-map" "started"
  require_rust_pr_delegate projection-map
  delegate_pr_command_to_rust projection-map "$@"
}

cmd_closeout() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_closeout
    return 0
  fi
  adl_obs_event "pr.sh" "closeout" "started" "issue" "${1:-}"
  require_rust_pr_delegate closeout
  delegate_pr_command_to_rust closeout "$@"
}

cmd_status() {
  require_cmd git
  note "Branch: $(current_branch)"
  git status -sb
}

cmd_ready() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_ready
    return 0
  fi
  require_rust_pr_delegate ready
  note "Deprecated compatibility path: prefer 'adl/tools/pr.sh doctor <issue> --mode ready ...'." >&2
  delegate_pr_command_to_rust ready "$@"
}

cmd_preflight() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_preflight
    return 0
  fi
  require_rust_pr_delegate preflight
  note "Deprecated compatibility path: prefer 'adl/tools/pr.sh doctor <issue> --mode preflight ...'." >&2
  delegate_pr_command_to_rust preflight "$@"
}

cmd_doctor() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" || "${1:-}" == "help" ]]; then
    usage_doctor
    return 0
  fi
  adl_obs_event "pr.sh" "doctor" "started" "issue" "${1:-}"
  require_rust_pr_delegate doctor
  delegate_pr_command_to_rust doctor "$@"
}

cmd_open() {
  die "open: GitHub CLI browser lookup is no longer supported by pr.sh; use the PR URL printed by the Rust finish/create path"
}

source_pr_helper "pr_usage.sh"

main() {
  if [[ "${ADL_PR_SH_TEMPLATE_RESOLVER_SELF_TEST:-0}" == "1" ]]; then
    resolve_prompt_template "${1:-sip}"
    return 0
  fi
  local cmd="${1:-}"; shift || true
  local first_arg="${1:-}"
  if [[ -n "$cmd" && "$cmd" != "help" && "$cmd" != "-h" && "$cmd" != "--help" && "$first_arg" != "help" && "$first_arg" != "-h" && "$first_arg" != "--help" ]]; then
    adl_obs_event "pr.sh" "command_start" "started" "subcommand" "$cmd"
  fi
  case "$cmd" in
    help) usage ;;
    create) cmd_create "$@" ;;
    init) cmd_init "$@" ;;
    repair-issue-body) cmd_repair_issue_body "$@" ;;
    run) cmd_run "$@" ;;
    start) cmd_start "$@" ;;
    doctor) cmd_doctor "$@" ;;
    ready) cmd_ready "$@" ;;
    preflight) cmd_preflight "$@" ;;
    finish) cmd_finish "$@" ;;
    validation) cmd_validation "$@" ;;
    watch) cmd_watch "$@" ;;
    closing-linkage) cmd_closing_linkage "$@" ;;
    issue) cmd_issue "$@" ;;
    projection-map) cmd_projection_map "$@" ;;
    closeout) cmd_closeout "$@" ;;
    card) source_pr_helper "pr_cards.sh"; cmd_card "$@" ;;
    output) source_pr_helper "pr_cards.sh"; cmd_output "$@" ;;
    output-card) source_pr_helper "pr_cards.sh"; cmd_output "$@" ;;
    cards) source_pr_helper "pr_cards.sh"; cmd_cards "$@" ;;
    open) cmd_open ;;
    status) cmd_status ;;
    -h|--help|"") usage ;;
    *) die "Unknown command: $cmd (try --help)" ;;
  esac
}

main "$@"
