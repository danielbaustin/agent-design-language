# Issue 5830 Design: Evidence-Grounded Cognitive Profiles

## Outcome And Sources

Define WP-13's bounded ACP profile in current Runtime v3 authority from `docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md` and landed memory, capability, Theory-of-Mind, intelligence, and governed-learning evidence. Retained Runtime v2 evidence may be consumed only through explicit versioned references.

## Owned Paths

- `adl-runtime-kernel/src/cognitive_profile.rs`
- `adl-runtime-kernel/src/lib.rs`
- `adl-runtime-kernel/tests/cognitive_profile.rs`
- `adl-runtime-kernel/tests/fixtures/cognitive_profile`
- `docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md`
- `.csdlc/prepared/issues/5830/validate-native-receipts.rb`
- `.csdlc/prepared/issues/5830/produce-native-receipt.rb`
- `.csdlc/evidence/5830`

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
    "id": "v092-5830-acp-doc-to-final-truth-v1",
    "paths": [
      "docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md"
    ],
    "issues": [
      5830,
      5843
    ],
    "order": [
      5830,
      5843
    ]
  }
]
```

## Contract

Profiles are deterministic evidence maps, not free-form personality labels. Every field must cite an allowed source category and current digest. Updates preserve prior revision linkage and explain additions/removals. Missing evidence, stale or forbidden refs, private-state leakage, unsupported label inference, identity mismatch, and attempts to derive reputation, standing, rights, personhood, or consciousness fail closed.

## Dependencies And Invariants

WP-10/#5827, WP-11/#5828, and WP-12/#5829 must be terminal; the v0.91.1 ToM/intelligence/governed-learning inputs remain bounded prerequisites. Public projection is strictly narrower than the internal evidence map.

## Validation

The exact `cognitive_profile` Runtime v3 integration-test target must run a nonzero count proving canonical records, revision linkage, unsupported-label rejection, stale evidence, root mismatch, forbidden paths, redaction failure, and reputation/standing non-inference. The issue-local producer must run that target on native GitHub Actions macOS and Linux jobs at exact candidate HEAD and retain a hashed source manifest, complete nextest log, and canonical semantic-output artifact. The independent validator recomputes those files and producer digest, parses the positive test count, verifies workflow/run/job identity, and requires byte-identical semantic outputs; ancestral SHA equivalence is forbidden.

## Rollback

Remove only the WP-13 cognitive-profile module, registration, integration
test, fixtures, and owned feature-document edits. Preserve all source evidence,
rejected profile records, and native receipts; rollback must not infer or alter
reputation, standing, identity, or authority.

## Non-Goals

Diagnosis, scalar moral verdicts, reputation, public standing, rights allocation, citizenship, raw private-state access, and autonomous profile mutation are excluded.
