# v0.91.4 Next Milestone Handoff

## Status

`WP-19` handoff refresh for next-milestone review.

This document is no longer a pre-milestone scaffold. It records the current
next-milestone recommendation after `WP-17` external review and `WP-18`
remediation landed.

Current release-tail state at this handoff:

- `WP-17` / `#3367`: closed after external review evidence landed through PR
  `#3551`.
- `WP-18` / `#3368`: closed after external review findings R1-R4 were fixed or
  dispositioned through `#3560` / PR `#3561`.
- `WP-19` / `#3369`: closed after next-milestone planning landed through PR
  `#3563`.
- `WP-20` / `#3370`: in progress for next-milestone review.
- `WP-21` / `#3371`: pending release ceremony and release-evidence closeout.

Do not treat this handoff as release approval. `WP-20` still needs to review
this plan, and `WP-21` still owns ceremony/evidence convergence.

## Purpose

v0.91.4 is the C-SDLC rollout-closeout milestone. This handoff preserves the
next-step decision so v0.91.4 can land cleanly without using chat memory to
reconstruct bridge scope.

The next milestone should consume v0.91.4 evidence and explicitly carry the
pre-v0.92 work that is useful but not required to close the v0.91.4 release
tail.

## Selected Next Milestone

The selected next milestone is `v0.91.5`.

`v0.91.5` should remain a real bridge milestone, not `v0.91.4A`. Its purpose is
to stabilize C-SDLC operations, multi-agent execution, public prompt records,
provider/model breadth, demo readiness, AEE completion routing, and v0.92
activation readiness before the first-birthday milestone opens.

Primary downstream planning package:

- `docs/milestones/v0.91.5/README.md`
- `docs/milestones/v0.91.5/WBS_v0.91.5.md`
- `docs/milestones/v0.91.5/WP_ISSUE_WAVE_v0.91.5.yaml`
- `docs/milestones/v0.91.5/V092_ACTIVATION_TEST_MAP_v0.91.5.md`

## Evidence Used

This handoff is based on the following tracked surfaces:

- v0.91.4 quality gate: `docs/milestones/v0.91.4/QUALITY_GATE_v0.91.4.md`
- v0.91.4 feature proof coverage:
  `docs/milestones/v0.91.4/FEATURE_PROOF_COVERAGE_v0.91.4.md`
- v0.91.4 demo matrix: `docs/milestones/v0.91.4/DEMO_MATRIX_v0.91.4.md`
- v0.91.4 internal review packet:
  `docs/milestones/v0.91.4/review/internal_review/`
- v0.91.4 external review packet:
  `docs/milestones/v0.91.4/review/third_party_review/`
- WP-18 remediation record:
  `docs/milestones/v0.91.4/review/third_party_review/V0914_EXTERNAL_REVIEW_REMEDIATION_2026-06-01.md`
- v0.91.5 bridge package: `docs/milestones/v0.91.5/`
- current GitHub issue truth for open v0.91.4 and v0.91.5 issues

## Completed v0.91.4 Inputs Safe To Rely On

The following completed v0.91.4 work can be treated as upstream input by
v0.91.5:

- lifecycle validation, routing, conductor, and editor hardening
- Software Development Polis actor-standing and shard-ownership proof
- merge-readiness and GitHub truth-preservation proof
- ObsMem transition-memory proof
- five-minute sprint repeatability and validation-tail measurement
- Parallel Validation Fabric lane separation and release-policy proof
- process-drift fail-closed regression proof
- docs/adoption, internal review, external review, and WP-18 remediation
  disposition surfaces

These inputs support bridge execution. They do not by themselves prove live
multi-agent usefulness, v0.92 first-birthday readiness, or a public prompt
records transition.

## Carry Forward To v0.91.5

The following work should carry forward into `v0.91.5`:

| Work area | Owner / route | Required bridge outcome |
| --- | --- | --- |
| Multi-agent C-SDLC stabilization | `#3415`, `#3501`, `#3503`, `#3504` | Prove useful bounded multi-agent issue execution or record a blocker before v0.92 depends on it. |
| Provider/model breadth | `#3505`, `#3549`, `#3562` | Cover OpenRouter, DeepSeek API, hosted models, local Ollama, remote Ollama, and external review provider lanes as separate evidence substrates. |
| Public C-SDLC prompt records | `#3472`-`#3476`, `#3553` | Export, render, validate, redact, index, and transition prompt records without deleting `.adl` history unsafely. |
| Demo readiness | `#3455`, `#3460`, `#3461` | Decide whether Celestial Rescue doubles as Unity Observatory proof or remains v0.92 demo preparation. |
| v0.92 activation testing | `#3502`, `#3377` | Ensure every feature surface that comes alive in v0.92 has owner issue, candidate WP, and test/proof posture. |
| AEE completion tranche | closed evidence inputs: `#3526`, `#3534`; live v0.91.5 route: `#3377` | Consume the closed AEE audit/tranche plans in `#3377`, then either seed concrete AEE proof/implementation follow-ons or explicitly block v0.92 readiness before MVP convergence. |
| Enterprise security separation | `#3538` | Plan separation of enterprise-security features from the main codebase without destabilizing MVP scope. |
| Runtime/polis observability | `#3556` plus follow-on issue as needed | Make deterministic logs and runtime/polis observability first-class proof surfaces. |
| Godel-Hadamard-Bayes paper | `#3541` | Produce a bounded paper packet without blocking v0.92 activation readiness. |

## Do Not Carry As v0.91.5 Release Blockers

The following should not silently become v0.91.5 release blockers unless a
v0.91.5 issue explicitly promotes them:

- CodeFriend sidecar product success as proof of C-SDLC default operation
- WildClawBench benchmark maturity or benchmark-win claims
- Unity polish beyond the bounded Celestial Rescue / Observatory readiness
  decision
- broad Rust refactoring unrelated to the bridge proof path
- full v0.92 birthday implementation
- v0.93 constitutional governance

## v0.92 Activation Inputs

`v0.91.5` must keep the v0.92 activation map complete and explicit. At minimum,
the bridge should preserve testing/routing for:

- AEE completion
- Memory v2 / ObsMem handoff
- ACP / cognitive profiles
- aptitude and capability selector
- identity and continuity
- affect, happiness, humor, and wellbeing surfaces
- Godel mechanics
- economics context
- Observatory and Unity demo readiness
- ACIP / provider communications
- provider/model matrix
- public prompt records

The canonical v0.91.5 activation map is:

- `docs/milestones/v0.91.5/V092_ACTIVATION_TEST_MAP_v0.91.5.md`

## Known Blockers And Residual Risks

Remaining v0.91.4 release-tail blockers:

- `WP-20` must review this handoff before ceremony.
- `WP-21` must converge release evidence and close the release ceremony.

Bridge risks for v0.91.5:

- Multi-agent coordination may remain slower or less useful than single-agent
  execution for small issues.
- Provider/model breadth may expose aptitude gaps rather than solve them.
- Public prompt records may require stricter redaction and deterministic
  template rendering before they are safe to publish.
- AEE completion may require an explicit follow-on tranche beyond the bridge if
  closure criteria reveal implementation gaps.
- Runtime/polis observability must not become log theater; logs need
  deterministic action-level value for debugging and review.

## Non-Goals

This handoff does not:

- approve the v0.91.4 release ceremony
- open v0.92
- implement the first birthday
- claim multi-agent execution is already useful enough for default operation
- claim prompt records are public-safe before export/redaction validation
- close or supersede v0.91.5 planning documents

## WP-20 Review Requirement

`WP-20` must review this handoff before the release ceremony.

That review should confirm:

- `v0.91.5` is still the correct next milestone
- all open v0.91.5 issues are intentionally routed
- no v0.91.4 release-tail blocker was hidden in bridge scope
- sidecar work remains separated from C-SDLC core release proof
- v0.92 activation surfaces are represented in the v0.91.5 map
- deferred work has issue or backlog routing

## Handoff Decision

Proceed through `WP-20` review with `v0.91.5` as the selected next milestone.
If WP-20 stays review-clean, WP-21 may consume this handoff for ceremony.
