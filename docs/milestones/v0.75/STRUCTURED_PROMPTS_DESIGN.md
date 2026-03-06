# Structured Prompts Design (ADL v0.75)

## Overview

ADL workflows are executed by agents that receive instructions derived from **structured execution cards**. Instead of manually writing prompts for each task, ADL defines a **deterministic prompt generation pipeline**:

Issue → Input Card → Prompt Generator → Agent Execution → Output Card → Reviewer

Structured prompts allow agents to execute work deterministically while preserving architectural guarantees such as replay safety, security invariants, and artifact traceability.

This document defines how prompts are generated from cards and how they interact with the ADL execution lifecycle.

---

## Design Goals

The structured prompt system exists to achieve the following goals:

• Deterministic agent execution
• Reproducible development workflows
• Machine-verifiable task specifications
• Replay-safe artifact generation
• Compatibility with multiple agent providers

Prompts should be **generated automatically** from card structure whenever possible.

---

## Core Concept

Input cards contain structured sections that define the execution contract for an agent. These sections are parsed by automation tooling and converted into a prompt that can be executed by an AI coding agent.

Key sections used for prompt generation:

• Goal
• Acceptance Criteria
• Inputs
• Constraints / Policies
• System Invariants
• Reviewer Checklist

These fields form the basis of the prompt sent to the execution agent.

---

## Structured Prompt Composition

A generated prompt is composed of the following components.

### Goal

Defines the task objective and high-level outcome.

### Acceptance Criteria

Defines completion requirements that must be satisfied for the task to be considered complete.

### Inputs

Defines artifacts, files, or documentation the agent must read before beginning work.

### Constraints / Policies

Defines execution policies such as:

• determinism requirements
• security requirements
• environment limitations

### System Invariants

Architectural guarantees that must not be violated.

Examples:

• deterministic execution
• replay-compatible artifacts
• no hidden state
• no secrets or prompts leaked into artifacts

### Reviewer Checklist

Machine-readable fields that allow automated validation tools to verify that the output of the task satisfies repository policies.

Example fields:

• determinism_required
• network_allowed
• replay_required
• security_sensitive

---

## Prompt Generation Pipeline

The prompt generator performs the following steps:

1. Load the input card.
2. Parse structured sections.
3. Assemble a deterministic execution prompt.
4. Inject system invariants into the prompt.
5. Emit prompt text suitable for an execution agent.

This process ensures that prompts remain consistent and reproducible.

---

## Prompt Generation Specification (Card → Prompt Schema)

To make prompt generation deterministic and simple to implement, ADL defines a minimal **prompt schema** derived directly from input cards.

The prompt generator should map card sections to prompt blocks using the following structure:

Prompt Structure:

TASK
- Goal

SUCCESS CRITERIA
- Acceptance Criteria

CONTEXT
- Inputs

EXECUTION CONSTRAINTS
- Constraints / Policies

ARCHITECTURAL INVARIANTS
- System Invariants

VALIDATION EXPECTATIONS
- Reviewer Checklist

This mapping allows a tool to transform any valid input card into a prompt without relying on heuristics.

Example generated prompt structure:

TASK:
<Goal>

SUCCESS CRITERIA:
<Acceptance Criteria>

CONTEXT:
<Inputs>

EXECUTION CONSTRAINTS:
<Constraints>

ARCHITECTURAL INVARIANTS:
<System Invariants>

VALIDATION EXPECTATIONS:
<Reviewer Checklist>

Design principles:

• Prompt generation must be deterministic for identical cards.
• No information outside the card should influence the generated prompt.
• Prompts must always include architectural invariants.
• Validation expectations must be present so automated reviewers can verify results.

This specification allows ADL tooling to generate prompts automatically and ensures compatibility with multiple execution agents.

---

## Output Card Integration

After the agent completes execution, results are written to an **output card**.

Output cards capture verification evidence such as:

• validation results
• determinism evidence
• replay artifacts
• security checks

These sections allow automated reviewers or CI systems to verify the results of agent execution.

---

## Future Automation

Future ADL tooling will extend this system with:

• automatic prompt generation from cards
• CI validation of output cards
• automated replay verification
• artifact schema validation

The long-term goal is a **fully automated development pipeline** where agents can execute tasks while preserving deterministic behavior and architectural guarantees.

---

## Relationship to Card Automation Pipeline

The structured prompt design is implemented by the tooling defined in issue #630 (Card Automation Pipeline).

That system will provide:

• card parsing
• prompt generation
• validation tooling
• CI integration

Together, these systems form the basis of ADL's deterministic agent execution framework.