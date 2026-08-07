# What Is ADL?

An AI model can write a convincing plan, call a tool, and produce code. None of those abilities answers the question that matters most when the result changes the world: **what made the action legitimate?**

Was the request understood correctly? Which constraints applied? Who granted authority? What actually ran? What evidence survived? Could another person inspect the decision and distinguish the plan from the outcome?

Agent Design Language, or ADL, is an open engineering effort built around those questions. It is a language of typed contracts, a runtime architecture for persistent agents, and a governance system for turning model proposals into bounded, reviewable action.

That combination matters. A schema without a runtime is only a description. A runtime without governance can execute the wrong thing efficiently. Governance without evidence becomes ceremony. ADL tries to keep all three connected.

## A model response is not an action

Contemporary agent systems often compress several different events into one flow: a model interprets a request, invents a strategy, chooses a tool, and causes an effect. The interface may feel seamless, but the system boundary is dangerously blurry.

ADL separates those events. Generative cognition may propose possibilities. Typed records preserve intent and constraints. A trusted runtime evaluates authority and policy. Execution produces traceable results. Review compares the result with the original goal.

The core rule is simple:

> Model output is a proposal, not execution authority.

This is not an argument against capable models. It is an argument for placing their capability inside a system that can explain and limit what happens next.

## Three connected layers

The first ADL layer is **language and contracts**. Structured records describe tasks, tools, authority, state transitions, evidence, and outcomes. The point is not to turn every thought into bureaucracy. The point is to make important boundaries machine-readable and reviewable.

The second layer is the **runtime**. ADL's Cognitive Spacetime Manifold, or CSM, treats time, state, memory, causality, identity continuity, and policy as first-class parts of an agent's world. A long-lived agent cannot be reduced to a prompt plus a transcript. It needs a durable account of what it inherited, what occurred, what changed, and what can be recovered.

The third layer is **governance and evidence**. Components such as the Freedom Gate mediate proposed actions. Universal Tool Schema describes what a tool is; ADL Capability Contract describes who may use it and under what authority. Audit records, review packets, and deterministic projections let operators examine the path from intent to effect.

These layers reinforce one another. Contracts provide structure. The runtime gives that structure a place to live. Governance decides which transitions are admitted. Evidence makes the decision contestable.

## Determinism around generative cognition

ADL does not assume that language models will become deterministic. Creativity and hypothesis generation are valuable precisely because they can produce alternatives that were not enumerated in advance.

Instead, ADL places nondeterministic generation inside a deterministic envelope. Inputs are explicit. Outputs have declared shapes. Budgets and stop conditions are visible. Side effects pass through authority checks. Important transitions produce durable artifacts. A reviewer can replay the governed parts of the process even when a model would not generate identical prose twice.

That architecture supports a sharper division of labor. Models can be imaginative where imagination is useful. Software remains strict where authority, money, data, identity, or external effects are involved.

## The same idea applied to software development

ADL also applies its architecture to its own development process through the Cognitive Software Development Lifecycle, or C-SDLC.

A code change begins with issue intent and a selected task. It gains an operative plan, validation plan, bounded worktree, review record, and outcome record. Git and pull requests remain essential, but they are not asked to carry every semantic distinction alone. The lifecycle records what was intended, what was attempted, what was proved, and what actually merged.

This matters more as coding agents become faster. When producing code becomes abundant, scarce attention shifts to coordination, evidence, review, and truthful closure. More output does not reduce the need for engineering judgment; it increases the cost of losing track of it.

## What exists, and what does not

ADL is not only a manifesto. The repository contains implemented Rust surfaces for typed runtime contracts, governed execution, tool schemas, capability contracts, bounded Gödel experiments, social-cognition foundations, and C-SDLC lifecycle operations. It also contains tests, fixtures, architecture decisions, and review evidence for those bounded claims.

But the boundary matters. ADL does not claim a completed autonomous society, legal personhood, consciousness, universal safety, or a production-ready first birthday for an identity-bearing Gödel agent. The v0.92 milestone is active development. Some of the most ambitious parts of the project remain plans whose acceptance criteria are intentionally stronger than starting a process and giving it a name.

That distinction is part of the design. A project about governed intelligence should not ask readers to confuse aspiration with evidence.

## A language for accountable agency

ADL's larger proposition is that agents need more than prompts and tools. They need a world with explicit state, a language for authority, a boundary between thought and action, and evidence that survives the moment of execution.

The project is exploring what happens when those requirements are treated as one architecture. The interesting result is not a claim that every problem is solved. It is a more precise set of questions for building systems that can act without making accountability disappear.

That is the thread the rest of this series follows: from the runtime world, to cognitive loops, to execution authority, governed tools, software delivery, continuous security, economics, social intelligence, and the still-open path ahead.

## Repository Sources

- [`adl/README.md`](../../../../../../adl/README.md)
- [`docs/planning/ADL_FEATURE_LIST.md`](../../../../../planning/ADL_FEATURE_LIST.md)
- [`docs/explainers/CSM.md`](../../../../../explainers/CSM.md)
- [`docs/cognitive-sdlc/architecture.md`](../../../../../cognitive-sdlc/architecture.md)
- [`docs/milestones/v0.92/README.md`](../../../README.md)
- [`adl-runtime-kernel/src/governance.rs`](../../../../../../adl-runtime-kernel/src/governance.rs)
- [`adl/src/godel/experiment_record.rs`](../../../../../../adl/src/godel/experiment_record.rs)
- [`adl/src/runtime_v2/theory_of_mind_foundation.rs`](../../../../../../adl/src/runtime_v2/theory_of_mind_foundation.rs)
- [`csdlc-v2/src/operator.rs`](../../../../../../csdlc-v2/src/operator.rs)
- [`docs/specs/uts/UTS_V1.0_SCHEMA.md`](../../../../../specs/uts/UTS_V1.0_SCHEMA.md)
- [`docs/specs/acc/ACC_V1.0_SPEC.md`](../../../../../specs/acc/ACC_V1.0_SPEC.md)
