# Contracts (ADL 1.0)

This document defines **contracts** in Agent Design Language (ADL) 1.0.

Contracts are the primary mechanism by which ADL makes agent execution predictable,
debuggable, and safe. They describe *what must be true* about inputs, outputs,
and intermediate results.

## What is a contract?

A **contract** is a declarative specification of required properties on data flowing
into or out of an ADL construct.

Contracts may apply to:

- tool inputs and outputs,
- task outputs,
- agent responses,
- or workflow step results written to state.

A contract answers questions such as:

- Is the output structurally valid?
- Are required fields present?
- Are constraints satisfied (length, format, allowed values)?
- Does the output conform to declared policy?

Contracts are evaluated by the runtime using **guards**.

## Guards

A **guard** is a runtime mechanism that evaluates a contract.

- Guards operate on *normalized* output.
- Guards either **pass** or **fail**.
- Guard failures are first-class events and **MUST** be observable when tracing is enabled.

ADL does not mandate a specific guard implementation. Runtimes may use:

- structural validators,
- schema validators,
- regular expressions,
- policy engines,
- or custom logic.

## Normalization and ordering

Contracts are evaluated in a fixed order relative to model output processing.

An ADL runtime **MUST** apply the following ordering:

1. **Normalization** — deterministic transformations applied to raw model output.
2. **Guard evaluation** — contract checks against normalized output.
3. **Repair / retry** — optional, bounded attempts to correct contract violations.

A runtime **MUST NOT** perform repair before guard evaluation.

This ordering invariant is critical for reproducibility and debuggability.

## Repair and retry

When a contract fails, a runtime **MAY** attempt repair according to a declared policy.

Repair strategies are runtime-defined, but commonly include:

- re-prompting the agent with a clear error message,
- constraining the response format more strictly,
- or aborting immediately when repair is not appropriate.

Repair **MUST** be bounded. Unbounded retries are forbidden.

If repair ultimately fails, the runtime **MUST** surface the contract violation to the caller.

## Contract profiles

ADL 1.0 introduces the concept of **contract profiles**.

A contract profile describes *how* a contract is expressed and evaluated.
Profiles allow ADL to support multiple validation strategies without hard-coding a single format.

The following profiles are anticipated in ADL 1.x:

### `json`

The `json` profile describes constraints on JSON-like data structures.

Typical uses:

- required fields
- basic types (string, number, boolean, object, array)
- simple structural constraints

This profile is intentionally lightweight and suitable for most ADL 1.0 use cases.

### `schema`

The `schema` profile represents a structured schema language, such as JSON Schema
(or a compatible subset).

Notes:

- ADL 1.0 does **not** require JSON Schema support.
- Runtimes **MAY** support schema validation as an extension.
- Future versions may define a portable schema subset for interoperability.

### `protobuf`

The `protobuf` profile is intended for binary and embedded environments.

Potential uses include:

- tool invocation envelopes,
- compact state snapshots,
- trace or event records.

ADL 1.0 does not define canonical `.proto` files, but explicitly reserves space
for Protobuf-based contracts in future versions.

### `text`

The `text` profile applies to unstructured textual output.

Examples include:

- length limits,
- required substrings,
- regular-expression constraints,
- markdown or formatting rules.

## Contract scope

Contracts may be declared at multiple levels.

- **Tool-level contracts** apply at tool boundaries.
- **Task-level contracts** apply to task outputs.
- **Agent-level contracts** apply to agent responses.
- **Workflow-level contracts** apply to values written to state.

When multiple contracts apply, the runtime **MUST** evaluate all relevant contracts
for the produced value.

## Design intent

Contracts are a core design principle of ADL:

- They make failures explicit instead of implicit.
- They shift validation from ad-hoc code into declarative specification.
- They enable deterministic, testable agent behavior.

ADL treats contract violations not as exceptions to be hidden, but as
signals to be observed, reasoned about, and—when appropriate—repaired.
