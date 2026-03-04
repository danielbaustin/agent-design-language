

# ObsMem (v0.8) — Bayesian Indexing + Provenance-Aware Retrieval

**Status:** Draft (incubation)

## 0. Executive summary

ObsMem is ADL’s **Observational Memory**: a deterministic, provenance-aware memory system built on **execution traces** (runs, activations, tool boundary events, artifacts).

The key idea in this doc is that ObsMem should not be “just embeddings.” It should include a **Bayesian evidence model** that estimates:

- the probability a retrieved item is relevant/useful
- the probability the evidence is trustworthy/provable
- the expected value/cost of using it

…using only **explicit, replayable artifacts**.

This enables smart RAG and operational intelligence:

- “Find the most similar failures to this one and what fixed them.”
- “Which provider/tool policy yields the best outcomes under my budget?”
- “Which patterns are stable and should be promoted?”

## 1. Goals and non-goals

### Goals

1. **Determinism-first memory**
   - ObsMem must be grounded in activation records and trace bundles.
2. **Provenance-aware retrieval**
   - Every retrieval result must include citations (run id, activation id, artifact pointers).
3. **Hybrid retrieval**
   - Combine structured filters + semantic retrieval + Bayesian ranking.
4. **Operational intelligence**
   - Use past executions to improve policies, debugging, authoring, and Gödel evaluation.

### Non-goals (v0.8)

- A general-purpose “personal memory” system.
- Autonomous background learning.
- Training/fine-tuning models inside ObsMem.

## 2. Terminology

- **Run**: an execution of a workflow (versioned plan + inputs).
- **Activation**: an immutable record of a step attempt.
- **Tool boundary event**: external interaction (LLM call, HTTP request, filesystem IO, etc.) captured as data.
- **Trace bundle**: a versioned export format for a run, indexable by ObsMem.
- **Evidence item**: an artifact or derived fact that can be cited.
- **Retrieval result**: a ranked list of evidence items with citations and a score breakdown.

## 3. The ObsMem data plane

### 3.1 Canonical artifacts

ObsMem indexes only what can be replayed and cited:

- Workflow plan (compiled)
- Run metadata
- Activation manifests
- Tool boundary manifests
- Errors and failure classification
- Output artifacts (structured outputs, logs, reports)
- Derived metrics (cost/latency/retry counts)

### 3.2 Minimal trace bundle schema (conceptual)

ObsMem expects a stable on-disk bundle; exact paths can evolve, but the **manifest contract** must be stable.

```
run/
  run.json
  plan.json
  activations/
    <activation_id>/
      activation.json
      inputs.json
      outputs.json
      tool_events.json
      errors.json
  artifacts/
    ...
  derived/
    metrics.json
    summaries.json
```

Required fields (high level):

- `run_id`, `workflow_id`, `workflow_version`
- `activation_id`, `step_id`, `attempt`, `status`
- stable hashes for `inputs`, `outputs`, and captured `tool_events`

## 4. Retrieval surfaces

ObsMem will serve at least three consumers:

1. **Runtime / replay debugging**
   - “Show me similar failures + likely fix patterns.”
2. **Authoring refinement**
   - “Use memory to propose a safer/better workflow plan.”
3. **Gödel evaluation**
   - “Use memory to propose mutations and score stability.”

### 4.1 Query types

- **Structured queries** (fast filters)
  - by failure class, tool/provider, step kind, retry count, latency/cost
- **Semantic queries** (text similarity)
  - embeddings over selected fields (error summaries, step outputs, short logs)
- **Hybrid queries**
  - structured filter → semantic candidate set → Bayesian ranking

### 4.2 Required retrieval output

Every result must include:

- citations: run_id / activation_id / artifact pointer
- score components: similarity, provenance/trust, recency, cost, stability
- a short “why this was returned” explanation

## 5. Bayesian evidence model

### 5.1 Why Bayes here?

Embeddings tell us “similar text,” but ADL needs **actionable evidence**:

- Is it relevant to *this* failure?
- Is it trustworthy / reproducible?
- Is it likely to help under current policy constraints?

Bayes gives a principled way to combine multiple signals.

### 5.2 Variables

Let:

- `E` = evidence item (a cited artifact)
- `Q` = query (structured + semantic)
- `R` = relevance event (E is relevant to Q)
- `T` = trust/provenance event (E is trustworthy)
- `S` = stability event (E came from a stable run / pattern)

We want a ranking score proportional to:

- **Expected utility** of using E:

`Score(E | Q) ∝ P(R | Q, E) * P(T | E) * P(S | E) * U(E)`

Where `U(E)` includes cost and constraints:

- positive utility: historically solved similar problems; high success rate
- negative utility: expensive, slow, flaky, high retry counts

### 5.3 Practical decomposition

For v0.8, keep it simple and explainable:

- `P(R | Q, E)` from semantic similarity + structured match features
- `P(T | E)` from provenance class (captured tool events vs loose logs vs user text)
- `P(S | E)` from run outcomes (success, retries) and pattern stability

Use a log-score form for stability and debugging:

`log Score = w_sim*sim + w_struct*f_struct + w_trust*f_trust + w_stab*f_stab + w_cost*f_cost + w_recency*f_recency`

Then interpret it as a calibrated probability later.

### 5.4 Evidence classes (provenance prior)

Assign evidence items to classes with different priors for `P(T)`:

1. **Captured boundary events** (highest trust)
   - tool calls with request/response captured
2. **Activation outputs** (high trust)
   - structured outputs with stable hashes
3. **Derived metrics** (medium-high)
   - computed from activation records
4. **Free-form logs** (medium)
   - useful but not guaranteed stable
5. **User annotations** (variable)

These priors should be explicit and configurable.

## 6. Index design

### 6.1 Storage (v0.8 scope)

ObsMem needs:

- a **structured index** (SQLite/Postgres; or local embedded DB)
- a **vector index** (optional; can be file-backed initially)

Decision can be deferred, but the API should not leak the backend.

### 6.2 Indexing pipeline

1. Ingest trace bundle manifest(s)
2. Normalize and store structured fields
3. Select text fields for embeddings
4. Compute derived features (cost, latency, retries, failure class)
5. Write citations/pointers to original artifacts

### 6.3 Core tables (conceptual)

- `runs(run_id, workflow_id, workflow_version, ts, status, cost, latency, ...)`
- `activations(activation_id, run_id, step_id, attempt, status, failure_class, ...)`
- `evidence(evidence_id, activation_id, kind, provenance_class, ptr, text_digest, ...)`
- `embeddings(evidence_id, vector_ref, model_id, ...)`
- `features(evidence_id, sim_features..., trust_features..., stability_features...)`

## 7. Smart RAG assembly

ObsMem should provide a **RAG kit** that produces:

- a deterministic evidence set
- a prompt context with citations
- a strict “answer only from evidence” instruction

Key rules:

- Evidence selection is deterministic given:
  - query
  - index state
  - configured retrieval parameters
- The final answer must cite `run_id/activation_id/artifact` for each claim.

## 8. Determinism contract

ObsMem must preserve deterministic replay and evaluation.

Rules:

1. **No hidden state**
   - All retrieval must be explainable from index state + query.
2. **Stable ranking**
   - Ties must be broken deterministically (e.g., by `(score, run_id, activation_id, evidence_id)` ordering).
3. **Versioned retrieval parameters**
   - Retrieval config (weights, thresholds, embedding model id) must be recorded.
4. **Reproducible evidence sets**
   - A workflow run should record the exact evidence ids used.

## 9. Privacy, redaction, and trust boundaries

ObsMem is powerful because it records reality. That increases risk.

v0.8 requirements:

- Redaction hooks for tool boundary events (e.g., secrets)
- “Do not index” flags for artifacts
- Retention policy controls (per-workflow and global)

## 10. Interfaces (v0.8)

### 10.1 CLI

- `adl obsmem ingest <trace_bundle>`
- `adl obsmem query --structured ... --semantic ...`
- `adl obsmem report <run_id>`

### 10.2 Rust API (conceptual)

- `ObsMem::ingest(bundle_path) -> IngestReport`
- `ObsMem::query(Query) -> RetrievalResults`
- `ObsMem::explain(result_id) -> ScoreBreakdown`

## 11. Success criteria (v0.8)

1. Index a handful of runs and answer:
   - “Find similar failures and what fixed them.”
2. Smart RAG returns a response with citations.
3. Analytics show:
   - cost/latency trends by tool/provider
4. Determinism:
   - repeated query returns the same ordered evidence set.

## 12. Open questions

- Which backend for structured index in v0.8: SQLite vs Postgres?
- Vector store: file-backed vs embedded DB vs external?
- Which text fields are safe/useful for embeddings?
- How do we define and encode failure taxonomy consistently across runtime + ObsMem?
- How do we score “usefulness” of evidence in a way Gödel can consume?

---

## Appendix A — Suggested next issues

These map to the v0.8 architecture EPIC breakdown:

- Trace Export Bundle v2
- Index + Retrieval API v1
- Smart RAG over runs v1
- Analytics + reporting v1

(See the `create_obsmem_issues_v0.8.sh` script.)