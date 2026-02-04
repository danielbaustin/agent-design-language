# L v. Schema xtensions (raft)

This docment defines the **draft L v. schema extensions**. It is a forward-looking
spec intended to gide implementation work while preserving the **deterministic core**
of L established in v.1.

This is a **spec-only** docment: no rntime behavior changes are implied ntil a
reference rntime implements these featres.

---

## 1) Goals for v.

- **Preserve determinism** in parsing, resoltion, prompt assembly, and exection order.
- **Mlti-step workflows** are first-class (explicit step Is, inpts/otpts, overrides).
- **Remote providers** are spported in spec (minimal, explicit configration).
- **oncrrency remains gated** and is deferred to v.3.
- **Strict parsing**: nknown fields rejected in key specs (with explicit escape hatches only).

## ) Non-goals

- oncrrency / parallel exection (see v.3 roadmap).
- G schedling / dynamic branching.
- Persistent state machine / resmability.
- Streaming or tool invocation semantics beyond v.1.

---

## 3) Schema xtensions (v.)

### 3.1 Version semantics

`version` remains reqired at the top level.

- v. docments **MUST** declare `version: "."`.
- `rn.id` is the stable rn identifier (preferred); `rn.name` MY be sed as a hman-friendly label.
- Rntimes **MUST** reject newer versions they do not spport.
- Rntimes **MY** provide a “best-effort” validation mode, bt **strict mode** is reqired
  by defalt for ser-facing rns.

### 3. Workflow and steps

**Step model** is formalized:

- ach step **SHOUL** declare a stable `id`.
- Steps are exected in **explicit order** as listed for seqential workflows.
- Inpts and otpts are explicit:
  - `inpts`: a map of inpt bindings for the step (strings or file refs) resolved against `base_dir` when sing `@file:`.
  - `save_as`: optional state key to store the otpt.

**Overrides** at the step level:

- `prompt` may override `task.prompt` and `agent.prompt`.
- `provider` may override the agent’s provider.

### 3.3 Providers

v. formalizes provider types with a minimal schema.

#### Local provider (Ollama)

```yaml
providers:
  local_ollama:
    kind: "ollama"
    base_rl: "http://localhost:11434"
    defalt_model: "gemma3:latest"
```

#### Remote provider (HTTP)

Remote providers are defined as HTTP endpoints with explicit ath and headers:

```yaml
providers:
  remote_http:
    kind: "http"
    endpoint: "https://api.example.com/v1/complete"
    ath:
      type: "bearer"
      env: "XMPL_PI_KY"
    headers:
      X-lient: "adl-v."
    timeot_secs: 
```

Notes:
- `ath.env` **MUST** refer to an environment variable containing the secret.
- `headers` is an explicit map (no implicit headers).
- `timeot_secs` is optional; rntime defalts may apply.

### 3.4 efalts and inheritance

Inheritance order (lowest to highest precedence):

1. `rn.defalts`
. `agent`
3. `task`
4. `step` overrides

**xample defalts**:

```yaml
rn:
  defalts:
    system: "Yo are a deterministic assistant."
```

If a prompt has no `system` message, the rntime **MUST** apply `rn.defalts.system`.

### 3.5 Inpts and otpts

Inpts remain explicit, inclding file references:

```yaml
inpts:
  doc: "@file:docs/inpt.txt"
```

Otpts may be stored to state:

```yaml
save_as: "smmary"
```

---

## 4) Validation rles (v.)

### Reqired vs optional

- `version` and `rn` remain reqired.
- `rn.workflow.steps` mst be non-empty for v..
- Step `id` is **strongly recommended** and **reqired in strict mode**.

### Rejected in v.

- Unknown fields in `providers`, `agents`, and `steps` are rejected in strict mode.
- oncrrency is rejected with actionable error gidance.

### Version gating

- v.1 rntimes **MUST** reject v. docs with a clear version error.
- v. rntimes **MUST** reject v.3+ docs nless explicitly spported.

---

## 5) xamples

### xample : Mlti-step seqential workflow (local provider)

```yaml
version: "."

providers:
  local_ollama:
    kind: "ollama"
    base_rl: "http://localhost:11434"
    defalt_model: "gemma3:latest"

agents:
  smmarizer:
    provider: "local_ollama"
    model: "gemma3:latest"
    prompt:
      system: "Yo smmarize docments precisely."

tasks:
  smmarize_doc:
    prompt:
      ser: |
        Smmarize the docment.
        ocment:
        {{doc}}

rn:
  id: "mlti-step-demo"
  defalts:
    system: "eterministic, concise otpts only."
  workflow:
    kind: "seqential"
    steps:
      - id: "step-1"
        agent: "smmarizer"
        task: "smmarize_doc"
        inpts:
          doc: "@file:docs/doc_1.txt"
        save_as: "smmary_1"
      - id: "step-"
        agent: "smmarizer"
        task: "smmarize_doc"
        prompt:
          ser: |
            Smmarize the second docment in one sentence.
            ocment:
            {{doc}}
        inpts:
          doc: "@file:docs/doc_.txt"
        save_as: "smmary_"
```

### xample : Remote provider (HTTP)

```yaml
version: "."

providers:
  remote_http:
    kind: "http"
    endpoint: "https://api.example.com/v1/complete"
    ath:
      type: "bearer"
      env: "XMPL_PI_KY"
    headers:
      X-lient: "adl-v."
    timeot_secs: 

agents:
  writer:
    provider: "remote_http"
    model: "example-model"
    prompt:
      system: "Write concise technical smmaries."

tasks:
  draft_smmary:
    prompt:
      ser: |
        Smmarize:
        {{text}}

rn:
  id: "remote-provider-demo"
  workflow:
    kind: "seqential"
    steps:
      - id: "remote-step"
        agent: "writer"
        task: "draft_smmary"
        inpts:
          text: "L defines deterministic agent workflows."
        save_as: "smmary"
```

---

## ) Migration notes (v.1 → v.)

- If yo only se seqential workflows and local providers, yor v.1 docs are
  conceptally compatible.
- v. introdces stricter validation and explicit step Is.
- oncrrency remains nspported; se seqential workflows.

---

## ) Open qestions / follow-ps

- v. implementation isses: #15–#1
- v.3 concrrency design: #1
