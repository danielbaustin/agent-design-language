# Issue 5825 Design: Birthday Contract And Negative Cases

## Outcome And Sources

Define the deterministic WP-08 birth decision consumed by the Birthday sprint. The contract is grounded in `docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md`, the negative suite in `docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md`, and the WP-08 row in `docs/milestones/v0.92/WBS_v0.92.md`.

## Owned Paths

- `adl-runtime-kernel/src/birthday.rs`
- `adl-runtime-kernel/src/lib.rs`
- `adl-runtime-kernel/tests/birthday.rs`
- `adl-runtime-kernel/tests/fixtures/birthday`
- `docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md`
- `.csdlc/prepared/issues/5825/validate-native-receipts.rb`
- `.csdlc/prepared/issues/5825/produce-native-receipt.rb`
- `.csdlc/evidence/5825`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Serialization Gates

```json
[
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-birthday-kernel-registration-v1",
    "paths": [
      "adl-runtime-kernel/src/lib.rs"
    ],
    "issues": [
      5825,
      5826,
      5827,
      5828,
      5829,
      5830,
      5831,
      5833
    ],
    "order": [
      5825,
      5826,
      5827,
      5828,
      5829,
      5830,
      5831,
      5833
    ]
  },
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-5825-birthday-doc-to-final-truth-v1",
    "paths": [
      "docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md"
    ],
    "issues": [
      5825,
      5843
    ],
    "order": [
      5825,
      5843
    ]
  }
]
```

## Contract

A birth result requires stable name and identity root, continuity head across bounded cycles, redaction-safe memory grounding, capability envelope, bounded ACP evidence, inherited moral context, witness set, receipt, and reviewer-visible validation. Process startup, task execution, snapshot, wake/resume, restore, admission, copied state, dormant rehydration, simulation, migration, suspension, restart, provisional citizenship, or any packet missing a required evidence surface must return a stable rejection reason.

## Dependencies And Invariants

WP-01/#5817 and WP-02A/#5801 must be terminally proven before execution. WP-01B/#5818 and WP-02/#5819 are distinct work packages and cannot satisfy those gates. The decision is deterministic over canonical inputs, fails closed on missing or contradictory evidence, and never upgrades existing v0.91.x birthday non-claims by implication.

## Validation

Focused tests must invoke the exact `birthday` integration-test target, assert that at least one test ran, and prove one valid packet plus every table-driven disqualifier and missing-evidence case. A claim-boundary scan rejects personhood, consciousness, production citizenship, governance, migration, and transport overclaims. The issue-local native producer must run that exact target on native GitHub Actions macOS and Linux jobs at the exact candidate HEAD, emit a hashed source manifest, preserve the complete nextest log, and require the test to write a canonical semantic-output artifact. The independent validator recomputes every digest, parses a positive test count from the retained log, validates workflow/run/job identity, and requires byte-identical semantic outputs; an ancestral or source-equivalent SHA is not accepted.

## Rollback

Remove only the WP-08 birthday module, its registration, integration test, and
fixtures, then restore the owned feature-document revision. Preserve all
historical birth-decision evidence and failed native receipts; rollback must
not alter predecessor records or convert a rejected candidate into a birth.

## Non-Goals

WP-09 identity construction, WP-10 continuity implementation, WP-11 memory behavior, WP-12 capability production, WP-13 profiles, WP-15 witnesses, WP-16 packet assembly, public launch, and v0.93 governance are outside this issue.
