

# ADL v0.3 Sprint Plan

**Owner:** Daniel + Codex (local) + “remote brain” (this session)

**Goal:** Ship v0.3 as the first release that demonstrates **ADL as an orchestrator of local agents** with a **clean, delightful demo UX** and **deterministic, inspectable execution**.

**Primary north-star demos (must be runnable by strangers):**
1. **Game Studio Demo (OpenAI-recognizable):** Run one command → generates/updates a small HTML game → opens in browser.
2. **Remote brain / local hands Demo:** “Planner” agent decomposes a task and drives local agents (Codex) through smaller commits with trace + artifacts.

**Non-goals (explicitly out of scope for v0.3):**
- True distributed multi-host scheduling / federation
- Hard real-time concurrency model
- Enterprise auth, multi-tenant storage backends

---

## 0.3 Themes

- **Demo UX first:** the happy path must be a single obvious command.
- **Artifacts over terminal spam:** step outputs become files; terminal shows links/paths.
- **Determinism + explainability:** stable step ids, stable ordering, actionable errors.
- **Workflow patterns:** sequence / DAG / debate as *library-level* constructs with great examples.

---

## Sprint cadence

We keep weekly sprints with a review/cleanup pass baked in.

- **Sprint length:** 7 days
- **Ship target:** 2 sprints (v0.3.0), with an optional 3rd hardening sprint (v0.3.1)
- **Weekly ritual:** plan → build → test → doc → release notes → review → cleanup

---

## Sprint 0 (Day 0–1): Planning + scaffolding

**Objective:** lock the plan and create the scaffolding so we don’t churn.

**Deliverables:**
- [ ] `DESIGN_GOALS_v0.3.md` finalized
- [ ] `WBS_v0.3.md` finalized (epics + feature slices)
- [ ] `DEMO_PLAYBOOK_v0.3.md` has exact commands + expected outputs
- [ ] v0.3 milestone in GitHub + epics/issues created

**Acceptance:** docs exist and are referenced by issue templates/cards.

---

## Sprint 1 (Week 1): Demo UX + Artifacts + Trace polish

### Epic E1 — Demo UX
**Goal:** One command runs the demo and opens the generated game.

Deliverables:
- [ ] `swarm run <adl.yaml> --run` supports **writing step outputs to files** by default for demo workflows
- [ ] `--out-dir` (or similar) controls where artifacts are written
- [ ] `--open` (macOS) opens the primary HTML artifact in browser automatically
- [ ] Quiet default: terminal output is concise and human-readable (no giant JSON blobs)

Acceptance criteria:
- [ ] `swarm run swarm/examples/v0-3-game-studio.adl.yaml --run --open` results in:
  - [ ] `out/<run-id>/game.html` (or stable path)
  - [ ] browser opens the generated game
  - [ ] trace/artifact index exists: `out/<run-id>/index.json` (or `index.md`)

### Epic E2 — Trace + determinism polish
Deliverables:
- [ ] Per-step timestamps + durations are human-readable
- [ ] Stable step identifiers preserved through resolve/plan/trace
- [ ] Errors are actionable (what failed + where + what to do next)

Acceptance:
- [ ] A failed step prints: step id, agent, provider, artifact paths, and a minimal remediation hint.

### Sprint 1 checklist
- [ ] Implementation complete
- [ ] Examples updated
- [ ] Tests added/updated
- [ ] Docs updated (README + demo playbook)
- [ ] Release notes drafted
- [ ] Review + cleanup pass (see below)

---

## Sprint 2 (Week 2): Patterns library + “Remote brain / local hands” demo

### Epic E3 — Patterns library (sequence / DAG / debate)
Deliverables:
- [ ] Pattern specs or library helpers:
  - [ ] `sequence()`
  - [ ] `dag()`
  - [ ] `debate()` → emits ADR artifact
- [ ] Examples for each pattern:
  - [ ] `examples/v0-3/sequence-*.adl.yaml`
  - [ ] `examples/v0-3/dag-*.adl.yaml`
  - [ ] `examples/v0-3/debate-*.adl.yaml`

Acceptance:
- [ ] Each example has a single-line “run me” command and produces artifacts.

### Epic E4 — Remote brain / local hands workflow
**Goal:** demonstrate that ADL can *drive* local agents through decomposed tasks.

Deliverables:
- [ ] “Planner” step produces a structured plan artifact (JSON/MD)
- [ ] “Implementer” steps consume plan slices and produce commits/artifacts
- [ ] A “review gate” step checks formatting/tests and summarizes what changed

Acceptance:
- [ ] Demo playbook shows:
  - [ ] a single command that generates plan + code artifacts
  - [ ] a second command that executes the implementation steps (or a single run that does both)

### Sprint 2 checklist
- [ ] Implementation complete
- [ ] Tests added/updated
- [ ] Docs updated
- [ ] Release notes + changelog
- [ ] Review + cleanup pass

---

## Optional Sprint 3 (Hardening / v0.3.1): Quality gates + preflight scripts

Only if needed after review issues land.

Deliverables:
- [ ] `tools/preflight.sh` (fmt/clippy/test/schema/examples)
- [ ] Quality gate step type (minimal)
- [ ] CI tightened (fast feedback)

---

## Review + cleanup pass (every sprint)

**Codex-first review:**
- [ ] run tests locally + CI green
- [ ] skim diffs for correctness + DX
- [ ] open issues for findings with severity labels

**3rd-party review (lightweight):**
- [ ] request a short audit of demo UX + determinism + docs clarity

**Cleanup:**
- [ ] prune merged branches (local + origin)
- [ ] ensure examples run from a clean clone
- [ ] update release notes + README commands

---

## Issue taxonomy / conventions (GitHub)

We’ll use labels + issue titles to emulate Jira structure:

- **Epics:** title starts with `[epic v0.3] ...` and has task list linking child issues.
- **Features:** title starts with `[feature v0.3] ...`
- **Bugs:** title starts with `[bug] ...`
- **Chores:** title starts with `[chore] ...`

Required labels:
- `track:roadmap`
- `version:v0.3`
- `type:*` (design/enhancement/bug/chore/docs)
- `area:*` (swarm/spec/tools/docs/tests)
- optional `severity:*`

---

## Definition of Done for v0.3.0

- [ ] Game Studio demo: single command generates a working HTML game and opens it.
- [ ] Remote brain/local hands demo: plan → implement → review produces artifacts and trace.
- [ ] Trace is human-readable with per-step durations.
- [ ] Patterns examples exist for sequence/dag/debate.
- [ ] CI green + preflight runnable.
- [ ] Release notes written.