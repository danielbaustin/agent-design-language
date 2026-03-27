# 📘 ADL Identity Architecture (v0.9x)

**Status:** Proposed target-state planning document for v0.9x
**Current runtime baseline:** v0.7
**Scope note:** This document captures desired end-state architecture. It is intentionally forward-looking and is not a normative description of v0.7 runtime behavior.

## Overview

This document defines how **agent identity** - both *security identity* and *persistent narrative identity* - is modeled, persisted, and invoked within ADL. The goal is to treat agents as **persistent, accountable entities** with coherent long-term self-representation, supporting deterministic replay, delegation, pushback, and reduced hallucination through structured identity grounding.

---

## Motivation

Autonomous agents in ADL do more than "invoke a model" - they maintain:

- **Narrative identity** (history, persona, values, and consistent behavior)
- **Security identity** (permissions, delegation context)
- **Operational continuity** (goal persistence and replayability)
- **Normative autonomy** (ability to push back or escalate when appropriate)

This identity framework is essential for trace interpretability, deterministic replay, accountability, and reducing identity drift/hallucinations seen in long-horizon agents. Research shows that **explicit and retrievable identity representations improve persona coherence and reduce drift** (e.g., the *Identity Retrieval-Augmented Generation (ID-RAG)* mechanism, which grounds agent identity in a dynamic knowledge graph of beliefs, traits, and values)[^1].

---

## Current vs Target Mapping

| Surface | v0.7 Current | v0.9x Target |
| --- | --- | --- |
| Trace identity fields | No first-class `agent_entity_id`, `identity_version`, `security_identity`, `model_ref` fields on each normalized trace event | Identity-aware trace model includes these references as deterministic replay inputs |
| Delegation outcomes | `allowed`, `denied`, `needs_approval` (delegation-policy vocabulary) | `ALLOW`, `REFUSE`, `ESCALATE`, `ASK_CLARIFY` (normative autonomy vocabulary) |
| Narrative identity persistence | No dedicated identity graph store in runtime | Versioned identity graph persisted in SQLite/libSQL with controlled commit rules |
| Replay context | Deterministic replay from existing stable trace and artifact surfaces | Replay context is expanded to include identity/version/model references and memory snapshot tags |
| Identity retrieval in decision loop | Not a first-class runtime step | Deterministic retrieval of identity subgraph is part of decision grounding |

**Migration intent:** introduce target surfaces incrementally without breaking v0.7 deterministic guarantees or existing trace consumers.

---

## Core Concepts

### 1. Agent Entity

An Agent Entity is the fundamental *actor* in ADL. It has multiple identity facets.

#### A. Security Identity

A **machine identity** used for authentication, authorization, and auditing. It must be:

- Unique and persistent
- Cryptographically verifiable (e.g., via JWT/OAuth or PKI)
- Included in all trace events

Security identity supports integration with existing IAM practices and does *not* entail custom cryptography.

**Sample structure (Rust conceptual):**

```rust
struct SecurityIdentity {
    agent_entity_id: String,         // Stable unique agent principal
    auth_token: String,              // e.g., JWT
    last_authenticated_at: DateTime,
}
```

#### B. Narrative Identity

A versioned, persistent self-representation - a structured model of an agent's persona, preferences, and values.

This is inspired by the ID-RAG paradigm in which identity is represented as a knowledge graph of beliefs, traits, goals, and values, and is explicitly queried during decision loops to ensure consistency across time and behavior[^1].

This narrative identity differs from episodic memory (ObsMem) - the latter captures experiences; narrative identity captures enduring self-concepts.

### 2. Identity Representation and Store

ADL persists **agent state** (identity + memory + replay anchors) behind a small storage abstraction. For v0.9x, the reference implementation is **SQLite / libSQL**.

**Why SQLite / libSQL (first pass):**
- Embedded, local-first, simple ops.
- ACID transactions (required for replay determinism and versioned identity commits).
- WAL mode supports high read concurrency.
- libSQL provides a natural upgrade path to replication/sync later without changing the logical model.

#### Store abstraction

The runtime should not depend directly on SQL. Instead, it uses a thin interface that can be backed by SQLite/libSQL now and swapped later if needed.

- **AgentStore**: owns agent entities and agent instances
- **IdentityStore**: versioned identity graph commits + queries
- **MemoryStore**: ObsMem snapshots and retrieval indices

> Note: We are not committing to a specific Rust trait shape in this document; the intent is to keep the boundary small and stable.

#### Identity Knowledge Graph (IKG)

ADL uses a **versioned identity graph** to represent narrative identity.

- **Nodes** represent persona elements: traits, values, long-term goals, stable beliefs, role constraints.
- **Edges** represent relationships: `supports`, `contradicts`, `derived_from`, `updated_by`, `reinforced_by`, etc.
- **Versions**: each commit produces a new `identity_version` (monotonic ID or content-addressed hash).

The IKG is conceptually aligned with *Identity Retrieval-Augmented Generation (ID-RAG)*: identity is stored as an explicit, retrievable structure and is queried during decision loops to maintain coherence across time[^1].

#### SQLite / libSQL schema (reference)

```sql
-- Agents (principals)
CREATE TABLE agent_entities (
    agent_entity_id TEXT PRIMARY KEY,
    created_at DATETIME,
    metadata JSON
);

-- Agent instances (a concrete incarnation/run)
CREATE TABLE agent_instances (
    agent_instance_id TEXT PRIMARY KEY,
    agent_entity_id TEXT NOT NULL,
    created_at DATETIME,
    metadata JSON,
    FOREIGN KEY(agent_entity_id) REFERENCES agent_entities(agent_entity_id)
);

-- Identity graph nodes
CREATE TABLE identity_nodes (
    node_id INTEGER PRIMARY KEY,
    label TEXT NOT NULL,
    properties JSON
);

-- Identity graph relationships
CREATE TABLE identity_edges (
    edge_id INTEGER PRIMARY KEY,
    from_node INTEGER NOT NULL,
    to_node INTEGER NOT NULL,
    relation TEXT NOT NULL,
    properties JSON,
    FOREIGN KEY(from_node) REFERENCES identity_nodes(node_id),
    FOREIGN KEY(to_node) REFERENCES identity_nodes(node_id)
);

-- Identity versions (commits)
CREATE TABLE identity_versions (
    identity_version TEXT PRIMARY KEY,
    agent_entity_id TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    description TEXT,
    root_node INTEGER,
    parent_identity_version TEXT,
    metadata JSON,
    FOREIGN KEY(agent_entity_id) REFERENCES agent_entities(agent_entity_id),
    FOREIGN KEY(root_node) REFERENCES identity_nodes(node_id)
);

-- Optional: link which nodes/edges belong to which identity_version.
-- For small graphs, you can also snapshot by duplicating nodes/edges per version.
CREATE TABLE identity_version_nodes (
    identity_version TEXT NOT NULL,
    node_id INTEGER NOT NULL,
    PRIMARY KEY(identity_version, node_id),
    FOREIGN KEY(identity_version) REFERENCES identity_versions(identity_version),
    FOREIGN KEY(node_id) REFERENCES identity_nodes(node_id)
);

CREATE TABLE identity_version_edges (
    identity_version TEXT NOT NULL,
    edge_id INTEGER NOT NULL,
    PRIMARY KEY(identity_version, edge_id),
    FOREIGN KEY(identity_version) REFERENCES identity_versions(identity_version),
    FOREIGN KEY(edge_id) REFERENCES identity_edges(edge_id)
);

-- ObsMem snapshots (episodic memory; content may be stored as JSON, CBOR bytes, or external blobs)
CREATE TABLE obsmem_snapshots (
    obsmem_snapshot_id TEXT PRIMARY KEY,
    agent_entity_id TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    summary TEXT,
    payload BLOB,
    metadata JSON,
    FOREIGN KEY(agent_entity_id) REFERENCES agent_entities(agent_entity_id)
);

-- Replay anchors (goal + schedule artifacts)
CREATE TABLE replay_anchors (
    replay_anchor_id TEXT PRIMARY KEY,
    agent_entity_id TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    goal_id TEXT NOT NULL,
    schedule_artifact_ref TEXT,
    metadata JSON,
    FOREIGN KEY(agent_entity_id) REFERENCES agent_entities(agent_entity_id)
);
```

Implementation notes:
- Prefer **WAL mode** for concurrency.
- Identity and memory updates should be committed atomically with the corresponding trace event (or with a trace pointer) to preserve replay semantics.

---

## Identity Integration into Execution

### Identity Version in Traces

Every trace event in ADL must include:

- `agent_entity_id` - the principal
- `agent_instance_id` - specific incarnation/run identifier
- `identity_version` - the narrative identity version used
- `obsmem_snapshot_id` - memory snapshot used for grounding
- `replay_anchor_id` - goal/schedule anchor tying the step to persistent intent
- `security_identity` - authentication context
- `model_ref` - model identity (provider, build fingerprint, inference parameters)


#### The Agent Identity Tuple (required for replay equivalence)

ADL treats an executing agent as more than a model invocation. The minimum stable set of identifiers that define the *who / with-what-self / under-what-model* context is the **Agent Identity Tuple**.

An event that is intended to be replayable MUST record these fields (directly or by stable reference):

- `agent_entity_id`: stable principal identity (who)
- `agent_instance_id`: a specific incarnation/run of the agent (which instance)
- `identity_version`: narrative identity graph version used for the decision (which self)
- `obsmem_snapshot_id`: memory snapshot loaded/queried for grounding (which experience state)
- `replay_anchor_id`: goal/schedule anchor that ties the step to persistent intent (which objective thread)
- `model_ref`: model build + runtime + inference parameters (which model)

If any tuple field is missing, replay tooling MUST treat the run as **non-equivalent** (or fail closed when strict replay is requested).

> Rationale: this tuple captures the minimum state required to debug drift, explain behavior, and reproduce long-horizon decisions.

#### The Environment Tuple (required for replay equivalence)

The Agent Identity Tuple captures *who* acted and with what internal state. Equivalent replay also depends on a minimal description of the *world* the agent observed and acted upon.

An event that is intended to be replayable MUST record (directly or by stable reference) the **Environment Tuple**:

- `tool_io_ref`: stable references to tool inputs/outputs used by the step (or hashes thereof)
- `artifact_ref`: stable references to any file/blob artifacts consumed or produced (including content hashes)
- `workspace_ref`: stable reference to the workspace state when relevant (e.g., repo root + commit SHA + dirty status)
- `policy_ref`: the policy bundle version(s) used for delegation/allow/deny decisions
- `contract_ref`: the contract bundle version(s) enforced for the step
- `clock_ref`: time/clock anchoring used by the step (wall-clock, monotonic clock, or explicit injected time)
- `net_ref`: network policy + egress allowlist context (or a marker that networking was disabled)

If any required Environment Tuple field is missing, replay tooling MUST treat the run as **non-equivalent** (or fail closed when strict replay is requested).

> Note: ADL can start by recording these as opaque references/hashes; richer structured capture can evolve without breaking the tuple concept.

#### `model_ref` (required)

`model_ref` is not just a display name. It is the minimum information required to make replay and auditing meaningful across providers and local runtimes.

Recommended fields:
- `provider`: `openai`, `local`, `bedrock`, `vertex`, etc.
- `model_name`: provider-facing model name
- `model_build_fingerprint`: stable fingerprint for the model build (e.g., provider revision ID, weights hash, or content-addressed identifier)
- `runtime`: engine + version for local models (`llama.cpp@...`, `vllm@...`, etc.)
- `inference_params_fingerprint`: stable hash of inference parameters (temperature, top_p, seed, max_tokens, tool settings)
- `model_artifact_ref` (optional but recommended for local models): URI/path + hash for the weights artifact(s) and any quantization/adapters (e.g., LoRA) used

Replays MAY run with a different model/build, but MUST mark the replay as **non-equivalent** and record the substituted `model_ref`.

Example trace snippet:

```json
{
  "trace_id": "...",
  "agent_entity_id": "...",
  "agent_instance_id": "...",
  "identity_version": "...",
  "obsmem_snapshot_id": "...",
  "replay_anchor_id": "...",
  "security_identity": { "...": "..." },
  "model_ref": { "...": "..." },
  "tool_io_ref": "...",
  "artifact_ref": "...",
  "workspace_ref": "...",
  "policy_ref": "...",
  "contract_ref": "...",
  "clock_ref": "...",
  "net_ref": "..."
}
```

This ensures traces replay under equivalent identity conditions, enabling deterministic reconstruction.

### Atomicity and Commit Semantics

To preserve determinism, ADL SHOULD commit the following as a single atomic unit (or commit them with an immutable pointer chain):

- the normalized trace event for the step
- any `identity_version` commit created during the step
- any `obsmem_snapshot_id` created or advanced during the step
- any `replay_anchor_id` updates (goal/schedule state)

If these commits are not atomic, replays can observe impossible combinations (e.g., a trace that references an identity version that was never committed).

**Acceptance criteria (target-state):**

- Trace schema includes explicit identity references with stable serialization rules.
- Normalized trace/replay tooling can diff and replay identity-aware traces deterministically.
- Legacy v0.7 traces remain readable via compatibility handling or explicit version gating.

### Identity Retrieval and Decision Loop

During an agent's decision step:

1. Generate identity query based on task context and goals
2. Retrieve relevant identity graph subset (fetch core beliefs/traits/values)
3. Ground generation/decision (include retrieved identity context with working memory and prompt context)
4. Apply policies and contracts (normative behavior including delegation rules and refusal/escalation policies)
5. Emit trace event including identity references

Research shows explicitly retrieved identity context mitigates drift and improves coherence across long horizons[^1].

### Identity Evolution and Commit Rules

Identity graph updates must follow controlled commit rules:

- Only certain update classes are automatic (e.g., adding contextual observations)
- Core trait/value changes require policy evaluation and explicit justification
- All commits are trace-linked and versioned

This prevents uncontrolled identity drift while enabling evolution.

**Acceptance criteria (target-state):**

- Identity commits are trace-linked with deterministic version identifiers.
- Policy-gated updates enforce stricter controls for trait/value mutations than observational updates.
- Replay can resolve and load the exact identity graph version referenced by execution traces.

---

## Delegation and Normative Autonomy

Agents support pushback outcomes:

| Outcome | Meaning |
| --- | --- |
| `ALLOW` | The request respects identity/policy |
| `REFUSE` | The agent must decline based on core constraints |
| `ESCALATE` | Requires higher authority/approval |
| `ASK_CLARIFY` | Needs more information to proceed |

These are first-class execution outcomes and are traceable.

**v0.7 bridge mapping (for migration clarity):**

- `ALLOW` maps from current `allowed`
- `REFUSE` maps from current `denied`
- `ESCALATE` maps from current `needs_approval`
- `ASK_CLARIFY` is a planned target outcome with no direct v0.7 delegation-policy equivalent

**Acceptance criteria (target-state):**

- Outcome vocabulary is standardized across policy evaluation, runtime decisions, and trace artifacts.
- Policy docs and runtime enums align on one canonical outcome set.
- Backward compatibility mappings are documented for v0.7-era artifacts.

---

## Replay and Continuity

For deterministic replay, every decision must include:
Equivalent replay requires both the **Agent Identity Tuple** and the **Environment Tuple**.

- `identity_version`
- `obsmem_snapshot_id`
- `model_ref` (fixed across replay unless documented)
- Schedule / goal artifacts

### Replay Equivalence Levels

ADL distinguishes replay modes so tooling can be honest about what is being reproduced:

- **Equivalent replay**: Agent Identity Tuple AND Environment Tuple match, and tool/environment inputs are identical.
- **Model-substituted replay**: `model_ref` differs; MUST be marked non-equivalent and record substitutions.
- **Identity-substituted replay**: `identity_version` and/or `obsmem_snapshot_id` differs; MUST be marked non-equivalent and record substitutions.
- **Best-effort replay**: missing tuple fields or missing artifacts; runs only when strict replay is not required and MUST clearly report gaps.

These form the fixed context for replay, ensuring behavior replication.

**Acceptance criteria (target-state):**

- Replay tooling fails closed when required identity/model snapshot references are missing.
- Replay metadata clearly distinguishes fixed inputs from allowed deltas.
- Deterministic replay tests cover identity-aware and legacy replay paths.

---

## Security Considerations

Agents operate in hybrid modes (delegated human authority + autonomous privileges) and must reconcile both:

- Access control decisions reference both security identity and narrative identity
- Logs record context, not just actions
- Least-privilege principles apply

Traditional IAM systems do not fully capture dynamic agentic identity requirements; ADL's layered model (security + narrative) explicitly addresses these gaps[^2].

Narrative identity storage and retrieval must also follow explicit privacy/safety constraints:

- Do not persist raw credentials, private keys, or token material in narrative identity graphs.
- Restrict or redact sensitive personal data classes unless explicitly allowed by policy.
- Ensure trace and identity artifacts remain audit-friendly without exposing confidential prompt/tool payloads.
- Apply least-privilege access controls to identity read/write operations and version history.

**Acceptance criteria (target-state):**

- Identity persistence policy defines prohibited data classes and redaction behavior.
- Security review validates separation of security identity artifacts and narrative identity data.
- Access auditing covers identity read/write and version-commit operations.

---

## Optional Future Enhancements

While SQLite / libSQL is the first-pass store, future enhancements may include:

- Native graph databases (e.g., Neo4j) for richer queries
- Sync/replication using libSQL replication
- Decentralized Identifiers (DIDs) for global persistent identity primitives[^3]

---

## Provider Capability Contracts and Runtime Probing (Proposed v0.9x+)

### Rationale

ADL distinguishes between a **model invocation** and an **agent execution context**. In practice, providers expose incomplete or marketing-driven descriptions of operational limits (e.g., context window size, feature support, tool-calling behavior, reasoning trace emission). There is no enforced, machine-readable standard for these capabilities.

For deterministic replay, scheduling, and identity integrity, ADL MUST NOT rely solely on provider-declared properties. Instead, ADL introduces:

1. A **Provider Capability Contract** (declared/static surface)
2. A **Runtime Probe Tool** (observed/measured surface)
3. A conservative **Effective Capability Envelope**

This prevents capability inflation, identity drift, and non-reproducible execution behavior.

---

### Provider Capability Contract (Declared Surface)

Each provider integration SHOULD expose a structured capability contract describing its declared features and limits.

Conceptual JSON shape:

```json
{
  "provider": "ollama",
  "model_name": "Qwen3.5:9b",
  "model_build_fingerprint": "sha256:...",
  "runtime": "ollama@0.17.5",
  "limits": {
    "max_context_tokens_theoretical": 32768,
    "max_output_tokens": 4096,
    "max_request_bytes": 10485760
  },
  "features": {
    "system_prompt": true,
    "tool_calling_native": false,
    "streaming": true,
    "reasoning_trace_visible": true,
    "vision": false,
    "json_mode": false
  }
}
```

Notes:

- These values MAY be incomplete or optimistic.
- They represent what the provider/runtime claims, not what is empirically verified.
- This contract becomes part of `model_ref` metadata in traces.

---

### Runtime Probe Tool (Observed Surface)

ADL introduces a CLI-level probing utility (target: `adl provider probe`) that measures operational characteristics empirically.

The probe tool SHOULD:

1. Measure **maximum accepted context size** (incremental/binary search).
2. Measure **maximum stable context size** (needle-in-haystack validation).
3. Measure **maximum output tokens before truncation**.
4. Detect reasoning trace leakage (e.g., `Thinking...` markers).
5. Evaluate JSON-schema compliance under constraint.
6. Record latency curves as context grows.
7. Record determinism stability (N identical runs with same seed).

The result is written as:

```json
{
  "provider_caps_observed": {
    "max_context_tokens_observed": 24576,
    "max_context_tokens_stable": 16384,
    "max_output_tokens_observed": 4096,
    "reasoning_trace_visible": true,
    "json_mode_reliability": 0.87,
    "determinism_hash_stability": 0.92
  },
  "environment": {
    "device": "mps",
    "memory_gb": 64,
    "kv_cache_type": "q8_0"
  },
  "timestamp": "2026-03-03T18:42:00Z"
}
```

This artifact SHOULD be:

- Stored in trace metadata
- Included in Learning Export Bundles
- Referenced by replay tooling

---

### Effective Capability Envelope

At runtime, ADL computes:

```
EffectiveCaps = min(DeclaredCaps, ObservedCaps)
```

Scheduling, chunking, and prompt budgeting MUST use the Effective Capability Envelope, not provider marketing claims.

If ObservedCaps are missing:

- Strict mode: fail closed
- Non-strict mode: warn and proceed with declared limits

---

### Integration with Agent Identity Tuple

The Agent Identity Tuple is extended conceptually to include:

- `provider_caps_declared_ref`
- `provider_caps_observed_ref`

Replays that alter capability envelopes MUST be marked **non-equivalent**, even if `model_ref` matches.

This ensures that:

- A model run under different memory constraints is not silently treated as identical.
- Capability drift becomes observable and auditable.

---

### Acceptance Criteria (Target-State)

- Provider capability schema is defined and versioned.
- Runtime probe CLI exists and emits structured JSON artifacts.
- Trace schema supports linking capability artifacts.
- Replay tooling distinguishes equivalent vs capability-substituted replays.
- Strict replay mode fails closed when capability artifacts are missing.

---

### Migration Strategy

- v0.9: Introduce schema + trace fields (no enforcement).
- v0.9.1: Ship probe CLI (manual invocation).
- v0.9.2+: Integrate EffectiveCaps into scheduling and budgeting.
- 3rd-party review required before strict replay enforcement.

---

---

## Summary

ADL identity architecture defines a multi-layer agent identity model:

- Security identity for authentication/authorization
- Narrative identity as a versioned knowledge graph supporting persistence and coherence
- Integrated trace model linking execution to identity versions and model identity
- Normative autonomy outcomes (`ALLOW`, `REFUSE`, `ESCALATE`, `ASK_CLARIFY`)
- Deterministic replay support through explicit identity tags

Together, this supports agents that remember who they are, act coherently over time, respect contracts and policies, and produce reproducible traces.

---

## Implementation Phases (Proposed)

1. Identity store foundations
   Define schema, versioning semantics, and compatibility boundaries for SQLite/libSQL-backed identity graph persistence.
2. Trace schema expansion
   Add identity/model reference fields with strict normalization and version-gated artifact parsing.
3. Decision-loop integration
   Introduce deterministic identity retrieval as a first-class runtime step for agent decision grounding.
4. Outcome vocabulary unification
   Standardize on `ALLOW`/`REFUSE`/`ESCALATE`/`ASK_CLARIFY` with explicit compatibility bridges for v0.7 artifacts.
5. Replay hardening
   Extend replay inputs and tests to include identity/version/model snapshot constraints with fail-closed behavior.

---

## Milestone Slicing (Proposed)

1. v0.75 (bridge and scaffolding)
   Keep existing trace/event contracts stable while introducing version-gated schema hooks, migration mappings, and no-op identity plumbing points.
2. v0.8 (identity substrate)
   Land identity store foundations, commit semantics, and deterministic retrieval interfaces without requiring full runtime adoption in every execution path.
3. v0.9 (runtime integration)
   Wire identity retrieval into decision loops, standardize target outcome vocabulary end-to-end, and expand replay inputs to require identity/model snapshot references.
4. v0.9x hardening (compat + audit)
   Complete compatibility tooling for legacy artifacts, add replay/trace regression coverage for mixed-version runs, and finalize security/privacy controls for narrative identity data.

---

[^1]: See Identity Retrieval-Augmented Generation (ID-RAG) research for structured identity grounding.
[^2]: Traditional IAM approaches (OAuth/JWT) handle authentication but not dynamic persona persistence.
[^3]: Decentralized Identifiers (DIDs) provide a standards-based persistent identity layer for future integration.
