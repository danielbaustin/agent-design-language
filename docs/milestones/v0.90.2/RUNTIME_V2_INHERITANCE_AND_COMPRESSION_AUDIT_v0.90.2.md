# Runtime v2 Inheritance And Compression Audit - v0.90.2

## Purpose

This is the WP-02 / D1 proof artifact for v0.90.2. It verifies what the
milestone can inherit from v0.90.1 before the first bounded CSM run work widens
into packet contracts and runtime implementation.

## Result

Status: PASS

v0.90.2 can inherit the v0.90.1 Runtime v2 substrate, CSM Observatory read-only
surfaces, release evidence packet, and compression workflow posture as the
starting point for Sprint 1. The inherited work is sufficient for WP-03 to
define the first CSM run packet contract without re-litigating v0.90.1 scope.

Important boundary: this audit does not claim that v0.90.2 has implemented the
first bounded CSM run yet. It only establishes the inherited substrate and
execution posture for the next work packages.

## Source Evidence

| Evidence | Inheritance Use |
| --- | --- |
| `docs/milestones/v0.90.1/RELEASE_EVIDENCE_v0.90.1.md` | Release-level index of v0.90.1 claims, proof families, reviews, and handoff state |
| `docs/milestones/v0.90.1/RELEASE_READINESS_v0.90.1.md` | Confirms Runtime v2 foundation, CSM Observatory, review, remediation, and release evidence are ready |
| `docs/milestones/v0.90.1/DEMO_MATRIX_v0.90.1.md` | Confirms Runtime v2 D1-D7, release D8, CSM Observatory D9/D9A, and quality D10 are landed |
| `docs/milestones/v0.90.1/features/RUNTIME_V2_FOUNDATION_PROTOTYPE.md` | Defines the bounded inherited Runtime v2 foundation proof and non-claims |
| `docs/milestones/v0.90.1/features/CSM_OBSERVATORY_VISIBILITY_PACKET.md` | Defines inherited CSM Observatory visibility packet contract |
| `docs/milestones/v0.90.1/features/CSM_OBSERVATORY_OPERATOR_REPORT.md` | Defines inherited deterministic operator report surface |
| `docs/milestones/v0.90.1/features/CSM_OBSERVATORY_OPERATOR_COMMAND_PACKETS.md` | Defines inherited read-only command-packet boundary for future operator interactions |
| `docs/milestones/v0.90.1/QUALITY_GATE_v0.90.1.md` | Defines inherited quality-gate posture and proof aggregation boundary |
| `adl/src/runtime_v2/` | Split Runtime v2 implementation tree inherited by v0.90.2 |
| `demos/fixtures/csm_observatory/` | Fixture-backed CSM Observatory packets inherited by v0.90.2 |

## Runtime v2 Inheritance

v0.90.2 inherits these Runtime v2 implementation surfaces:

- `adl/src/runtime_v2/manifold.rs`
- `adl/src/runtime_v2/kernel_loop.rs`
- `adl/src/runtime_v2/citizen.rs`
- `adl/src/runtime_v2/snapshot.rs`
- `adl/src/runtime_v2/invariant.rs`
- `adl/src/runtime_v2/operator.rs`
- `adl/src/runtime_v2/security.rs`
- `adl/src/runtime_v2/foundation.rs`
- shared contracts, types, validators, and tests under `adl/src/runtime_v2/`

Inherited proof claims:

- bounded persistent manifold root exists
- provisional citizen records and indexes exist
- bounded kernel service loop exists
- snapshot and rehydration proof exists
- invariant violation artifact exists
- operator control report exists
- security-boundary proof exists
- integrated Runtime v2 foundation proof packet exists

Inherited non-claims:

- no first true Gödel-agent birthday
- no full moral/emotional civilization
- no cross-polis migration
- no v0.92 identity/capability rebinding
- no complete red/blue/purple security ecology

## CSM Observatory Inheritance

v0.90.2 inherits a read-only Observatory substrate:

- visibility packet schema: `adl.csm_visibility_packet.v1`
- fixture packet: `demos/fixtures/csm_observatory/proto-csm-01-visibility-packet.json`
- operator command packet fixture: `demos/fixtures/csm_observatory/proto-csm-01-operator-command-packets.json`
- operator report generator and validation scripts under `adl/tools/`
- CSM CLI surface for packet/report/console-reference bundle generation

Inherited proof claims:

- CSM state can be projected into a bounded visibility packet
- operator reports can be generated deterministically from that packet
- static console and CLI proofs are fixture-backed and read-only
- Observatory packets include claim-boundary and evidence-level language

Inherited non-claims:

- no live CSM mutation through the Observatory
- no live first-run capture yet
- no unrestricted operator control plane
- no god-view access to future private citizen state

## Compression Readiness

v0.90.2 can inherit the v0.90.1 compression model with the following rules:

- use conductor-guided ADL lifecycle for every issue
- start tracked work only after issue-mode `pr run` binds a worktree
- keep root `main` tracked-clean
- keep issue cards local and truthful
- use focused validation for docs-only and fixture-only issues
- use fuller validation for runtime, schema, security, and release issues
- record exact proof surfaces in each SOR
- do not use compression as permission to skip tests, demos, or release truth

Compression readiness: PASS

Rationale: WP-01 generated the v0.90.2 issue wave and local task bundles, #2246
bound successfully through issue-mode `pr run`, and the v0.90.2 planning package
already maps Sprint 1 gates before runtime implementation widens.

## Gaps And Follow-Up Homes

| Gap | Disposition | Follow-Up Home |
| --- | --- | --- |
| First CSM run packet does not exist yet | Expected | WP-03 #2247 |
| Invariant and violation contract needs v0.90.2-specific expansion | Expected | WP-04 #2248 |
| Live `proto-csm-01` run artifacts do not exist yet | Expected | WP-05 through WP-10 |
| Recovery and quarantine are not yet implemented for the first run | Expected | WP-11 #2255 and WP-12 #2256 |
| Governed adversarial hook remains bounded future work | Expected | WP-13 #2257 |
| Integrated first-run proof packet does not exist yet | Expected | WP-14 #2258 |

None of these gaps block WP-03. They define the next work packages.

## D1 Proof Classification

Classification: proving

D1 proves that v0.90.2 targets the actual v0.90.1 substrate and can execute
through the compressed workflow. It proves this by indexing the inherited
Runtime v2 implementation tree, v0.90.1 proof docs, CSM Observatory fixture
surfaces, and current v0.90.2 issue-wave execution posture.

D1 does not prove the first bounded CSM run. That proof belongs to later demos,
especially D3 through D10.

## Review Checklist

- v0.90.1 Runtime v2 foundation surfaces found: PASS
- v0.90.1 CSM Observatory surfaces found: PASS
- v0.90.1 release evidence and readiness records found: PASS
- v0.90.2 issue wave and WP-02 task bundle found: PASS
- Compression workflow can bind issue-mode worktree: PASS
- Later-scope non-claims preserved: PASS

## Validation Commands

The WP-02 authoring pass should validate this report with:

```bash
test -f docs/milestones/v0.90.1/RELEASE_EVIDENCE_v0.90.1.md
test -f docs/milestones/v0.90.1/RELEASE_READINESS_v0.90.1.md
test -d adl/src/runtime_v2
test -f demos/fixtures/csm_observatory/proto-csm-01-visibility-packet.json
run the local-path and secret-marker scan against touched docs
git diff --check
```
