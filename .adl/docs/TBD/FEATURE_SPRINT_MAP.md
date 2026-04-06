# FEATURE_SPRINT_MAP.md

## Purpose

Map the active feature docs in `.adl/docs/TBD/` onto concrete milestone homes so there is no unallocated architecture left in the planning corpus.

This document is the detailed roadmap-normalization pass for the remaining TBD feature docs.

Primary goals:
- every active TBD feature doc gets a milestone home
- planning/meta/backlog artifacts that are intentionally not milestone feature docs are explicitly called out as retained TBD material
- `v0.87` stays focused on the already-seeded substrate work while absorbing the operational skill schemas it clearly owns
- `v0.87.1` completes the runtime environment rather than deferring it behind later cognition/governance layers
- `v0.88` through `v0.95` stay coherent rather than becoming generic overflow buckets
- heavy CBAC / policy / isolation / secrets work is pushed later instead of distorting the runtime-first milestones
- MTT remains real roadmap work and is explicitly placed in `v0.93`
- OSS sentience/continuity identity stays distinct from enterprise authn/authz identity

This pass intentionally ignores:
- backlog docs
- `CODE_REVIEW_SKILL_NOTES.md`
- `MILESTONE_RESTRUCTURING.md`

---

## Planning Rules

### 1. Runtime first
The runtime must be made real before later cognition, governance, and enterprise control layers expand.

### 2. No unowned active TBD docs
Every active feature/architecture document in `.adl/docs/TBD/` must map to a milestone or an explicit split-boundary decision.
Backlog and planning/meta artifacts may remain in `TBD`, but they must be named explicitly as such.

### 3. Heavy security later
Full CBAC, policy-engine, sandbox hardening, provider trust/isolation, secrets, and compliance-heavy work move later unless a smaller runtime prerequisite must stay core.

### 4. Operational skills stay core
The issue/bootstrap/doctor/janitor skill schemas belong in the OSS control-plane roadmap, not an enterprise track.

### 5. Identity split
Sentience, continuity, temporal grounding, and fork/join identity remain in the OSS roadmap.
RBAC, enterprise authentication, authorization, and compliance-heavy identity belong later.

### 6. Feature-doc discipline
Every milestone should have clear feature-owner docs; planning-only notes should not masquerade as canonical feature surfaces.

---

## Milestone Distribution Summary

### `v0.87`
Current substrate work plus operational skill schemas already implied by the control-plane and PR-tooling surface.

### `v0.87.1`
Runtime completion: runtime environment, lifecycle, execution boundaries, local resilience, Shepherd orchestration.

### `v0.88`
Chronosense, instinct, and bounded agency.

### `v0.89`
Execution intelligence: GHB, AEE-adjacent cognition, and reasoning patterns.

### `v0.90`
Reasoning representation, signed truth, and query.

### `v0.91`
Affect, kindness, moral resources, humor, wellbeing.

### `v0.92`
Identity substrate, capability substrate, continuity validation, fork/join identity behavior.

### `v0.93`
Social/ethical reasoning and MTT.

### `v0.94`
Heavy governance, CBAC, policy engine, sandbox isolation, provider trust/isolation, secrets, and enterprise-heavy auth/control.

### `v0.95`
MVP convergence, walkthrough, demos, tooling migration, optional Zed.

---

## Full Feature-Doc Set By Milestone

## `v0.87` — Seeded substrate plus operational skill schemas

### Existing `v0.87` feature docs

| Doc | Feature it owns |
|---|---|
| `docs/milestones/v0.87/features/TRACE_SCHEMA_V1.md` | canonical Trace v1 event vocabulary and schema contract |
| `docs/milestones/v0.87/features/TRACE_RUNTIME_EMISSION.md` | inline runtime emission of trace events at real execution/control points |
| `docs/milestones/v0.87/features/TRACE_ARTIFACT_MODEL.md` | artifact truth model and artifact-to-trace linkage rules |
| `docs/milestones/v0.87/features/TRACE_VALIDATION_AND_REVIEW.md` | what makes trace valid and review-grade |
| `docs/milestones/v0.87/features/TRACE_REVIEW_PIPELINE.md` | staged pipeline from trace/artifacts to review outputs |
| `docs/milestones/v0.87/features/TRACE_OBSMEM_INGESTION.md` | trace-to-memory ingestion boundary and transformation rules |
| `docs/milestones/v0.87/features/PROVIDER_SUBSTRATE_FEATURE.md` | provider substrate v1 plus portability/config compatibility surface |
| `docs/milestones/v0.87/features/SHARED_OBSMEM_IMPLEMENTATION.md` | shared ObsMem foundation substrate |
| `docs/milestones/v0.87/features/OPERATIONAL_SKILLS_SUBSTRATE.md` | bounded operational skills as first-class runtime/control-plane surfaces |
| `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md` | architectural simplification of PR tooling and control-plane ownership |
| `docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md` | concrete feature-level PR tooling simplification scope |
| `docs/milestones/v0.87/features/REVIEW_SURFACE_FORMALIZATION.md` | canonical review output structure and severity/evidence model |
| `docs/milestones/v0.87/features/PR_TOOLING_SKILLS.md` | skill family for PR/control-plane workflows |
| `docs/milestones/v0.87/features/PREFLIGHT_CHECK_SKILL.md` | preflight skill contract and execution surface |

### Additional feature docs to promote from TBD into `v0.87`

| TBD doc | Feature it should own once promoted | Promote / copy destination |
|---|---|---|
| `ISSUE_BOOTSTRAP_SKILL_INPUT_SCHEMA.md` | input contract for issue-bootstrap skill workflows | `docs/milestones/v0.87/features/ISSUE_BOOTSTRAP_SKILL_INPUT_SCHEMA.md` |
| `PR_DOCTOR_SKILL_INPUT_SCHEMA.md` | input contract for doctor skill workflows | `docs/milestones/v0.87/features/PR_DOCTOR_SKILL_INPUT_SCHEMA.md` |

### `v0.87` distribution logic
- Keep all currently seeded trace, provider, shared-memory, review, and control-plane substrate work here.
- Absorb the operational skill schemas here because they directly strengthen the existing control-plane surface.
- `PR_JANITOR_SKILL_INPUT_SCHEMA.md` currently lives at `.adl/docs/skills/PR_JANITOR_SKILL_INPUT_SCHEMA.md` rather than in `TBD`; fold it into the `v0.87` skills work under issue `1299`.
- Treat the WP-08 PR-skill-family goal as a deliberate extension of the already-seeded control-plane scope, not as a redefinition of the milestone away from substrate work.
- WP-08 should aim to leave a bounded PR-process skill family in place, not merely a single demonstration skill.
- Even after that work lands, the skills subsystem should still be treated as an incomplete first-class-system candidate with additional follow-on work likely required later.
- Redistribution under `1316` should move future docs into `.adl/docs/v0.*planning/`; promotion into `docs/milestones/v0.*/features/` should happen only when the milestone is open or intentionally made public.

---

## `v0.87.1` — Runtime completion

### Feature docs to move from TBD into `v0.87.1`

| TBD doc | Feature it should own once moved | Destination |
|---|---|---|
| `ADL_RUNTIME_ENVIRONMENT.md` | runtime environment contract and implementation-facing environment model | `.adl/docs/v0.87.1planning/ADL_RUNTIME_ENVIRONMENT.md` |
| `ADL_RUNTIME_ENVIRONMENT_ARCHITECTURE.md` | overall runtime environment architecture for ADL execution | `.adl/docs/v0.87.1planning/ADL_RUNTIME_ENVIRONMENT_ARCHITECTURE.md` |
| `AGENT_LIFECYCLE.md` | agent lifecycle phases and invoke-path lifecycle semantics | `.adl/docs/v0.87.1planning/AGENT_LIFECYCLE.md` |
| `EXECUTION_BOUNDARIES.md` | hard execution boundaries between user, agent, skill, tool, and provider | `.adl/docs/v0.87.1planning/EXECUTION_BOUNDARIES.md` |
| `LOCAL_RUNTIME_RESILIENCE.md` | runtime resilience and local failure hardening behavior | `.adl/docs/v0.87.1planning/LOCAL_RUNTIME_RESILIENCE.md` |
| `SHEPHERD_RUNTIME_MODEL.md` | orchestration/supervision model for the runtime control plane | `.adl/docs/v0.87.1planning/SHEPHERD_RUNTIME_MODEL.md` |

### `v0.87.1` distribution logic
- This milestone makes the runtime real before later cognition/governance layers expand.
- It depends on the `v0.87` trace, provider, shared-memory, review, and control-plane substrate already existing.
- `EXECUTION_BOUNDARIES.md` stays core and early; it is not an enterprise-only doc.

---

## `v0.88` — Chronosense and temporal grounding

### Existing `v0.88` planning docs

| Doc | Feature it owns |
|---|---|
| `.adl/docs/v0.88planning/APTITUDE_MODEL.md` | aptitude model for bounded capability shaping |
| `.adl/docs/v0.88planning/INSTINCT_MODEL.md` | instinct model itself |
| `.adl/docs/v0.88planning/INSTINCT_RUNTIME_SURFACE.md` | runtime surface for instinct signals |
| `.adl/docs/v0.88planning/PHI_METRICS_FOR_ADL.md` | phi/integration metrics for bounded agency evaluation |
| `.adl/docs/v0.88planning/SUBSTANCE_OF_TIME.md` | chronosense framing and temporal substrate direction |
| `.adl/docs/v0.88planning/WP_INSTINCT_AND_BOUNDED_AGENCY.md` | bounded-agency workplan tying instinct and agency together |

### Additional feature docs to move from TBD into `v0.88`

| TBD doc | Feature it should own once moved | Destination |
|---|---|---|
| `CHRONOSENSE_AND_IDENTITY.md` | chronosense as the precursor to continuity and temporal self-location | `.adl/docs/v0.88planning/CHRONOSENSE_AND_IDENTITY.md` |
| `TEMPORAL_SCHEMA_V01.md` | temporal data model supporting chronosense and persistence | `.adl/docs/v0.88planning/TEMPORAL_SCHEMA_V01.md` |

### `v0.88` distribution logic
- Keep temporal grounding, instinct, aptitude, and bounded-agency work here.

---

## `v0.89` — Intelligence layer: GHB, AEE, reasoning patterns

### Existing `v0.89` planning docs

| Doc | Feature it owns |
|---|---|
| `.adl/docs/v0.89planning/AEE_CONVERGENCE_MODEL.md` | bounded AEE convergence behavior |
| `.adl/docs/v0.89planning/FREEDOM_GATE_V2.md` | second-generation Freedom Gate / citizenship band |
| `.adl/docs/v0.89planning/SECURITY_AND_THREAT_MODELING.md` | threat-model precursor work associated with the convergence band |

### Additional feature docs to move from TBD into `v0.89`

| TBD doc | Feature it should own once moved | Destination |
|---|---|---|
| `GHB_EXECUTION_MODEL.md` | execution model for GHB loops | `.adl/docs/v0.89planning/GHB_EXECUTION_MODEL.md` |
| `GHB_ALGORITHM_AND_STATE_SPACE_COMPRESSION.md` | GHB algorithm details and compression behavior | `.adl/docs/v0.89planning/GHB_ALGORITHM_AND_STATE_SPACE_COMPRESSION.md` |
| `REASONING_PATTERNS_CATALOG.md` | catalog of reusable reasoning patterns for the intelligence layer | `.adl/docs/v0.89planning/REASONING_PATTERNS_CATALOG.md` |

### `v0.89` distribution logic
- Keep AEE convergence, GHB execution, and reasoning patterns together.
- This is the right home for the catalog instead of `v0.90`.
- This milestone is about execution intelligence and reusable cognitive patterns, not final truth representation.

---

## `v0.90` — Reasoning graph, signed trace, trace query

### Existing `v0.90` planning docs

| Doc | Feature it owns |
|---|---|
| `.adl/docs/v0.90planning/HYPOTHESIS_ENGINE_REASONING_GRAPH_V0.9.md` | hypothesis engine and reasoning-graph baseline |
| `.adl/docs/v0.90planning/SIGNED_TRACE_ARCHITECTURE.md` | signed-trace architecture |
| `.adl/docs/v0.90planning/TRACE_QUERY_LANGUAGE.md` | trace query language |

### `v0.90` distribution logic
- Keep reasoning-graph structure, signed trace, and trace query together.
- No TBD additions are required here in this reconciled plan.
- This milestone is about representation, signed truth, and query over reasoning artifacts rather than the execution-intelligence loop itself.

---

## `v0.91` — Affect, kindness, moral resources, humor, wellbeing

### Full `v0.91` feature-doc set

| Doc | Feature it owns |
|---|---|
| `.adl/docs/v0.91planning/AFFECT_MODEL_v0.90.md` | affect engine/model |
| `.adl/docs/v0.91planning/CULTIVATING_INTELLIGENCE.md` | cultivation framing for intelligence growth |
| `.adl/docs/v0.91planning/HUMOR_AND_ABSURDITY.md` | humor/absurdity subsystem |
| `.adl/docs/v0.91planning/KINDNESS_MODEL.md` | kindness model |
| `.adl/docs/v0.91planning/MORAL_RESOURCES_SUBSYSTEM.md` | moral-resources subsystem |
| `.adl/docs/v0.91planning/WELLBEING_AND_HAPPINESS.md` | wellbeing and happiness surfaces |

### `v0.91` distribution logic
- Keep affective and moral-cognition surfaces together.

---

## `v0.92` — Identity substrate, capability substrate, continuity enforcement

### Existing `v0.92` planning docs

| Doc | Feature it owns |
|---|---|
| `.adl/docs/v0.92planning/ADL_IDENTITY_ARCHITECTURE.md` | identity substrate architecture |
| `.adl/docs/v0.92planning/ADL_PROVIDER_CAPABILITIES.md` | provider capability substrate |
| `.adl/docs/v0.92planning/NAMES_AND_IDENTITY.md` | naming and identity semantics |
| `.adl/docs/v0.92planning/NARRATIVE_IDENTITY_CONTINUITY.md` | narrative continuity layer |
| `.adl/docs/v0.92planning/PROVIDER_CAPABILITY_AND_TRANSPORT_ARCHITECTURE.md` | capability/transport architecture |

### Additional feature docs to move from TBD into `v0.92`

| TBD doc | Feature it should own once moved | Destination |
|---|---|---|
| `CAPABILITY_MODEL.md` | general capability model feeding the identity/capability substrate | `.adl/docs/v0.92planning/CAPABILITY_MODEL.md` |
| `CONTINUITY_VALIDATION.md` | continuity validation logic | `.adl/docs/v0.92planning/CONTINUITY_VALIDATION.md` |
| `CONTINUITY_VALIDATION_SCHEMA.md` | continuity validation schema | `.adl/docs/v0.92planning/CONTINUITY_VALIDATION_SCHEMA.md` |
| `FORK_JOIN_AND_IDENTITY.md` | fork/join semantics under identity continuity | `.adl/docs/v0.92planning/FORK_JOIN_AND_IDENTITY.md` |

### `v0.92` distribution logic
- Keep sentience/continuity identity, capabilities, continuity validation, and concurrency/identity interaction together.
- Do not confuse this with enterprise authn/authz identity.

---

## `v0.93` — Social, ethical, and MTT extension layer

### Existing `v0.93` planning docs

| Doc | Feature it owns |
|---|---|
| `.adl/docs/v0.93planning/ADL_AGENT_RIGHTS_AND_DUTIES.md` | rights/duties surface |
| `.adl/docs/v0.93planning/ADL_AGENT_SOCIAL_CONTRACT.md` | agent social contract |
| `.adl/docs/v0.93planning/ADL_CONSTITUTIONAL_DELEGATION.md` | constitutional delegation model |
| `.adl/docs/v0.93planning/CITIZENSHIP_MODEL.md` | citizenship semantics |
| `.adl/docs/v0.93planning/RELATIONSHIP_MODEL.md` | relationship model |
| `.adl/docs/v0.93planning/REPUTATION_AND_TRUST.md` | reputation and trust subsystem |
| `.adl/docs/v0.93planning/V09_CONSTITUTIONAL_DELEGATION_WORKPLAN.md` | workplan for constitutional delegation |

### Additional feature docs to move from TBD into `v0.93`

| TBD doc | Feature it should own once moved | Destination |
|---|---|---|
| `COGNITIVE_ETHICS.md` | cognitive-ethics framing for governed delegation and social contract work | `.adl/docs/v0.93planning/COGNITIVE_ETHICS.md` |
| `A_LA_RECHERCHE_DU_TEMPS_PERDU_MENTAL_TIME_TRAVEL_MTT_V1.md` | mental time travel extension layer | `.adl/docs/v0.93planning/A_LA_RECHERCHE_DU_TEMPS_PERDU_MENTAL_TIME_TRAVEL_MTT_V1.md` |

### `v0.93` distribution logic
- Keep delegation, social contract, rights/duties, relationship, trust, ethics, and MTT together.
- MTT is explicitly in scope for the roadmap here and is not dropped or hand-waved away.

---

## `v0.94` — Heavy governance, security, sandbox, provider trust/isolation

### Existing `v0.94` planning docs
- `.adl/docs/v0.94planning/` is currently empty and should receive the docs listed below.

### Feature docs to move from TBD into `v0.94`

| TBD doc | Feature it should own once moved | Destination |
|---|---|---|
| `CBAC_ARCHITECTURE.md` | CBAC architecture | `.adl/docs/v0.94planning/CBAC_ARCHITECTURE.md` |
| `POLICY_ENGINE.md` | policy-engine architecture | `.adl/docs/v0.94planning/POLICY_ENGINE.md` |
| `PROVIDER_TRUST_AND_ISOLATION_ARCHITECTURE.md` | provider trust and isolation architecture | `.adl/docs/v0.94planning/PROVIDER_TRUST_AND_ISOLATION_ARCHITECTURE.md` |
| `SANDBOX_RUNTIME_ISOLATION_ARCHITECTURE.md` | sandbox runtime isolation architecture | `.adl/docs/v0.94planning/SANDBOX_RUNTIME_ISOLATION_ARCHITECTURE.md` |
| `SECRETS_AND_DATA_GOVERNANCE.md` | secrets and data-governance model | `.adl/docs/v0.94planning/SECRETS_AND_DATA_GOVERNANCE.md` |
| `SECURITY_MODEL_PLANNING.md` | umbrella security-model planning | `.adl/docs/v0.94planning/SECURITY_MODEL_PLANNING.md` |

### Split-boundary docs requiring explicit handling

| TBD doc | Required decision |
|---|---|
| `SECURE_EXECUTION_MODEL.md` | keep core runtime/execution semantics aligned with OSS milestones; move enterprise/compliance/enforcement-heavy parts later |
| `IDENTITY_AND_AUTHENTICATION_ARCHITECTURE.md` | keep sentience/continuity identity in OSS roadmap; move authn/authz/RBAC/compliance identity later |

### Split-boundary resolution rule
- core execution semantics remain in OSS milestone bands
- sentience, continuity, and temporal identity remain in OSS milestone bands
- enterprise enforcement, compliance, authn/authz, RBAC, and isolation-heavy extensions move to `v0.94`

### `v0.94` distribution logic
- Put heavy governance, CBAC, sandbox, trust/isolation, secrets, and enterprise-heavy control work here.
- Do not compromise the OSS runtime/cognition roadmap for enterprise needs.

---

## `v0.95` — MVP convergence, walkthrough, demos, tooling migration, optional Zed

### Existing `v0.95` planning docs

| Doc | Feature it owns |
|---|---|
| `.adl/docs/v0.95planning/MVP_WALKTHROUGH_AND_DEMOS.md` | MVP walkthrough and demos |
| `.adl/docs/v0.95planning/PLATFORM_CONVERGENCE_PLAN.md` | full-platform convergence plan |
| `.adl/docs/v0.95planning/SECURITY_AND_THREAT_MODELING.md` | security/threat-model convergence view |
| `.adl/docs/v0.95planning/TOOLING_RUST_MIGRATION_PLAN.md` | Rust tooling migration plan |
| `.adl/docs/v0.95planning/TRACE_QUERY_LANGUAGE.md` | trace-query convergence copy if intentionally retained here |
| `.adl/docs/v0.95planning/ZED_INTEGRATION_WITH_ADL.md` | optional Zed integration |

### `v0.95` distribution logic
- Keep final walkthrough, convergence, tooling migration, optional Zed, and late-promotion cleanup here.

---

## What Should Remain In TBD After 1316

After the redistribution pass, `TBD` should retain only:
- backlog docs
- planning/meta docs used to drive redistribution
- genuinely new untriaged drafts

Expected remaining docs:
- `BACKLOG_COGNITIVE_SPACETIME.md`
- `BACKLOG_FREEDOM_DESIGNED.md`
- `CODE_REVIEW_SKILL_NOTES.md`
- `FEATURE_SPRINT_MAP.md`
- `LATER_MILESTONE_FEATURE_PROMOTION_CANDIDATES.md`
- `MILESTONE_RESTRUCTURING.md`
- `NEW_FEATURE_MAP.md`

These are intentionally retained as backlog or planning/meta artifacts and are not part of milestone feature distribution.

---

## Immediate Follow-on Actions

1. Create `.adl/docs/v0.87.1planning/`.
2. Move the runtime-completion docs into `v0.87.1planning`.
3. Promote the skill-schema docs into the `v0.87` feature surface.
4. Move the remaining active TBD docs into the milestone planning directories listed above.
5. Resolve the two split-boundary docs explicitly rather than forcing them wholesale into a single track.
6. Reduce `TBD` to backlog and planning-meta material.
