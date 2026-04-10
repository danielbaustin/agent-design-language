# Demo Matrix - v0.88

## Metadata
- Milestone: `v0.88`
- Version: `v0.88`
- Date: `TBD`
- Owner: `Daniel Austin`
- Related issues / work packages: TBD during v0.88 planning

## Purpose
Define the canonical milestone demo program: which bounded demos exist, which milestone claims they prove, how to run them, and what artifacts or proof surfaces reviewers should inspect.

## How To Use
- Use this document for runnable milestone evidence, not for broad feature brainstorming.
- Keep demo rows and per-demo sections aligned so a reviewer can move from summary -> execution -> proof surface without reconstructing context by hand.
- Prefer bounded, replayable, copy/paste-friendly commands over aspirational demo descriptions.
- If a milestone claim cannot yet be shown through a runnable demo, say so explicitly and record the substitute proof surface.
- Keep names stable across milestones where practical so comparisons remain easy.
- If a section is not relevant, include a one-line rationale instead of deleting it.

## Scope

In scope for `v0.88`:
- TBD during v0.88 planning
- TBD during v0.88 planning
- TBD during v0.88 planning

Out of scope for `v0.88`:
- TBD during v0.88 planning
- TBD during v0.88 planning

## Runtime Preconditions

Working directory:

```bash
TBD during v0.88 planning
```

Deterministic runtime / provider assumptions:

```bash
TBD during v0.88 planning
```

Additional environment / fixture requirements:
- TBD during v0.88 planning
- TBD during v0.88 planning

## Related Docs
- Design contract: `docs/milestones/v0.88/DESIGN_v0.88.md`
- WBS / milestone mapping: `docs/milestones/v0.88/WBS_v0.88.md`
- Sprint / execution plan: `docs/milestones/v0.88/SPRINT_v0.88.md`
- Release / checklist context: `docs/milestones/v0.88/MILESTONE_CHECKLIST_v0.88.md`
- Other proof-surface docs: TBD during v0.88 planning

## Demo Coverage Summary

Use this table as the fast review surface for milestone coverage.

| Demo ID | Demo title | Milestone claim / WP proved | Command entry point | Primary proof surface | Success signal | Determinism / replay note | Status |
|---|---|---|---|---|---|---|---|
| D1 | TBD during v0.88 planning | TBD during v0.88 planning | `TBD during v0.88 planning` | `TBD during v0.88 planning` | TBD during v0.88 planning | TBD during v0.88 planning | TBD during v0.88 planning |
| D2 | TBD during v0.88 planning | TBD during v0.88 planning | `TBD during v0.88 planning` | `TBD during v0.88 planning` | TBD during v0.88 planning | TBD during v0.88 planning | TBD during v0.88 planning |
| D3 | TBD during v0.88 planning | TBD during v0.88 planning | `TBD during v0.88 planning` | `TBD during v0.88 planning` | TBD during v0.88 planning | TBD during v0.88 planning | TBD during v0.88 planning |

Status guidance:
- `PLANNED` = intended but not yet validated
- `READY` = runnable and locally validated
- `BLOCKED` = known dependency or missing proof surface
- `LANDED` = milestone evidence exists and is ready for review

## Coverage Rules
- Every major milestone claim should map to a runnable demo or an explicit alternate proof surface.
- Every demo should name one primary proof surface that a reviewer can inspect directly.
- Commands should be copy/paste-ready and should not require private local state.
- Success signals should say what to check, not just “command exits 0”.
- Determinism / replay notes should explain how stability is judged.

## Demo Details

Repeat one block per demo in the coverage summary.

### D1) Demo TBD

Description:
- TBD during v0.88 planning
- TBD during v0.88 planning

Milestone claims / work packages covered:
- TBD during v0.88 planning
- TBD during v0.88 planning

Commands to run:

```bash
TBD during v0.88 planning
```

Expected artifacts:
- `TBD during v0.88 planning`
- `TBD during v0.88 planning`
- `TBD during v0.88 planning`

Primary proof surface:
- `TBD during v0.88 planning`

Secondary proof surfaces:
- `TBD during v0.88 planning`
- `TBD during v0.88 planning`

Expected success signals:
- TBD during v0.88 planning
- TBD during v0.88 planning

Determinism / replay notes:
- TBD during v0.88 planning
- TBD during v0.88 planning

Reviewer checks:
- TBD during v0.88 planning
- TBD during v0.88 planning

Known limits / caveats:
- TBD during v0.88 planning

---

### D2) Demo TBD

Description:
- TBD during v0.88 planning

Milestone claims / work packages covered:
- TBD during v0.88 planning

Commands to run:

```bash
TBD during v0.88 planning
```

Expected artifacts:
- `TBD during v0.88 planning`
- `TBD during v0.88 planning`

Primary proof surface:
- `TBD during v0.88 planning`

Expected success signals:
- TBD during v0.88 planning

Determinism / replay notes:
- TBD during v0.88 planning

Reviewer checks:
- TBD during v0.88 planning

Known limits / caveats:
- TBD during v0.88 planning

---

### D3) Demo TBD

Description:
- TBD during v0.88 planning

Milestone claims / work packages covered:
- TBD during v0.88 planning

Commands to run:

```bash
TBD during v0.88 planning
```

Expected artifacts:
- `TBD during v0.88 planning`

Primary proof surface:
- `TBD during v0.88 planning`

Expected success signals:
- TBD during v0.88 planning

Determinism / replay notes:
- TBD during v0.88 planning

Reviewer checks:
- TBD during v0.88 planning

Known limits / caveats:
- TBD during v0.88 planning

## Cross-Demo Validation

Required baseline validation:

```bash
TBD during v0.88 planning
```

Cross-demo checks:
- TBD during v0.88 planning
- TBD during v0.88 planning
- TBD during v0.88 planning

Failure policy:
- If one demo is blocked, record the blocker and say whether milestone review can proceed with an alternate proof surface.
- If deterministic behavior is expected but not observed, record the exact unstable artifact or command output.

## Determinism Evidence

Evidence directory / run root:
- `TBD during v0.88 planning`

Repeatability approach:
- TBD during v0.88 planning
- TBD during v0.88 planning

Normalization rules:
- TBD during v0.88 planning
- TBD during v0.88 planning

Observed results summary:
- TBD during v0.88 planning
- TBD during v0.88 planning
- TBD during v0.88 planning

## Reviewer Sign-Off Surface

For each demo, the reviewer should be able to answer:
- What milestone claim does this demo prove?
- Which command should be run first?
- Which artifact or trace is the primary proof surface?
- What deterministic or replay guarantee is being claimed?
- What caveats or substitutions apply?

Review owners:
- TBD during v0.88 planning
- TBD during v0.88 planning

Review status:
- TBD during v0.88 planning

## Notes
- TBD during v0.88 planning
- TBD during v0.88 planning

## Exit Criteria
- The milestone’s major claims are mapped to bounded demos or explicit alternate proof surfaces.
- Each demo has runnable commands, expected artifacts, and a clear success signal.
- Determinism / replay expectations are explicit where required.
- A reviewer can inspect the matrix and locate the primary proof surface for each demo without extra reconstruction work.
