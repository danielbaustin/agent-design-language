# Structured Output Record

Template: 1.0.0

Issue: 5788

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Current-target owner builds now delegate through one --locked Cargo validation boundary that restores invocation-created tracked lockfile drift to exact pre-invocation bytes.

## Artifacts

- adl/tools/install_owner_binaries.sh
- adl/tools/run_cargo_validation.sh
- adl/tools/run_owner_validation_lane.sh
- adl/tools/test_owner_binary_install.sh
- adl/tools/test_owner_validation_lane.sh
- adl/tools/test_run_cargo_validation.sh

## Execution

- Removed sunset v1 binaries from the default owner inventory.
- Delegated owner validation builds to the hardened installer.
- Added exact-byte lockfile capture, drift reporting, and restoration.
- Removed a nested broad C-SDLC execution from the focused plan contract.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_owner_binary_install.sh",
      "&&",
      "bash",
      "adl/tools/test_run_cargo_validation.sh",
      "&&",
      "bash",
      "adl/tools/test_owner_validation_lane.sh"
    ],
    "purpose": "Prove exact lock restoration, current inventory, --locked enforcement, and focused validation delegation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5788/focused-tooling-contracts"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
