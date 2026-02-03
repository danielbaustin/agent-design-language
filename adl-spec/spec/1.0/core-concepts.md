# Core Concepts (ADL 1.0)

## Design-time vs runtime

## The six primary design-time abstractions
- Provider
- Tool
- Agent
- Task
- Workflow
- Run

## Runtime-only concepts (non-normative)
- Session
- Trace
- Repair / Retry

## Canonical execution pipeline (non-normative summary)

## State model (ADL 1.0)

# Core Concepts (ADL 1.0)

This document defines the core concepts of Agent Design Language (ADL) 1.0.

ADL is a **design-time** language: it describes *what* an agent system is intended to do and the contracts it must satisfy.
A runtime is responsible for turning that intent into concrete execution against a specific model provider and tool environment.

## Design-time vs runtime

ADL 1.0 separates **design-time intent** from **runtime execution**.

- **Design-time (ADL document):** the declarative specification of providers, tools, agents, tasks, workflows, runs, and contracts.
- **Runtime (implementation):** the process that executes a run, calls model providers and tools, manages state, and records traces.

This separation is intentional:

- It enables multiple runtimes (local, cloud, embedded, test harness) to execute the same ADL document.
- It supports deterministic prompt assembly and predictable contract enforcement.
- It keeps ADL stable while execution strategies evolve.

**Normative scope.** The ADL specification defines:

- the meaning of ADL constructs (Provider/Tool/Agent/Task/Workflow/Run),
- the required invariants that affect observable behavior (e.g., deterministic prompt assembly and output pipeline ordering), and
- the minimum interoperability expectations for runtimes.

**Non-normative scope.** The specification does **not** mandate a particular runtime architecture, storage system, UI, or tracing backend.

## The six primary design-time abstractions

ADL 1.0 is organized around six design-time abstractions. Each is defined by its intent and its contracts.

### Provider

A **Provider** describes a model endpoint and its invocation parameters.

A provider definition typically includes:

- a provider kind (e.g., OpenAI-compatible, local model runtime, vendor-specific API),
- a model identifier,
- default generation parameters (temperature, max tokens, etc.), and
- authentication and routing details (handled by the runtime).

A runtime **MUST** treat provider configuration as part of deterministic prompt execution: given the same ADL document and inputs, the provider selection and parameters are the same.

### Tool

A **Tool** is a callable capability exposed to agents.

Tools may represent:

- local functions,
- network services,
- device I/O,
- database or file operations,
- or other runtimes (tool-as-adapter).

Tools define **input and output contracts**. A runtime **MUST** enforce these contracts at the tool boundary (see output pipeline ordering in `normative-language.md`).

### Agent

An **Agent** defines a prompting strategy and tool access policy.

An agent typically includes:

- structured prompt sections (e.g., system / developer / instructions / examples),
- references to tools it may call,
- optional memory and context policies (runtime-managed), and
- optional response contracts (format, schema, or constraints).

An agent is a *design-time role*; it is not a running process by itself.

### Task

A **Task** is a unit of work that an agent can perform.

A task typically specifies:

- which agent performs it,
- required inputs (names and types/constraints),
- an expected output contract (e.g., JSON object shape, required fields, plain-text constraints), and
- optional repair policy (how the runtime should retry on contract failure).

Tasks are designed to be reusable across workflows.

### Workflow

A **Workflow** composes tasks into a directed sequence (or graph) of steps.

A workflow step typically includes:

- the task to execute,
- how inputs are bound from prior state,
- where outputs are stored in state (`save_as` / equivalent), and
- optional step-level overrides (e.g., provider selection, timeout, retries).

The workflow is the primary orchestration unit in ADL 1.0.

### Run

A **Run** binds concrete input values to a workflow.

A run provides:

- named inputs,
- a workflow reference,
- optional runtime hints (e.g., tracing enabled, seed, max steps) that do not change the meaning of the workflow.

A run is still design-time data. The runtime creates a **session** to execute it.

## Runtime-only concepts (non-normative)

The following concepts are useful for implementers and users, but they are runtime-defined and are not themselves ADL language constructs.

### Session

A **Session** is an instance of executing a run.

A session typically has:

- a unique identifier,
- timestamps and environment metadata,
- a snapshot of the ADL document and resolved inputs,
- trace and artifact outputs.

### Trace

A **Trace** is the runtime’s structured record of what happened during a session.

Tracing commonly includes:

- prompt materialization (or hashed prompt segments when sensitive),
- model requests and responses (or references),
- tool calls and results,
- guard evaluations and failures,
- repair attempts and outcomes.

### Repair / Retry

**Repair** is a bounded strategy the runtime uses when contracts fail.

Examples include:

- re-asking the agent with an explicit error message,
- applying a constrained reformat instruction,
- switching to a stricter decoding strategy,
- or failing fast when a violation is not repairable.

ADL 1.0 requires that repair occurs only *after* normalization and guard evaluation.

## Canonical execution pipeline (non-normative summary)

While runtimes may differ internally, the observable pipeline is expected to follow this conceptual order:

1. **Resolve inputs** for the selected run and workflow.
2. **Assemble prompts deterministically** from agent/task definitions and resolved inputs.
3. **Invoke provider** (model call).
4. **Normalize output** (deterministic transformations).
5. **Evaluate guards / contracts** (format and policy validation).
6. **Repair / retry** (bounded, optional).
7. **Commit outputs to state** and continue the workflow.
8. **Record trace** and produce session artifacts.

The invariants around determinism and ordering are defined in `normative-language.md`.

## State model (ADL 1.0)

ADL workflows operate against a runtime-managed **state**.

### ADL 1.0 state (baseline)

For interoperability, ADL 1.0 assumes a simple model:

- State is a hierarchical key-value store (a JSON-like object graph).
- Workflow step outputs are written under explicit keys (e.g., `save_as`).
- Inputs are read from state via explicit bindings.

Runtimes **MUST** treat state updates as part of the session traceable behavior: a user should be able to understand *what* keys were read and written at each step.

### Future expansion: typed state and portable contracts

ADL 1.x intentionally leaves room for stronger typing and portable contracts:

- **Schemas.** ADL may adopt JSON Schema (or a compatible subset) to describe structured outputs and tool I/O where interoperability matters.
  ADL 1.0 does not require JSON Schema, and runtimes may support alternative validators.

- **Binary/embedded interchange.** For constrained devices and local/edge runtimes, ADL tool payloads and session artifacts may use **Protocol Buffers (Protobuf)**.
  A future version may define canonical `.proto` messages for:

  - tool invocation envelopes,
  - trace/event records,
  - and compact state snapshots.

- **Contract profiles.** Future versions may define contract “profiles” (e.g., `json`, `schema`, `protobuf`, `regex`, `markdown`) so that runtimes can negotiate support.

The goal is to keep ADL 1.0 simple and readable while enabling rigorous validation and efficient interchange as the ecosystem grows.