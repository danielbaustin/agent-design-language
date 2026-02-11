# ADL v0.3 Work Breakdown Structure

**Intent:** turn v0.3 design goals + sprint plan into an executable backlog with clear epics, features, demos, and acceptance criteria.

## Conventions

### GitHub issue taxonomy (Jira-ish)
Use **title prefixes** and **labels** together:

- **Epic:** `EPIC v0.3: <name>`  (label: `type:design`, `version:v0.3`, `track:roadmap`)
- **Feature:** `[v0.3][feature] <name>` (label: `type:enhancement`, `version:v0.3` + `area:*`)
- **Bug:** `[v0.3][bug] <name>` (label: `type:bug`, `version:v0.3` + `area:*`)
- **Chore:** `[v0.3][chore] <name>` (label: `type:chore`, `version:v0.3` + `area:*`)
- **Docs:** `[v0.3][docs] <name>` (label: `type:docs`, `version:v0.3` + `area:docs`)

**Areas:** `area:swarm`, `area:spec`, `area:tools`, `area:docs`, `area:tests`, `area:security`.

### Definition of Done (per issue)
Each issue should include:
- **Goal** (1–2 sentences)
- **Acceptance criteria** (bulleted)
- **Demo / command(s)** to prove it works
- **Tests** added/updated
- **Docs** updated (if user-facing)

### Sprint hygiene
Every sprint includes:
- a **review pass** (Codex-first + human)
- a **cleanup bucket** (branch hygiene, docs drift, examples drift)

---

## North-star demos (what v0.3 must *show*)

### Demo A — “Remote brain / local hands” (ADL orchestrates Codex-like flow)
A remote coordinator agent decomposes a task into small local tasks, executes them, and produces artifacts + trace.

**Target story:** “Plan, design, and build an MCP server for Apple `say` tool in ~5 minutes with ~80% coverage.”

**Proof:** one command runs the workflow and produces:
- `out/` artifacts (design doc, code, tests)
- a trace file
- a short human summary

### Demo B — Game Studio v2 (cooler game than RPS)
ADL generates a small HTML/JS game with assets and tests, auto-opens output.

**Proof:** `swarm demo ...` (or equivalent) opens the HTML and produces a shareable artifact bundle.

---

## EPIC 0 — Project scaffolding (v0.3 planning artifacts)
**Outcome:** docs and backlog stay coherent; WBS maps to sprint plan.

- [ ] **[v0.3][chore] Create/maintain v0.3 planning docs**
  - Files: `.adl/DESIGN_GOALS_v0.3.md`, `.adl/SPRINT_PLAN_v0.3.md`, `.adl/WBS_v0.3.md`, `.adl/DEMO_PLAYBOOK_v0.3.md`, `.adl/DECISIONS_v0.3.md`, `.adl/RELEASE_CHECKLIST_v0.3.md`, `.adl/REVIEW_CHECKLIST.md`
  - AC: docs exist, referenced from README or `.adl/README.md` (optional)

- [ ] **[v0.3][docs] Create and maintain v0.3 core documentation set**
  - Goal: establish authoritative design + planning docs for v0.3
  - AC:
    - DESIGN_GOALS_v0.3.md completed
    - SPRINT_PLAN_v0.3.md completed
    - WBS_v0.3.md reviewed and up to date
    - DEMO_PLAYBOOK_v0.3.md completed
    - DECISIONS_v0.3.md seeded with initial decisions
    - Docs referenced from repo README or `.adl/README.md`

---

## EPIC 1 — Demo UX & CLI ergonomics (make ADL irresistible)
**Outcome:** demos are one-command, quiet-by-default, file-based outputs, auto-open.

### Features
- [ ] **[v0.3][feature] One obvious happy-path demo command**
  - AC: `swarm demo <name>` (or `swarm run --demo <name>`) works with minimal flags
  - AC: prints a short summary + paths to artifacts

- [ ] **[v0.3][feature] Quiet-by-default + structured logs**
  - AC: default run output is readable; verbose logs only with `--verbose`
  - AC: trace file still emitted when requested

- [ ] **[v0.3][feature] Artifact output routing (per-step files)**
  - AC: step outputs written to `out/<run-id>/<step-id>/...`
  - AC: no awk/tee required

- [ ] **[v0.3][feature] Auto-open HTML artifacts**
  - AC: when output contains `index.html` (or declared artifact), `--open` opens browser
  - AC: safe behavior on CI/headless (no open)

### Demos / examples
- [ ] **[v0.3][chore] Upgrade game demo to “Game Studio v2”**
  - AC: generates a more interesting game than RPS (still safe/non-political)
  - AC: produces `out/.../game/index.html` and auto-open works

---

## EPIC 2 — Observability & trace as a product surface
**Outcome:** traces are human-friendly and machine-usable; replay/debug is easy.

### Features
- [ ] **[v0.3][feature] Human-readable trace timestamps + per-step durations**
  - AC: trace includes wall-clock timestamps + durations with sensible precision
  - AC: CLI summary prints total runtime and slowest steps

- [ ] **[v0.3][feature] Trace export formats**
  - AC: JSONL (event stream) is stable
  - AC: optional summarized JSON for UI consumption

- [ ] **[v0.3][feature] `swarm trace show` / `swarm trace summarize`**
  - AC: prints timeline table; highlights failures; points to artifacts

---

## EPIC 3 — Coordination patterns library (sequence / dag / debate)
**Outcome:** express coordination modes directly and run them with auto-advance.

### Features
- [ ] **[v0.3][feature] `sequence()` pattern**
  - AC: linear workflows expressible with minimal boilerplate

- [ ] **[v0.3][feature] `dag()` pattern (dependencies + joins)**
  - AC: parallel branches + join supported
  - AC: deterministic step ids preserved across plan/trace

- [ ] **[v0.3][feature] `debate()` pattern producing ADR artifact**
  - AC: outputs `adr.md` + `decision.json`
  - AC: supports roles + rubric; auto-synthesis when quorum met

### Examples
- [ ] **[v0.3][examples] One runnable example per pattern**
  - AC: each example runs with the happy-path demo command

---

## EPIC 4 — Runtime scheduling: auto-advance + deterministic execution
**Outcome:** runtime dispatches ready work automatically; deterministic behavior is enforced.

### Features
- [ ] **[v0.3][feature] Ready-queue scheduler**
  - AC: runnable steps automatically queued and executed
  - AC: supports basic max-parallel setting (even if concurrency is limited initially)

- [ ] **[v0.3][feature] Deterministic resolver guarantees**
  - AC: explicit step ids preserved
  - AC: stable ordering for plan printing and trace emission

- [ ] **[v0.3][bug] Fix placeholder state handoff in coordinator examples**
  - AC: example demonstrates real data flow (no literal placeholders)

---

## EPIC 5 — Security & gates (visible, lightweight, shippable)
**Outcome:** security posture is visible in runtime; gates are first-class steps.

### Features
- [ ] **[v0.3][feature] `quality_gate` step**
  - AC: runs fmt/clippy/test/coverage thresholds as configured

- [ ] **[v0.3][feature] `security_gate` step (minimal)**
  - AC: basic checks (e.g., deny network/tooling where prohibited, dependency advisories as optional)

- [ ] **[v0.3][docs] Document security model and defaults**

---

## EPIC 6 — Tooling: PR hygiene + offline workflows
**Outcome:** pr.sh accelerates, never derails; supports offline and bulk flows.

### Features
- [ ] **[v0.3][feature] pr.sh: offline card generation when --no-fetch-issue**
  - AC: can generate cards without GitHub API

- [ ] **[v0.3][feature] One-command preflight scripts**
  - AC: `./swarm/.local/preflight.sh` (or similar) runs fmt/clippy/test/schema/examples
  - AC: callable from CI and locally

- [ ] **[v0.3][chore] Branch hygiene scripts**
  - AC: prune merged branches locally + on origin safely

---

## EPIC 7 — Spec hygiene + layout
**Outcome:** spec artifacts are canonical, discoverable, and referenced consistently.

### Features
- [ ] **[v0.3][chore] Move schemas out of /examples into canonical location**
  - AC: update docs + tests + tooling refs

- [ ] **[v0.3][docs] Spec map / index**
  - AC: “where things live” documented

---

## Sprint mapping (proposed)

### Sprint 0.3-A (Demo-first)
- EPIC 1 (Demo UX) — MUST
- EPIC 2 (Trace UX) — MUST
- EPIC 0 (Scaffolding) — MUST
- EPIC 0: v0.3 docs complete and reviewed — MUST

### Sprint 0.3-B (Patterns + scheduler)
- EPIC 3 (Patterns) — MUST
- EPIC 4 (Auto-advance + determinism) — MUST

### Sprint 0.3-C (Hardening, if needed)
- EPIC 5 (Gates) — SHOULD
- EPIC 6/7 (tooling/spec hygiene) — SHOULD

---

## Backlog seeds (import from existing v0.2 review issues)
Some v0.2 review findings become v0.3 prerequisites/inputs:
- Determinism / step ids (carry into EPIC 4)
- Trace readability (carry into EPIC 2)
- Example state handoff (carry into EPIC 4)
- CLI/demo UX (carry into EPIC 1)

---

## Notes / decisions log pointers
Record key choices in `.adl/DECISIONS_v0.3.md`:
- CLI verb set (`run` vs `demo` vs `status/next/explain`)
- artifact directory structure
- trace schema stability promises
- pattern syntax (YAML shape) and version-gating