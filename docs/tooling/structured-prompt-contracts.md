# Structured Prompt Contracts

## Purpose

ADL now treats the core workflow artifacts as structured prompt surfaces with machine-checkable contracts:

- `Structured Issue Prompt` (SIP)
- `Structured Task Prompt` (STP)
- `Structured Plan Prompt` (SPP)
- `Structured Review Prompt` (SRP)
- `Structured Outcome Record` (SOR)

The canonical semantic lifecycle is:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

This document defines the first bounded contract layer for those artifacts so tooling can validate stable machine-readable structure without freezing high-value prose too early.

See `card-lifecycle.md` for the card roles and the distinction between file
creation order and lifecycle activation order.

See `../templates/CARD_LIFECYCLE_TEMPLATE_TARGETS.md` for the current
template-target shape and compatibility aliases for all five cards.

## Canonical Contract Files

The original schema-backed contract files remain:

- `adl/schemas/structured_task_prompt.contract.yaml`
- `adl/schemas/structured_implementation_prompt.contract.yaml`
- `adl/schemas/structured_output_record.contract.yaml`

The historical `structured_implementation_prompt` schema name is retained as a
compatibility surface while the semantic `SIP` meaning moves to Structured
Issue Prompt. The historical `structured_output_record` schema name is retained
while the semantic `SOR` meaning moves to Structured Outcome Record. The landed
`v0.91` `SPP`/`SRP` slice is validator-backed but not yet represented as
dedicated tracked schema files. Their bounded contract currently lives in the
Rust validator entrypoint itself.

These schema files, plus the validator-backed `SPP`/`SRP` rules, are the
current source of truth for:

- required metadata fields
- required section presence
- normalized enum vocabularies already stable enough to enforce
- normalized scalar formats already stable enough to enforce
- phase-specific blank-field allowances where bootstrap artifacts are intentionally incomplete

## Validation Entry Point

Use:

```bash
adl tooling validate-structured-prompt --type stp --input <path>
adl tooling validate-structured-prompt --type sip --phase bootstrap --input <path>
adl tooling validate-structured-prompt --type sor --phase bootstrap --input <path>
adl tooling validate-structured-prompt --type spp --input <path>
adl tooling validate-structured-prompt --type srp --input <path>
```

The validator currently enforces:

- required section presence
- required metadata field presence
- normalized slugs and task identifiers
- version and branch formats
- GitHub issue / PR URL formats where those fields are machine-readable
- boolean normalization
- selected enum vocabularies
- UTC ISO 8601 / RFC3339 date-time formatting with trailing `Z` for machine-readable timestamp fields (`YYYY-MM-DDTHH:MM:SSZ`)
- bounded front-matter contracts for durable planning and review-prompt artifacts

## Timestamp Standard

For active machine-readable timestamp surfaces, ADL uses one house rule:

- UTC ISO 8601 / RFC3339 with trailing `Z`
- canonical example: `2026-03-28T09:14:00Z`
- preferred placeholder name in active docs and path conventions: `<timestamp_utc_z>`

This applies to structured prompt execution fields, review metadata, and active report path conventions unless a surface explicitly documents a different historical compatibility rule.

For SIP validation, the validator also delegates Prompt Spec validation to:

- `adl tooling lint-prompt-spec`

## Field Classification Rules

The first-pass contracts classify machine-readable fields into four buckets:

- enum
- boolean
- identifier / slug / stable reference
- free text

Fields should remain free text only when:

- the content is genuinely open-ended prose, or
- the vocabulary is not yet stable enough to enforce without harming authoring quality

## Stable Enums Introduced Here

Examples of enum-constrained values now treated as stable:

- STP `action`
- STP `status`
- SIP `Required outcome type`
- SOR `Status`
- SOR `Integration state`
- SOR `Verification scope`
- SOR `Main Repo Integration.Result`

## Outcome-Record Integration Semantics

Structured Outcome Records intentionally separate three related ideas:

- `Integration state`
  - lifecycle state of the integrated artifact set (`worktree_only`, `pr_open`, `merged`, `closed_no_pr`)
- `Verification scope`
  - where the verification commands were run (`worktree`, `pr_branch`, `main_repo`)
- `Worktree-only paths remaining`
  - whether any required artifacts still exist only outside the main repository path

These fields should not be conflated.

In particular:

- `Integration state: pr_open` does not imply verification happened in a worktree
- `Integration state: pr_open` should still truthfully report any worktree-only paths remaining
- `Integration state: closed_no_pr` should only be used for an intentional no-PR closure with the disposition recorded
- `Integration method used: direct write in main repo` should normally pair with `Verification scope: main_repo`
- deviations are allowed, but should be explained in the record rather than left ambiguous

## Execution-Record Quality Expectations

Structured Outcome Records are machine-auditable execution records, not narrative summaries.

In practice that means:

- every listed validation command should also say what it verified
- deterministic scripts or fixtures should be identified as determinism evidence, not merely listed
- rerunnable deterministic scripts and fixtures count as replay evidence when they reproduce the same result
- security and privacy checks should state what was checked and how it was checked
- the record should make its primary proof surface easy to identify during review
- section headers should not be left empty; if something does not apply, the record should say why

## Absolute-Path Policy

Outcome records should prefer repository-relative paths in validation commands and artifact references.

The intended rule is:

- unjustified absolute host paths should not appear in final recorded validation commands, artifact references, or machine-readable summaries
- if an absolute path is operationally required, it should be explicitly justified rather than silently recorded

This keeps `absolute_path_leakage_detected: false` meaningful for the final recorded artifact, even when the underlying shell execution may have used host-absolute paths internally.

## Phase Model

The validator supports bounded lifecycle phases where needed:

- `bootstrap`
- `authored`
- `completed`

In `bootstrap`, some machine-readable fields may remain blank while the artifact is being initialized.

This is especially important for:

- SIPs generated before issue-mode `pr run` binds a worktree
- SORs generated before execution has happened

The phase model is intentionally narrow. It is not a full workflow state machine.

## Template Lifecycle Status Targets

The template target for new cards is to distinguish file presence from
authoritative lifecycle truth:

- `scaffold`: file exists for path stability, but the stage is not active.
- `draft`: issue-specific content is being authored or reviewed.
- `active`: card is the authoritative surface for its lifecycle stage.
- `reviewed`: SRP review results have been recorded.
- `pr_open`: SOR outcome truth is represented by an open PR.
- `merged`: SOR outcome truth is merged and ready for closeout/final audit.
- `closed_no_pr`: SOR records an intentional no-PR closure.
- `superseded`: card was replaced by a later revision or issue.
- `legacy_compatible`: old naming or shape is retained and detectable during
  migration.

These values are template targets for the migration wave. Enforcement belongs
to the validator and doctor-readiness surface introduced by the C-SDLC
card-lifecycle migration.

## Doctor Lifecycle Readiness

`pr doctor` reports lifecycle readiness separately from ordinary card
existence. Its `card_lifecycle` JSON object, mirrored by `CARD_LIFECYCLE_*`
text lines, includes:

- the canonical order `SIP -> STP -> SPP -> SRP -> SOR`
- the active stage and next required stage
- `pr_run_readiness`, based on complete-enough `SIP`, `STP`, and `SPP`
- `pr_finish_readiness`, based on final SRP review truth plus complete or final
  SOR output truth
- per-card state values for `pre_run`, `scaffold`, `active`, `complete`,
  `final`, and `legacy_compatible`

Freshly bootstrapped issues may report `SIP` and `SPP` as `pre_run` while the
branch/worktree is still unbound. That state is not an editor-fixable defect by
itself; it records that the issue is structurally ready for `pr run`, where the
execution branch and worktree make those cards concrete.

The bootstrapped `SPP` should still be useful design-time plan truth. It should
be generated from source issue context rather than from a generic placeholder,
so reviewers can inspect the execution sequence, proof gates, and replan
triggers before work starts. In contrast, branch-bound SPP/SRP drift with a
`next_editor` remains a real card-local blocker and must be routed through the
matching editor skill.

Legacy SRP policy scaffolds are not valid new SRP prompt artifacts. The
structured-prompt validator should fail them closed, while `pr doctor` may
still classify retained historical scaffolds as `legacy_compatible` so they can
be routed through `srp-editor` instead of being mistaken for final review
readiness.

## SRP/SOR Finish And Closeout Handoff

`pr finish` and `pr closeout` should treat `SRP` and `SOR` as paired but
separate readiness surfaces:

- `SRP` supplies review instructions, findings, dispositions, residual risk, and
  recommended outcome.
- `SOR` supplies changed paths, validation, integration state, closeout state,
  unresolved follow-ups, and final issue truth.

A finish-ready issue should not treat file presence alone as review readiness.
The final `SRP` needs review results or an explicit review-policy exception, and
the `SOR` needs truthful publication-state output such as `pr_open`.

A closeout-ready issue should preserve the final `SRP` as review-learning
evidence while updating the `SOR` to the terminal GitHub and local closeout
state. Future `ObsMem` ingestion should receive those as distinct memory inputs,
not as one collapsed summary.

For common release-tail issues, use the deterministic fact-sync helper instead
of hand-editing SRP/SOR truth:

```sh
adl tooling srp-sor-update \
  --facts /path/to/srp-sor-facts.yaml \
  --srp .adl/<version>/tasks/<issue>/srp.md \
  --sor .adl/<version>/tasks/<issue>/sor.md
```

The fact packet may provide `review`, `validation`, `integration`,
`release_tail`, and `metrics` sections. The `release_tail` section records the
watcher disposition, PR state, closeout state, and residual risks used during
finish/closeout. Terminal closeout claims fail closed unless review,
validation, PR, and integration facts all support that terminal state. Missing
facts remain unclaimed; the helper does not infer review or validation success
from file presence.

See `srp-sor-obsmem-handoff-v0.91.2.md` for the bounded `v0.91.2` handoff
model and follow-on enforcement routing.

## Intentionally Deferred

This first contract layer does not attempt to solve all editing-control-plane validation.

Deferred work includes:

- full lifecycle enforcement across `pr init`, issue-mode `pr run`, `pr finish`, and `pr closeout`
- migration of all historical artifacts
- freezing high-value prose beyond section presence and selected stable scalars
- full schema coverage for every future authoring/editor surface
