#!/usr/bin/env bash
set -euo pipefail

CARD_PATHS_LIB="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/card_paths.sh"
# shellcheck disable=SC1090
source "$CARD_PATHS_LIB"

die() {
  echo "ERROR: $*" >&2
  exit 1
}

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/editor_action.sh contract [--format text|json]
  adl/tools/editor_action.sh prepare --phase init|doctor-ready|run|finish --issue <number> --slug <slug> [--version <vN.N[.P]>] [--title <title>] [--paths <paths>]
  adl/tools/editor_action.sh start --issue <number> --branch codex/<issue>-<slug> [--slug <slug>] [--dry-run]

Purpose:
  Thin editor-adjacent adapter for bounded control-plane actions.

Current actions:
  contract Print the supported near-term editor adapter surface.
  prepare  Validate fields and print one current lifecycle command for a human to run from the repo root.
  start    Legacy compatibility action for the older v0.85 editor demo path.
USAGE
}

emit_contract_text() {
  cat <<'EOF'
editor_adapter_schema: editor.command_adapter.v2
supported_actions:
  - action: prepare
    adapter_entry: adl/tools/editor_action.sh prepare --phase init|doctor-ready|run|finish --issue <number> --slug <slug> [--version <vN.N[.P]>] [--title <title>] [--paths <paths>]
    maps_to: copy-only adl/tools/pr.sh lifecycle command
    invocation_mode: browser_prepared_human_run
    browser_direct: false
    status: supported
    phases:
      - init
      - doctor-ready
      - run
      - finish
legacy_compatibility_actions:
  - action: start
    adapter_entry: adl/tools/editor_action.sh start --issue <number> --branch codex/<issue>-<slug> [--slug <slug>] [--dry-run]
    maps_to: adl/tools/pr.sh start
    invocation_mode: legacy_thin_adapter
    browser_direct: false
    status: deprecated_compatibility
unsupported_browser_direct_actions:
  - pr create
  - pr init
  - pr doctor
  - pr ready
  - pr run
  - pr finish
  - pr janitor
  - pr closeout
notes:
  - Browser/editor surfaces may prepare or copy lifecycle commands, but must not claim direct browser execution.
  - The current taught path is pr init, pr doctor/ready, pr run, pr finish, pr janitor, and pr closeout through repo-owned tooling and skills.
  - The legacy start action remains only so older deterministic demos can keep validating compatibility until they are retired.
EOF
}

emit_contract_json() {
  cat <<'EOF'
{
  "schema_version": "editor.command_adapter.v2",
  "supported_actions": [
    {
      "action": "prepare",
      "adapter_entry": "adl/tools/editor_action.sh prepare --phase init|doctor-ready|run|finish --issue <number> --slug <slug> [--version <vN.N[.P]>] [--title <title>] [--paths <paths>]",
      "maps_to": "copy-only adl/tools/pr.sh lifecycle command",
      "invocation_mode": "browser_prepared_human_run",
      "browser_direct": false,
      "status": "supported",
      "phases": ["init", "doctor-ready", "run", "finish"]
    }
  ],
  "legacy_compatibility_actions": [
    {
      "action": "start",
      "adapter_entry": "adl/tools/editor_action.sh start --issue <number> --branch codex/<issue>-<slug> [--slug <slug>] [--dry-run]",
      "maps_to": "adl/tools/pr.sh start",
      "invocation_mode": "legacy_thin_adapter",
      "browser_direct": false,
      "status": "deprecated_compatibility"
    }
  ],
  "unsupported_browser_direct_actions": [
    "pr create",
    "pr init",
    "pr doctor",
    "pr ready",
    "pr run",
    "pr finish",
    "pr janitor",
    "pr closeout"
  ],
  "notes": [
    "Browser/editor surfaces may prepare or copy lifecycle commands, but must not claim direct browser execution.",
    "The current taught path is pr init, pr doctor/ready, pr run, pr finish, pr janitor, and pr closeout through repo-owned tooling and skills.",
    "The legacy start action remains only so older deterministic demos can keep validating compatibility until they are retired."
  ]
}
EOF
}

repo_root() {
  git rev-parse --show-toplevel 2>/dev/null || die "Not in a git repo"
}

normalize_issue_or_die() {
  local raw="$1"
  local normalized
  normalized="$(card_issue_normalize "$raw" 2>/dev/null)" || die "invalid issue number: $raw"
  echo "$normalized"
}

derive_slug_from_branch() {
  local issue="$1" branch="$2"
  [[ "$branch" =~ ^codex/([0-9]+)-([a-z0-9][a-z0-9-]*)$ ]] || die "branch must match codex/<issue>-<slug>"
  local branch_issue="${BASH_REMATCH[1]}"
  local slug="${BASH_REMATCH[2]}"
  [[ "$branch_issue" == "$issue" ]] || die "branch issue prefix ($branch_issue) does not match issue number ($issue)"
  echo "$slug"
}

shell_quote() {
  local value="$1"
  printf "'%s'" "${value//\'/\'\\\'\'}"
}

version_or_default() {
  local value="$1"
  if [[ -z "$value" ]]; then
    echo "v0.90"
    return
  fi
  [[ "$value" =~ ^v[0-9]+\.[0-9]+(\.[0-9]+)*$ ]] || die "version must match vN.N or vN.N.P"
  echo "$value"
}

emit_prepare_command() {
  local phase="$1" issue="$2" slug="$3" version="$4" title="$5" paths="$6"
  case "$phase" in
    init)
      printf './adl/tools/pr.sh init %s --slug %s --version %s\n' "$issue" "$slug" "$version"
      ;;
    doctor-ready)
      printf './adl/tools/pr.sh doctor %s --slug %s --version %s --mode ready\n' "$issue" "$slug" "$version"
      ;;
    run)
      printf './adl/tools/pr.sh run %s --slug %s --version %s\n' "$issue" "$slug" "$version"
      ;;
    finish)
      if [[ -z "$title" ]]; then
        title="[${version}] Issue ${issue} closeout"
      fi
      if [[ -z "$paths" ]]; then
        paths="<paths>"
      fi
      printf './adl/tools/pr.sh finish %s --title %s --paths %s\n' "$issue" "$(shell_quote "$title")" "$(shell_quote "$paths")"
      ;;
    *) die "--phase must be one of: init, doctor-ready, run, finish" ;;
  esac
}

ACTION="${1:-}"
[[ -n "$ACTION" ]] || { usage; exit 1; }
shift || true

case "$ACTION" in
  prepare|start|contract) ;;
  -h|--help) usage; exit 0 ;;
  *) die "unsupported action: $ACTION" ;;
esac

ISSUE=""
BRANCH=""
SLUG=""
DRY_RUN=false
FORMAT="text"
PHASE=""
VERSION=""
TITLE=""
PATHS_ARG=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --phase) PHASE="$2"; shift 2 ;;
    --issue) ISSUE="$2"; shift 2 ;;
    --branch) BRANCH="$2"; shift 2 ;;
    --slug) SLUG="$2"; shift 2 ;;
    --dry-run) DRY_RUN=true; shift ;;
    --format) FORMAT="$2"; shift 2 ;;
    --version) VERSION="$2"; shift 2 ;;
    --title) TITLE="$2"; shift 2 ;;
    --paths) PATHS_ARG="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) die "unknown argument: $1" ;;
  esac
done

if [[ "$ACTION" == "contract" ]]; then
  case "$FORMAT" in
    text) emit_contract_text ;;
    json) emit_contract_json ;;
    *) die "--format must be text or json" ;;
  esac
  exit 0
fi

[[ -n "$ISSUE" ]] || die "--issue is required"

ISSUE="$(normalize_issue_or_die "$ISSUE")"

if [[ "$ACTION" == "prepare" ]]; then
  [[ -n "$PHASE" ]] || die "--phase is required"
  [[ -n "$SLUG" ]] || die "--slug is required"
  [[ "$SLUG" =~ ^[a-z0-9][a-z0-9-]*$ ]] || die "slug must match [a-z0-9-]+"
  VERSION="$(version_or_default "$VERSION")"
  emit_prepare_command "$PHASE" "$ISSUE" "$SLUG" "$VERSION" "$TITLE" "$PATHS_ARG"
  exit 0
fi

[[ -n "$BRANCH" ]] || die "--branch is required"

if [[ -z "$SLUG" ]]; then
  SLUG="$(derive_slug_from_branch "$ISSUE" "$BRANCH")"
else
  [[ "$SLUG" =~ ^[a-z0-9][a-z0-9-]*$ ]] || die "slug must match [a-z0-9-]+"
  derive_slug_from_branch "$ISSUE" "$BRANCH" >/dev/null
fi

ROOT="$(repo_root)"
cd "$ROOT"

CMD=(./adl/tools/pr.sh start "$ISSUE" --slug "$SLUG")

if [[ "$DRY_RUN" == true ]]; then
  printf '%s\n' "${CMD[*]}"
  exit 0
fi

exec "${CMD[@]}"
