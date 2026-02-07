# ADL v0.2 Schema Extensions (Draft)

This document defines the **draft ADL v0.2 schema extensions**. It is a forward-looking
spec intended to guide implementation work while preserving the **deterministic core**
of ADL established in v0.1.

This is a **spec-only** document: no runtime behavior changes are implied until a
reference runtime implements these features.

---

## 1) Goals for v0.2

- **Preserve determinism** in parsing, resolution, prompt assembly, and execution order.
- **Multi-step workflows** are first-class (explicit step IDs, inputs/outputs, overrides).
- **Remote providers** are supported in spec (minimal, explicit configuration).
- **Concurrency remains gated** and is deferred to v0.3.
- **Strict parsing**: unknown fields rejected in key specs (with explicit escape hatches only).

## 2) Non-goals

- Concurrency / parallel execution (see v0.3 roadmap).
- Graph scheduling / dynamic branching.
- Persistent state machine / resumability.
- Streaming or tool invocation semantics beyond v0.1.

---

## 3) Schema extensions (v0.2)

### 3.1 Version semantics

`version` remains required at the top level.

- v0.2 documents **MUST** declare `version: "0.2"`.
- `run.id` is the stable run identifier (preferred); `run.name` MAY be used as a human-friendly label.
- Runtimes **MUST** reject newer versions they do not support.
- Runtimes **MAY** provide a “best-effort” validation mode, but **strict mode** is required
  by default for user-facing runs.

### 3.2 Workflow and steps

v0.2 formalizes a **multi-step workflow model** with explicit ordering and data flow.

- Each workflow defines an ordered list of `steps`.
- Steps are executed **strictly in the order listed**.
- Each step **SHOULD** declare a stable `id`; step `id` is **REQUIRED in strict mode**.

#### Inputs and outputs

- `inputs` is an explicit map of named bindings for the step.
- Input values may be literal strings or file references using `@file:<path>`.
- File inputs are resolved relative to `base_dir`.

- A step may declare `save_as`, which stores the step’s primary output under a named
  key in workflow state.
- Stored outputs are immutable once written and may be referenced by later steps.

#### Referencing prior outputs (workflow state)

- `save_as` writes the step’s primary output into workflow state under the given key.
- Later steps may reference stored state values **only** from earlier steps.
- Reference syntax in prompts uses template variables:
  - If a step saved `save_as: "summary_1"`, later prompts may reference it as `{{summary_1}}`.
- State keys are write-once:
  - Reusing a `save_as` key within a workflow is a **validation error** in strict mode.
  - Referencing an unknown key is a **validation error** in strict mode.

Note: v0.2 state values are treated as opaque strings. Structured outputs are deferred.

#### Overrides

- `prompt` at the step level overrides `task.prompt` and `agent.prompt`.
- `provider` at the step level overrides the agent’s provider selection.

### 3.3 Providers

v0.2 formalizes provider types with a minimal schema.

#### Local provider (Ollama)

```yaml
providers:
  local_ollama:
    kind: "ollama"
    base_url: "http://localhost:11434"
    default_model: "gemma3:latest"
```

#### Remote provider (HTTP)

Remote providers are defined as HTTP endpoints with explicit auth and headers:

```yaml
providers:
  remote_http:
    kind: "http"
    endpoint: "https://api.example.com/v1/complete"
    auth:
      type: "bearer"
      env: "XMPL_API_KEY"
    headers:
      X-Client: "adl-v0.2"
    timeout_secs: 30
```

Notes:
- `auth.env` **MUST** refer to an environment variable containing the secret.
- `headers` is an explicit map (no implicit headers).
- `timeout_secs` is optional; runtime defaults may apply.

### 3.4 Defaults and inheritance

Inheritance order (lowest to highest precedence):

1. `run.defaults`
2. `agent`
3. `task`
4. `step` overrides

**Example defaults**:

```yaml
run:
  defaults:
    system: "You are a deterministic assistant."
```

If a prompt has no `system` message, the runtime **MUST** apply `run.defaults.system`.

### 3.5 Inputs and outputs

Inputs remain explicit, including file references:

```yaml
inputs:
  doc: "@file:docs/input.txt"
```

Outputs may be stored to state:

```yaml
save_as: "summary"
```

### 3.6 Determinism constraints (v0.2)

To preserve ADL’s deterministic core, v0.2 enforces the following constraints:

- Workflow execution order is fixed and explicit.
- Step outputs are written once and never mutated.
- Later steps may only depend on:
  - Their declared `inputs`
  - Explicitly named outputs from earlier steps
- No hidden global state is permitted.
- Provider configuration must be fully explicit in the document or environment.
- Given identical inputs, configuration, and provider behavior, a run MUST be
  reproducible in structure and ordering.

---

## 4) Validation rules (v0.2)

### Required vs optional

- `version` and `run` remain required.
- `run.workflow.steps` must be non-empty for v0.2.
- Step `id` is **strongly recommended** and **required in strict mode**.

### Rejected in v0.2

- Unknown fields in `providers`, `agents`, and `steps` are rejected in strict mode.
- Concurrency is rejected with actionable error guidance.

### Version gating

- v0.1 runtimes **MUST** reject v0.2 docs with a clear version error.
- v0.2 runtimes **MUST** reject v0.3+ docs unless explicitly supported.

---

## 5) Examples

### Example : Multi-step sequential workflow (local provider)

```yaml
version: "0.2"

providers:
  local_ollama:
    kind: "ollama"
    base_url: "http://localhost:11434"
    default_model: "gemma3:latest"

agents:
  summarizer:
    provider: "local_ollama"
    model: "gemma3:latest"
    prompt:
      system: "You summarize documents precisely."

tasks:
  summarize_doc:
    prompt:
      user: |
        Summarize the document.
        Document:
        {{doc}}

run:
  id: "multi-step-demo"
  defaults:
    system: "Deterministic, concise outputs only."
  workflow:
    kind: "sequential"
    steps:
      - id: "step-1"
        agent: "summarizer"
        task: "summarize_doc"
        inputs:
          doc: "@file:docs/doc_1.txt"
        save_as: "summary_1"
      - id: "step-2"
        agent: "summarizer"
        task: "summarize_doc"
        prompt:
          user: |
            Here is the summary of the first document:
            {{summary_1}}

            Now summarize the second document in one sentence.
            Document:
            {{doc}}
        inputs:
          doc: "@file:docs/doc_2.txt"
        save_as: "summary_2"
```

### Example : Remote provider (HTTP)

```yaml
version: "0.2"

providers:
  remote_http:
    kind: "http"
    endpoint: "https://api.example.com/v1/complete"
    auth:
      type: "bearer"
      env: "XMPL_API_KEY"
    headers:
      X-Client: "adl-v0.2"
    timeout_secs: 30

agents:
  writer:
    provider: "remote_http"
    model: "example-model"
    prompt:
      system: "Write concise technical summaries."

tasks:
  draft_summary:
    prompt:
      user: |
        Summarize:
        {{text}}

run:
  id: "remote-provider-demo"
  workflow:
    kind: "sequential"
    steps:
      - id: "remote-step"
        agent: "writer"
        task: "draft_summary"
        inputs:
          text: "ADL defines deterministic agent workflows."
        save_as: "summary"
```

---

## 6) Migration notes (v0.1 → v0.2)

- If you only use sequential workflows and local providers, your v0.1 docs are
  conceptually compatible.
- v0.2 introduces stricter validation and explicit step IDs.
- Concurrency remains unsupported; use sequential workflows.

---

## 7) Open questions / follow-ups

- v0.2 implementation issues: #16–#20, #35–#38
- v0.3 concurrency design: TBD (track as a future roadmap issue)
