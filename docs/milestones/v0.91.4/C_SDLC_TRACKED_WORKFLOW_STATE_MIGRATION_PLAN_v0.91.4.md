# v0.91.4 C-SDLC Tracked Workflow State Migration Plan

## Status

Tracked migration plan for completing the Cognitive SDLC rollout.

This document promotes the durable workflow-state migration plan from local
planning notes into the public/auditable `v0.91.4` milestone package.

## Decision

By the end of `v0.91.4`, all durable C-SDLC truth must be tracked in Git.

The canonical C-SDLC record is:

- GitHub issue and PR truth
- tracked repo files
- tracked workflow records
- tracked proof, review, trace, and release evidence

Local `.adl` state may remain for ephemeral execution support only. Google
Workspace, if used at all, is optional scratch/staging; it is not part of
C-SDLC and is not canonical lifecycle truth.

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
- editor skills understand tracked card records
- sprint tooling writes canonical tracked sprint artifacts by default
- closeout enforces durable record integration truth
- SORs reference tracked signed trace bundles when proof is durable
- ObsMem ingestion consumes tracked evidence, not local lore

## Completion Bar

`v0.91.4` cannot honestly close if:

- durable C-SDLC cards remain local-only
- sprint closeout truth remains local-only
- review findings/dispositions remain local-only
- release evidence omits tracked proof packets
- signed trace bundles are absent for durable C-SDLC proof
- ObsMem ingestion depends on untracked local evidence
- GWS or any other external scratch surface is treated as canonical C-SDLC truth

## Non-Claims

This plan does not require:

- full historical `.adl` backfill
- full TQL / trace-query implementation
- Google Workspace
- replacing GitHub issues, PRs, CI, branch protection, or human review
