#!/usr/bin/env bash
set -euo pipefail

activation_map="docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md"
handoff="docs/milestones/v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md"
feature="docs/milestones/v0.91.8/features/V092_HANDOFF_v0.91.8.md"
ledger="docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md"
readme="docs/milestones/v0.92/README.md"

git diff --check

grep -Fq "V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md" "$readme"

for hash in \
  11151e0beab02b1667f6505b7f8992bfd47d2f8f \
  fc75f4fc697262f89f99461679a406be0b4b3775 \
  f7258b07e9da414bfee518f0c89a76071bc03ee8 \
  d4825d4be9ed14ed6060dd33cbdafe5eaa5efcd2 \
  e1b6a34e4763a79d1c40c641e64c0c061a0aa96c
do
  grep -Fq "$hash" "$activation_map"
  grep -Fq "$hash" "$ledger"
done

for disposition in \
  accepted_platform_input \
  handoff_owned \
  blocked_with_evidence \
  deferred_non_claim
do
  grep -Fq "$disposition" "$activation_map"
done

grep -Fq "#4762" "$activation_map"
grep -Fq "#4762" "$handoff"
grep -Fq "#4762" "$feature"
grep -Fq "#4762" "$ledger"

if grep -E -n '(/Users/|/private/tmp|github\.token|OPENAI_API_KEY|AWS_SECRET_ACCESS_KEY|BEGIN [A-Z ]*PRIVATE KEY)' \
  "$activation_map" "$handoff" "$feature" "$ledger"
then
  echo "host-local path or secret marker found" >&2
  exit 1
fi

if grep -E -n 'v0\.92 (is )?(ready|activation ready)|birthday readiness: ready' \
  "$activation_map" "$handoff" "$feature" "$ledger"
then
  echo "v0.92 readiness overclaim found" >&2
  exit 1
fi

echo "activation bridge validation passed"
