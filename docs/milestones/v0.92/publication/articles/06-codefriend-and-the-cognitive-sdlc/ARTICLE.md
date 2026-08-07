# CodeFriend and the Cognitive SDLC

Coding agents have made producing a patch dramatically easier. They have not made delivering trustworthy software easy.

The difficult work still surrounds the code: understanding intent, choosing scope, preserving repository state, proving behavior, reviewing evidence, handling conflicts, and recording what actually reached the default branch.

As generation accelerates, those coordination costs become more visible. ADL's answer is the **Cognitive Software Development Lifecycle**, or C-SDLC. CodeFriend is the product direction that applies evidence-backed repository cognition and review to real software projects.

## A pull request is necessary, but not sufficient

Git and pull requests are excellent substrates. They preserve changes, branches, review conversations, checks, and merge history. But an AI-native development process needs additional semantic state.

What did the issue ask for? Which task was selected? What plan governed the implementation? Which validations were intended? What did review find? Which findings were fixed or deferred? Did the final outcome match the plan? What evidence should a later agent inherit?

C-SDLC represents one bounded change as a cognitive state transition. ADL's canonical sequence of lifecycle records, called cards, separates input intent, selected task, operative plan, validation plan, structured review, and outcome truth. A bound worktree gives the change a concrete execution context. Typed tools validate and advance lifecycle state.

The distinction between plan and outcome is especially important. If the two share one mutable narrative, a failed experiment can be rewritten to look like the original intention. Separate records preserve learning and accountability.

## Software development as a polis

C-SDLC treats software delivery as a small governed society of human and AI participants.

Each actor has scoped standing and responsibility. One agent may implement a bounded issue. Another may review the exact changed surface. CI supplies integration evidence. A human retains merge and governance authority. Worktrees, branches, and issue records make ownership visible.

This model is useful because parallel agents can produce more work than a maintainer can safely absorb. The hard problem shifts from generation to convergence: preventing duplicate effort, detecting drift, preserving review independence, and ensuring that “done” means the same thing across issue, branch, pull request, checks, and repository state.

Governance is not an obstacle placed outside development. It is the structure that lets multiple capable actors cooperate without erasing responsibility.

## What CodeFriend adds

CodeFriend's current foundation is an evidence-first repository review workflow.

It builds a bounded packet of repository facts, preserves findings and uncertainty, and synthesizes structured reports for readers outside the change. CodeFriend has not been delivered to external customers. Specialist review lanes, architecture and dependency views, and remediation-and-test proposals belong to the planned alpha, not to today's bounded package. Findings should point to source evidence or clearly identify inference. Skipped surfaces and residual risks stay visible.

This is more than automated linting. CodeFriend's planned architecture-cognition layer is meant to reason about module boundaries, coupling, change amplification, architecture drift, decision records, and likely blast radius. Executable governance can turn selected architecture rules into fitness functions and CI-friendly checks.

The product direction also keeps human judgment in the loop. A risk score is advisory unless explicitly configured as a gate. A generated finding does not become truth merely because a model wrote it. Publication and customer delivery remain deliberate decisions.

## Evidence is part of the product

Traditional reports often hide the path from observation to conclusion. An AI-generated report can make that problem worse because polished language creates undeserved confidence.

CodeFriend treats evidence, assumptions, skipped surfaces, uncertainty, redaction, and provenance as first-class outputs. A useful review should let a maintainer answer:

- What repository surface was actually examined?
- Which claims are direct observations and which are inferences?
- What was not tested?
- Can a finding be traced to a file, contract, or execution result?
- Does a proposed remediation stay within the evidence?

The report becomes a navigable argument rather than a verdict from an opaque reviewer.

## What exists today

ADL has a working typed C-SDLC v2 lifecycle and a bounded CodeFriend productization package with review workflow, report, and evidence contracts. Those are real foundations.

CodeFriend is not yet a shipped full product. The planning canon targets a dedicated alpha milestone with a usable product shell, external-repository adapter, evidence core, architecture cognition, executable governance, specialist review engine, and tested delivery workflow. Later roadmap bands extend beyond that alpha.

The distinction protects both users and the project. A collection of internal skills can demonstrate a product thesis without yet providing the reliability, packaging, support, and usability of a finished product.

## Better software through visible thought

C-SDLC and CodeFriend share one premise: software engineering improves when important reasoning becomes inspectable without pretending that every thought can be formalized.

Agents can explore, implement, and review. Typed lifecycle state can preserve boundaries. Evidence can connect findings to source. Humans can exercise authority with a clearer view of what happened.

The goal is not development without people. It is development in which faster machine cognition strengthens, rather than dissolves, engineering accountability.

## Repository Sources

- [`docs/planning/codefriend/README.md`](../../../../../planning/codefriend/README.md)
- [`docs/planning/codefriend/CODEFRIEND_V1_BUILD_PLAN.md`](../../../../../planning/codefriend/CODEFRIEND_V1_BUILD_PLAN.md)
- [`docs/milestones/v0.91.2/features/CODEFRIEND_PRODUCTIZATION.md`](../../../../v0.91.2/features/CODEFRIEND_PRODUCTIZATION.md)
- [`docs/cognitive-sdlc/architecture.md`](../../../../../cognitive-sdlc/architecture.md)
- [`docs/default_workflow.md`](../../../../../default_workflow.md)
