# v0.90.2 Planning Package

## Status

Active milestone package. v0.90.2 is open and the issue wave has been created
as #2245-#2264 from the reviewed milestone package, with WP-14A added as
#2301 to restore the explicit demo matrix and feature proof demo lane.

Current execution state:

- WP-01 is #2245 and finalizes this package after issue-wave creation.
- WP-02 through WP-20 are #2246-#2264.
- WP-14A is #2301 and owns feature-by-feature demo/proof coverage before
  WP-15 docs and review convergence.
- WP-02 provides the Runtime v2 inheritance and compression audit before
  runtime implementation widens.
- WP-03 provides the code-backed CSM run packet contract and fixture definition
  that later Runtime v2 work packages must consume.
- WP-06 provides bounded resource-pressure scheduling evidence for the first
  governed episode while leaving Freedom Gate mediation to WP-07.
- WP-07 routes the scheduled action through the Freedom Gate with a bounded
  mediation decision.
- WP-08 adds the D5 invalid-action fixture, violation packet, and negative
  tests proving rejection before commit without side effects.
- WP-09 adds the D6 snapshot, rehydration report, wake continuity proof, and
  duplicate-active-head guard evidence.
- WP-10 adds the D7 Observatory visibility packet and operator report generated
  from the bounded first-run artifacts.

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
- Feature proof coverage: `FEATURE_PROOF_COVERAGE_v0.90.2.md`
- Release readiness / WP-15 convergence:
  `RELEASE_READINESS_v0.90.2.md`
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

## WP-05 Boot And Admission Gate

WP-05 lands the D3 boot/admission proof surface for `proto-csm-01`. The
code-backed boot manifest, citizen roster, and boot/admission trace prove that
two worker citizens have traceable identity handles for the bounded first-run
spine while preserving the provisional boundary and avoiding any true-birthday
claim.

## WP-06/WP-07 Governed Episode Gate

WP-06 lands the bounded D4 resource-pressure scheduler evidence, and WP-07
adds the Freedom Gate mediation artifact for the scheduled non-trivial action.
Together they prove a governed episode can be selected under pressure and
mediated before execution.

## WP-08 Invalid Action Rejection Gate

WP-08 lands the D5 rejection proof surface. The invalid-action fixture attempts
to bypass the mediated Freedom Gate decision, and the violation packet proves
the normal policy path rejected that action before commit while preserving
`transition_refused_state_unchanged`. This is still fixture-backed Runtime v2
evidence, not a live CSM run or first true birthday claim.

## WP-09 Snapshot Wake Continuity Gate

WP-09 lands the D6 continuity proof surface. The snapshot manifest and
rehydration report are consumed by a CSM-specific wake continuity proof that
checks snapshot checksum, restore-before-active-state, and
`no_duplicate_active_citizen_instance` before wake. The first-run trace now
records snapshot capture, rehydration validation, and duplicate-safe wake as
contiguous events while preserving the non-live and no-birthday boundaries.

## WP-10 Observatory Visibility Gate

WP-10 lands the D7 operator visibility proof surface. The Runtime v2
Observatory packet uses the inherited `adl.csm_visibility_packet.v1` contract
and is generated from the CSM run packet, boot/admission evidence, first-run
trace, snapshot/rehydration artifacts, and wake-continuity proof. The operator
report is rendered from the same packet and validated against packet truth,
including the event sequence, allow/refuse counts, wake-continuity evidence, and
explicit no-live-run/no-birthday boundary.

## WP-11 Recovery Eligibility Gate

WP-11 lands the D8 recovery decision boundary. The eligibility model consumes
WP-08 invalid-action rejection evidence and WP-09 wake-continuity evidence to
produce one safe-resume decision and one quarantine-required decision. The
positive path requires a declared predecessor, validated rehydration, and a
single active head; the negative path refuses ambiguous or duplicate-head
recovery and hands evidence preservation to WP-12. This does not implement the
quarantine state machine or widen into live Runtime v2 execution.

## WP-12 Quarantine State Machine Gate

WP-12 completes D8 by adding the quarantine state machine and evidence hold. The
unsafe recovery fixture consumes the WP-11 quarantine-required decision, the
quarantine artifact blocks execution pending operator review, and the evidence
preservation artifact records the immutable source evidence that must survive
until review. This proves quarantine behavior for the bounded first-run evidence
without claiming live Runtime v2 execution, first true birthday, or v0.92
identity rebinding.

## WP-13 Governed Adversarial Hardening Gate

WP-13 lands D9 as a bounded, code-backed hardening proof. The rules of
engagement authorize one adversarial pressure path against the quarantined
recovery boundary, forbid committed-state mutation, release from quarantine,
evidence pruning, and live forking, and require reviewable evidence. The
adversarial hook proves the pressure remains contained by the WP-12 quarantine
artifact. The hardening probes then record fail-closed duplicate activation,
snapshot-integrity, and trace/replay-gap negative paths, with a summary proof
packet for WP-14 to consume. This remains an operator-scoped proof surface, not
a live CSM run, first true birthday, v0.92 identity rebinding, or complete
red/blue/purple security ecology.

## WP-14 Integrated First CSM Run Demo Gate

WP-14 lands D10 as the bounded, code-backed flagship demo for v0.90.2. The
`adl runtime-v2 integrated-csm-run-demo --out artifacts/v0902/demo-d10-integrated-csm-run`
command executes a deterministic ten-stage CSM evidence spine, prints the stage
summary and Observatory operator report to stdout, and writes the CSM run
contract, invariant/violation artifacts,
boot/admission evidence, governed episode trace, Freedom Gate mediation,
invalid-action rejection, wake-continuity proof, Observatory packet/report,
recovery decisions, quarantine evidence, governed hardening artifacts, and the
integrated first-run transcript/proof packet into one reviewer-facing bundle.
This proves the v0.90.2 first-run evidence package is connected end to end,
while preserving the non-claims for unbounded live execution, first true birthday, v0.91 civic
substrate, v0.92 identity rebinding, and a complete security ecology.

## WP-14A Demo Program Gate

WP-14A restores the standard ADL milestone pattern where feature-by-feature
demo and proof coverage is explicit before docs/review convergence. WP-13 owns
the governed adversarial hook, WP-14 owns the integrated first CSM run proof,
and WP-14A verifies that each v0.90.2 feature claim has a runnable demo command,
test-backed proof packet, fixture-backed artifact, documented non-proving
status, or explicit deferral. `FEATURE_PROOF_COVERAGE_v0.90.2.md` is the D11
coverage record for that verification. WP-15 consumes that coverage record
rather than inventing missing demos during review convergence.

## WP-15 Docs, Quality, And Review Convergence Gate

WP-15 aligns the release-truth surfaces after implementation and demo coverage
have landed. `RELEASE_READINESS_v0.90.2.md` is the convergence record for this
gate: it points reviewers to the current milestone docs, demo matrix,
feature-proof coverage record, top-level README/changelog/review guide, crate
version, validation commands, explicit non-claims, and remaining release-tail
work. WP-15 does not replace WP-16 internal review, WP-17 external review,
WP-18 remediation, WP-19 handoff, or WP-20 ceremony.
