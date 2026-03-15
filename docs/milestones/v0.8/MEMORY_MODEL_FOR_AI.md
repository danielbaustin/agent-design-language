

# ADL Memory Model for AI Agents

> **Status:** v0.8 planning doc (living)
>
> **Purpose:** Define a coherent, implementable memory architecture for ADL agents covering **Observational Memory (ObsMem)**, reflection (Godel scientific loop), and adaptive execution.
>

>
> **Acknowledgement:** ADL's Observational Memory work is directly inspired by the *Observational Memory* feature in the **Mastra** framework (mastra-ai/mastra). We adapted the concept to be **event-native** and grounded in ADL trace bundles, while crediting Mastra for the original idea and its cost/performance framing.

---

## TL;DR

Most agent memory systems are built on the **conversation fallacy**:

- *conversation -> summarize -> store text*

This produces narrative logs that are:

- lossy,
- non-deterministic,
- hard to query,
- hard to replay,
- and weak at driving learning.

**ADL ObsMem is event-native and structured**:

- *runtime events -> observations -> tuples -> indices -> query + retrieval -> reflection -> (semantic + procedural) updates*

This enables:

- deterministic replay,
- measurable learning over time,
- multi-agent knowledge transfer,
- cost reduction via stable prompt prefixes (cache friendly),
- and tight integration with ADL's trace bundles.

---

## Design goals

1. **Event-native memory**: memory is built from *what the agent did or observed*, not from raw chat transcripts.
2. **Deterministic and replayable**: memory construction must be reproducible from trace bundles when possible.
3. **Queryable**: memory must support structured queries (filters, joins, aggregations).
4. **Composable context**: generate prompt context from layered memory (identity / semantic / episodic / recent).
5. **Cost-aware**: maximize stable context prefixes (prompt caching) and minimize tokens.
6. **Provider-agnostic**: memory works with any model/provider.
7. **Safety + integrity**: memory is evidence-tracked; it should distinguish facts from hypotheses and include provenance.

---

## The four memory layers

ADL treats memory as **four distinct layers**, each with a different job:

| Layer | What it is | Question it answers | Where it lives |
|---|---|---|---|
| **Working** | The current prompt context | What am I thinking about now? | Prompt assembly |
| **Episodic** | Structured record of experience | What happened? What did I try? | ObsMem store |
| **Semantic** | Consolidated knowledge derived from episodes | What is true or likely true? | Knowledge store (derived) |
| **Procedural** | Skills/strategies derived from reflection | How should I act next time? | Policy / strategy store |

A powerful agent is not one memory store. It is **a pipeline from episodic to semantic and procedural memory**.

---

## High-level architecture

```text
                 ┌─────────────────────────────────────┐
                 │             ADL Runtime              │
                 │ (cards, tools, providers, execution) │
                 └─────────────────────────────────────┘
                                  │
                                  ▼
                         ┌────────────────┐
                         │   Event Stream  │
                         │ (Activation log)│
                         └────────────────┘
                                  │
                                  ▼
                     ┌─────────────────────────┐
                     │ Observation Extractor    │
                     │ (rules + model assists)  │
                     └─────────────────────────┘
                                  │
                                  ▼
                     ┌─────────────────────────┐
                     │ ObsMem Tuple Store       │
                     │ (episodic memory)        │
                     └─────────────────────────┘
                                  │
                 ┌────────────────┴────────────────┐
                 │                                 │
                 ▼                                 ▼
       ┌───────────────────┐              ┌───────────────────┐
       │  ObsMem Indices     │              │ Prompt Assembler  │
       │ (query + retrieval) │              │ (working memory)  │
       └───────────────────┘              └───────────────────┘
                 │                                 │
                 ▼                                 ▼
      ┌────────────────────┐            ┌──────────────────────────┐
      │ Gödel Reflection     │            │ Agent Execution (next)   │
      │ (scientific loop)    │            │ uses memory layers       │
      └────────────────────┘            └──────────────────────────┘
                 │
        ┌────────┴─────────┐
        ▼                  ▼
┌─────────────────┐  ┌──────────────────┐
│ Semantic Updates │  │ Procedural Updates│
│ (knowledge)      │  │ (strategies/policy)
└─────────────────┘  └──────────────────┘
``` 

---

## The key mistake most systems make

### The conversation fallacy

Systems like conversation summary memory, and many RAG memories, store narrative text such as:

- User asked X; agent tried A, then B.

This is fine for chat UX, but weak for agent cognition.

### What agents actually experience

Agents experience **events**:

- tool call started
- tool call returned
- compilation failed
- test failed
- retry policy applied
- patch applied
- PR created

Therefore **ObsMem must be built from runtime events** (Activation log + trace bundles), not from chat.

---

## Observational Memory (ObsMem)

### Definition

**ObsMem** is ADL's episodic memory layer: a structured, append-only record of **observations derived from runtime events**, with provenance.

ObsMem is not just storage; it is a contract:

- what we record,
- how we index it,
- how we retrieve it,
- and how it feeds reflection + adaptive execution.

### Why observational

Because the memory stores *observations about what happened* (evidence) rather than raw transcripts.

---

## ObsMem tuple model

A minimal observation tuple shape:

```text
ObservationTuple {
  ts: Timestamp,
  run_id: RunId,
  agent_id: AgentId,
  step_id: StepId?,

  kind: ObservationKind,           // what type of observation
  subject: SubjectRef,             // what object it refers to
  outcome: OutcomeRef?,            // success/failure/result pointer

  facts: Map<String, Value>,       // structured fields
  evidence: EvidenceRef,           // links back to activation log / trace
  confidence: f32,                 // calibrated confidence

  ttl: Duration?,                  // optional expiry
  tags: [String],                  // optional tags for retrieval
}
```

**Key principles:**

1. **Evidence first**: every tuple must link back to trace evidence.
2. **Structured**: facts are machine-operable.
3. **Append-only**: updates create new tuples; they do not rewrite history.
4. **Versioned**: schemas evolve; tuples must be forward-compatible.

---

## Evidence model and determinism

ADL has a unique advantage: **trace bundles**.

- Trace bundles allow us to reconstruct the event stream.
- ObsMem can be (re)derived deterministically from traces.

### Canonical Evidence View

ObsMem should support a canonical evidence view for a run:

- activation log entries
- tool results
- patches
- test outputs
- artifacts

ObsMem tuples reference this evidence by stable IDs.

---

## ObsMem indexing

ObsMem needs indices tuned for agent use cases.

### Core indices

- **By run_id**: retrieve all observations for a run.
- **By agent_id**: retrieve agent-specific history.
- **By kind**: failures, tool outcomes, decisions, etc.
- **By subject**: file path, ticket id, repo path, URL, etc.
- **By tags**: fast filtering (e.g., `"rust"`, `"borrow_checker"`).

### Summary surfaces

Two first-class derived surfaces (mentioned in planning):

- `run_summary`: concise structured summary of a run
- `exec_summary`: execution-centric summary (what happened, what failed, what fixed it)

These can be computed from tuples and stored as derived records.

---

## Prompt context assembly

ADL prompt context should be assembled from memory layers in a stable, cache-friendly structure:

```text
[Stable prefix]
  - Agent identity & charter
  - Semantics (high confidence knowledge)
  - Procedural policies (current strategy set)
  - Long-lived key observations

[Volatile tail]
  - Recent raw messages / instructions
  - Recent observations/events
  - Current task state
```

### Why stable prefixes matter

Stable prefixes increase prompt caching hit rates and reduce cost.

> **Note:** Mastra's Observational Memory demonstrates the economic value of stable windows. ADL should adopt that cost insight while keeping ADL's event-native structure.

---

## Reflection: Gödel scientific loop as memory consolidation

Humans consolidate episodic memory into knowledge and skill. ADL does this via the Gödel loop.

### Loop

1. Observe (events → tuples)
2. Detect anomalies / failures
3. Form hypotheses
4. Run experiments
5. Update knowledge + strategy

```text
Episodic (ObsMem)
   │
   ▼
Reflection (Gödel loop)
   │
   ├──► Semantic updates  (facts, rules, world model)
   │
   └──► Procedural updates (strategies, policies, retry plans)
```

### What is stored where

- **ObsMem** stores *what happened* (with evidence).
- **Semantic memory** stores *what we believe is true*.
- **Procedural memory** stores *how we should behave*.

---

## Procedural memory and Adaptive Execution

Procedural memory is where ADL becomes uniquely powerful.

### Example: learned retry policy

From episodes:

- compile failed with borrow checker
- clone strategy succeeded

Derive procedural rule:

```text
If failure.kind == "rust.borrow_checker" then
  prefer strategy "clone_fix" before other strategies
```

### Adaptive Execution Engine

The Adaptive Execution Engine is the runtime that:

- applies fallback/retry policies,
- records results in ObsMem,
- and updates procedural policies (through reflection or online learning).

This is stick-to-itiveness made portable: **provider-agnostic persistence**.

---

## Comparison: Mastra Observational Memory vs ADL ObsMem

Mastra's Observational Memory (as referenced in discussion) is a strong example of **conversation compression** and **stable prompt windows**.

### Credit & intent

This document intentionally **credits Mastra** as the originator of the Observational Memory concept (Observer/Reflector, dated compressed log, stable-vs-recent windowing, and the associated prompt-caching economics). ADL's approach is an implementation variant tailored to our goals (structured tuples, evidence/provenance, deterministic replay), not an attempt to diminish Mastra's work or business. If you are evaluating memory systems, you should read Mastra's docs and source first.

- Mastra repo: https://github.com/mastra-ai/mastra

### Mastra strengths

- background observer/reflector agents
- compressed, dated text log
- stable vs recent window pattern
- cost savings via prompt caching

### Mastra limitations (relative to ADL)

- primarily **text narrative** (harder to query)
- weak determinism (summaries vary)
- limited evidence/provenance modelf
- not inherently tied to replay / trace bundles

### ADL strengths

- event-native ingestion (activation log + traces)
- structured tuples and indices
- deterministic replay and evidence
- designed to feed Gödel loop + adaptive execution

### Synthesis

ADL borrows the windowing and cache insight from Mastra while keeping ADL's core advantage:

- **structured episodic memory grounded in evidence**.

---

## Storage engine notes

For early iterations, a pragmatic store is appropriate.

- **SQLite / libSQL**: simple, portable, good defaults

Future candidates:

- **DuckDB**: powerful analytical queries
- **redb**: Rust-native embedded KV
- **FoundationDB**: distributed, if/when needed

Key requirement: ability to store tuples + indices + derived summaries.

---

## Interfaces and contracts (implementation sketch)

### Ingestion

- Input: Activation log entries + trace bundle artifacts
- Output: observation tuples appended to store

### Query

- filtered tuple scan
- index lookups
- derived surfaces: `run_summary`, `exec_summary`

### Retrieval (prompt builder)

- takes a `MemoryRequest`:
  - task context
  - agent identity
  - desired kinds/tags
  - token budget
- returns:
  - stable memory prefix
  - recent tail

### Provenance

Every tuple must include:

- `evidence.ref` → stable trace bundle IDs
- optional `source` classification (tool output, model output, human input)

---

## Open questions

1. **Confidence calibration:** how do we score and update confidence?
2. **Contradictions:** how do we represent conflicting observations?
3. **TTL and garbage collection:** which tuples expire and why?
4. **Cross-run generalization:** when do episodic facts become semantic facts?
5. **Multi-agent sharing:** what is the boundary between private vs shared memory?
6. **Safety:** what memory items must be redacted or access-controlled?

---

## Roadmap alignment

This doc supports the ObsMem work packages and related v0.8 goals:

- ObsMem v1 schema and ingestion
- ObsMem query and retrieval
- Evidence rendering and trace bundle v2
- Gödel loop docs + demo (failure → hypothesis → experiment)
- Adaptive policy and online learning loop (v0.8+)

---

## Appendix: Example observation kinds

A starter taxonomy for `ObservationKind`:

- `tool.call`
- `tool.result`
- `exec.step.started`
- `exec.step.completed`
- `failure.detected`
- `failure.classified`
- `patch.proposed`
- `patch.applied`
- `test.run`
- `test.failure`
- `build.run`
- `build.failure`
- `retry.applied`
- `policy.updated`
- `hypothesis.created`
- `experiment.run`
- `experiment.result`

---

## Appendix: Example prompt memory window

```text
# Agent Identity
- You are BuilderAgent. Charter: produce minimal, correct patches.

# Semantic Memory (high confidence)
- Repo uses `swarm/tools/pr.sh` workflow; keep primary checkout on main.

# Procedural Memory (current policies)
- If rust borrow checker error: try clone_fix, then refactor ownership.

# Key Observations (long-lived)
- OBSMEM tuples are evidence-linked to trace bundles.

# Recent Events
- Run 8f2...: compile failed; retry clone_fix succeeded.

# Task
- Implement ObsMem query API.
```

---

## Appendix: Why ADL ObsMem is a platform differentiator

Most frameworks can orchestrate calls.

ADL can:

- **measure what happened** (deterministically)
- **learn what works** (procedural memory)
- **explain why** (evidence view)
- **replay and prove** (trace bundles)

This combination - ObsMem + Evidence + Replay + Reflection + Adaptive Execution - is the core of a trustable platform for agentic systems.
