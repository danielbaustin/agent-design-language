# v0.91.4 WP-15 Docs And Adoption Review

## Metadata

- Milestone: `v0.91.4`
- Work package: `WP-15`
- Issue: `#3365`
- Date: `2026-05-31`
- Review type: docs/adoption readiness before internal and external review
- Status: `draft_for_pr_review`

## Scope

This review checks whether v0.91.4 is legible to an external reviewer before
the internal and third-party review tail continues.

In scope:

- root project status surfaces
- changelog and crate version truth
- v0.91.4 milestone planning and release-tail docs
- v0.91.5 bridge routing references where they affect v0.91.4 review truth
- C-SDLC adoption docs, prompt-template docs, and tooling docs
- sidecar boundaries for CodeFriend and WildClawBench

Out of scope:

- release approval
- internal review replacement
- third-party review replacement
- broad runtime validation
- rewriting historical milestone docs whose old milestone-specific references
  remain correct in context

## Review Summary

The v0.91.4 docs are broadly ready for the review tail. The core story is
coherent: v0.91.4 is the active C-SDLC default-operation hardening milestone,
v0.91.3 is complete, and the bridge work that would otherwise expand scope has
been routed to v0.91.5.

The review found a small number of stale or under-specified reviewer-facing
status statements and corrected them in this PR:

- Root `README.md` no longer says v0.91.4 issue setup is merely underway; it
  now says v0.91.4 is in Sprint 4 docs/review/release-tail convergence.
- `CHANGELOG.md` now records that v0.91.4 is in the review/release-tail
  sequence and that bridge work is routed to v0.91.5.
- `docs/milestones/v0.91.4/README.md` now explicitly includes the
  WildClawBench sidecar in the sidecar success/exit criteria rather than only
  naming CodeFriend.
- `docs/milestones/v0.91.4/RELEASE_PLAN_v0.91.4.md` now includes
  WildClawBench beside CodeFriend as a sidecar that must be complete,
  truthfully blocked, or routed before release.
- `docs/milestones/v0.91.4/DEMO_MATRIX_v0.91.4.md` now conforms to the current
  planning-template structure while preserving its proof and non-claim content.

## Checks Performed

| Surface | Result | Notes |
| --- | --- | --- |
| Root README | corrected | Active status updated from setup-era language to Sprint 4 review-tail truth. |
| Changelog | corrected | v0.91.4 active entry now names review/release-tail state and v0.91.5 bridge routing. |
| Cargo/TOML metadata | pass | `adl/Cargo.toml` and `adl/Cargo.lock` both report `0.91.4`. |
| v0.91.4 README | corrected | WildClawBench sidecar boundary is now represented beside CodeFriend. |
| v0.91.4 canonical planning docs | pass | All 10 canonical planning docs pass the current planning-template structure validator. |
| v0.91.4 checklist/release plan | pass | Release gates remain unchecked where evidence is not complete; bridge routing is explicit. |
| v0.91.4 sprint plan | pass | Sprint 4, sidecar, and bridge-routed work are separated. |
| v0.91.5 package | pass | v0.91.5 is represented as `draft_pre_open` bridge work, not a replacement for v0.91.4 release closeout. |
| C-SDLC docs | pass | Core lifecycle remains `SIP -> STP -> SPP -> SRP -> SOR`; local `.adl` remains support state, not public release truth. |
| Prompt templates | pass | Current prompt-template registry and docs are present under `docs/templates/prompts/`. |

## v0.91.4 / v0.91.5 Boundary

v0.91.4 remains the active release-tail milestone. Its job is to finish Sprint 4
closeout and prove C-SDLC default-operation truth without further scope
expansion.

v0.91.5 is the bridge milestone for work that should not block v0.91.4 release
closeout:

- multi-agent stabilization
- provider/model matrix
- public prompt records
- demo readiness
- first-birthday preflight
- AEE completion routing
- enterprise-security repo/module separation planning

This boundary should be preserved during WP-16 and WP-17 review. If a reviewer
finds bridge-scope work inside v0.91.4 release gates, route it explicitly
instead of turning it into hidden release scope.

## Adoption Notes

Future ADL issues should use the process described by `AGENTS.md`:

- use `workflow-conductor` for lifecycle routing
- create all five prompt cards from the active prompt-template registry
- make `SIP`, `STP`, and `SPP` design-time ready before execution
- edit cards only with editor skills
- work in bound worktrees, not on `main`
- run bounded pre-PR review
- record final execution and integration truth in `SOR`
- complete closeout after merge or closure

The docs now state this process consistently enough for external review. The
remaining risk is operational discipline, not missing public-facing process
description.

## Validation

Focused validation is appropriate because this issue is docs-only.

Validation run:

- `git diff --check`
- targeted `rg` scans for `v0.91.4`, `v0.91.5`, `0.91.4`, `0.91.5`, `WP-15`,
  and sidecar boundary language
- `adl/Cargo.toml` and `adl/Cargo.lock` version checks
- focused touched-path existence checks for reviewer-facing docs and
  `docs/templates/prompts/current.json`
- v0.91.4 planning-template validation for all 10 canonical planning docs:
  `README`, `VISION`, `DESIGN`, `DECISIONS`, `WBS`, `SPRINT`, `DEMO_MATRIX`,
  `MILESTONE_CHECKLIST`, `RELEASE_PLAN`, and `RELEASE_NOTES`

Broad Rust tests are intentionally not required for this issue because no
runtime behavior or Rust code changes are part of WP-15.

## Residual Risks

- PR `#3539` corrected issue `#3537` to v0.91.4 tracker truth; closeout should
  verify the root feature-list language remains aligned if later review edits
  touch that surface again.
- Some historical docs still mention older WP numbering or milestone-specific
  release-tail patterns. Those are not defects unless they are presented as
  current v0.91.4 truth.
- v0.91.5 docs are intentionally `draft_pre_open`; do not treat them as v0.92
  launch evidence until v0.91.5 itself executes and closes.
