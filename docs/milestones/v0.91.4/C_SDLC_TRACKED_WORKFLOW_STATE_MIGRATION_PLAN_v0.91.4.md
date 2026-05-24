# v0.91.4 C-SDLC Tracked Workflow State Migration Plan

## Status

Tracked migration plan for completing the Cognitive SDLC rollout.

This document promotes the durable workflow-state migration plan from local
planning notes into the public/auditable C-SDLC docs and `v0.91.4` milestone
package.

The canonical cross-milestone workflow-state summary is
[`docs/cognitive-sdlc/tracked-workflow-state.md`](../../cognitive-sdlc/tracked-workflow-state.md).

## Decision

By the end of `v0.91.4`, ADL's durable C-SDLC truth must be tracked in Git.

The general C-SDLC theory requires durable, replayable workflow state rather
than a specific storage backend. ADL uses tracked Git state for this migration
because it is the clearest currently available substrate for observable
repository-state transitions.

The canonical C-SDLC record is:

- GitHub issue and PR truth
- tracked repo files
- tracked workflow records
- tracked proof, review, trace, and release evidence

Local `.adl` state may remain for ephemeral execution support only. Durable
C-SDLC lifecycle truth must not depend on an untracked or private scratch
surface.

The prompt-template and editor transition plan is tracked in
[`docs/planning/C_SDLC_PROMPT_TEMPLATE_EDITOR_TRANSITION_PLAN.md`](../../planning/C_SDLC_PROMPT_TEMPLATE_EDITOR_TRANSITION_PLAN.md).
That plan defines how versioned prompt templates, Rust-owned field validation,
browser-assisted human review, and editor skills feed this durable workflow
state migration. This document owns the tracked-record destination; the
transition plan owns the template/editor adoption path.

## Canonical Namespace

By default operation, durable C-SDLC records should live under this repo-local
tracked namespace:

```text
docs/milestones/<version>/review/evidence/csdlc/
  <version>/
    issues/
    sprints/
    evidence/
    traces/
```

For `v0.91.4`, new durable workflow records should use
`docs/milestones/v0.91.4/review/evidence/csdlc/` unless a future migration issue deliberately changes
the namespace contract.

Issue-local records should use a predictable issue directory such as:

```text
docs/milestones/v0.91.4/review/evidence/csdlc/issues/<issue-number>-<slug>/
  sip.md
  stp.md
  spp.md
  srp.md
  sor.md
  plan-history.md
  evidence/
  traces/
```

Sprint records should use the matching sprint namespace:

```text
docs/milestones/v0.91.4/review/evidence/csdlc/sprints/<sprint-id>/
```

The namespace is intentionally repo-local. In a large organization with many
repositories, every repo can keep its own public, inspectable workflow truth
under `docs/milestones/<version>/review/evidence/csdlc/`, while organization-level systems index, mirror,
summarize, or feed ObsMem from those tracked records. The org-level system
should not replace the repo-local source of truth.

Local `.adl/` paths may still exist as execution cache, local staging, or tool
scratch space. They are not sufficient as the canonical durable record once
C-SDLC default operation is claimed.

## Durable Records That Must Be Tracked

The tracked C-SDLC record includes:

- `SIP`, `STP`, `SPP`, `SRP`, and `SOR`
- sprint state and sprint closeout artifacts
- issue closeout records
- review findings, dispositions, and residual risks
- release evidence
- proof packets, demo evidence, replay artifacts, and promoted traces
- signed trace bundles for durable C-SDLC proof
- ObsMem ingestion surfaces derived from tracked evidence
- canonical C-SDLC docs, schemas, metrics, and process notes

The only things that should remain untracked are genuinely ephemeral local
cache, temporary helper files, secrets, machine-local scratch, and generated
artifacts explicitly declared non-durable.

## SPP Tracking Contract

By default operation, durable `SPP` truth must be public, tracked, and
issue-local.

`SPP` is the operative execution plan for one issue or transition. It records:

- current step
- next step
- required proof before proceeding
- material replan triggers
- explicit out-of-bounds work

`SPP` must not become the home for sprint orchestration, review findings, or
outcome truth. Sprint state belongs to sprint-conductor state and closeout
artifacts. Review truth belongs in `SRP`. Output and integration truth belongs
in `SOR`.

If the execution path materially changes, the tracked `SPP` or tracked
plan-history surface must be updated before execution continues against the new
plan. Local `.adl` mirrors may support execution, but they cannot be the only
authoritative C-SDLC `SPP` record after the default-operation cutover.

## Signed Trace Requirement

Signed trace proof should not wait for full trace query.

Before C-SDLC becomes ADL's default software-development path, durable
C-SDLC runs should produce a minimal signed trace bundle:

- `trace.jsonl` or an equivalent trace manifest
- digest manifest covering the trace and key proof artifacts
- `trace.signature.json` or equivalent signature record
- replay manifest and verification result
- SOR linkage to the signed trace bundle
- SRP/SOR/closeout linkage into ObsMem ingestion

Full TQL/queryable trace can remain later. Signed proof cannot. C-SDLC
governance, review, closeout, and memory should not depend on unsigned
local-only artifacts.

## Migration Phases

### Phase 1: Track New Work First

Start with:

- all new software-development issues
- all new sprint umbrellas
- all new closeout artifacts
- all new feature docs
- all five structured prompts for real issue execution
- signed trace proof for C-SDLC transition runs

### Phase 2: Normalize The Active Milestone

Migrate the active milestone's durable workflow records into tracked state:

- open issue cards
- active sprint records
- recently closed issues still used for milestone evidence
- current review and release artifacts
- signed trace proof packets and verification results

### Phase 3: Backfill Only High-Value History

Backfill older records only when they are still needed for:

- active reference value
- release evidence value
- governance value
- ObsMem ingestion value

Do not migrate every historical `.adl` artifact just because it exists.

## Tooling Implications

`v0.91.4` should update or schedule tooling so durable workflow state is no
longer silently local-only:

- workflow skills classify durable versus ephemeral artifacts
- editor skills write, normalize, or mirror durable card records under
  `docs/milestones/<version>/review/evidence/csdlc/issues/`
- `spp-editor` preserves issue-local operative plan truth and refuses to widen
  `SPP` into sprint orchestration
- sprint tooling writes canonical tracked sprint artifacts under
  `docs/milestones/<version>/review/evidence/csdlc/sprints/` by default
- closeout enforces durable record integration truth
- SORs reference tracked signed trace bundles under the durable namespace when
  proof is durable
- ObsMem ingestion consumes tracked evidence, not local lore

## Completion Bar

`v0.91.4` cannot honestly close if:

- the durable workflow namespace remains undefined or unused for new C-SDLC
  records
- durable C-SDLC cards remain local-only
- sprint closeout truth remains local-only
- review findings/dispositions remain local-only
- release evidence omits tracked proof packets
- signed trace bundles are absent for durable C-SDLC proof
- ObsMem ingestion depends on untracked local evidence
- any untracked or private scratch surface is treated as canonical C-SDLC truth

## Non-Claims

This plan does not require:

- full historical `.adl` backfill
- full TQL / trace-query implementation
- replacing GitHub issues, PRs, CI, branch protection, or human review
