#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROMPT_LINT="$ROOT_DIR/adl/tools/lint_prompt_spec.sh"

die() {
  echo "ERROR: $*" >&2
  exit 1
}

usage() {
  cat <<'USAGE'
Usage: validate_structured_prompt.sh --type <stp|sip|sor> --input <path> [--phase <phase>]
USAGE
}

TYPE=""
INPUT=""
PHASE=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --type) TYPE="$2"; shift 2 ;;
    --input) INPUT="$2"; shift 2 ;;
    --phase) PHASE="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) die "unknown argument: $1" ;;
  esac
done

[[ -n "$TYPE" ]] || die "missing --type"
[[ -n "$INPUT" ]] || die "missing --input"
[[ -f "$INPUT" ]] || die "input not found: $INPUT"
case "$TYPE" in
  stp|sip|sor) ;;
  *) die "unsupported --type: $TYPE" ;;
esac

trim() {
  local s="$1"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  printf '%s' "$s"
}

strip_quotes() {
  local s
  s="$(trim "$1")"
  if [[ "$s" == \"*\" && "$s" == *\" ]]; then
    s="${s:1:${#s}-2}"
  elif [[ "$s" == \'*\' && "$s" == *\' ]]; then
    s="${s:1:${#s}-2}"
  fi
  printf '%s' "$s"
}

valid_slug() { [[ "$1" =~ ^[a-z0-9]+(-[a-z0-9]+)*$ ]]; }
valid_task_id() { [[ "$1" =~ ^issue-[0-9]{4}$ ]]; }
valid_version() { [[ "$1" =~ ^v[0-9]+\.[0-9]+$ ]]; }
valid_branch() { [[ "$1" =~ ^codex/[a-z0-9][a-z0-9-]*$ ]]; }
valid_bool() { [[ "$1" == "true" || "$1" == "false" ]]; }
valid_github_issue_url() { [[ "$1" =~ ^https://github\.com/[^/]+/[^/]+/issues/[0-9]+$ ]]; }
valid_github_pr_url() { [[ "$1" =~ ^https://github\.com/[^/]+/[^/]+/pull/[0-9]+$ ]]; }
valid_reference() { [[ "$1" =~ ^https?://.+$ || "$1" =~ ^[A-Za-z0-9._/-]+$ ]]; }
valid_iso8601_datetime() { [[ "$1" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]]; }
valid_freeform_placeholder() { [[ "$1" == "none" || "$1" == "not_applicable" || "$1" == "n/a" ]]; }

contains_absolute_host_path() {
  local file="$1"
  rg -n -e '(^|[^A-Za-z])(\/Users\/|\/home\/|[A-Za-z]:\\)' "$file" >/dev/null 2>&1
}

extract_front_matter() {
  local file="$1"
  local first closer
  first="$(sed -n '1p' "$file")"
  [[ "$(trim "$first")" == "---" ]] || die "missing YAML front matter opener"
  closer="$(awk 'NR>1 && $0 ~ /^---[[:space:]]*$/ { print NR; exit }' "$file")"
  [[ -n "$closer" ]] || die "missing YAML front matter closer"
  sed -n "2,$((closer-1))p" "$file"
}

extract_body() {
  local file="$1"
  local closer
  closer="$(awk 'NR>1 && $0 ~ /^---[[:space:]]*$/ { print NR; exit }' "$file")"
  [[ -n "$closer" ]] || die "missing YAML front matter closer"
  sed -n "$((closer+1)),\$p" "$file"
}

fm_scalar() {
  local file="$1" key="$2"
  awk -v k="$key" '
    index($0, k ":") == 1 {
      sub(/^[^:]*:[[:space:]]*/, "", $0)
      print $0
      exit
    }
  ' "$file"
}

fm_nested_scalar() {
  local file="$1" parent="$2" child="$3"
  awk -v p="$parent" -v c="$child" '
    index($0, p ":") == 1 { in_parent=1; next }
    in_parent && $0 ~ /^[^[:space:]]/ { exit }
    in_parent && index($0, "  " c ":") == 1 {
      sub(/^[^:]*:[[:space:]]*/, "", $0)
      print $0
      exit
    }
  ' "$file"
}

fm_array_count() {
  local file="$1" key="$2"
  awk -v k="$key" '
    BEGIN { count=0; printed=0 }
    index($0, k ":") == 1 { in_block=1; next }
    in_block && $0 ~ /^[^[:space:]]/ { print count; printed=1; exit }
    in_block && $0 ~ /^  - / { count++; next }
    END { if (in_block && !printed) print count }
  ' "$file"
}

section_exists() {
  local file="$1" section="$2"
  awk -v s="$section" '$0 == "## " s { found=1; exit } END { exit(found ? 0 : 1) }' "$file"
}

md_field() {
  local file="$1" key="$2"
  awk -v k="$key" '
    /^## / { if (seen_header) exit }
    {
      if ($0 ~ ("^" k ":")) {
        sub(/^[^:]*:[[:space:]]*/, "", $0)
        print $0
        exit
      }
    }
  ' "$file"
}

md_block_field() {
  local file="$1" block="$2" key="$3"
  awk -v b="$block" -v k="$key" '
    $0 == b ":" || $0 == "## " b { in_block=1; next }
    in_block && /^## / { exit }
    in_block && /^[A-Za-z].*:[[:space:]]*$/ { exit }
    in_block && $0 ~ ("^- " k ":") {
      sub(/^- [^:]*:[[:space:]]*/, "", $0)
      print $0
      exit
    }
  ' "$file"
}

md_block_field_has_content() {
  local file="$1" block="$2" key="$3"
  awk -v b="$block" -v k="$key" '
    BEGIN { found=0 }
    $0 == b ":" || $0 == "## " b { in_block=1; next }
    in_block && /^## / { exit 1 }
    in_block && $0 ~ ("^- " k ":[[:space:]]*$") {
      getline
      while ($0 ~ /^[[:space:]]*$/) {
        if (getline <= 0) exit 1
      }
      if ($0 ~ /^  / || $0 ~ /^- /) { found=1; exit 0 }
      exit 1
    }
    in_block && $0 ~ ("^- " k ":[[:space:]]*.+$") { found=1; exit 0 }
    END { if (!found) exit 1 }
  ' "$file"
}

require_nonblank() {
  local label="$1" value="$2"
  [[ -n "$(trim "$value")" ]] || die "missing required field: $label"
}

require_stp_sections() {
  local body="$1"
  local sections=(
    "Summary"
    "Goal"
    "Required Outcome"
    "Deliverables"
    "Acceptance Criteria"
    "Repo Inputs"
    "Dependencies"
    "Demo Expectations"
    "Non-goals"
    "Issue-Graph Notes"
    "Notes"
    "Tooling Notes"
  )
  local s
  for s in "${sections[@]}"; do
    section_exists "$body" "$s" || die "missing required section: $s"
  done
}

require_sip_sections() {
  local file="$1"
  local sections=(
    "Goal"
    "Required Outcome"
    "Acceptance Criteria"
    "Inputs"
    "Target Files / Surfaces"
    "Validation Plan"
    "Demo / Proof Requirements"
    "Constraints / Policies"
    "System Invariants (must remain true)"
    "Reviewer Checklist (machine-readable hints)"
    "Non-goals / Out of scope"
    "Notes / Risks"
    "Instructions to the Agent"
  )
  local s
  for s in "${sections[@]}"; do
    section_exists "$file" "$s" || die "missing required section: $s"
  done
}

require_sor_sections() {
  local file="$1"
  local sections=(
    "Summary"
    "Artifacts produced"
    "Actions taken"
    "Main Repo Integration (REQUIRED)"
    "Validation"
    "Verification Summary"
    "Determinism Evidence"
    "Security / Privacy Checks"
    "Replay Artifacts"
    "Artifact Verification"
    "Decisions / Deviations"
    "Follow-ups / Deferred work"
  )
  local s
  for s in "${sections[@]}"; do
    section_exists "$file" "$s" || die "missing required section: $s"
  done
}

validate_stp() {
  local file="$1" fm body
  fm="$(mktemp -t stp_fm_XXXXXX)"
  body="$(mktemp -t stp_body_XXXXXX)"
  trap 'rm -f "$fm" "$body"' RETURN
  extract_front_matter "$file" >"$fm"
  extract_body "$file" >"$body"

  require_stp_sections "$body"

  local v
  v="$(strip_quotes "$(fm_scalar "$fm" "issue_card_schema")")"; require_nonblank "issue_card_schema" "$v"; [[ "$v" == "adl.issue.v1" ]] || die "issue_card_schema must be one of: adl.issue.v1"
  v="$(strip_quotes "$(fm_scalar "$fm" "wp")")"; require_nonblank "wp" "$v"
  v="$(strip_quotes "$(fm_scalar "$fm" "slug")")"; require_nonblank "slug" "$v"; valid_slug "$v" || die "slug must be a normalized slug"
  v="$(strip_quotes "$(fm_scalar "$fm" "title")")"; require_nonblank "title" "$v"
  v="$(fm_array_count "$fm" "labels")"; [[ "$v" -ge 1 ]] || die "labels must contain at least 1 item(s)"
  v="$(strip_quotes "$(fm_scalar "$fm" "issue_number")")"; require_nonblank "issue_number" "$v"; [[ "$v" =~ ^[0-9]+$ ]] || die "issue_number must be an integer"
  v="$(strip_quotes "$(fm_scalar "$fm" "status")")"; require_nonblank "status" "$v"; [[ "$v" =~ ^(draft|active|complete)$ ]] || die "status must be one of: draft, active, complete"
  v="$(strip_quotes "$(fm_scalar "$fm" "action")")"; require_nonblank "action" "$v"; [[ "$v" =~ ^(create|edit|close|split|supersede)$ ]] || die "action must be one of: create, edit, close, split, supersede"
  v="$(fm_array_count "$fm" "depends_on")"; [[ -n "$v" ]] || die "missing required field: depends_on"
  v="$(strip_quotes "$(fm_scalar "$fm" "milestone_sprint")")"; require_nonblank "milestone_sprint" "$v"
  v="$(fm_array_count "$fm" "required_outcome_type")"; [[ "$v" -ge 1 ]] || die "required_outcome_type must contain at least 1 item(s)"
  v="$(fm_array_count "$fm" "repo_inputs")"; [[ -n "$v" ]] || die "missing required field: repo_inputs"
  v="$(fm_array_count "$fm" "canonical_files")"; [[ -n "$v" ]] || die "missing required field: canonical_files"
  v="$(strip_quotes "$(fm_scalar "$fm" "demo_required")")"; require_nonblank "demo_required" "$v"; valid_bool "$v" || die "demo_required must be true or false"
  v="$(fm_array_count "$fm" "demo_names")"; [[ -n "$v" ]] || die "missing required field: demo_names"
  v="$(fm_array_count "$fm" "issue_graph_notes")"; [[ -n "$v" ]] || die "missing required field: issue_graph_notes"
  v="$(strip_quotes "$(fm_nested_scalar "$fm" "pr_start" "enabled")")"; require_nonblank "pr_start.enabled" "$v"; valid_bool "$v" || die "pr_start.enabled must be true or false"
  v="$(strip_quotes "$(fm_nested_scalar "$fm" "pr_start" "slug")")"; require_nonblank "pr_start.slug" "$v"; valid_slug "$v" || die "pr_start.slug must be a normalized slug"
}

validate_sip() {
  local file="$1" v
  require_sip_sections "$file"
  v="$(trim "$(md_field "$file" "Task ID")")"; require_nonblank "Task ID" "$v"; valid_task_id "$v" || die "Task ID must match issue-0000"
  v="$(trim "$(md_field "$file" "Run ID")")"; require_nonblank "Run ID" "$v"; valid_task_id "$v" || die "Run ID must match issue-0000"
  v="$(trim "$(md_field "$file" "Version")")"; require_nonblank "Version" "$v"; valid_version "$v" || die "Version must match v0.85-style version format"
  v="$(trim "$(md_field "$file" "Title")")"; require_nonblank "Title" "$v"
  v="$(trim "$(md_field "$file" "Branch")")"; require_nonblank "Branch" "$v"; valid_branch "$v" || die "Branch must be a codex/ branch"
  v="$(trim "$(md_block_field "$file" "Context" "Issue")")"; require_nonblank "Context.Issue" "$v"; valid_github_issue_url "$v" || die "Context.Issue must be a GitHub issue URL"
  v="$(trim "$(md_block_field "$file" "Context" "PR")")"; [[ -z "$v" || $(valid_github_pr_url "$v"; echo $?) -eq 0 ]] || die "Context.PR must be a GitHub PR URL"
  v="$(trim "$(md_block_field "$file" "Context" "Source Issue Prompt")")"; require_nonblank "Context.Source Issue Prompt" "$v"; valid_reference "$v" || die "Context.Source Issue Prompt must be a repo-relative reference or URL"
  v="$(trim "$(md_block_field "$file" "Context" "Docs")")"; require_nonblank "Context.Docs" "$v"
  v="$(trim "$(md_block_field "$file" "Context" "Other")")"; require_nonblank "Context.Other" "$v"
  v="$(trim "$(md_block_field "$file" "Execution" "Source issue-prompt slug")")"; [[ -z "$v" || $(valid_slug "$v"; echo $?) -eq 0 ]] || die "Execution.Source issue-prompt slug must be a normalized slug"
  v="$(trim "$(md_block_field "$file" "Execution" "Required outcome type")")"; [[ -z "$v" || "$v" =~ ^(code|docs|tests|demo|combination)$ ]] || die "Execution.Required outcome type must be one of: code, docs, tests, demo, combination"
  v="$(trim "$(md_block_field "$file" "Execution" "Demo required")")"; [[ -z "$v" || "$v" == "true" || "$v" == "false" ]] || die "Execution.Demo required must be true or false"

  "$PROMPT_LINT" --input "$file" >/dev/null 2>&1 || die "Prompt Spec lint failed for $file"
}

validate_sor() {
  local file="$1" v status start_time end_time integration_state verification_result summary_text actions_line validation_line
  require_sor_sections "$file"
  v="$(trim "$(md_field "$file" "Task ID")")"; require_nonblank "Task ID" "$v"; valid_task_id "$v" || die "Task ID must match issue-0000"
  v="$(trim "$(md_field "$file" "Run ID")")"; require_nonblank "Run ID" "$v"; valid_task_id "$v" || die "Run ID must match issue-0000"
  v="$(trim "$(md_field "$file" "Version")")"; require_nonblank "Version" "$v"; valid_version "$v" || die "Version must match v0.85-style version format"
  v="$(trim "$(md_field "$file" "Title")")"; require_nonblank "Title" "$v"
  v="$(trim "$(md_field "$file" "Branch")")"; require_nonblank "Branch" "$v"; valid_branch "$v" || die "Branch must be a codex/ branch"
  status="$(trim "$(md_field "$file" "Status")")"; require_nonblank "Status" "$status"; [[ "$status" =~ ^(NOT_STARTED|IN_PROGRESS|DONE|FAILED)$ ]] || die "Status must be one of: NOT_STARTED, IN_PROGRESS, DONE, FAILED"
  start_time="$(trim "$(md_block_field "$file" "Execution" "Start Time")")"; [[ -z "$start_time" || $(valid_iso8601_datetime "$start_time"; echo $?) -eq 0 ]] || die "Execution.Start Time must be UTC ISO 8601 / RFC3339 with trailing Z (YYYY-MM-DDTHH:MM:SSZ)"
  end_time="$(trim "$(md_block_field "$file" "Execution" "End Time")")"; [[ -z "$end_time" || $(valid_iso8601_datetime "$end_time"; echo $?) -eq 0 ]] || die "Execution.End Time must be UTC ISO 8601 / RFC3339 with trailing Z (YYYY-MM-DDTHH:MM:SSZ)"
  integration_state="$(trim "$(md_block_field "$file" "Main Repo Integration (REQUIRED)" "Integration state")")"; [[ -z "$integration_state" || "$integration_state" =~ ^(worktree_only|pr_open|merged)$ ]] || die "Main Repo Integration.Integration state must be one of: worktree_only, pr_open, merged"
  v="$(trim "$(md_block_field "$file" "Main Repo Integration (REQUIRED)" "Verification scope")")"; [[ -z "$v" || "$v" =~ ^(worktree|pr_branch|main_repo)$ ]] || die "Main Repo Integration.Verification scope must be one of: worktree, pr_branch, main_repo"
  verification_result="$(trim "$(md_block_field "$file" "Main Repo Integration (REQUIRED)" "Result")")"; [[ -z "$verification_result" || "$verification_result" =~ ^(PASS|FAIL)$ ]] || die "Main Repo Integration.Result must be one of: PASS, FAIL"

  if [[ "$PHASE" == "completed" ]]; then
    [[ "$status" =~ ^(DONE|FAILED)$ ]] || die "completed-phase SOR Status must be DONE or FAILED"

    summary_text="$(awk '/^## Summary$/ { getline; while ($0 ~ /^[[:space:]]*$/) getline; print; exit }' "$file")"
    actions_line="$(awk '/^## Actions taken$/ { getline; while ($0 ~ /^[[:space:]]*$/) getline; print; exit }' "$file")"
    validation_line="$(awk '/^## Validation$/ { getline; while ($0 ~ /^[[:space:]]*$/) getline; print; exit }' "$file")"

    [[ -n "$start_time" ]] || die "completed-phase SOR requires Execution.Start Time"
    [[ -n "$end_time" ]] || die "completed-phase SOR requires Execution.End Time"
    [[ -n "$(trim "$summary_text")" ]] || die "completed-phase SOR requires non-empty Summary content"
    [[ -n "$(trim "$actions_line")" && ! "$(trim "$actions_line")" =~ ^-?[[:space:]]*$ ]] || die "completed-phase SOR requires non-empty Actions taken content"
    [[ -n "$(trim "$validation_line")" && ! "$(trim "$validation_line")" =~ ^-?[[:space:]]*$ ]] || die "completed-phase SOR requires non-empty Validation content"
    [[ -n "$integration_state" && "$integration_state" != "worktree_only" ]] || die "completed-phase SOR requires Integration state to be pr_open or merged"
    if [[ -z "$verification_result" ]]; then
      md_block_field_has_content "$file" "Main Repo Integration (REQUIRED)" "Result" || die "completed-phase SOR requires Main Repo Integration.Result"
    fi
  fi
}

contains_absolute_host_path "$INPUT" && die "$TYPE contains disallowed absolute host path"

case "$TYPE" in
  stp) validate_stp "$INPUT" ;;
  sip) validate_sip "$INPUT" ;;
  sor) validate_sor "$INPUT" ;;
esac

printf 'PASS: %s contract valid for %s\n' "$TYPE" "$INPUT"
