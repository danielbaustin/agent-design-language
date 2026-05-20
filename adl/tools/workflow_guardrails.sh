#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  adl/tools/workflow_guardrails.sh main-write [--repo <path>]
  adl/tools/workflow_guardrails.sh closeout-watch --version <version> [--repo <owner/name>] [--root <repo-root>] [--report <path>]
  adl/tools/workflow_guardrails.sh safe-report-command (--command <text> | --file <path>)
  adl/tools/workflow_guardrails.sh card-drift --issue <number> --version <version> [--slug <slug>] [--root <repo-root>] [--mode <full|ready|preflight>] [--json]

Workflow guardrails for Sprint 3 / WP-16:
- fail closed on dirty tracked writes from main/master
- surface pending closeout-wave candidates
- reject unsafe shell command shapes for Markdown report generation
- delegate card-drift checks to `pr doctor`
EOF
}

die() {
  echo "workflow-guardrails: $*" >&2
  exit 1
}

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
default_repo_root="$(cd "${script_dir}/../.." && pwd)"

repo_root_from() {
  local root="${1:-}"
  if [[ -n "$root" ]]; then
    printf '%s\n' "$root"
  else
    git rev-parse --show-toplevel 2>/dev/null || die "unable to determine repo root"
  fi
}

cmd_main_write() {
  local repo=""
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --repo) repo="${2:-}"; shift 2 ;;
      -h|--help) usage; exit 0 ;;
      *) die "main-write: unknown arg: $1" ;;
    esac
  done

  repo="$(repo_root_from "$repo")"
  local branch
  branch="$(git -C "$repo" rev-parse --abbrev-ref HEAD)"
  if [[ "$branch" != "main" && "$branch" != "master" ]]; then
    echo "PASS main-write branch=$branch"
    return 0
  fi

  local tracked_status
  tracked_status="$(git -C "$repo" status --porcelain --untracked-files=no)"
  if [[ -z "$tracked_status" ]]; then
    echo "PASS main-write branch=$branch clean=true"
    return 0
  fi

  echo "BLOCKED main-write branch=$branch clean=false" >&2
  echo "$tracked_status" >&2
  die "tracked changes on $branch would make workflow execution unsafe; move the changes into a bound issue worktree or clear them first"
}

cmd_closeout_watch() {
  local version=""
  local repo_slug=""
  local repo_root=""
  local report=""
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --version) version="${2:-}"; shift 2 ;;
      --repo) repo_slug="${2:-}"; shift 2 ;;
      --root) repo_root="${2:-}"; shift 2 ;;
      --report) report="${2:-}"; shift 2 ;;
      -h|--help) usage; exit 0 ;;
      *) die "closeout-watch: unknown arg: $1" ;;
    esac
  done
  [[ -n "$version" ]] || die "closeout-watch: missing --version"
  repo_root="$(repo_root_from "${repo_root:-$default_repo_root}")"
  if [[ -z "$report" ]]; then
    report="${repo_root}/.adl/reports/closeout/closeout-watch-${version}.md"
  fi

  local cmd=(bash "${repo_root}/adl/tools/closeout_completed_issue_wave.sh" --version "$version" --report "$report" --report-only)
  if [[ -n "$repo_slug" ]]; then
    cmd+=(--repo "$repo_slug")
  fi
  (cd "$repo_root" && "${cmd[@]}")

  local candidate_count
  candidate_count="$(awk '/^candidate_issues:/ {print $2; exit}' "$report")"
  candidate_count="${candidate_count:-0}"
  if [[ "$candidate_count" != "0" ]]; then
    echo "BLOCKED closeout-watch version=$version candidates=$candidate_count report=$report" >&2
    die "closed/completed issues still require local closeout; run the closeout wave before treating the sprint as clean"
  fi

  echo "PASS closeout-watch version=$version candidates=0 report=$report"
}

cmd_safe_report_command() {
  local command_text=""
  local command_file=""
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --command) command_text="${2:-}"; shift 2 ;;
      --file) command_file="${2:-}"; shift 2 ;;
      -h|--help) usage; exit 0 ;;
      *) die "safe-report-command: unknown arg: $1" ;;
    esac
  done
  if [[ -n "$command_file" ]]; then
    [[ -f "$command_file" ]] || die "safe-report-command: missing file: $command_file"
    command_text="$(cat "$command_file")"
  fi
  [[ -n "$command_text" ]] || die "safe-report-command: provide --command or --file"

  if has_unsafe_command_substitution "$command_text"; then
    echo "BLOCKED safe-report-command" >&2
    echo "Unsafe command substitution detected in report-generation command." >&2
    echo "Use a quoted heredoc (for example, <<'EOF') or a language-native writer instead." >&2
    exit 1
  fi

  echo "PASS safe-report-command"
}

has_unsafe_command_substitution() {
  local command_text="$1"
  local heredoc_end=""
  local single_marker="<<'"
  local double_marker='<<"'
  local slash_marker="<<\\"
  local line scan after delimiter
  while IFS= read -r line || [[ -n "$line" ]]; do
    if [[ -n "$heredoc_end" ]]; then
      if [[ "$line" == "$heredoc_end" ]]; then
        heredoc_end=""
      fi
      continue
    fi

    scan="$line"
    if [[ "$line" == *"$single_marker"* ]]; then
      scan="${line%%${single_marker}*}"
      after="${line#*${single_marker}}"
      delimiter="${after%%\'*}"
      [[ -n "$delimiter" ]] && heredoc_end="$delimiter"
    elif [[ "$line" == *"$double_marker"* ]]; then
      scan="${line%%${double_marker}*}"
      after="${line#*${double_marker}}"
      delimiter="${after%%\"*}"
      [[ -n "$delimiter" ]] && heredoc_end="$delimiter"
    elif [[ "$line" == *"$slash_marker"* ]]; then
      scan="${line%%${slash_marker}*}"
      after="${line#*${slash_marker}}"
      delimiter="${after%%[[:space:]]*}"
      [[ -n "$delimiter" ]] && heredoc_end="$delimiter"
    fi

    if grep -Eq '\$\(|`' <<<"$scan"; then
      return 0
    fi
  done <<<"$command_text"

  return 1
}

cmd_card_drift() {
  local issue=""
  local version=""
  local slug=""
  local repo_root=""
  local mode="full"
  local emit_json="0"
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --issue) issue="${2:-}"; shift 2 ;;
      --version) version="${2:-}"; shift 2 ;;
      --slug) slug="${2:-}"; shift 2 ;;
      --root) repo_root="${2:-}"; shift 2 ;;
      --mode) mode="${2:-}"; shift 2 ;;
      --json) emit_json="1"; shift ;;
      -h|--help) usage; exit 0 ;;
      *) die "card-drift: unknown arg: $1" ;;
    esac
  done
  [[ -n "$issue" ]] || die "card-drift: missing --issue"
  [[ -n "$version" ]] || die "card-drift: missing --version"
  repo_root="$(repo_root_from "${repo_root:-$default_repo_root}")"

  local cmd=(bash "${repo_root}/adl/tools/pr.sh" doctor "$issue" --version "$version" --mode "$mode")
  if [[ -n "$slug" ]]; then
    cmd+=(--slug "$slug")
  fi
  if [[ "$emit_json" == "1" ]]; then
    cmd+=(--json)
  fi
  (cd "$repo_root" && "${cmd[@]}")
}

subcommand="${1:-}"
if [[ -z "$subcommand" || "$subcommand" == "-h" || "$subcommand" == "--help" || "$subcommand" == "help" ]]; then
  usage
  exit 0
fi
shift || true

case "$subcommand" in
  main-write) cmd_main_write "$@" ;;
  closeout-watch) cmd_closeout_watch "$@" ;;
  safe-report-command) cmd_safe_report_command "$@" ;;
  card-drift) cmd_card_drift "$@" ;;
  *) die "unknown subcommand: $subcommand" ;;
esac
