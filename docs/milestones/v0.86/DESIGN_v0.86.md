# Design: v0.86 Bounded Cognitive System

## Metadata
- Milestone: `v0.86`
- Version: `0.86`
- Date: `2026-03-27`
- Owner: `adl`
- Related issues: `#882`

## Purpose
Define the integrated design for v0.86 as the first **working bounded cognitive system** in ADL.

This milestone establishes a complete but bounded cognitive loop including signals, arbitration, execution, evaluation, and adaptation. The design must align with the actual v0.86 feature-doc set and drive implementation, not just narrative coherence.

The canonical v0.86 feature-doc set is:
- `AGENCY_AND_AGENTS.md`
- `COGNITIVE_ARBITRATION.md`
- `COGNITIVE_LOOP_MODEL.md`
- `COGNITIVE_STACK.md`
- `FAST_SLOW_THINKING_MODEL.md`
- `FREEDOM_GATE.md`
- `LOCAL_AGENT_DEMOS.md`
- `CONCEPT_PLANNING_FOR_v0.86.md` (normative conceptual integration)

Supporting but non-feature docs may inform this design, but they do not expand milestone scope.

## Problem Statement
ADL now has a stabilized repository and stronger execution discipline, but it still risks fragmenting its early cognitive architecture into separate documents that do not yet form a single operational system.

Without an integrated v0.86 design, the project could end up with:
- multiple competing control models
- arbitration that is described but not operational
- agency that remains rhetorical rather than structured
- local demos that prove pieces but not the system

v0.86 addresses this by defining one coherent cognitive system that ties together:
- cognitive stack
- cognitive loop
- cognitive signals (instinct and affect)
- arbitration and fast/slow reasoning
- bounded execution (AEE-lite)
- evaluation signals and termination
- minimal reframing and adaptation
- memory participation (ObsMem-lite)
- Freedom Gate decision control
- bounded candidate selection and local proof surfaces

## Goals
- Establish one authoritative cognitive stack and one authoritative cognitive loop for the milestone.
- Implement cognitive arbitration as a real routing and control surface.
- Implement fast/slow thinking as explicit execution modes under arbitration control.
- Implement the Freedom Gate as a real decision boundary.
- Define bounded candidate selection so agency is operational rather than metaphorical.
- Ensure every major control-stage decision is externally visible in artifacts or demo surfaces.
- Produce local demos that prove the cognitive system is real.
- Introduce cognitive signals (instinct and affect) as first-class inputs to the loop.
- Introduce bounded execution (AEE-lite) with observable iteration behavior.
- Introduce evaluation signals and termination conditions.
- Introduce minimal frame adequacy and reframing capability.
- Introduce observable memory participation (ObsMem-lite).

## Non-Goals
- Full-scale instinct model beyond signal surfaces.
- Full Φ_ADL metric system.
- Full convergence engine beyond bounded execution.
- Advanced affect modeling beyond signal inputs.
- Identity, governance, or signed-trace subsystems.
- Full multi-agent generalization.

## Scope
### In scope
- canonical cognitive stack definition
- canonical cognitive loop definition
- cognitive signals (instinct and affect) as loop inputs
- arbitration and fast/slow routing
- bounded execution loops (AEE-lite)
- evaluation signals and termination conditions
- minimal frame adequacy and reframing
- initial memory participation (ObsMem-lite)
- bounded candidate selection / early agency behavior
- Freedom Gate decision checkpoint
- local demo surfaces proving the integrated system
- explicit artifacts or traces showing decisions and state transitions

### Out of scope
- full instinct model beyond signal surfaces
- PHI / Φ_ADL metrics
- full convergence engine behavior
- advanced reframing and meta-reasoning systems
- full reasoning graph and signed-trace architecture
- affect, identity, governance, or constitutional systems beyond bounded inputs

## Requirements
### Functional
- Implement the **Cognitive Stack** as the authoritative layer model for v0.86.
- Implement the **Cognitive Loop Model** as the authoritative execution flow for v0.86.
- Implement **Cognitive Arbitration** with explicit decision outputs.
- Implement **Fast/Slow Thinking** as distinct execution paths under arbitration control.
- Implement **Agency and Candidate Selection** so there is an explicit pre-execution choice surface.
- Implement the **Freedom Gate** to allow, defer, or refuse candidate actions under bounded control.
- Implement **Local Agent Demo(s)** that exercise the integrated loop end-to-end.
- Implement cognitive signals (instinct and affect) as structured inputs to the loop.
- Implement bounded execution (AEE-lite) with at least one observable iteration.
- Implement evaluation signals (e.g., progress, contradiction, failure).
- Implement minimal frame adequacy and bounded reframing behavior.
- Implement observable memory participation (ObsMem-lite).

### Non-functional
- Outputs must be deterministic or at least explainable enough for review.
- Control decisions must be inspectable in artifacts, traces, or structured demo outputs.
- The milestone must preserve roadmap discipline by not silently absorbing later-milestone systems.
- All canonical planning docs must agree on the v0.86 bounded cognitive path.

## Proposed Design
### Overview
v0.86 is organized around one bounded control path:

signals → candidate selection → arbitration → fast/slow execution (bounded loop) → evaluation → reframing → memory participation → Freedom Gate → action / refuse / defer

This path may iterate in a bounded manner (AEE-lite), with evaluation signals influencing subsequent candidate selection, arbitration, or reframing.

This path is supported by:
- a canonical cognitive stack
- a canonical cognitive loop
- explicit decision outputs at control boundaries
- local demos that prove the design in practice

### Core Design Principles

#### 1. One authoritative cognitive stack
The stack document defines the major layers of control. v0.86 must not permit multiple competing stack definitions in parallel docs.

#### 2. One authoritative cognitive loop
The loop document defines the execution flow. It must describe:
- step order
- decision boundaries
- iteration semantics if any
- termination / completion semantics for the v0.86 control layer
- integration of signals, evaluation, and bounded iteration

#### 3. Arbitration is first-class
Arbitration is not just commentary on model choice. It is a control mechanism that decides:
- which reasoning path to use
- whether the problem is low-cost / high-cost
- whether fast-path execution is sufficient
- when slower deliberation is justified

#### 4. Fast/slow thinking is operational
Fast and slow thinking must exist as explicit execution modes, not just conceptual language. They must differ in behavior, cost, or review surface.

#### 5. Agency must be bounded and inspectable
Agency in v0.86 is implemented through bounded candidate selection and action control. It is not unbounded autonomy.

#### 6. Freedom Gate is the decision boundary
The Freedom Gate evaluates candidate actions after arbitration and before action. It may:
- allow
- defer
- refuse

This gate must produce an inspectable event or decision artifact.

#### 7. Signals, evaluation, and adaptation are first-class
The system must incorporate:
- instinct and affect signals as inputs
- evaluation signals after execution
- bounded adaptation via reframing when needed

These must influence control flow and be externally visible.

### Interfaces / Data Contracts

#### Candidate Selection Output
Minimum expected surface:
- candidate_actions
- candidate_rationale
- selected_candidate

#### Arbitration Output
Minimum expected surface:
- route_selected
- confidence
- risk_class
- reasoning_mode (`fast` / `slow` / `defer`)

#### Freedom Gate Output
Minimum expected surface:
- gate_decision (`allow` / `defer` / `refuse`)
- decision_reason
- selected_action_or_none

#### Demo / Review Surface
Minimum expected surface:
- command to run
- expected artifact or output
- explanation of what the demo proves

### Execution Semantics
1. Receive cognitive signals (instinct, affect) and context.
2. Generate or identify bounded candidate actions.
3. Perform arbitration over candidates and context.
4. Select a fast or slow reasoning path.
5. Execute within a bounded loop (AEE-lite).
6. Emit evaluation signals (progress, contradiction, failure).
7. Optionally perform bounded reframing if the frame is inadequate.
8. Pass the resulting action proposal through the Freedom Gate.
9. Execute allowed action, or emit refusal / deferral behavior.
10. Record structured output to make the full path reviewable.

## Risks and Mitigations
- Risk: multiple docs define conflicting control models
  - Mitigation: enforce one authoritative stack and one authoritative loop
- Risk: arbitration is described but not real
  - Mitigation: require structured arbitration outputs and demo proof
- Risk: Freedom Gate remains philosophical
  - Mitigation: require allow/defer/refuse outputs and at least one working case
- Risk: demos prove isolated pieces instead of the system
  - Mitigation: require at least one end-to-end local demo showing the integrated path
- Risk: later-milestone concepts leak back into v0.86
  - Mitigation: include bounded cognitive signals (instinct and affect) as in-scope, while excluding only full PHI metrics, full convergence behavior, identity, and governance systems from this design

## Alternatives Considered
- Docs-only consistency pass
  - Tradeoff: clearer language, but no proof of execution
- Independent component demos
  - Tradeoff: easier to land, but does not prove integrated cognitive system
- Pushing v0.86 concepts into later milestones
  - Tradeoff: cleaner scope, but delays the first believable cognitive-control layer

## Validation Plan
- Run at least one local demo that exercises signals, candidate selection, arbitration, bounded execution, evaluation, and Freedom Gate behavior.
- Verify that arbitration visibly selects different paths under different conditions.
- Verify that the Freedom Gate emits at least one allow or refusal case in a structured way.
- Verify that the stack, loop, arbitration, and Freedom Gate docs do not contradict each other.
- Verify that demo documentation explains what each run proves.
- Verify that at least one bounded execution iteration occurs (AEE-lite).
- Verify that evaluation signals influence subsequent behavior.
- Verify that at least one reframing or adaptation occurs.
- Verify that memory participation is observable in outputs.

### Success Criteria
- One integrated bounded cognitive path exists and can be demonstrated.
- Arbitration outputs are structured and reviewable.
- Fast and slow paths are both meaningful and distinguishable.
- Freedom Gate behavior is real and observable.
- Docs and demos agree on the same system.
- Bounded execution loop is real and observable.
- Evaluation signals are emitted and used.
- Cognitive signals (instinct and affect) influence arbitration or execution behavior.
- At least one adaptive change (reframing or equivalent) occurs.
- Memory participation is visible.

### Fallback
If the full integrated path is not ready, the fallback is to preserve a minimal but truthful v0.86 path that still demonstrates:
- canonical stack
- canonical loop
- arbitration
- one fast/slow distinction
- one Freedom Gate decision case

## Exit Criteria
- The v0.86 design matches the actual feature-doc set.
- The cognitive stack and loop are canonical and contradiction-free.
- Arbitration and fast/slow reasoning are implemented as real control surfaces.
- Agency is represented through bounded candidate selection, not just prose.
- Freedom Gate operates as a real decision boundary.
- Local demos prove the cognitive system end-to-end.
- The bounded cognitive system (signals, loop, arbitration, execution, evaluation, adaptation) is fully represented and demonstrable without claiming later-milestone depth.