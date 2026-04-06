TBD Feature Distribution and Milestone Mapping (Reconciled)

Purpose

This document defines the redistribution of the active feature documents in .adl/docs/TBD/ into milestone planning directories or clearly scoped later tracks.

Goals:
	•	eliminate TBD as a dumping ground for active feature docs
	•	assign every relevant active TBD feature doc to a milestone or explicit split-boundary decision
	•	introduce v0.87.1 for runtime completion work
	•	keep operational skill schemas in the OSS core roadmap
	•	separate sentience/continuity identity from enterprise auth/RBAC identity
	•	preserve MTT as real roadmap work in v0.93

⸻

Guiding Principles
	1.	One architectural theme per milestone
	2.	Runtime first
	3.	Operational skills belong in core ADL
	4.	Do not compromise the OSS runtime/cognition roadmap for enterprise needs
	5.	Enterprise enforcement, authz, sandboxing, and compliance can land later
	6.	Planning first → promotion later

⸻

Milestone Distribution

⸻

v0.87 — Substrate (Expanded only for skill schemas)

Scope remains the current seeded substrate milestone, but the operational skill schemas are absorbed here because they directly support the existing control-plane and PR-tooling work.

Includes:
	•	Trace schema + emission + validation
	•	Provider substrate (base abstraction)
	•	ObsMem foundation
	•	Review pipeline + PR tooling
	•	Operational skills substrate
	•	Skill input schemas:
		•	ISSUE_BOOTSTRAP_SKILL_INPUT_SCHEMA.md
		•	PR_DOCTOR_SKILL_INPUT_SCHEMA.md
		•	PR_JANITOR_SKILL_INPUT_SCHEMA.md

Location note:
	•	ISSUE_BOOTSTRAP_SKILL_INPUT_SCHEMA.md and PR_DOCTOR_SKILL_INPUT_SCHEMA.md currently live under .adl/docs/skills/
	•	PR_JANITOR_SKILL_INPUT_SCHEMA.md currently lives at .adl/docs/skills/PR_JANITOR_SKILL_INPUT_SCHEMA.md and should be folded into the v0.87 skills work under issue 1299 rather than treated as a TBD doc

Implementation note:
	•	WP-08 / issue 1299 should aim to land a bounded family of roughly 5-6 PR-process skills where feasible, not a single toy skill.
	•	That still does not make the skills subsystem fully first-class; additional later follow-on work is expected.

⸻

v0.87.1 — Runtime Completion (NEW)

Purpose:
Complete the runtime as a deterministic, enforceable execution environment before later cognition and governance layers expand.

Planning Directory:
.adl/docs/v0.87.1planning/

Feature Docs:

Runtime Core
	•	ADL_RUNTIME_ENVIRONMENT.md
	•	ADL_RUNTIME_ENVIRONMENT_ARCHITECTURE.md

Execution Model
	•	AGENT_LIFECYCLE.md
	•	EXECUTION_BOUNDARIES.md

Stability
	•	LOCAL_RUNTIME_RESILIENCE.md

Orchestration
	•	SHEPHERD_RUNTIME_MODEL.md

⸻

v0.88 — Chronosense + Temporal Grounding

Purpose:
Introduce time, temporal self-location, and the first substrate for continuity-aware identity.

Feature Docs:
	•	CHRONOSENSE_AND_IDENTITY.md
	•	TEMPORAL_SCHEMA_V01.md

⸻

v0.89 — Intelligence Layer: GHB, AEE, Reasoning Patterns

Purpose:
Introduce self-improving reasoning, execution semantics, and reusable reasoning structures.

Feature Docs:
	•	GHB_EXECUTION_MODEL.md
	•	GHB_ALGORITHM_AND_STATE_SPACE_COMPRESSION.md
	•	REASONING_PATTERNS_CATALOG.md

⸻

v0.90 — Reasoning Graph + Signed Truth

Purpose:
Establish structured reasoning graphs, signed trace, and queryable truth surfaces.

Existing Feature Set:
	•	HYPOTHESIS_ENGINE_REASONING_GRAPH_V0.9.md
	•	SIGNED_TRACE_ARCHITECTURE.md
	•	TRACE_QUERY_LANGUAGE.md

No TBD additions required in this pass.

⸻

v0.91 — Affect + Moral Cognition

No TBD redistribution required in this pass; existing scope retained.

⸻

v0.92 — Identity Enforcement Layer

Purpose:
Make identity continuity and capability semantics operational in the OSS roadmap.

Feature Docs:
	•	CAPABILITY_MODEL.md
	•	CONTINUITY_VALIDATION.md
	•	CONTINUITY_VALIDATION_SCHEMA.md
	•	FORK_JOIN_AND_IDENTITY.md

This milestone is for sentience/continuity identity, not enterprise RBAC/authz.

⸻

v0.93 — Social / Ethical / Temporal Extension Layer

Purpose:
Introduce social/ethical reasoning and preserve MTT as real roadmap work.

Feature Docs:
	•	COGNITIVE_ETHICS.md
	•	A_LA_RECHERCHE_DU_TEMPS_PERDU_MENTAL_TIME_TRAVEL_MTT_V1.md

⸻

v0.94 — Heavy Security / Governance / Enterprise-Oriented Control

Purpose:
Provide later-cycle security, enforcement, isolation, and governance work without distorting the core runtime-first OSS sequence.

Feature Docs:
	•	CBAC_ARCHITECTURE.md
	•	POLICY_ENGINE.md
	•	PROVIDER_TRUST_AND_ISOLATION_ARCHITECTURE.md
	•	SANDBOX_RUNTIME_ISOLATION_ARCHITECTURE.md
	•	SECRETS_AND_DATA_GOVERNANCE.md
	•	SECURITY_MODEL_PLANNING.md

Split-boundary docs requiring explicit handling:
	•	SECURE_EXECUTION_MODEL.md
	•	IDENTITY_AND_AUTHENTICATION_ARCHITECTURE.md

Rule:
	•	core runtime/execution semantics stay aligned with OSS milestones
	•	sentience, continuity, and temporal identity stay aligned with OSS milestones
	•	authn/authz/RBAC/compliance/enforcement-heavy content can land in the later enterprise-oriented band

Planning promotion rule:
	•	redistribution moves docs into .adl/docs/v0.*planning/ first
	•	promotion into docs/milestones/v0.*/features/ happens when that milestone opens or is intentionally made public

⸻

What Stays In TBD After Redistribution

TBD should retain only:
	•	incomplete drafts
	•	backlog docs
	•	meta-planning docs for redistribution/restructuring

Expected remaining docs:
	•	BACKLOG_COGNITIVE_SPACETIME.md
	•	BACKLOG_FREEDOM_DESIGNED.md
	•	CODE_REVIEW_SKILL_NOTES.md
	•	FEATURE_SPRINT_MAP.md
	•	LATER_MILESTONE_FEATURE_PROMOTION_CANDIDATES.md
	•	MILESTONE_RESTRUCTURING.md
	•	NEW_FEATURE_MAP.md

⸻

Next Steps
	1.	Create .adl/docs/v0.87.1planning/
	2.	Move the runtime-completion docs into v0.87.1planning
	3.	Absorb the skill-schema docs into the v0.87 feature surface
	4.	Move the remaining active TBD docs into their assigned milestone planning directories
	5.	Handle the two split-boundary docs explicitly rather than forcing them wholesale into enterprise or OSS
	6.	Reduce TBD to backlog and planning-meta material
