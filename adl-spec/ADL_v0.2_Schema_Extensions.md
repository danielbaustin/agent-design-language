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
- DAG scheduling / dynamic branching.
- Persistent state machine / resumability.
- Streaming or tool invocation semantics beyond v0.1.

---

## 3) Schema Extensions (v0.2)

### 3.1 Version semantics

`version` remains required at the top level.

- v0.2 documents **MUST** declare `version: "0.2"`.
- `run.id` is the stable run identifier (preferred); `run.name` MAY be used as a human-friendly label.
- Runtimes **MUST** reject newer versions they do not support.
- Runtimes **MAY** provide a “best-effort” validation mode, but **strict mode** is required
  by default for user-facing runs.

### 3.2 Workflow and steps

**Step model** is formalized:

- Each step **SHOULD** declare a stable `id`.
- Steps are executed in **explicit order** as listed for sequential workflows.
- Inputs and outputs are explicit:
  - `inputs`: a map of input bindings for the step (strings or file refs) resolved against `base_dir` when using `@file:`.
  - `save_as`: optional state key to store the output.

**Overrides** at the step level:

- `prompt` may override `task.prompt` and `agent.prompt`.
- `provider` may override the agent’s provider.

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
      env: "EXAMPLE_API_KEY"
    headers:
      X-Client: "adl-v0.2"
    timeout_secs: 60
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

### Example A: Multi-step sequential workflow (local provider)

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
            Summarize the second document in one sentence.
            Document:
            {{doc}}
        inputs:
          doc: "@file:docs/doc_2.txt"
        save_as: "summary_2"
```

### Example B: Remote provider (HTTP)

```yaml
version: "0.2"

providers:
  remote_http:
    kind: "http"
    endpoint: "https://api.example.com/v1/complete"
    auth:
      type: "bearer"
      env: "EXAMPLE_API_KEY"
    headers:
      X-Client: "adl-v0.2"
    timeout_secs: 60

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

- v0.2 implementation issues: #15–#18
- v0.3 concurrency design: #21
