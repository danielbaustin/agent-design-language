# #4761 Capability Envelope Preparation

Status: preparation-only handoff for later execution.

## Boundary

This artifact prepares issue #4761 for a later execution session. It does not write, publish, or claim the v0.92 capability envelope.

The single concern is `capability-envelope`: the later executor must assemble an evidence-backed envelope for birthday/MVP claims, or record an operator-approved blocker when required evidence is missing.

Execution-time claim acquisition is deferred. The current typed doctor state can remain blocked on `claim_not_live` during this preparation lane because the operator directed that unrelated global closeout or claim reconciliation must not block #4761 preparation.

## Evidence Inputs

The later envelope may consume these inputs only after checking exact current revisions and retained proof state:

- `docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml`: #4761 is listed under WP-21 as `v0.92 capability envelope`.
- `docs/milestones/v0.91.8/WBS_v0.91.8.md` and `docs/milestones/v0.91.8/README.md`: v0.91.8 milestone routing and handoff context.
- `docs/milestones/v0.91.8/FEATURE_PROOF_COVERAGE_v0.91.8.md`: WP-21 names #4761 among v0.92 handoff/planning truth surfaces.
- `docs/milestones/v0.91.8/review/V0918_WP14A_PLATFORM_ACCEPTANCE_5384.md`: platform acceptance boundary and non-claims that #4761 must not overstate.
- `docs/milestones/v0.91.8/review/runtime_v3_acceptance_5361.v1.json`: runtime v3 acceptance evidence input, subject to exact-revision verification.
- `docs/milestones/v0.91.8/review/runtime/5665_feature_adapter_matrix.json`: runtime adapter matrix input for capability limits.
- `docs/milestones/v0.91.8/evidence/wp12/manifest.json` and sibling platform reports: cross-platform evidence inputs for supported/unsupported capability claims.
- `docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml`, `docs/milestones/v0.92/WBS_v0.92.md`, and `docs/milestones/v0.92/SPRINT_v0.92.md`: future v0.92 consumption plan, not completion evidence.
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`: activation consumption rules, including capability evidence and selector boundaries.
- `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`: feature contract that names capability envelopes for provider, model, tool, skill, authority, and limit claims.
- `docs/milestones/v0.92/DEMO_MATRIX_v0.92.md`: birthday demo expectations that require bounded capability evidence and explicit non-claims.

## Future Output Paths

The later execution session should write only bounded #4761 output paths unless the operator explicitly widens scope:

- `.csdlc/evidence/4761/capability-envelope/inputs.v1.json`: exact input inventory with revision, digest, claim class, and proof status per source.
- `.csdlc/evidence/4761/capability-envelope/envelope.v1.json`: machine-readable capability envelope.
- `.csdlc/evidence/4761/capability-envelope/non-claims.v1.md`: unsupported claims, missing evidence, and birthday/MVP exclusions.
- `.csdlc/evidence/4761/capability-envelope/review.v1.md`: pre-publication review of the generated envelope.
- `.csdlc/evidence/4761/capability-envelope/validation.v1.log`: smallest proving validation output.
- `.csdlc/issues/4761/cards/`: typed C-SDLC v2 card projections updated only through v2 tooling after live claim acquisition.

Shared milestone docs, v0.92 feature docs, PR publication, merge, and closeout are out of scope for this preparation lane.

## COTS And Budgets

No COTS dependency is introduced by this preparation. Later execution may consume existing provider/model/runtime evidence, but must not add provider SDKs, services, hosted jobs, or external connectors under #4761 without an explicit typed plan and review.

Preparation budget used here is documentation-only and issue-local. Later execution should use a `medium` planning profile unless the exact input inventory shows the envelope can be generated and validated without touching shared milestone surfaces.

## PVF

Preparation validation is `prep-local`:

- deterministic: yes
- resource profile: small
- expected commands: `csdlc-install resolve --repo /Volumes/FastWork/adl-wp-4761 --issue 4761`, `csdlc-doctor --repo /Volumes/FastWork/adl-wp-4761 --issue 4761`, `git diff --check`
- expected doctor truth for this preparation lane: `claim_not_live` may be deferred because execution-time claim acquisition is explicitly out of scope

Later envelope validation must prove, at minimum:

- every supported capability claim maps to a retained input with exact revision or digest
- every unsupported claim is listed as a non-claim or blocker
- provider/model/tool/skill/authority/limit categories are present
- birthday/MVP consumers can distinguish evidence-backed readiness from planning-only text

## One Concern

Concern: `capability-envelope`.

Do not absorb Memory Palace implementation, identity/birthday implementation, ACP profile writing, demo execution, provider expansion, runtime changes, or C-SDLC closeout reconciliation into #4761.
