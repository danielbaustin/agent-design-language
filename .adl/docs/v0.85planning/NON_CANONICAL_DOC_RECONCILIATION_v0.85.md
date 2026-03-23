# Planning Doc Reconciliation And Coverage Map

## Purpose

This document is now the working coverage map for the planning corpus from
`.adl/docs/v085planning/` forward and the tracked milestone docs from
`docs/milestones/v0.85/` forward.

Its job is to answer four questions for every planning file in the versioned
planning directories:

- what the file is for
- whether it is already represented in the roadmap / milestone structure
- whether it should become canonical, remain planning-only, or be deprecated
- whether it represents any remaining roadmap/documentation gap

We will use this document to make the future milestone directories complete and
consistent without silently dropping important planning work.

## Scope

Included here:

- all files in `.adl/docs/v085planning/`
- all files in `.adl/docs/v0.86planning/`
- all files in `.adl/docs/v09planning/`
- all files in `.adl/docs/v0.95planning/`
- all files in `docs/milestones/v0.85/`
- all files in `docs/milestones/v0.9/`

Out of scope for the table itself:

- earlier planning directories before `v0.85`
- non-existent future tracked milestone directories
- non-versioned whitepapers

Important out-of-scope dependency:

- `FREEDOM_DESIGNED.md` lives in `.adl/docs/whitepapers/` rather than a
  versioned planning directory, but it is roadmap-relevant and should be
  considered alongside the moral / constitutional cognition documents.

## Status Legend

| Status | Meaning |
| --- | --- |
| `Canonical overlap` | Already has a tracked/canonical counterpart or is effectively part of the canonical milestone set. |
| `Roadmap-critical` | Needed to make the roadmap/domain coverage honest and complete, even if not yet canonical. |
| `Promotable planning` | Worth preserving and likely promotable later, but not yet the canonical home. |
| `Planning-only` | Useful source material, notes, or admin history; should not drive canonical structure directly. |
| `Deprecated / archive` | Historical or superseded; keep only as reference if needed. |

## Existing Tracked Milestone Coverage

Tracked milestone directories currently present at `v0.85+`:

- `docs/milestones/v0.85/`
- `docs/milestones/v0.9/`

Tracked milestone directories not yet present:

- `docs/milestones/v0.86/`
- `docs/milestones/v0.95/`
- `docs/milestones/v1.0/`

That means this reconciliation document needs to do two jobs at once:

- map the planning-source docs that still need future canonical homes
- verify that the already-tracked milestone docs are represented and aligned

## Directory Assessment

### v0.85 Planning

This directory mixes:

- canonical-leaning `v0.85` milestone materials
- planning-only source material
- issue/admin reconciliation artifacts
- a few still-important future-looking docs that feed the larger roadmap

| File | Role | Status | Recommendation | Notes |
| --- | --- | --- | --- | --- |
| `ADL_EXPLAINED.md` | Explanatory / white-paper-style overview | `Promotable planning` | Promote later if a public/internal explanatory surface is wanted | Strong narrative summary, but not a feature spec. |
| `ADL_FEATURE_LIST.md` | Product/platform summary | `Promotable planning` | Promote later as positioning if desired | High-level synthesis, not detailed implementation guidance. |
| `AFFECTIVE_REASONING_MODEL.md` | Affect + reasoning integration planning | `Canonical overlap` | Resolved — canonical only | The tracked milestone version is the source of truth; the stale planning duplicate should be retired rather than merged. |
| `AFFECT_MODEL_v0.85.md` | Bounded affect model | `Roadmap-critical` | Move to `v0.91` planning; no v0.85 merge required right now | Future roadmap-critical planning surface; optional later rename can drop the `v0.85` suffix once the future planning tree is stable. |
| `DECISIONS_v0.85.md` | Milestone decisions log | `Canonical overlap` | Preserve and sync if needed | Canonical milestone support surface rather than stray planning. |
| `DESIGN_v0.85.md` | Milestone design doc | `Canonical overlap` | Preserve and sync if needed | Core milestone planning surface. |
| `DOCUMENTATION_RECONCILIATION_PLANNING.md` | Documentation cleanup planning | `Planning-only` | Keep as source material only | Useful for internal cleanup, not a target canonical doc. |
| `EDITING_ARCHITECTURE.md` | Editor / authoring architecture | `Promotable planning` | Mine for future canonical docs if needed | Real feature-bearing material, partly reflected in shipped/editor work. |
| `HTA_EDITOR_PLANNING.md` | HTA/editor workflow planning | `Promotable planning` | Keep as source material for authoring lifecycle work | Real implementable planning, not just narrative. |
| `HUMAN-IN-THE-LOOP-DESIGN-NOTES.MD` | HITL notes | `Planning-only` | Keep as notes unless folded into canonical authoring docs | Directionally useful but note-like. |
| `LAYER_8_IMPLEMENTATION.md` | Provider-contract / inference-substrate gap analysis | `Roadmap-critical` | Keep planning-only for now, but account for it explicitly in roadmap and future milestone docs | Important remaining feature area; not yet a tight implementation spec. |
| `MIDFLIGHT_REVIEW_ISSUES.md` | Review/reconciliation notes | `Planning-only` | Keep as historical planning record | Not feature-bearing. |
| `MILESTONE_CHECKLIST_v0.85.md` | Milestone checklist | `Canonical overlap` | Preserve and sync if needed | Canonical milestone support surface. |
| `NON_CANONICAL_DOC_RECONCILIATION_v0.85.md` | This reconciliation doc | `Promotable planning` | Keep current and use as the planning coverage map | Should now act as the inventory/control surface for reconciliation work. |
| `REASONING_GRAPH_SCHEMA_V0.85.md` | Reasoning-graph schema | `Canonical overlap` | Preserve and sync if needed | Already grounded in milestone/runtime work. |
| `RELEASE_NOTES_v0.85.md` | Release notes | `Canonical overlap` | Preserve and sync if needed | Canonical milestone support surface. |
| `RELEASE_PLAN_v0.85.md` | Release plan | `Canonical overlap` | Preserve and sync if needed | Canonical milestone support surface. |
| `ROAD_TO_v0.95.md` | Master roadmap / convergence doc | `Roadmap-critical` | Keep as the roadmap driver and continue updating | Core roadmap surface for future milestone coverage. |
| `Revised-issue-planning.md` | Issue-graph restructuring plan | `Planning-only` | Keep as historical planning/admin artifact | Useful history, not a feature doc. |
| `SPRINT_v0.85.md` | Sprint plan | `Canonical overlap` | Preserve and sync if needed | Canonical milestone support surface. |
| `SWARM_REMOVAL_PLANNING.md` | Tooling migration planning | `Promotable planning` | Keep as source material; fold remaining parts into tooling migration docs | Partly implemented, still relevant as migration history. |
| `V095_MVP_BOUNDARY.md` | Scope guardrail | `Roadmap-critical` | Keep planning-only, but use to constrain roadmap/promotions | Important boundary control, not a feature spec. |
| `VISION_v0.85.md` | Milestone vision | `Canonical overlap` | Preserve and sync if needed | Canonical milestone support surface. |
| `WBS_v0.85.md` | Work breakdown structure | `Canonical overlap` | Preserve and sync if needed | Canonical milestone support surface. |
| `WHY_RUST_FOR_ADL.md` | Rust rationale note | `Deprecated / archive` | Do not promote | Already marked deprecated; historical rationale only. |
| `reorg-issue-body-v0.85.md` | Issue-body artifact | `Planning-only` | Archive later if desired | Historical admin artifact. |

### v0.86 Planning

This directory is the main next-wave conceptual and architectural planning set.
It contains both central cognition docs and a small number of late-admitted
roadmap gaps that must be reflected explicitly.

| File | Role | Status | Recommendation | Notes |
| --- | --- | --- | --- | --- |
| `AEE_CONVERGENCE_MODEL.md` | AEE convergence architecture | `Roadmap-critical` | Promote into future milestone docs once terminology is normalized | Important planned model, but still needs consistency cleanup. |
| `AGENCY_AND_AGENTS.md` | Agency framing / agent ontology | `Promotable planning` | Keep as synthesis/source material | Broad conceptual framing rather than immediate bounded spec. |
| `COGNITIVE_ARBITRATION.md` | Arbitration architecture | `Roadmap-critical` | Promote as part of the cognition stack/loop set once reconciled | Central cognition architecture surface. |
| `COGNITIVE_LOOP_MODEL.md` | Cognitive loop authority doc | `Roadmap-critical` | Promote as a core future canonical doc after consolidation | One of the main roadmap-driving cognition docs. |
| `COGNITIVE_STACK.md` | Layered cognition architecture | `Roadmap-critical` | Promote as part of the cognition stack/loop set after consistency pass | Core future canonical architecture surface. |
| `CONCEPT_PLANNING.md` | Cross-cutting concept summary | `Promotable planning` | Keep only if it remains distinct from the loop/stack docs | Risk of duplicating other “authoritative” summaries. |
| `FAST_SLOW_THINKING_MODEL.md` | Fast/slow cognition model | `Roadmap-critical` | Keep and integrate with the other cognition docs | Important to the bounded cognition story. |
| `HUMOR_AND_ABSURDITY.md` | Humor/frame-shift cognition model | `Roadmap-critical` | Keep and assign explicitly to a future milestone slice | Already recognized as a real roadmap item. |
| `INSTINCT_MODEL.md` | Instinct model | `Roadmap-critical` | Keep and integrate with arbitration/loop docs | Important planned cognition domain. |
| `INTELLECTUAL_INFLUENCES.md` | Intellectual lineage/reference doc | `Planning-only` | Keep as reference only | Useful context, not a roadmap/control doc. |
| `KINDNESS_MODEL.md` | Kindness behavior model | `Roadmap-critical` | Keep and explicitly represent in future milestone docs | One of the late-recognized roadmap gaps. |
| `MORAL_RESOURCES_SUBSYSTEM.md` | Moral resources / memory / evaluation subsystem | `Roadmap-critical` | Keep and explicitly represent in future milestone docs | One of the late-recognized roadmap gaps. |
| `PHI_METRICS_FOR_ADL.md` | Phi / metrics exploration | `Planning-only` | Keep as exploratory material unless promoted intentionally | Interesting but not clearly part of the committed roadmap. |
| `SIGNED_TRACE_ARCHITECTURE.md` | Signed trace / provenance architecture | `Roadmap-critical` | Keep and explicitly represent in future milestone docs | One of the late-recognized roadmap gaps. |
| `SUBSTANCE_OF_TIME.md` | Temporal continuity / self-location model | `Roadmap-critical` | Keep and assign to a future cognition/identity slice | Important roadmap concept, but should avoid over-strong metaphysical language in canonical form. |
| `TOOLING_RUST_MIGRATION_PLAN.md` | Tooling migration / hardening plan | `Roadmap-critical` | Keep and explicitly represent in future milestone docs | One of the late-recognized roadmap gaps. |
| `VISION_NOTES__INSTINCT_AGENCY_AND_CIVILIZING_LLMS.md` | Vision notes / speculative framing | `Planning-only` | Keep as notes, mine carefully, do not promote directly | Contains useful ideas but also more speculative framing. |
| `WP_CANDIDATE__INSTINCT_AND_BOUNDED_AGENCY.md` | Work-package candidate draft | `Planning-only` | Keep as source material for issue writing only | Not a finished peer doc. |
| `ZED_INTEGRATION_WITH_ADL.md` | Zed integration planning | `Promotable planning` | Explicitly defer post-`v0.95` unless re-promoted | Recognized, but intentionally outside the current must-have set. |

### v0.9 Planning

This directory is the constitutional / delegation / identity wave. It contains
real future-roadmap material plus a few schema/example artifacts that should be
preserved alongside the design docs.

| File | Role | Status | Recommendation | Notes |
| --- | --- | --- | --- | --- |
| `ADL_AGENT_RIGHTS_AND_DUTIES.md` | Rights/duties model | `Roadmap-critical` | Keep and plan future canonical placement | Important constitutional/agent-governance surface. |
| `ADL_AGENT_SOCIAL_CONTRACT.md` | Social contract model | `Roadmap-critical` | Keep and plan future canonical placement | Closely related to constitutional/governance work. |
| `ADL_CONSTITUTIONAL_DELEGATION.md` | Constitutional delegation architecture | `Roadmap-critical` | Keep and plan future canonical placement | Major future roadmap area. |
| `ADL_IDENTITY_ARCHITECTURE.md` | Identity architecture | `Roadmap-critical` | Keep and plan future canonical placement | Important future identity/continuity surface. |
| `ADL_PROVIDER_CAPABILITIES.md` | Provider capability model | `Roadmap-critical` | Keep and connect to Layer 8/provider-contract work | Important counterpart to the Layer 8 roadmap gap. |
| `HYPOTHESIS_ENGINE_REASONING_GRAPH_V0.9.md` | Hypothesis engine / reasoning graph future architecture | `Roadmap-critical` | Keep and plan future canonical placement | Future extension of reasoning-graph work. |
| `V09_CONSTITUTIONAL_DELEGATION_WORKPLAN.md` | Constitutional delegation workplan | `Promotable planning` | Keep as roadmap/workplan source material | Strong implementation sequencing surface. |
| `adl_constitution.yaml` | Constitution schema/example artifact | `Promotable planning` | Keep with the constitutional docs | Not prose, but important supporting artifact. |
| `delegation_contract.example.yaml` | Delegation contract example | `Promotable planning` | Keep with the delegation docs | Important supporting artifact. |
| `freedom_gate_event.example.yaml` | Freedom-gate event example | `Promotable planning` | Keep with the constitutional docs | Supports freedom-gate/constitutional event modeling. |

### v0.95 Planning

This directory is currently small, but it already contains future canonical
surface area that should not be forgotten.

| File | Role | Status | Recommendation | Notes |
| --- | --- | --- | --- | --- |
| `TRACE_QUERY_LANGUAGE.md` | Trace-query language design | `Roadmap-critical` | Keep and plan future canonical placement | Important future observability / trace interrogation surface. |

## Tracked Milestone Docs

### docs/milestones/v0.85

This directory is the current canonical milestone set for `v0.85`. It already
contains the core release-planning and proof surfaces, plus a few forward-looking
architecture docs that should stay aligned with the planning corpus.

| File | Role | Status | Recommendation | Notes |
| --- | --- | --- | --- | --- |
| `AFFECTIVE_REASONING_MODEL.md` | Affect + reasoning milestone doc | `Canonical overlap` | Keep canonical | Canonical tracked surface. |
| `AFFECT_MODEL_v0.85.md` | Bounded affect milestone doc | `Canonical overlap` | Keep aligned with planning/source docs | Canonical tracked surface. |
| `CLUSTER_EXECUTION.md` | Execution/runtime planning | `Promotable planning` | Keep canonical unless superseded by stronger runtime docs | Partly implemented, partly future-facing. |
| `COGNITIVE_LOOP_MODEL_v0.85.md` | Canonical cognition loop reference | `Canonical overlap` | Preserve and reconcile carefully against `v0.86` loop docs | Architecture reference, not proof of full implementation. |
| `COGNITIVE_STACK_v0.85.md` | Canonical cognition stack reference | `Canonical overlap` | Preserve and reconcile carefully against `v0.86` stack docs | Architecture reference, not proof of full implementation. |
| `DECISIONS_v0.85.md` | Milestone decisions log | `Canonical overlap` | Keep canonical | Milestone support surface. |
| `DEMO_MATRIX_v0.85.md` | Demo matrix / proof-surface matrix | `Canonical overlap` | Keep canonical and continue filling via WP-18 | Important release/readiness surface. |
| `DESIGN_v0.85.md` | Milestone design doc | `Canonical overlap` | Keep canonical | Core milestone doc. |
| `EMOTION_MODEL.md` | Affect/emotion terminology bridge doc | `Promotable planning` | Reconcile terminology with affect docs before future reuse | Canonical tracked, but naming/consistency deserves cleanup. |
| `HUMAN-IN-THE-LOOP-DESIGN-NOTES.MD` | HITL notes | `Promotable planning` | Keep if still serving milestone review; otherwise fold into better docs later | Canonical-tracked notes surface. |
| `MILESTONE_CHECKLIST_v0.85.md` | Checklist | `Canonical overlap` | Keep canonical | Core milestone control surface. |
| `MILESTONE_ISSUE_RECONCILIATION_v0.85.md` | Issue reconciliation log | `Planning-only` | Keep as milestone admin history | Not a feature/architecture doc. |
| `README.md` | Milestone index | `Canonical overlap` | Keep canonical and current | Entry point for the milestone directory. |
| `REASONING_GRAPH_SCHEMA_V0.85.md` | Reasoning-graph schema | `Canonical overlap` | Keep canonical | Tied to real runtime surface. |
| `RELEASE_NOTES_v0.85.md` | Release notes | `Canonical overlap` | Keep canonical | Milestone support surface. |
| `RELEASE_PLAN_v0.85.md` | Release plan | `Canonical overlap` | Keep canonical | Milestone support surface. |
| `SPRINT_v0.85.md` | Sprint plan | `Canonical overlap` | Keep canonical | Milestone support surface. |
| `STRUCTURED_PROMPT_ARCHITECTURE.md` | Prompt architecture | `Canonical overlap` | Keep canonical and aligned with editor/task-bundle work | Important authoring/control-plane surface. |
| `SWARM_REMOVAL_PLANNING.md` | Tooling migration doc | `Promotable planning` | Keep canonical for now, but reconcile with later tooling migration docs | Migration/history plus future cleanup. |
| `VISION_v0.85.md` | Milestone vision | `Canonical overlap` | Keep canonical | Core milestone doc. |
| `WBS_v0.85.md` | Work breakdown structure | `Canonical overlap` | Keep canonical | Core milestone doc. |
| `WHY_RUST_FOR_ADL.md` | Rust rationale note | `Deprecated / archive` | Deprecate / do not promote further | Historical rationale only. |

### Shared v0.85 Mirror Drift

The following files exist in both `.adl/docs/v085planning/` and
`docs/milestones/v0.85/`. Their current sync state matters because the tracked
milestone directory should be treated as canonical for milestone-management
surfaces.

| File | Sync state | Current disposition | Notes |
| --- | --- | --- | --- |
| `AFFECTIVE_REASONING_MODEL.md` | Different | Resolved — canonical only | Canonical wording is already normalized; retire the stale planning duplicate rather than merging it. |
| `AFFECT_MODEL_v0.85.md` | Different | Move planning copy to `v0.91` planning; no v0.85 merge required right now | Not a duplicate of `AFFECTIVE_REASONING_MODEL.md`; treat as future roadmap-critical planning rather than a v0.85 canonical merge target. |
| `DECISIONS_v0.85.md` | Different | Canonical wins | Treat `docs/milestones/v0.85/` as source of truth. |
| `DESIGN_v0.85.md` | Different | Canonical wins | Treat `docs/milestones/v0.85/` as source of truth. |
| `HUMAN-IN-THE-LOOP-DESIGN-NOTES.MD` | Same | Already aligned | No action needed. |
| `MILESTONE_CHECKLIST_v0.85.md` | Different | Canonical wins | Treat `docs/milestones/v0.85/` as source of truth. |
| `REASONING_GRAPH_SCHEMA_V0.85.md` | Same | Already aligned | No action needed. |
| `RELEASE_NOTES_v0.85.md` | Different | Canonical wins | Treat `docs/milestones/v0.85/` as source of truth. |
| `RELEASE_PLAN_v0.85.md` | Different | Canonical wins | Treat `docs/milestones/v0.85/` as source of truth. |
| `SPRINT_v0.85.md` | Different | Canonical wins | Treat `docs/milestones/v0.85/` as source of truth. |
| `SWARM_REMOVAL_PLANNING.md` | Same | Already aligned | No action needed. |
| `VISION_v0.85.md` | Different | Canonical wins | Treat `docs/milestones/v0.85/` as source of truth. |
| `WBS_v0.85.md` | Different | Canonical wins | Treat `docs/milestones/v0.85/` as source of truth. |
| `WHY_RUST_FOR_ADL.md` | Different | Deprecate / do not sync forward | The planning copy is intentionally deprecated. |

### docs/milestones/v0.9

This directory is currently sparse. It already contains two important future
canonical docs, but it is far from a complete milestone directory.

| File | Role | Status | Recommendation | Notes |
| --- | --- | --- | --- | --- |
| `ADL_IDENTITY_ARCHITECTURE.md` | Identity architecture | `Canonical overlap` | Keep canonical and use as anchor for the future `v0.9` directory | Important future architecture surface. |
| `ADL_PROVIDER_CAPABILITIES.md` | Provider capability model | `Canonical overlap` | Keep canonical and align with Layer 8/provider work | Important future infrastructure surface. |

## Net Assessment

Across the versioned planning corpus from `v0.85` forward, the files break down
into five practical groups:

1. Canonical milestone overlap / support surfaces
   - the tracked `v0.85` docs
   - the existing tracked `v0.9` docs
   - the mirrored `.adl` milestone-support docs

2. Roadmap-critical future architecture
   - cognition stack/loop/arbitration/instinct/fast-slow
   - AEE convergence
   - temporal continuity
   - moral / kindness / constitutional cognition
   - signed trace
   - tooling Rust migration
   - Layer 8 provider-contract maturation
   - identity / delegation / provider capabilities
   - trace query language

3. Promotable planning source material
   - explanatory docs
   - workplans
   - example schemas/artifacts
   - editor/authoring source material

4. Planning-only notes/admin history
   - issue restructuring artifacts
   - loose notes and references
   - work-package candidate drafts

5. Deprecated / archival material
   - `WHY_RUST_FOR_ADL.md`

## Remaining Roadmap / Documentation Gaps

As of this reconciliation pass, the main roadmap-sensitive items that must stay
visible are:

- `ROAD_TO_v0.95.md`
- `V095_MVP_BOUNDARY.md`
- `LAYER_8_IMPLEMENTATION.md`
- `AEE_CONVERGENCE_MODEL.md`
- `COGNITIVE_LOOP_MODEL.md`
- `COGNITIVE_STACK.md`
- `COGNITIVE_ARBITRATION.md`
- `FAST_SLOW_THINKING_MODEL.md`
- `INSTINCT_MODEL.md`
- `HUMOR_AND_ABSURDITY.md`
- `SUBSTANCE_OF_TIME.md`
- `KINDNESS_MODEL.md`
- `MORAL_RESOURCES_SUBSYSTEM.md`
- `SIGNED_TRACE_ARCHITECTURE.md`
- `TOOLING_RUST_MIGRATION_PLAN.md`
- `ADL_IDENTITY_ARCHITECTURE.md`
- `ADL_CONSTITUTIONAL_DELEGATION.md`
- `ADL_PROVIDER_CAPABILITIES.md`
- `TRACE_QUERY_LANGUAGE.md`

And the tracked canonical milestone surfaces that must stay complete and aligned
while the future directories are built are:

- `docs/milestones/v0.85/README.md`
- `docs/milestones/v0.85/VISION_v0.85.md`
- `docs/milestones/v0.85/DESIGN_v0.85.md`
- `docs/milestones/v0.85/WBS_v0.85.md`
- `docs/milestones/v0.85/SPRINT_v0.85.md`
- `docs/milestones/v0.85/MILESTONE_CHECKLIST_v0.85.md`
- `docs/milestones/v0.85/RELEASE_PLAN_v0.85.md`
- `docs/milestones/v0.85/RELEASE_NOTES_v0.85.md`
- `docs/milestones/v0.85/DEMO_MATRIX_v0.85.md`
- `docs/milestones/v0.9/ADL_IDENTITY_ARCHITECTURE.md`
- `docs/milestones/v0.9/ADL_PROVIDER_CAPABILITIES.md`

And one intentionally recognized deferral:

- `ZED_INTEGRATION_WITH_ADL.md`

## Recommended Next Documentation Passes

1. Build the future milestone directories from the `Roadmap-critical` set
   first, not from the note/admin/history docs.
2. Treat the cognition docs as a consolidation cluster:
   loop, stack, arbitration, instinct, fast/slow, humor, time.
3. Treat the moral / constitutional docs as a second consolidation cluster:
   kindness, moral resources, freedom-designed, rights/duties, social
   contract, constitutional delegation.
4. Treat signed trace, tooling migration, Layer 8/provider capabilities, and
   trace query as an infrastructure/proof cluster.
5. Preserve explanatory, note, and issue-history docs, but do not let them
   silently define canonical structure.

## Layer 8 Assessment

### Short answer

`LAYER_8_IMPLEMENTATION.md` still identifies a real missing feature area, but
it is not yet sufficient by itself as a complete implementation spec.

### What is already real

The repo already has a provider abstraction and runtime integration. The gap is
not “no provider layer.” The gap is the stronger contract layer above that
abstraction.

### What remains missing

Before Layer 8 can be treated as implementation-ready, we still need bounded
definitions for:

- provider request/response contracts
- capability description / negotiation
- replayable inference artifacts
- prompt/inference normalization
- provider proof surfaces and tests

### Current stance

Treat Layer 8 as:

- roadmap-critical
- still planning-only
- best grouped with provider capabilities, signed trace, tooling hardening, and
  future trace/query work
