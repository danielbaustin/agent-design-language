# v0.90.2 Planning Package

## Status

Active milestone package. v0.90.2 is open and the issue wave has been created
as #2245-#2264 from the reviewed milestone package.

Current execution state:

- WP-01 is #2245 and finalizes this package after issue-wave creation.
- WP-02 through WP-20 are #2246-#2264.
- WP-02 provides the Runtime v2 inheritance and compression audit before
  runtime implementation widens.
- WP-03 provides the code-backed CSM run packet contract and fixture definition
  that later Runtime v2 work packages must consume.

## Thesis

v0.90.2 is the first bounded CSM run milestone.

v0.90.1 built the Runtime v2 substrate and the first visibility window.
v0.90.2 should make a small governed world actually run: boot a manifold, admit
citizens, schedule governed episodes, reject one invalid action, snapshot,
rehydrate, wake, and emit Observatory-visible evidence.

The hardening work still matters, but it should wrap the first CSM run rather
than replace it. The milestone should prove both sunny-day execution and
legible failure behavior.

## Directory Shape

- root planning docs and WP YAML live in this directory
- feature contracts live under `features/`
- context and later-band backgrounders live under `ideas/`

## Scope Boundary

In scope:

- first bounded CSM run for `proto-csm-01`
- manifold boot, citizen admission, governed episode execution, resource
  scheduling, local snapshot, local rehydrate, and wake continuity
- Observatory-visible packet, operator report, and proof surfaces
- invariant expansion across normal, failure, recovery, and quarantine paths
- stable violation artifacts suitable for review, demos, and release evidence
- recovery and quarantine semantics
- stronger operator review surfaces for failure and recovery paths
- security-boundary evidence that defends the polis without redefining Runtime v2
- release evidence that makes the hardening proof easy to audit

Out of scope:

- first true Gödel-agent birthday
- full v0.91 moral, emotional, kindness, humor, wellbeing, cultivation, or
  harm-prevention substrate
- v0.92 identity/capability rebinding, migration semantics, or birth record
- complete red/blue/purple adversarial ecology
- business-product execution for CodeBuddy or capability testing

## Canonical Planning Docs

- Vision: `VISION_v0.90.2.md`
- Design: `DESIGN_v0.90.2.md`
- Work Breakdown Structure: `WBS_v0.90.2.md`
- Sprint plan: `SPRINT_v0.90.2.md`
- Decisions log: `DECISIONS_v0.90.2.md`
- Demo matrix: `DEMO_MATRIX_v0.90.2.md`
- Inheritance and compression audit:
  `RUNTIME_V2_INHERITANCE_AND_COMPRESSION_AUDIT_v0.90.2.md`
- CSM run packet contract: `CSM_RUN_PACKET_CONTRACT_v0.90.2.md`
- Feature index: `FEATURE_DOCS_v0.90.2.md`
- Milestone checklist: `MILESTONE_CHECKLIST_v0.90.2.md`
- Release plan: `RELEASE_PLAN_v0.90.2.md`
- Release notes draft: `RELEASE_NOTES_v0.90.2.md`
- Issue wave draft: `WP_ISSUE_WAVE_v0.90.2.yaml`

## Execution Rule

This package is the tracked milestone source for v0.90.2 execution. The issue
wave mapping lives in `WP_ISSUE_WAVE_v0.90.2.yaml`, and execution should proceed
through issue worktrees with SOR evidence updated as each WP closes.

## Compression Rule

v0.90.2 should inherit the v0.90.1 compression model:

- issue wave and task cards should be generated from the reviewed planning
  package
- work starts in issue worktrees only
- docs-only and fixture-only issues use focused local validation plus CI gating
- runtime, schema, security, and release issues use fuller validation
- every SOR records the validation profile used and the exact proof surfaces

Compression is allowed only when it makes evidence easier to produce and review.
It is not permission to skip demos, tests, or release truth.

## WP-02 Inheritance Gate

`RUNTIME_V2_INHERITANCE_AND_COMPRESSION_AUDIT_v0.90.2.md` is the D1 proof
artifact for this milestone. It records that v0.90.2 can inherit the v0.90.1
Runtime v2 foundation, CSM Observatory read-only surfaces, release evidence, and
compression workflow posture while preserving the later-scope non-claims.

## WP-03 Contract Gate

`CSM_RUN_PACKET_CONTRACT_v0.90.2.md` is the WP-03 / D2 contract gate. It records
the code-backed `runtime_v2.csm_run_packet_contract.v1` shape, the
`proto-csm-01` fixture definition, the pre-live-run artifact set, and the
review target that WP-04 through WP-14 must preserve.

## WP-04 Invariant And Violation Gate

WP-04 extends the D2 contract gate with code-backed invariant and violation
artifacts. The invariant map fixture fixes the D2 invariant set before WP-05
boot/admission, and the violation schema fixture fixes the negative-path shape
that WP-08 must consume. This proves contract readiness, not a live CSM run.
