

# ADL / Swarm v0.3 — Design Goals and Planning Artifacts

**Status:** Draft (living document)

This document is the *source of truth* for v0.3 scope, priorities, and acceptance criteria.
It exists to keep the work honest: **project planning first, design second, coding third, testing fourth**.

---

## 1. One-sentence vision
ADL v0.3 turns Swarm into a **deterministic, contract-governed workflow engine** that can run recognizable coordination patterns (sequence/DAG/debate) with **quiet, file-based outputs**, strong traceability, and a demo that feels like "remote brain guiding local hands."

---

## 2. North Star demos (must stay non-controversial)
We will ship v0.3 with demos that are safe, short, and legible.

### Demo A — “Game Studio” (v0.3 canonical happy path)
**Goal:** One obvious command produces a playable HTML game and opens it.

**Definition of Done (Demo A):**
- Running the example produces **HTML artifact(s) written to disk** under an `out/` directory.
- Terminal output is **quiet** and human-readable (no walls of HTML).
- The workflow is deterministic enough that reruns have stable step ids and trace structure.
- Optional but preferred: automatically opens the generated HTML in the browser.

### Demo B — “Remote brain / local hands” (flow emulation)
**Goal:** Demonstrate how we work today: a remote coordinator agent plans and delegates to local agents/tools, producing traceable artifacts.

**Definition of Done (Demo B):**
- A coordinator workflow produces: `plan.md`, `design.md`, `implementation.md` or code artifacts, and `review.md`.
- The workflow demonstrates at least one "handoff" where output from one step is referenced as an input for another.
- The trace/plan preserves **explicit step ids** (deterministic).

**NOTE:** The MCP “say tool” demo is a *stretch target* for v0.3 and a *must* for v0.4 unless we find it’s trivial.

---

## 3. v0.3 themes

### Theme 1 — Determinism and Traceability
v0.3 must make it hard to "lose" what happened.

**Key outcomes:**
- Stable, explicit step ids preserved through resolve → plan → trace.
- Better timestamps/durations (human-friendly), with totals.
- Trace artifacts are usable without awk/terminal copying.

### Theme 2 — Coordination patterns as first-class UX
Borrow the useful mental model from `team-tasks` (linear/DAG/debate) but implement it as a real engine.

**Key outcomes:**
- Patterns are recognizable in examples and docs.
- CLI has a “small verb” feel where it helps (status/next/explain), but we keep scope realistic.

### Theme 3 — Demo UX and “one happy-path command”
If the demo is hard to run, ADL looks fake.

**Key outcomes:**
- One canonical command per demo.
- Quiet output by default.
- All artifacts are written to `out/` (or `.adl/runs/`), with paths printed.

---

## 4. Scope boundaries

### In-scope for v0.3
- Demo UX: write step outputs to files; quiet runs; optional auto-open for HTML artifacts.
- Determinism fixes: preserve explicit step ids; reject unknown YAML fields where appropriate.
- Trace UX: human readable timestamps + per-step durations.
- Spec hygiene: move schemas to canonical location; eliminate inconsistent terminology.
- Tooling hygiene: preflight/review scripts; branch hygiene docs/script.
- Coordination patterns: *documented + demonstrated* (even if implemented via existing multi-step semantics).

### Explicitly out-of-scope for v0.3 (v0.4+)
- True concurrency scheduling / race-safe parallel execution across steps.
- Multi-node distributed execution / remote executors.
- Cryptographic signing / attestation.
- Speculative execution.
- Full TUI/graph rendering.

---

## 5. Architecture notes (design constraints)

### Determinism rules
- Step ids in the authored YAML are authoritative.
- Derived ids (if any) must be stable, but preference is to require explicit ids for key steps.
- Plan printing and trace emission must use the same ids.

### Artifact rules
- Each step may produce one or more **named artifacts**.
- Artifacts must be persisted to a run directory.
- The CLI must print where artifacts went.

### Output channels
- **stdout/stderr**: reserved for human-readable status.
- **artifacts**: structured outputs, including generated HTML, logs, JSON traces.

---

## 6. Deliverables (documents + checklists)
These are the v0.3 planning artifacts we will maintain.

### A. Design artifacts (required)
- **This file:** `.adl/DESIGN_GOALS_v0.3.md`
- `.adl/SPRINT_PLAN_v0.3.md` — sprint breakdown and priorities
- `.adl/WBS_v0.3.md` — work breakdown structure (epics → issues)
- `.adl/DEMO_PLAYBOOK_v0.3.md` — exact commands + expected outputs for demos

### B. Engineering checklists (required)
- `.adl/RELEASE_CHECKLIST_v0.3.md`
- `.adl/REVIEW_CHECKLIST.md` — what we do for Codex-first + 3rd-party reviews

### C. Optional (nice-to-have)
- `.adl/DECISIONS_v0.3.md` — lightweight ADR log (1–2 paragraphs per decision)

---

## 7. Sprint checklist (repeat every sprint)
This is the invariant cadence.

### Sprint start
- [ ] Scope: choose epics/issues for the sprint (no creep)
- [ ] Confirm demo impact: does this sprint move a demo forward?
- [ ] Define acceptance criteria for each issue
- [ ] Ensure input cards exist for each issue

### During sprint
- [ ] Use `pr.sh` for every issue
- [ ] Keep PRs small and reviewable
- [ ] Keep CI green on main

### Sprint end (mandatory)
- [ ] Codex-first review pass + capture issues
- [ ] 3rd-party review pass + capture issues
- [ ] Branch hygiene: prune merged/stale branches
- [ ] Demo run-through: execute both demos from a clean checkout
- [ ] Release notes delta (what shipped / what deferred)

---

## 8. v0.3 Work Breakdown Structure (WBS)
Top-level epics for v0.3 (we will mirror these in GitHub).

### Epic E1 — Demo UX and artifacts
- Quiet run (no terminal spam)
- Write step outputs to files
- Auto-open HTML artifacts (when requested)
- One canonical happy-path command per demo

### Epic E2 — Determinism and schema hygiene
- Preserve explicit step ids end-to-end
- deny_unknown_fields for key specs
- Move schemas to canonical location (no schemas under examples)

### Epic E3 — Trace UX
- Human-friendly timestamps (calendar time)
- Per-step durations + total duration
- Better precision for sub-second steps

### Epic E4 — Tooling / repo hygiene
- One-command preflight/review script
- Branch hygiene script/docs
- Terminology cleanup (input/output, remove legacy terms)

### Epic E5 — Examples and docs
- Game Studio demo (canonical)
- Coordinator handoff demo
- Docs that explain how to run + what artifacts appear

---

## 9. Acceptance gates (how we decide v0.3 is shippable)

### Engineering gates
- [ ] CI green
- [ ] Tests updated/added for key fixes
- [ ] Examples run on a clean checkout

### Product gates
- [ ] Demo A produces a playable HTML game file and is easy to run
- [ ] Demo B demonstrates coordinator handoff and produces durable artifacts
- [ ] Running a demo does not require awk/terminal copy/paste

---

## 10. Open questions (to resolve early)
- Where is the canonical run output directory: `out/` vs `.adl/runs/`?
- Should auto-open be default for HTML artifacts, or gated by a flag?
- What’s the minimal artifact naming convention for step outputs?
