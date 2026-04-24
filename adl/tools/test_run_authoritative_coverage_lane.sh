#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

plan="$(bash "$ROOT_DIR/adl/tools/run_authoritative_coverage_lane.sh" --print-plan)"

for required in \
  "authority=push_main" \
  "event_name=push" \
  "mode=full_authoritative_all_features" \
  "features=all_features" \
  "workspace=full"
do
  if ! grep -F "$required" <<<"$plan" >/dev/null 2>&1; then
    echo "missing authoritative coverage plan token: $required" >&2
    exit 1
  fi
done

policy_plan="$(bash "$ROOT_DIR/adl/tools/run_authoritative_coverage_lane.sh" --authority pr_policy_surface_tooling_only --event-name pull_request --print-plan)"

for required in \
  "authority=pr_policy_surface_tooling_only" \
  "event_name=pull_request" \
  "mode=bounded_policy_surface_pr" \
  "features=default" \
  "workspace=bounded_policy_surface"
do
  if ! grep -F "$required" <<<"$policy_plan" >/dev/null 2>&1; then
    echo "missing policy-surface authoritative coverage plan token: $required" >&2
    exit 1
  fi
done

runtime_policy_plan="$(bash "$ROOT_DIR/adl/tools/run_authoritative_coverage_lane.sh" --authority pr_policy_surface_runtime_mixed --event-name pull_request --print-plan)"

for required in \
  "authority=pr_policy_surface_runtime_mixed" \
  "event_name=pull_request" \
  "mode=full_authoritative_all_features" \
  "features=all_features" \
  "workspace=full"
do
  if ! grep -F "$required" <<<"$runtime_policy_plan" >/dev/null 2>&1; then
    echo "missing mixed policy-surface authoritative coverage plan token: $required" >&2
    exit 1
  fi
done

echo "PASS test_run_authoritative_coverage_lane"
