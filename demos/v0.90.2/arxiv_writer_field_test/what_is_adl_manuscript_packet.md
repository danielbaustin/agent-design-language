# arXiv-Style Manuscript Packet: What Is ADL?

## Metadata

- Skill: arxiv-paper-writer
- Subject: `What Is ADL?`
- Date: 2026-04-20
- Output Location: `demos/v0.90.2/arxiv_writer_field_test/what_is_adl_manuscript_packet.md`
- Issue: `#2272`
- Publication State: draft packet only; not submitted; not author-approved

## Target

- Mode: `draft_from_source_packet`
- Source Packet: `demos/v0.90.2/arxiv_writer_field_test/what_is_adl_source_packet.md`
- Paper Domain: agent runtime systems, workflow orchestration, software
  engineering reviewability
- Target Sections: title options, abstract draft, outline, selected section
  drafts, claim-boundary report, citation-gap report, reviewer notes

## Packet

- Produced Sections:
  - title options
  - abstract draft
  - section outline
  - selected draft sections
  - claim-boundary report
  - citation-gap report
  - reviewer notes
- Primary Draft Surface: this file
- Result: PASS

### Title Options

1. What Is ADL? A Contract-First Runtime for Inspectable Agent Workflows
2. Agent Design Language: Deterministic Orchestration and Reviewable Proof
   Surfaces for AI Workflows
3. From Prompt Choreography to Runtime Truth: The Agent Design Language

Recommended working title: **What Is ADL? A Contract-First Runtime for
Inspectable Agent Workflows**.

### Abstract Draft

Agent systems are increasingly asked to perform work that must survive code
review, operational review, and postmortem analysis. Many current workflows,
however, still rely on prompt-level coordination whose execution state,
failure behavior, and evidence trail are difficult to inspect after the fact.
Agent Design Language (ADL) addresses this gap as a contract-first runtime and
orchestration platform for AI workflows. In the current repository, ADL combines
structured workflow artifacts, a Rust reference runtime and CLI, deterministic
execution planning, bounded runtime behavior, trace and artifact emission,
review surfaces, and milestone proof packages. This paper introduces ADL's
current system boundary, explains its core execution and review model, and
separates implemented baseline capabilities from active and planned milestone
work. The contribution is not a claim of benchmark superiority or autonomous
agent birth; it is a concrete engineering model for making agent workflow
execution inspectable, falsifiable, and reviewable as repository evidence.

### Section Outline

1. **Introduction**
   - Problem: agent workflows need execution truth, not hidden prompt theater.
   - Thesis: ADL makes agent work inspectable through contracts, runtime
     semantics, traces, artifacts, and review packages.
   - Boundary: this paper describes current repository-backed surfaces and does
     not claim full identity, governance, or moral/emotional completion.

2. **System Overview**
   - ADL as language, Rust runtime, CLI, control-plane workflow, and review
     package ecosystem.
   - Structured artifacts: providers, tools, agents, tasks, workflows, and runs.
   - Milestones as proof packages rather than marketing labels.

3. **Runtime Model**
   - Deterministic planning and bounded execution.
   - Concurrency, retries, failure policy, signing, verification, and artifact
     emission as reviewable runtime surfaces.
   - Trace and run-manifest outputs as the system's ground truth.

4. **Reviewability and Proof Surfaces**
   - Demo matrices, milestone packages, review docs, task bundles, STP/SIP/SOR
     records, and validation commands.
   - The difference between ordinary demos, reviewer packages, and release or
     quality proof packages.

5. **Implemented Baseline and Active Boundaries**
   - Implemented baseline: deterministic runtime, review surfaces, operational
     skills, cognitive proof paths, bounded Godel-style experimentation, and
     long-lived runtime proof.
   - Active and planned boundaries: Runtime v2 hardening, first meaningful CSM
     run, moral/emotional substrate, identity/capability continuity,
     governance, economics, and MVP convergence.

6. **Limitations**
   - No claim of external benchmark superiority.
   - No claim of complete identity-bearing citizens or first true Godel-agent
     birthday.
   - No claim that source packets replace literature review, human authorship,
     or external validation.

7. **Future Work**
   - Related-work pass with real citations.
   - Empirical evaluation of reliability and review usefulness.
   - Follow-on papers for Cognitive Spacetime Manifold and Godel Agents and
     ADL.

### Selected Draft Section: Introduction

AI agents are often evaluated at the level of output quality, but engineering
teams also need to know what happened during execution. They need to inspect
which workflow was intended, which steps ran, which artifacts were produced,
which failures occurred, and which claims can be reconstructed after the run.
Without those surfaces, agent work remains difficult to review and difficult to
trust, even when a particular result looks useful.

Agent Design Language (ADL) starts from the opposite premise: agent work should
be designed as an explicit engineering surface. The repository now contains a
language, Rust runtime, CLI, control-plane workflow, milestone proof packages,
and reviewable demo surfaces that make execution claims inspectable. ADL's
central commitment is runtime truth: what was planned, what ran, what artifacts
were emitted, and what a reviewer can verify without relying on hidden chat
state.

This paper introduces ADL as it exists today. It does not claim that ADL has
completed every future layer of its roadmap. In particular, it does not claim
the first true Godel-agent birthday, full identity continuity, full governance,
or moral/emotional civilization. Instead, it presents the implemented baseline:
deterministic orchestration, bounded execution semantics, trace and artifact
surfaces, operational workflow records, and milestone proof packages that make
agent systems more reviewable.

### Selected Draft Section: System Overview

ADL is best understood as a contract-first orchestration system. Its structured
artifacts define providers, tools, agents, tasks, workflows, and runs. Those
artifacts are not merely documentation. They are compiled into deterministic
plans and executed through a runtime that records observable evidence. The
runtime and CLI provide the practical execution layer; the docs, demos, and
review packages provide the review layer.

The repository also treats milestones as proof packages. A milestone is not
only a version label; it carries feature documents, demo matrices, validation
commands, review notes, and release boundaries. This discipline is central to
ADL's claim. A feature should be considered implemented only when there is a
bounded runtime surface, proof surface, or reviewable artifact set that makes
the claim inspectable.

### Selected Draft Section: Limitations

The current source packet does not support claims of benchmark superiority,
industry adoption, peer review, or completed identity-bearing agency. It also
does not contain a related-work survey. Before submission, this manuscript
needs a sourced literature pass covering workflow engines, agent frameworks,
provenance, deterministic replay, and software reviewability.

The source packet does support a narrower and more important claim: ADL has
become a concrete repository-backed system for making agent workflow execution
inspectable. That claim should remain the center of the paper until additional
experiments, citations, or external evaluations are available.

## Claim Boundary Report

- Unsupported Claims Present: false in the draft packet; unsupported candidate
  claims are listed below rather than included as paper claims.
- Claim Labels Used:
  - SUPPORTED
  - NEEDS_CITATION
  - NEEDS_EVIDENCE
  - AUTHOR_DECISION
  - REMOVE_OR_WEAKEN
- Notes: The draft intentionally uses source-backed repository claims and
  avoids comparative or publication claims.

| Claim | Label | Disposition |
| --- | --- | --- |
| ADL is a deterministic orchestration system with a Rust runtime and CLI. | SUPPORTED | Keep, cite repository docs as project evidence. |
| ADL emphasizes trace artifacts, review surfaces, and milestone proof packages. | SUPPORTED | Keep, supported by README and feature list. |
| ADL has completed Runtime v2 foundation work but not full identity or birthday scope. | SUPPORTED | Keep with v0.90.1 boundary language. |
| ADL is better than existing agent frameworks. | NEEDS_EVIDENCE | Do not claim without comparative study. |
| ADL eliminates hallucination or execution error. | REMOVE_OR_WEAKEN | Remove; unsupported and misleading. |
| ADL should be credited to specific authors or affiliations. | AUTHOR_DECISION | Requires human author confirmation. |
| ADL is novel relative to prior workflow or agent systems. | NEEDS_CITATION | Requires related-work review before submission. |

## Citation Gap Report

- Citations Invented: false
- Citation Gaps Present: true
- Missing Bibliographic Details:
  - prior work on workflow engines and orchestration systems
  - prior work on agent frameworks and tool-using LLM systems
  - prior work on deterministic replay, provenance, and artifact traceability
  - prior work on software engineering review and operational postmortems
  - prior work on agent evaluation, governance, and safety boundaries

No citation title, author, venue, DOI, arXiv id, or year was invented in this
packet.

## Submission Boundary

- Submission Attempted: false
- Publication Claimed: false
- Human Author Approval Required: true
- Reason: This is a bounded field-test packet. It has not received human author
  approval, related-work completion, citation completion, or publication
  review. Submission and external publication are explicitly out of scope for
  issue `#2272`.

## Follow-up

- Recommended Next Step: human review of the manuscript packet, followed by a
  separate related-work/citation source-packet issue if the `What Is ADL?`
  manuscript is promoted beyond internal drafting.
