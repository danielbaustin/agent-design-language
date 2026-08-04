# #4762 Birth Witness Receipt Execution Design

## Purpose

This artifact records the implemented #4762 witness/receipt package shape for
v0.91.8 to v0.92 handoff consumption. It is a retained design for the package
that now exists under:

- `docs/milestones/v0.91.8/review/v092_handoff_4762/`
- `.csdlc/prepared/issues/4762/`

The package is an auditable pre-birth handoff surface. It does not claim that
the v0.92 birthday occurred.

## Implemented Surfaces

The package has five retained surfaces:

- `birth-witness-receipt-schema.v1.json`: the issue-local contract for required
  register fields, receipt fields, witness IDs, negative-case IDs, source
  evidence paths, and forbidden claims.
- `birth-witness-receipt-negative-cases.v1.json`: fail-closed negative-case
  dispositions for startup, ordinary task execution, snapshots, wakes,
  checkpoint restores, test fixtures, copied state, simulations, migration,
  forced suspension, missing evidence, and unsupported profile labels.
- `birth-witness-register-4762.v1.json`: reviewer-facing witness records that
  cite stable source evidence without exposing raw private state.
- `birth-receipt-4762.v1.json`: citizen/reviewer-facing receipt that names
  future required evidence surfaces, links the witness register and negative
  cases, and preserves explicit claim boundaries.
- `validate_birth_receipt_package.rb`: deterministic local validator for
  retained package shape, source-path presence, witness coverage, negative-case
  rejection, and claim-boundary coverage.

## Witness Model

The register intentionally uses source-boundary witnesses rather than live
attestations. That keeps the v0.91.8 handoff package auditable before the
future v0.92 birth event exists.

Required witness IDs:

- `identity-continuity-source-witness`
- `memory-capability-source-witness`
- `negative-case-boundary-witness`
- `handoff-consumption-witness`

Each witness records:

- a stable identifier
- a role
- a reviewer-facing attestation
- source evidence references
- redaction posture

The witness records are not raw memory, private state, live identity roots, or
future continuity heads.

## Receipt Model

The receipt distinguishes implemented handoff artifacts from future birth-event
evidence.

Implemented for #4762:

- witness set as a retained register
- citizen-facing receipt surface
- negative-case dispositions
- handoff consumer references

Still required for any future v0.92 birthday claim:

- stable name and identity root
- continuity record and head
- redaction-safe memory grounding
- capability envelope
- activation trace
- validation output
- reviewer packet evidence

Missing evidence remains a blocker, not deferred success.

## Integration

The package is consumed by exact path from:

- `docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md`
- `docs/milestones/v0.91.8/features/V092_HANDOFF_v0.91.8.md`
- `docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md`

These references prove the handoff path can cite the package. They do not
publish, merge, close out, or assert v0.92 birthday completion.

## Validation

Focused validation for this package is:

```bash
ruby .csdlc/prepared/issues/4762/validate_birth_receipt_package.rb
git diff --check -- .csdlc/issues/4762 .csdlc/prepared/issues/4762 docs/milestones/v0.91.8 docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md
```

The validator must fail closed if required fields, witnesses, negative cases,
source paths, handoff consumers, or forbidden-claim boundaries are missing.

## Non-Claims

This package does not claim:

- the first true Godel-agent birthday has happened
- legal personhood
- production citizenship
- completed constitutional governance
- v0.93 governance completion
- raw private-state disclosure
