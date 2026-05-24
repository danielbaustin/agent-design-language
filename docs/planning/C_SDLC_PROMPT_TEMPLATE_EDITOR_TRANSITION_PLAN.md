# C-SDLC Prompt Template And Editor Transition Plan

## Status

Tracked transition plan for issue `#3291`.

This document records how ADL moves from chat-shaped or hand-rewritten issue
cards toward versioned C-SDLC prompt templates, Rust-owned validation, and
browser-assisted human review.

This is a planning document. It does not claim that the full transition is
complete. It names the default path, the remaining work, and the handoff into
`v0.91.4`.

## Source Inputs

- `#3286`: SemVer C-SDLC prompt templates.
- `#3289`: first-pass human C-SDLC prompt form editor.
- `#3298`: dogfood fixes for prompt-template bootstrap and readiness defects.
- `#3296`: open follow-on for enforcing card-status transitions in skills and
  lifecycle tooling.
- `#3312`: open follow-on for piloting planning-template generation against
  `v0.91.4` docs.
- `docs/templates/prompts/current.json`
- `docs/templates/prompts/1.0.0/`
- `docs/tooling/csdlc-prompt-editor/README.md`
- `docs/planning/DESIGN_TIME_CARD_COMPLETION_PLAN.md`
- `docs/milestones/v0.91.3/C_SDLC_TRACKED_SOURCE_PACKAGE_v0.91.3.md`
- `docs/milestones/v0.91.4/C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md`

## Decision

C-SDLC prompt records become first-class workflow objects.

The canonical lifecycle remains:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

The transition target is:

- prompt templates are versioned objects under `docs/templates/prompts/`
- Rust owns the template registry, field model, enum values, and validation
  rules
- browser form editors are human review and recovery surfaces, not a second
  source of card semantics
- issue cards are copied from templates and filled as deterministic forms, not
  regenerated from scratch by prose memory
- `SIP`, `STP`, and `SPP` are design-time ready before execution starts
- `SRP` is a complete review prompt before review, then records review results
- `SOR` is a pre-execution scaffold until execution, publication, merge, and
  closeout truth exist
- durable prompt records move into tracked C-SDLC workflow state during
  `v0.91.4`

## Object Model

### Template Objects

Templates are stable, versioned source objects:

```text
docs/templates/prompts/current.json
docs/templates/prompts/1.0.0/sip.md
docs/templates/prompts/1.0.0/stp.md
docs/templates/prompts/1.0.0/spp.md
docs/templates/prompts/1.0.0/srp.md
docs/templates/prompts/1.0.0/sor.md
```

The active template set is selected by `current.json`. Future incompatible
changes should add a new SemVer directory and update the registry instead of
silently changing old template meaning.

### Field Model

The field model should be Rust-owned.

Rust should define:

- fixed fields
- editable fields
- locked/system fields
- enum values
- required/optional rules
- status transitions
- Markdown rendering rules
- sample rendering and validator checks

Python helpers may remain callers or compatibility surfaces. They should not
own the canonical field semantics.

### Form Editors

The browser editor is a human-facing form over the Rust-owned model.

It should:

- show locked/system fields without making them editable
- expose only fields a human can reasonably edit for the current card type
- validate local form fields before export
- hide generated Markdown unless the user asks to inspect or copy it
- export Markdown that validators and editor skills can consume

It should not:

- write directly to Git
- call GitHub
- bypass editor skills
- create another independent definition of card semantics

### Issue Cards

Issue cards are issue-local instances of the templates.

They should remain human readable and machine parseable. The design-time state
for a normal issue is:

| Card | Pre-execution target |
| --- | --- |
| `SIP` | issue-specific and ready |
| `STP` | issue-specific and ready |
| `SPP` | reviewed or approved operative issue plan |
| `SRP` | complete review prompt with results absent until review |
| `SOR` | truthful pre-execution scaffold |

### Tracked Records

During `v0.91.4`, durable C-SDLC records should move under the tracked
workflow namespace defined by the workflow-state migration plan:

```text
workflow/c-sdlc/<version>/issues/<issue-number>-<slug>/
  sip.md
  stp.md
  spp.md
  srp.md
  sor.md
  plan-history.md
  evidence/
  traces/
```

Local `.adl/` files may remain execution cache or staging surfaces. They should
not be the only canonical durable prompt record once C-SDLC default operation
is claimed.

### ObsMem Feed Surfaces

ObsMem should consume tracked C-SDLC evidence, not chat lore or local-only
scratch state.

Useful feed surfaces include:

- final `SPP` and plan-history records
- `SRP` findings, dispositions, and residual risks
- `SOR` changed-path and validation truth
- signed trace manifests and verification results
- sprint closeout records and release evidence

## Adoption Sequence

### Phase 0: Current State

`#3286`, `#3289`, and `#3298` have established the first usable substrate:

- versioned prompt templates exist
- the browser editor exists
- bootstrap/readiness defects from the first dogfood pass were fixed
- `pr doctor` can block on incomplete design-time `SPP` state

This is enough to start using the process carefully. It is not enough to claim
the transition is complete.

### Phase 1: v0.91.3 Remaining Issues

For the rest of `v0.91.3`:

- use conductor routing for every issue lifecycle stage
- bootstrap cards through the repo-native template path
- use editor skills for card normalization
- ensure `SIP`, `STP`, and `SPP` are design-time ready before `pr run`
- leave `SRP` review results open until review
- leave `SOR` truthful to actual lifecycle state
- record process defects in the appropriate follow-on rather than silently
  compensating in chat

Open `v0.91.3` issues should not be mass-rewritten inside this planning issue.
They should be handled by the existing catch-up and enforcement issues.

### Phase 2: v0.91.4 WP-01 And Preflight

`v0.91.4` WP-01 should seed the milestone issue wave with this process from
the start:

1. create or confirm GitHub issues through the conductor path
2. create complete source issue prompts
3. generate all five cards from the active template set
4. run structured prompt validation
5. run doctor/preflight for design-time card readiness
6. route card defects to the matching editor skill
7. open the browser editor for human review when field-level review is useful
8. record any transition defects as named issues before execution proceeds

Sprint umbrellas and sidecar issues should receive the same card treatment.
CodeFriend sidecar cards may be prepared in the same mechanical way, but the
sidecar remains product setup, not C-SDLC core proof.

### Phase 3: v0.91.4 Default Operation

Before `v0.91.4` can claim C-SDLC default operation:

- `card_status` transitions must be enforced by skills and lifecycle tooling
- sprint-conductor must block on child-card readiness before child execution
- editor skills must understand template versions and allowed transitions
- durable prompt records must be tracked under `workflow/c-sdlc/`
- SRP/SOR closeout rules must be deterministic enough to prevent overclaiming
- minimal signed trace proof must exist for durable C-SDLC runs
- ObsMem ingestion should point at tracked evidence surfaces

## Card Status Transition Target

The state machine should remain small and deterministic.

Recommended operational states:

| State | Meaning |
| --- | --- |
| `draft` | created but not ready for the next lifecycle gate |
| `ready` | complete enough for the next gate, but not yet explicitly approved |
| `reviewed` | reviewed and acceptable for execution or review handoff |
| `approved` | explicitly approved when the process requires stronger sign-off |
| `completed` | lifecycle work for the card is complete |
| `blocked` | card cannot advance until a named blocker is resolved |
| `superseded` | card was replaced by a newer canonical record |

Rules:

- `SIP`, `STP`, and `SPP` must not allow execution while generic or `draft`.
- `SPP` can return to `draft` when execution materially diverges from the
  tracked plan.
- `SRP` cannot be `completed` until review findings and dispositions are
  recorded.
- `SOR` cannot be `completed` until closeout truth exists.
- `card_status` is lifecycle/audit status; it is not the same as SOR execution
  `Status`.

`#3296` owns the first enforcement pass for these rules.

## Human Review Flow

The intended human-review loop is:

1. Open the card in the browser form editor.
2. Review locked fields for issue identity, branch, version, source prompt, and
   generated timestamp.
3. Edit only the bounded fields that are legitimately human-authored for that
   card type.
4. Click validate.
5. Inspect Markdown only when needed.
6. Copy or save the exported Markdown into the issue bundle.
7. Run the matching structured prompt validator.
8. Continue only if doctor/preflight accepts the card state.

This gives humans a Jira-like review surface without hiding the Markdown record
or making the browser editor authoritative by itself.

## Required Follow-Ons

The transition plan depends on these already-created follow-ons:

- `#3296`: enforce C-SDLC card-status transitions in skills and lifecycle
  tooling.
- `#3312`: pilot project-planning-template generation against `v0.91.4` docs.

Potential additional follow-ons, if not absorbed by those issues:

- add tracked `workflow/c-sdlc/` mirroring for prompt records
- add editor-skill tests that use SemVer template fixtures
- add source-prompt validation guidance so old issue bodies cannot omit
  required sections unnoticed
- add a small browser-editor review runbook for milestone WP-01 operators

## Non-Goals

This plan does not:

- make GWS part of C-SDLC
- replace GitHub issues, PRs, branch protection, human review, or CI
- implement the full tracked workflow-state migration
- rewrite every existing historical card
- claim C-SDLC default operation before `v0.91.4` proves it

## Success Criteria

The transition succeeds when:

- future issue waves are seeded with complete design-time cards
- agents stop hand-rolling cards and instead fill template instances
- humans can review cards through the browser editor without reading all
  generated Markdown by default
- doctor, workflow-conductor, sprint-conductor, and editor skills agree about
  card readiness
- durable prompt records are tracked and inspectable
- SRP/SOR truth feeds review, closeout, trace, and ObsMem without overclaiming
