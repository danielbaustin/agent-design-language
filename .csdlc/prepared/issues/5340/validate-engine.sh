#!/usr/bin/env bash
set -euo pipefail

mode="${1:-}"
case "${mode}" in
  warm-cache|focused|quality|determinism|budgets) ;;
  *) echo "usage: $0 warm-cache|focused|quality|determinism|budgets" >&2; exit 64 ;;
esac

repo_root="$(git rev-parse --show-toplevel)"
common="$(git rev-parse --path-format=absolute --git-common-dir)"
primary="$(dirname "${common}")"
validator="${primary}/.adl/bin/csdlc-v2/csdlc-validate"
request="${repo_root}/.csdlc/prepared/issues/5340/pvf/${mode}.json"
source "${repo_root}/.csdlc/prepared/issues/5340/fetch-dependency.sh"

output="$(${validator} --request "${request}")"
printf '%s\n' "${output}"
lane="engine-${mode}"
jq -e --arg lane "${lane}" '
  .schema == "csdlc.pvf.report.v1" and
  .disposition == "local_pass" and
  (.evidence | length) == 1 and
  .evidence[0].lane == $lane and
  .evidence[0].status == "passed"
' <<<"${output}" >/dev/null
