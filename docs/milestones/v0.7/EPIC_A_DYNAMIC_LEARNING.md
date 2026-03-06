

# EPIC-A: Dynamic Learning for ADL (v0.7+)

> **Thesis:** ADL becomes a force-multiplier for *small/local models* by combining (1) structured workflows, (2) deterministic orchestration, and (3) an auditable learning loop that improves routing/policy/prompting over time — **without** requiring base-model training.
>
> **Design constraint:** Learning must be **safe, reversible, and explainable**. No silent drift.

---

## 0. Context

ADL originally emerged from repeated frustration (Hal9000 era) with small models that could not reliably do basic tasks. The vision is that a framework can:

- **aggregate multiple small local models** (even sequentially under tight hardware constraints),
- use them as **specialized plug-ins**,
- and achieve results comparable to larger cloud models for many workflows.

A closely related strategy appears in the literature (e.g., SuperICL: using a small fine-tuned model to augment a larger black-box LLM), but ADL’s goal is broader: make *plug-in cognition* a **first-class runtime pattern**.

---

## 1. Scope and positioning

This epic is intentionally *the learning substrate*, not the entire “self-improving system.”

### In-scope for v0.7

- **Learning surfaces**: capture run outcomes in a canonical artifact (`run_summary.json`) alongside existing trace.
- **Feedback attachment**: optional human ratings/notes per run (`feedback.json`).
- **Suggestions**: generate explainable recommendations from historical runs (`suggestions.json`).
- **Overlays**: apply recommendations as deterministic, reversible overrides (`overlay.yaml`) when explicitly enabled.
- **Export**: produce stable JSONL datasets for offline analysis/training.

### Explicitly out-of-scope for v0.7

- On-the-fly gradient training (LoRA mid-run).
- Auto-promotion of learned behavior without evaluation gates.
- Any system that mutates source workflow specs.
- Mandatory GPU training infra.

---

## 2. Goals

### G1 — Safe, reversible learning
- Learning must be opt-in (modes: **observe → suggest → apply**).
- Overlays are stored as files, diffable, and deletable.
- Every applied override is logged (overlay hash + overridden fields).

### G2 — Works with API models and local models
- The learning loop must produce value even when we can’t tune provider weights.
- The artifacts must be structured so we *can* later tune local models offline.

### G3 — First-order wins in v0.7
Deliver learning features with immediate ROI:
1) provider routing preference per step
2) retry/fallback policies from error signatures
3) prompt fragment selection (include/exclude named fragments)

---

## 3. Core concept: overlays, not drift

**Overlay** = a deterministic patch applied at runtime resolution time.

- Does not rewrite workflow YAML.
- Scoped (workflow_id, step_id, provider_profile).
- Reversible (delete overlay / disable learning).
- Logged (trace + run summary).

This aligns with ADL’s “control-plane layering” philosophy:

```
workflow spec  →  runtime defaults  →  overlays (opt-in)  →  effective execution
```

---

## 4. User-facing CLI

### 4.1 Run modes
Add `--learn` to `swarm run`:

- `--learn=off` (default)
- `--learn=observe`  (write run_summary)
- `--learn=suggest`  (write run_summary + suggestions)
- `--learn=apply`    (apply overlay + write run_summary + log overlay hash)

### 4.2 Learning commands
Add `swarm learn`:

- `swarm learn suggest [--workflow <id>] [--since <date>]`
  - reads historical runs + feedback
  - writes suggestions
  - optional `--write-overlay` to update overlay

- `swarm learn apply [--workflow <id>]`
  - materializes overlay from latest suggestions

- `swarm learn export --format jsonl`
  - exports (prompt/tool calls/output + metadata + score + feedback)

- `swarm feedback add --run-id <id> --rating <1-5> [--note "..."]`
  - writes feedback.json

---

## 5. Artifact layout

```
.adl/
  runs/
    <run_id>/
      trace.json
      run_summary.json
      feedback.json            (optional)
      suggestions.json         (optional)
      artifacts/...
  learning/
    overlays/
      <workflow_id>.yaml
    suggestions/
      <workflow_id>/
        <timestamp>_<run_id>.json
    exports/
      <timestamp>_dataset.jsonl
```

---

## 6. Data model

### 6.1 `run_summary.json` (canonical learning record)

```json
{
  "schema_version": "v0.7",
  "run_id": "...",
  "workflow_id": "...",
  "workflow_version": "v0.7",
  "provider_profile": "...",
  "started_at": "RFC3339",
  "ended_at": "RFC3339",
  "status": "success|error|paused",
  "overlay": {
    "applied": true,
    "overlay_path": ".adl/learning/overlays/<workflow_id>.yaml",
    "overlay_sha256": "..."
  },
  "steps": [
    {
      "step_id": "brief",
      "provider": { "kind": "local|remote", "profile": "local-fast" },
      "started_at": "...",
      "ended_at": "...",
      "latency_ms": 1234,
      "tokens": { "in": 0, "out": 0, "total": 0 },
      "attempts": 1,
      "status": "success|error|skipped|paused",
      "error": { "code": "timeout|schema|transport|...", "message": "..." },
      "inputs": { "hash": "..." },
      "outputs": { "hash": "...", "artifact_paths": ["..."] }
    }
  ],
  "score": {
    "value": 0.0,
    "max": 1.0,
    "signal": "command|file_exists|jsonschema|tests_pass",
    "details": "..."
  }
}
```

Notes:
- `tokens` may be unknown for some providers; keep fields stable and use `0` when unavailable.
- Store hashes in summaries; raw prompts can be exported via `swarm learn export`.

### 6.2 `feedback.json`

```json
{
  "schema_version": "v0.7",
  "run_id": "...",
  "rating": 1,
  "note": "string",
  "created_at": "RFC3339"
}
```

### 6.3 `suggestions.json` (explainable)

```json
{
  "schema_version": "v0.7",
  "workflow_id": "...",
  "generated_at": "RFC3339",
  "basis": { "runs_considered": 42 },
  "suggestions": [
    {
      "type": "provider_preference",
      "scope": { "step_id": "brief" },
      "recommend": { "provider_profiles": ["local-fast", "remote-openai"] },
      "why": { "success_rate": { "local-fast": 0.95 }, "latency_ms": { "local-fast": 900 } }
    },
    {
      "type": "retry_policy",
      "scope": { "step_id": "brief" },
      "recommend": {
        "max_attempts": 2,
        "on_error": [{ "match": "timeout", "switch_provider_profile": "remote-openai" }]
      },
      "why": { "error_counts": { "timeout": 7 } }
    },
    {
      "type": "prompt_fragments",
      "scope": { "step_id": "brief" },
      "recommend": { "include": ["brief_style_v2"], "exclude": ["brief_style_v1"] },
      "why": { "avg_score_delta": 0.12 }
    }
  ]
}
```

---

## 7. Overlay format + semantics

### 7.1 `overlay.yaml`

```yaml
schema_version: v0.7
workflow_id: demo-workflow
updated_at: 2026-02-22T00:00:00Z
steps:
  brief:
    provider_profile_preference:
      - local-fast
      - remote-openai
    retry:
      max_attempts: 2
      on_error:
        - match: timeout
          switch_provider_profile: remote-openai
    prompt_fragments:
      include: [brief_style_v2]
      exclude: [brief_style_v1]
```

### 7.2 Precedence rules
Overlays apply only when `--learn=apply`:

1) Step spec (explicit in workflow YAML)
2) Overlay override
3) CLI defaults / provider defaults
4) Engine defaults

Overlays may **not** bypass security/trust policies. They only choose among allowed options.

### 7.3 Logging requirements
When an overlay is applied:
- record overlay sha256 in `run_summary.json`
- emit a trace event `OverlayApplied { step_id, fields_overridden }`

---

## 8. Learner v0 (v0.7)

Learner v0 is intentionally light-weight and explainable.

### 8.1 Provider preference
Compute per `(workflow_id, step_id, provider_profile)`:
- success rate
- mean latency (successes only)
- mean score (if present)

Recommend highest success rate; tie-break by score then latency.

### 8.2 Retry/fallback from error signatures
Normalize errors by:
- `error.code` plus coarse message signatures (substring patterns)

Recommend:
- `max_attempts`
- error-triggered fallback provider switches

### 8.3 Prompt fragment selection
If prompt composition uses named fragments:
- observe fragment set → score/feedback
- recommend include/exclude sets correlated with better outcomes

---

## 9. Scoring (reward signal)

### 9.1 Supported scoring signals (v0.7)
- `command`: run command; exit code is pass/fail; optional numeric parse
- `file_exists`: required artifacts exist
- `jsonschema`: output validates against schema
- `tests_pass`: wrapper for `cargo test` or custom command

### 9.2 Safety
Scoring failure must not crash the run. It records `score.value=0` with details.

---

## 10. Small model plug-in protocol (v0.7 design target)

This is a **key philosophical driver**: make small models useful by treating them as *typed plug-ins*.

### 10.1 Plug-in step contract
A plug-in model step should be able to output a structured result like:

```json
{
  "kind": "candidate_plan|label|router_decision|critique|extract",
  "value": { "...": "..." },
  "confidence": 0.0,
  "evidence": ["..."],
  "notes": "..."
}
```

### 10.2 How the coordinator uses it
Downstream steps (often larger models) can:
- incorporate the plug-in output as a **constraint**
- use it as a **proposal** for the next action
- use `confidence` as a routing/policy feature

### 10.3 How learning helps
EPIC-A enables plug-in viability by learning:
- which plug-ins are reliable for which steps
- when to fall back to a stronger model
- how to assemble prompts to best exploit plug-in outputs

---

---

## 10A. Local-first prompting principles (grounded in real local-LLM behavior)

Local models behave differently than cloud LLM products:

- They are **static** during a session (no long-term adaptation to you).
- They tend to be **more literal** about vague instructions and incomplete prompts.
- Cloud platforms often add behind-the-scenes assistance (reasoning scaffolds, retrieval, tool glue, etc.) that local runners typically do not.

Therefore ADL should treat “prompting” as **contract design**, not conversation.

### 10A.1 Principles

1) **One job per step**
   - A plug-in step should do *one thing*: classify / extract / critique / route.
   - Avoid mixing generate + judge + format + tool-use into a single small-model call.

2) **Explicit structure over “figure it out”**
   - Prefer numbered sub-steps, fixed headings, and stable templates.
   - Avoid cloud-style prompts like “make this better” or “what do you think?” without defining success criteria.

3) **Delimiters + stable sections**
   - Use consistent delimiters (e.g., `---`, `###`) to separate:
     - context
     - instructions
     - examples
     - expected output
   - Keep delimiter meaning consistent across workflows.

4) **Schema-first outputs**
   - Small models should output **typed JSON/YAML** that is:
     - validated
     - scored
     - safe to consume by downstream steps.

5) **Few-shot examples as fixtures**
   - Examples are part of the interface contract (not optional garnish).
   - Store them as named prompt fragments so EPIC-A can learn which fixture sets work best.

6) **Confidence is mandatory**
   - Even if rough, `confidence` is the routing signal:
     - accept small-model output when high
     - escalate/fallback when low

### 10A.2 Mapping to EPIC-A deliverables

- `run_summary.json` makes outcomes measurable.
- scoring makes “better” concrete.
- suggestions/overlays let us systematically tighten prompts and routes.
- exports provide training datasets later (optional LoRA/classifiers), without requiring them in v0.7.

---

## 11. Roadmap:

We have multiple v0.7 epics (security envelope, checkpointing, delegation/policy engine, ObsMem surfaces). EPIC-A can become a “gravitational” epic if it expands beyond substrate.

### v0.7 (learning substrate)
- run_summary + feedback + suggestions + overlays
- export JSONL
- plug-in step contract documented (no new provider types required)

### v0.8 (evaluation + promotion)
- evaluation harness and benchmark suites per workflow
- promotion gates (suggest → apply only after passing eval)
- richer scoring signals

### v0.9 (offline training + adapters)
- adapter registry for local providers (LoRA optional)
- scheduled offline fine-tune pipeline (nightly)
- controlled rollout + rollback support

### v1.0 (cohesive story)
- stable UX for: plug-ins + learning + resilience + security
- “small models feel useful” demos and reference workflows

---

## 12. Acceptance criteria

### Artifacts
- Observe/suggest/apply modes reliably write `run_summary.json`.
- Export produces deterministic JSONL.

### Overlays
- Overlay application changes behavior deterministically.
- Applied overlay hash + overridden fields recorded.

### Suggestions
- Suggestions include an explainable `why`.
- Suggestions can be converted into overlays.

### Tests
- Schema round-trip tests for new artifacts.
- CLI smoke tests for learn modes.
- Deterministic ordering and stable output tests.

---

## 13. Open questions (v0.7 defaults)

- Prompt fragment storage: recommend named fragments in workflow YAML; deterministic assembly.
- Raw prompt storage: store hashes in summaries; export raw content through `swarm learn export`.
- Error normalization: start with error codes + substring signatures; iterate.
