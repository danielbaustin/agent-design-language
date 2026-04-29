# Source Packet: What Is ADL?

## Metadata

- Packet: `what_is_adl_source_packet`
- Intended paper: `What Is ADL?`
- Issue: `#2639`
- Status: draft input for initial manuscript review
- Publication state: not submitted; not publication-ready

## Purpose

Provide one bounded, source-grounded packet for the first serious launch-facing
draft of `What Is ADL?`.

The paper should answer a simple but demanding question:

> What has ADL actually become, and why should a technical reader believe that
> it is a real system rather than prompt theater?

## Target Reader

This packet is aimed at:

- systems engineers evaluating whether ADL is a runtime or just a notation
- AI/agent researchers looking for a bounded architectural account of ADL
- technically serious launch readers who need a durable reference beyond Medium
  articles or milestone notes

## Core Thesis

ADL is now a deterministic runtime, control plane, and cognitive orchestration
system for agent work that must survive inspection. Its central contribution is
not benchmark theater or vague autonomy rhetoric. Its contribution is an
engineering model in which workflow intent, execution, adaptation, review, and
release evidence are represented explicitly enough to be audited, challenged,
and improved.

## Required Paper Shape

The draft should present ADL as a coherent system with at least these themes:

1. deterministic runtime and execution semantics
2. explicit control-plane and task-bundle workflow
3. trace, artifact, and review surfaces as truth-bearing evidence
4. bounded cognitive stack, including Freedom Gate and AEE
5. bounded Godel-agent and experiment-system direction
6. trust-under-adversary and adversarial runtime posture
7. active boundaries: what is real now versus what remains later work

## Primary Source Surfaces

### `README.md`

Supported facts:

- ADL is described as a deterministic orchestration system for AI workflows.
- The repository contains a language, Rust runtime, CLI, review surfaces, and
  milestone proof packages.
- ADL emphasizes explicit contracts, bounded runtime behavior, durable
  artifacts, and repository-visible proof.
- The project presents itself as an engineering system that can survive code
  review, operational review, and postmortem analysis.

### `docs/planning/ADL_FEATURE_LIST.md`

Supported facts:

- ADL is no longer just a language idea or schema set.
- Implemented baseline capabilities include deterministic workflow execution,
  a real Rust runtime and CLI, trace and artifact reviewability, operational
  skills, task-bundle workflow, bounded cognitive proof paths, bounded Godel
  experimentation, ObsMem, long-lived runtime substrates, and milestone proof
  packages.
- AEE 1.0 convergence, Freedom Gate baseline and v2, adversarial runtime
  posture, and Runtime v2 foundations are all represented as specific feature
  bands rather than generic ambition.
- Planned later bands are explicitly separated from implemented ones.

### `docs/architecture/ADL_ARCHITECTURE.md`

Supported facts:

- ADL has a repository-first architecture with runtime, authoring/control-plane,
  workflow-skill, and review layers.
- Execution is compiled into explicit plans, then run through bounded provider
  and tool boundaries.
- STP, SIP, and SOR are the canonical issue-execution packet surfaces.
- Traces and artifacts are treated as truth-bearing runtime evidence.
- Long-lived agents remain cycle-bounded rather than becoming opaque daemons.

### `docs/milestones/v0.86/features/CONCEPT_PLANNING_FOR_v0.86.md`

Supported facts:

- ADL defines a bounded cognitive loop:
  `instinct -> affect -> arbitration -> freedom_gate -> execution (AEE) ->
  evaluation -> reframing -> memory`.
- The loop is intended to be inspectable and bounded rather than mystical.
- Freedom Gate is a substrate constraint, not a personality quirk.

### `docs/milestones/v0.89/features/AEE_CONVERGENCE_MODEL.md`

Supported facts:

- AEE convergence is treated as a real runtime surface, not just a retry story.
- Convergence must be visible through explicit progress signals, stop
  conditions, iteration counts, and strategy-change records.
- ADL distinguishes `converged`, `stalled`, `bounded_out`, `policy_stop`, and
  related termination states.

### `docs/milestones/v0.89/features/FREEDOM_GATE_V2.md`

Supported facts:

- Freedom Gate v2 is a structured judgment boundary.
- Allow, defer, refuse, and escalate are governed runtime outcomes.
- Constraint is intended to live in the substrate rather than in the
  temperament of one model or session.

### `docs/milestones/v0.89/features/GODEL_EXPERIMENT_SYSTEM.md`

Supported facts:

- ADL has a bounded experiment system with baseline/variant pairing, evaluation
  plans, bounded mutations, and explicit adoption/rejection decisions.
- System-improvement claims are meant to become experiment artifacts instead of
  hidden preferences.
- The current bounded Godel lane is evidence-bearing, not open-ended recursive
  self-modification.

### `docs/milestones/v0.89/features/ADL_TRUST_MODEL_UNDER_ADVERSARY.md`

Supported facts:

- ADL explicitly distinguishes trusted and reduced-trust surfaces under contest.
- Contested trust is a runtime concern, not merely a documentation warning.
- Revalidation and escalation paths are reviewer-visible.

### `docs/milestones/v0.89.1/features/ADL_ADVERSARIAL_RUNTIME_MODEL.md`

Supported facts:

- ADL treats continuous intelligent opposition as a first-class operating
  condition.
- The attack surface is modeled as dynamic rather than static.
- Later adversarial lanes must preserve traceability, attribution,
  policy-bounded operation, and replay/review visibility.

### `docs/milestones/v0.90/README.md`, `docs/milestones/v0.90.1/README.md`,
`docs/milestones/v0.90.2/README.md`, `docs/milestones/v0.90.3/README.md`

Supported facts:

- ADL has progressed from deterministic execution and bounded cognition into
  long-lived agents, Runtime v2, citizen-state security, observatory surfaces,
  and stronger review proof.
- Each milestone explicitly states both delivered surfaces and deferred scope.

### `REVIEW.md`

Supported facts:

- ADL treats external review, internal review, remediation, and proof surfaces
  as part of the product story rather than release ceremony only.

## Allowed Claims

| Claim | Status | Evidence |
| --- | --- | --- |
| ADL is a deterministic runtime and orchestration platform with a Rust runtime and CLI. | SUPPORTED | `README.md`; `docs/planning/ADL_FEATURE_LIST.md`; `docs/architecture/ADL_ARCHITECTURE.md` |
| ADL uses explicit issue/task cards, worktrees, and PR lifecycle controls as part of its control plane. | SUPPORTED | `docs/architecture/ADL_ARCHITECTURE.md`; `docs/default_workflow.md` |
| ADL treats traces, artifacts, review packets, and milestone proof packages as execution truth surfaces. | SUPPORTED | `README.md`; `docs/architecture/ADL_ARCHITECTURE.md`; `REVIEW.md` |
| ADL includes a bounded cognitive stack with instinct, affect, arbitration, Freedom Gate, AEE, evaluation, reframing, and memory. | SUPPORTED | `docs/milestones/v0.86/features/CONCEPT_PLANNING_FOR_v0.86.md` |
| ADL has a bounded Godel/experiment lane rather than an unconstrained self-modifying agent story. | SUPPORTED | `docs/milestones/v0.89/features/GODEL_EXPERIMENT_SYSTEM.md` |
| ADL assumes adversarial pressure as a normal runtime condition and exposes related trust surfaces. | SUPPORTED | `docs/milestones/v0.89/features/ADL_TRUST_MODEL_UNDER_ADVERSARY.md`; `docs/milestones/v0.89.1/features/ADL_ADVERSARIAL_RUNTIME_MODEL.md` |
| ADL is the best agent framework or is already empirically superior to alternatives. | REMOVE_OR_WEAKEN | No comparative evidence in this packet |
| ADL has already delivered full identity-bearing citizens, governance, or complete economic society. | REMOVE_OR_WEAKEN | Deferred in milestone docs |
| ADL has solved AGI, alignment, or general intelligence in deployed product form. | REMOVE_OR_WEAKEN | Unsupported by this packet |

## Required Cautions

- The paper may be bold about architecture, posture, and design commitments.
- The paper must stay careful about external novelty, superiority, or empirical
  effectiveness claims unless later citation work supports them.
- The paper should separate current ADL from later identity, governance,
  wellbeing, economics, and full social-cognition bands.

## Citation And Evidence Gaps

The draft can proceed before these are filled, but they must be flagged:

- related work on workflow engines, orchestration systems, and reproducible
  execution
- related work on agent frameworks and tool-using language-model systems
- related work on provenance, artifact traceability, and replay/reviewability
- literature positioning for cognitive/runtime-control concepts
- any external novelty comparison beyond repo-backed project evidence

## Recommended Section Order

1. Introduction: why agent systems need runtime truth
2. What ADL is: runtime, control plane, and review system
3. Deterministic execution and artifact truth
4. Cognitive stack, AEE, and Freedom Gate
5. Godel agents, bounded adaptation, and experiment evidence
6. Adversarial security posture and governed operation
7. Why ADL is different
8. Limits, active boundaries, and future work

## Non-Goals For This Draft

- final arXiv submission formatting
- complete external related-work pass
- claim of launch readiness without human review
- folding the other planned papers into this manuscript

## Boundary

This packet is drafting evidence for internal review. It is not a publication
claim, an authorship-complete manuscript, or a submission package.
