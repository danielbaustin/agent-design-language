#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 6 ]; then
  echo "usage: run_operational_selector_transition.sh <selector-executable> <authenticated-health-probe-executable> <candidate-ref> <prior-ref> <candidate-health-url> <prior-health-url>" >&2
  exit 64
fi

selector=$1
health_probe=$2
candidate_ref=$3
prior_ref=$4
candidate_health=$5
prior_health=$6

case "$candidate_health" in https://*) ;; *) echo "candidate health URL must use HTTPS" >&2; exit 65 ;; esac
case "$prior_health" in https://*) ;; *) echo "prior health URL must use HTTPS" >&2; exit 65 ;; esac

"$selector" activate --selector "$candidate_ref"
if ! "$health_probe" "$candidate_health"; then
  "$selector" activate --selector "$prior_ref"
  "$health_probe" "$prior_health"
  echo "candidate Runtime v3 health failed; prior selector was restored" >&2
  exit 70
fi
"$selector" activate --selector "$prior_ref"
"$health_probe" "$prior_health"
printf 'operational_selector_transition candidate=%s prior=%s health=restored\n' "$candidate_ref" "$prior_ref"
