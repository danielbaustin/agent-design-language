#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

for retired in validate_structured_prompt.sh lint_prompt_spec.sh; do
  if [[ -e "adl/tools/$retired" ]]; then
    echo "assertion failed: sunset prompt-validation wrapper was restored: $retired" >&2
    exit 1
  fi
done

# Valid current-schema bootstrap creates all six typed cards and reaches a clean
# doctor result. Invalid or incomplete v1.0.3 registry inputs fail before any
# issue state is authored.
cargo test --quiet --locked --manifest-path csdlc-v2/Cargo.toml --test gate2 \
  bootstrap_constructs_all_six_cards_and_ready_doctor -- --exact
cargo test --quiet --locked --manifest-path csdlc-v2/Cargo.toml --test gate2 \
  native_registry_is_required_and_shape_checked_before_issue_authoring -- --exact

# The current validation owner proves both atomic valid finalization and an
# actual csdlc-validate CLI execution whose machine output redacts lane argv.
cargo test --quiet --locked --manifest-path csdlc-v2/Cargo.toml --test gate4 \
  finalize_is_one_atomic_implemented_transition_and_failure_writes_no_state -- --exact
cargo test --quiet --locked --manifest-path csdlc-v2/Cargo.toml --test gate4 \
  validate_cli_redacts_machine_readable_command -- --exact
python3 adl/tools/test_prompt_template_structure_schemas.py

rg -q '"csdlc_prompt_template_set": "1.0.3"' docs/templates/prompts/current.json
rg -q 'csdlc-validate' csdlc-v2/operator/skills/csdlc-v2-validate/SKILL.md

echo "PASS: typed structured-prompt validation accepts current cards and rejects invalid state"
