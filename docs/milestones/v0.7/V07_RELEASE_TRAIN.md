

# v0.7 Release Train (0.7.0 + 0.7.x minors)

> **Policy:** Keep all v0.7 planning artifacts under `.adl/` until v0.7 is complete.
>
> **Ritual:** Every release (0.7.0, 0.7.1, 0.7.2, …) begins with a **documentation generation run**.

---

## 0. Why a release train?

EPIC-A (Dynamic Learning) is “gravitational”: if we attempt to ship the full learning loop (observe + suggest + apply + eval + training) inside v0.7.0, it will crowd out the original v0.7 work.

This plan:
- ships the original v0.7 runtime foundation as **v0.7.0**,
- then delivers learning in **small, demoable slices** as **v0.7.x minors**.

---

## 1. Definitions

### 1.1 “Surfaces” vs “Learning”

- **Learning surfaces** are run artifacts and stable identifiers that make learning possible later.
  - examples: stable `run_id`, stable `step_id`, durable trace events, consistent artifact paths.

- **Learning loop features** are the mechanisms that compute and apply improvements.
  - examples: `run_summary.json`, scorers, learner suggestions, overlays apply, dataset export.

Rule of thumb:
- v0.7.0 may include **surfaces**.
- v0.7.x minors ship the **learning loop**.

### 1.2 “No silent drift” principle

Learning must be:
- **opt-in**
- **auditable**
- **reversible**

Overlays (when we add them) must never mutate source workflow YAML.

---

## 2. Release ritual: documentation generation run

Every release begins with a deterministic documentation pass.

### 2.1 Output expectations

The doc-gen run must produce:
- a top-level summary of what changed in the release
- updated ADRs / design notes if required
- updated examples if required
- a “release notes” section capturing:
  - new flags / CLI changes
  - new artifacts written
  - breaking changes (should be rare in minors)

### 2.2 Gate

No release starts coding until the doc-gen run output exists in `.adl/docs/`.

---

## 3. v0.7.0 (foundation release)

**Goal:** ship the original v0.7 runtime foundation without depending on EPIC-A learning mechanics.

### 3.1 Scope (v0.7.0)

Ship these epics in v0.7.0:
- **EPIC-B** Delegation runtime (paper-driven) + patterns work
- **EPIC-E** Security envelope + trust model hardening
- **EPIC-F** Runtime resilience + checkpointing
- **EPIC-C** ObsMem integration + learning **surfaces** (NOT the learning loop)
- **EPIC-D** Cleanup + deferred hard systems work

### 3.2 EPIC-A in v0.7.0

EPIC-A is **not** a delivery epic for v0.7.0.

Allowed in v0.7.0 (surfaces only):
- stable `run_id` semantics
- stable `step_id` semantics
- trace event naming/fields stabilizations
- consistent artifact directory conventions (e.g., `.adl/runs/<run_id>/...`)

Not allowed in v0.7.0 (learning loop):
- `run_summary.json`
- scorers
- learner suggestions
- overlay apply
- dataset export

### 3.3 Optional compatibility stub (recommended)

If low-cost, land a no-op CLI surface in v0.7.0:
- `swarm run --learn=off` exists and is the default
- other values return a clear “not implemented in v0.7.0” error

This reduces CLI churn across minors.

---

## 4. v0.7.x minors (learning train)

**Goal:** deliver EPIC-A as incremental slices that each provide value.

### v0.7.1 — Observe (+ optional feedback)

Deliver:
- `run_summary.json` emission per run (and partial summaries on pause/resume if applicable)
- schema + tests
- (optional) `swarm feedback add --run-id ... --rating ... --note ...`

Demo definition:
- runs produce a canonical learning record that is stable and diffable.

### v0.7.2 — Score

Deliver:
- scoring hooks (initial signals):
  - command
  - file_exists
  - jsonschema_valid
  - tests_pass
- score captured in run_summary

Demo definition:
- “better” is measurable; regression is visible.

### v0.7.3 — Suggest

Deliver:
- learner v0 suggestions (explainable):
  - provider preference per step (success/score/latency)
  - retry/fallback suggestions from error signatures
  - prompt fragment include/exclude suggestions (when fragments exist)
- `swarm learn suggest`

Demo definition:
- ADL produces actionable, explainable advice from prior runs.

### v0.7.4 — Apply (overlays)

Deliver:
- overlay format + deterministic runtime application under `--learn=apply`
- overlay sha256 recorded into trace + run_summary
- `swarm learn apply` (materialize overlay from latest suggestions)

Demo definition:
- closed-loop improvement exists but remains fully reversible.

### v0.7.5 — Export

Deliver:
- `swarm learn export --format jsonl`
- deterministic export of:
  - prompts/tool calls/outputs (as available)
  - metadata + score + feedback

Demo definition:
- offline evaluation/training is unlocked without changing runtime.

---

## 5. Work management rules

### 5.1 Avoid milestone sprawl

If a proposed change requires:
- evaluation harnesses
- promotion gates
- automated training
- adapter registry + hot swap

…it is **not** EPIC-A v0.7.x and should be planned for v0.8+.

### 5.2 Keep examples and specs close to the work

Canonical examples for EPIC-A live in:
- `.adl/examples/` (e.g., `local-plugin-pattern.adl.yaml`)

Canonical planning docs live in:
- `.adl/docs/v07planning/`

### 5.3 Always record “what changed”

Each minor release must include:
- updated release notes from doc-gen run
- an explicit list of:
  - new artifacts written
  - new CLI flags
  - any breaking changes (ideally none)

---

## 6. Quick reference: issue anchors

Current v0.7 epics:
- #413 EPIC-B
- #414 EPIC-C
- #415 EPIC-D
- #429 EPIC-E
- #430 EPIC-F
- #412 EPIC-A (learning train; primarily v0.7.1+)