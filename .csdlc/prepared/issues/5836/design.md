# Issue 5836 Design: First Birthday Demo

## Decision

WP-18 adds one runnable, fail-closed proof harness over the landed v0.92
birthday path. The positive case must be emitted by the integrated Runtime
path; a hand-authored fixture, lifecycle receipt, or cached packet cannot earn
positive proof credit. A companion negative suite rejects ordinary lifecycle
events and incomplete evidence.

## Source Baseline

- `docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md`
- `docs/milestones/v0.92/DEMO_MATRIX_v0.92.md` D1 through D6
- `docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md`
- `docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md`
- `docs/milestones/v0.91.8/review/v092_handoff_4762/`

## Owned Paths

- `adl/tools/demo_v092_first_birthday.sh`
- `adl/tools/validate_v092_first_birthday_packet.py`
- `adl/tools/test_v092_first_birthday_demo.sh`
- `demos/v0.92/first-birthday`
- `docs/milestones/v0.92/DEMO_MATRIX_v0.92.md`
- `docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md`
- `docs/milestones/v0.92/external_launch/PUBLIC_LAUNCH_COPY_v0.92.md`
- `docs/milestones/v0.92/external_launch/REVIEWER_FAQ_AND_CLAIM_BOUNDARY_v0.92.md`
- `.csdlc/evidence/5836`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Serialization Gates

```json
[
  {
    "schema": "csdlc.serialization_gate.v1",
    "id": "v092-birthday-demo-matrix-v1",
    "paths": [
      "docs/milestones/v0.92/DEMO_MATRIX_v0.92.md"
    ],
    "issues": [
      5834,
      5836,
      5840
    ],
    "order": [
      5834,
      5836,
      5840
    ]
  }
]
```

## Proof Architecture

The runner records exact source revision, command argv, environment posture,
Runtime-produced birthday record, stable identity, continuity chain, redacted
memory references, capability envelope, ACP profile, witnesses, receipt,
validation report, and reviewer index. It writes deterministic artifacts under
the issue evidence directory and declares allowed nondeterminism.

The negative suite covers startup, wake, restore, snapshot, admission, copied
state, simulation, a named fixture without continuity, missing witness,
missing receipt, broken lineage, ungrounded memory, and an incomplete
capability envelope. Every negative case must produce a typed rejection reason.

## Execution Plan

1. Verify #5825 through #5830 plus #5832, #5833, and #5834 are landed and identify their exact schemas and commands.
2. Bind the proof runner to those real outputs without duplicating feature logic.
3. Implement the positive harness and deterministic packet validator.
4. Implement the negative matrix and ensure failures do not become shell-only success.
5. Reconcile the canonical public launch copy and reviewer FAQ against accepted
   packet evidence, then emit a fail-closed publication-gate checklist without
   authorizing publication.
6. Update D1-D6 with exact commands, artifacts, and status only after proof passes.
7. Run focused tests, redaction/path hygiene, replay, native macOS and Linux
   proof, publication-gate validation, and exact-head review.

## Failure And Platform Lanes

- Linux and macOS each run a native lane over the same packet contract;
  platform-specific command differences and source revisions are retained
  rather than hidden. A local compatibility check cannot substitute for either
  native receipt.
- The publication-gate validator rejects absent accepted witness/receipt proof,
  unresolved negative cases, stale exact-head review, missing launch documents,
  unsupported claims, or absent operator authorization. Passing it prepares a
  checklist and never publishes.
- Missing Runtime binary, dependency artifact, or schema is a blocker.
- Private memory, credentials, provider payloads, and machine-local paths must
  not enter retained or reviewer-facing artifacts.
- Interrupted execution must leave an explicit failed/incomplete packet.

## Rollback

Remove the v0.92 demo harness and its owned launch-copy projections, restore
the prior D1-D6 rows, and retain all failed/incomplete packets and validation
logs under issue evidence. Rollback must not delete dependency artifacts,
rewrite Runtime-produced birthday records, or promote a historical fixture to
positive proof.

## Non-Goals

- Observatory or Unity consumer integration (#5837).
- Provider-neutral comparison (#5838).
- Governance completion (#5839).
- Legal personhood, consciousness, production citizenship, or public launch.

## Exit Evidence

The retained packet must be reproducible from one documented command, prove a
real positive path and all named negatives, pass platform/redaction checks, and
receive exact-head review with no unresolved actionable finding.
