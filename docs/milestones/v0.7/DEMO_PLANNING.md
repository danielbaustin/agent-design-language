# v0.7 WP-13 Demo Matrix + Integration Demos

> Purpose: inventory competitor demo archetypes and implement ADL equivalents (and “better-than” differentiators) as part of **v0.7 WP-13**.
>
> Execution model: this document is written to be **directly executable by Codex.app** as a sequence of small, low-conflict issues/PRs.

## Goals

1. **Parity:** ADL can run the same demo archetypes as competing platforms (CrewAI, AutoGen, LangGraph, Google ADK, etc.).
2. **Better-than:** each demo proves at least one ADL advantage:
   - deterministic replay
   - contract-first validation and repair
   - canonical trace + auditable artifacts
   - bounded concurrency with deterministic merge
3. **Integration-ready:** demos run end-to-end via CLI and are suitable for CI (optional model calls gated by env).

## Non-goals (v0.7)

- Full UI / web dashboard for demos
- Heavy external dependencies
- Long-running hosted service

## Principles

- **Single demo harness:** a consistent way to run demos, capture artifacts, and validate outputs.
- **Hermetic defaults:** demos run with `--dry-run` / mocked providers unless explicitly enabled.
- **Trace-first:** every demo emits a canonical `run_summary.json` + trace events.
- **Small PRs:** each demo is an independent unit, with minimal shared-file overlap.

## Determinism Rules (v0.7 Contract)

All demos must obey strict determinism in `--dry-run` mode:

- Golden outputs must match byte-for-byte.
- Artifact ordering must be stable and canonical.
- Trace event ordering must be deterministic.
- No wall-clock timestamps unless explicitly seeded/deterministic.
- No random UUIDs unless seeded.
- No nondeterministic map/set iteration in emitted JSON.
- Artifact filenames must be predictable and replay-friendly.

Any violation is considered a regression and blocks merge.

---

## Deliverables

### D1. Demo Harness (shared)

- Directory: `.adl/demos/`
- Each demo has:
  - `README.md` (purpose, how to run, expected artifacts)
  - `demo.adl.yaml` (or equivalent)
  - `golden/` expected artifacts (for `--dry-run` / deterministic mode)
  - `scripts/run.sh` convenience wrapper

### D2. Demo Matrix

- A table mapping competitor archetype → ADL pattern/spec primitive → implementation status.

### D3. v0.7 Demo Set

Initial set for WP-13 (prioritized):

1. Sequential pipeline (state passing)
2. Parallel fan-out/merge (bounded concurrency)
3. Writer–Critic loop (bounded iteration)
4. Group chat / managed multi-agent conversation
5. Swarm routing / handoff policy
6. Agentic RAG (agent decides when to retrieve)
7. Supervisor agent (team orchestration)
8. SQL agent (sandboxed tool)
9. Human-in-the-loop pause/resume (checkpoint story)
10. Memory demo (SQLite-backed minimal memory)

## Learning Integration (EPIC-A Alignment)

All demos must integrate with the v0.7 learning surfaces:

- Emit `run_summary.json` v1.
- Support scoring hooks (when enabled) and emit `scores.json`.
- Emit `suggestions.json` if scoring produces suggestions.
- Be compatible with overlay application (opt-in).
- Be exportable via Dataset Export v1 without modification.

Demos serve as canonical fixtures for validating:
- Artifact model v1
- Scoring hooks v1
- Suggestions schema v1
- Overlay v1 behavior

---

## Demo Matrix

| ID | Archetype (Competitor) | Description | ADL Primitive / Pattern | ADL “Better-than” Proof | Status | Security Surface Exercised |
|---:|---|---|---|---|---|---|
| D-01 | Sequential pipeline (ADK, Crew) | step1→step2→step3 with state passing | workflow steps + state keys | deterministic replay + contract validation | planned | Signing + verification profile (when remote enabled) |
| D-02 | Parallel fan-out/merge (ADK) | parallel sub-tasks + merge | fork/join + deterministic merge | bounded concurrency + canonical aggregation | planned | Scheduler policy + bounded concurrency invariants |
| D-03 | Writer–Critic loop (ADK) | iterative refinement until threshold | loop/repeat-until pattern | explicit termination contract + trace per iter | planned | Loop termination invariants |
| D-04 | Group chat (AutoGen) | multi-agent turn-taking w/ manager | conversation topology → compiled DAG | deterministic speaker selection + transcript trace | planned | Delegation trace + speaker policy |
| D-05 | Swarm routing (AutoGen) | handoffs between agents | policy-driven router + state machine | policy audit trail + replay | planned | Routing policy + audit trail |
| D-06 | Agentic RAG (LangGraph) | agent chooses retrieve vs think | retriever tool + tool gating | cached retrieval + replayable decisions | planned | Tool gating + retrieval audit |
| D-07 | Supervisor (LangGraph) | supervisor delegates to specialists | supervisor pattern → subflows | explicit decision contract | planned | Delegation policy enforcement |
| D-08 | SQL agent (LangGraph) | agent generates/executes SQL safely | sql tool + sandbox + schema | strict output typing + denylist/allowlist | planned | Sandbox + SQL allowlist enforcement |
| D-09 | HITL pause/resume (LangGraph) | pause for human edit then continue | pause step + resume patch | traceable human input + checkpoint metadata | planned | Checkpoint token integrity |
| D-10 | Memory demo (Agno/Crew) | write/read memory across steps | memory tool interface + sqlite impl | deterministic serialization + audit | planned | Memory serialization + redaction surface |
| D-11 | Enterprise signed remote workflow | End-to-end signed workflow with remote execution + verification profile | signing + remote exec + policy enforcement | proves trust boundaries + replay under signature constraints | planned | Security envelope + signing trust policy + overlay guardrails |

---

## Execution Plan (WP-13)

### Phase 0 — Wiring (1 PR)

**P0-1: Add demo harness skeleton**

- Create `.adl/demos/README.md` describing conventions.
- Add `.adl/demos/_template/` with:
  - `README.md`
  - `demo.adl.yaml`
  - `golden/`
  - `scripts/run.sh`
- Add `.adl/demos/run_all.sh`:
  - Runs demos in `--dry-run` mode
  - Validates golden outputs

Acceptance criteria:
- `./.adl/demos/run_all.sh` succeeds in a fresh clone (no model calls).

---

### Phase 1 — Core workflow demos (3 PRs)

**P1-1: D-01 Sequential pipeline demo**

Scenario:
- Input: a local text file
- Step A: normalize (contract: valid UTF-8, trimmed)
- Step B: summarize (mock provider by default)
- Step C: extract bullet list (contract: JSON array of strings)

Artifacts:
- `run_summary.json`
- `trace.jsonl`
- `outputs/summary.md`
- `outputs/bullets.json`

Acceptance criteria:
- Deterministic output in `--dry-run` mode equals `golden/`.
- Contract validation failures are surfaced with structured errors.

**P1-2: D-02 Parallel fan-out/merge demo**

Scenario:
- Input: 8 short paragraphs
- Fan-out: per-paragraph extraction (parallel)
- Join: deterministic merge into canonical order

Acceptance criteria:
- Merge ordering is deterministic (stable sort by paragraph index).
- Concurrency is bounded (configurable max).

**P1-3: D-03 Writer–Critic loop demo**

Scenario:
- Start with a draft paragraph
- Critic scores against rubric
- Loop until score ≥ threshold or max iters

Acceptance criteria:
- Loop emits per-iteration trace events.
- Termination reason is explicit in `run_summary.json`.

---

### Phase 2 — Multi-agent orchestration demos (2 PRs)

**P2-1: D-04 Group chat demo**

Scenario:
- Agents: Planner, Implementer, Reviewer
- Manager controls turn order
- Output: final plan + review notes

Acceptance criteria:
- Deterministic speaker selection in `--dry-run`.
- Transcript is emitted and replayable.

**P2-2: D-05 Swarm routing / handoff demo**

Scenario:
- Router agent picks which specialist handles a task
- Specialists: Research, Code, QA
- Router decision is logged + explainable

Acceptance criteria:
- Router decisions recorded as structured events.
- Re-run with same inputs yields same routing in deterministic mode.

---

### Phase 3 — Tooling demos (4 PRs)

**P3-1: D-06 Agentic RAG demo**

Scenario:
- A small local corpus under `.adl/demos/d-06-agentic-rag/corpus/`
- Retriever tool returns top-k chunks
- Agent decides whether to call retriever

Acceptance criteria:
- Retrieval calls are logged with query + results hash.
- Cached retrieval makes replay deterministic.

**P3-2: D-07 Supervisor demo**

Scenario:
- Supervisor decomposes into 2–3 subtasks
- Delegates to worker agents
- Merges results

Acceptance criteria:
- Delegation decisions emit a structured “delegation decision” event.

**P3-3: D-08 SQL agent demo**

Scenario:
- Local SQLite db under demo directory
- Tool enforces allowlist of tables + read-only
- Agent answers a question by generating SQL

Acceptance criteria:
- SQL tool rejects non-SELECT or disallowed tables.
- Results serialized in a typed JSON format.

**P3-4: D-09 HITL pause/resume demo**

Scenario:
- Workflow pauses after drafting
- Human provides a patch file
- Workflow resumes and finishes

Acceptance criteria:
- `pause` emits a “resume token” artifact.
- Resume uses patch and logs human input metadata.

---

### Phase 4 — Memory demo (1 PR)

**P4-1: D-10 Memory demo**

Scenario:
- Step 1 writes key facts to memory
- Step 2 later reads and uses them
- Default backend: SQLite file in demo dir

Acceptance criteria:
- Deterministic serialization (stable ordering, canonical JSON).
- Memory reads/writes are traced with redaction support (if enabled).

---

### Phase 5 — Enterprise Proof Demo (1 PR)

**P5-1: D-11 Enterprise signed remote workflow**

Scenario:
- Workflow signed with an allowed `key_id`.
- Remote execution enabled.
- Verification profile enforced (algorithm + key source policy).
- Attempted overlay that weakens trust policy is rejected.

Acceptance criteria:
- Run fails if `key_id` missing when required.
- Run fails if algorithm not in allowed list.
- Deterministic replay succeeds when policy satisfied.
- Overlay cannot bypass signing or sandbox checks.
- Trace clearly records verification decisions.

---

## CI Contract

- `./.adl/demos/run_all.sh --dry-run` must pass in CI.
- Golden artifact diffs block merge unless explicitly approved.
- CI must validate:
  - Deterministic artifact emission
  - Trace stability
  - Schema validation of run_summary / scores / suggestions (if present)
- Enterprise demo (D-11) must run in a deterministic mock mode by default.

---

## Commands (operator runbook)

### Run a single demo

```
./.adl/demos/d-01-seq-pipeline/scripts/run.sh --dry-run
```

### Run all demos

```
./.adl/demos/run_all.sh --dry-run
```

---

## Definition of Done (WP-13)

- Demo harness merged
- At least D-01 through D-05 merged and runnable in `--dry-run`
- Demo matrix maintained and accurate
- CI job (or documented command) validates golden artifacts
- Each merged demo includes:
  - README
  - deterministic run mode
  - trace + run_summary emission
