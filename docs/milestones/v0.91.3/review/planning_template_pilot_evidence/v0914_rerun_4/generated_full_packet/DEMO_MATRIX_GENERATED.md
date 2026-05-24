<!--
Generated Planning Draft
planning_template_set: 1.0.0
template: demo_matrix
template_path: docs/templates/planning/1.0.0/demo_matrix.md
generation_status: generated_draft
claim_boundary: generated draft only; not reviewed or approved
-->

> Generated planning draft. This file proves only template filling;
> it is not reviewed, approved, released, merged, or lifecycle-true.
# v0.91.4 Demo Matrix

## Status

`draft pilot`

## Metadata
- Milestone: `v0.91.4`
- Version: `v0.91.4`
- Date: `2026-05-24`
- Owner: `ADL planning-template pilot`
- Related issues / work packages: v0.91.4 work package sequence

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

In scope for `v0.91.4`:
- C-SDLC milestone planning, routing, and proof surface.
- C-SDLC milestone planning, routing, and proof surface.
- C-SDLC milestone planning, routing, and proof surface.

Out of scope for `v0.91.4`:
- Publication or release approval without review evidence.
- Publication or release approval without review evidence.

## Runtime Preconditions

Working directory:

```bash
cd /path/to/agent-design-language
```

Deterministic runtime / provider assumptions:

```bash
Use documented repo commands and record provider/runtime exceptions explicitly.
```

Additional environment / fixture requirements:
- Python 3 and repository checkout.
- Python 3 and repository checkout.

## Related Docs
- Design contract: `docs/milestones/v0.91.4/DESIGN_v0.91.4.md`
- WBS / milestone mapping: `docs/milestones/v0.91.4/WBS_v0.91.4.md`
- Sprint / execution plan: `docs/milestones/v0.91.4/SPRINT_v0.91.4.md`
- Release / checklist context: `docs/milestones/v0.91.4/MILESTONE_CHECKLIST_v0.91.4.md`
- Other proof-surface docs: other related docs

## Demo Coverage Summary

Use this table as the fast review surface for milestone coverage.

| Demo ID | Demo title | Milestone claim / WP proved | Command entry point | Primary proof surface | Success signal | Determinism / replay note | Status |
|---|---|---|---|---|---|---|---|
| D1 | C-SDLC planning proof demo | Structural planning-template readiness. | `python3 adl/tools/validate_planning_template.py ...` | `Generated draft and validation output.` | Required sections present and placeholders resolved. | Same inputs produce same generated draft structure. | draft |
| D2 | C-SDLC planning proof demo | Structural planning-template readiness. | `python3 adl/tools/validate_planning_template.py ...` | `Generated draft and validation output.` | Required sections present and placeholders resolved. | Same inputs produce same generated draft structure. | draft |
| D3 | C-SDLC planning proof demo | Structural planning-template readiness. | `python3 adl/tools/validate_planning_template.py ...` | `Generated draft and validation output.` | Required sections present and placeholders resolved. | Same inputs produce same generated draft structure. | draft |

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

### D-pilot - C-SDLC planning proof demo

Description:
- Demonstrates generated planning docs against reviewed milestone docs.
- Demonstrates generated planning docs against reviewed milestone docs.

Milestone claims / work packages covered:
- Structural planning-template readiness.
- Structural planning-template readiness.

Commands to run:

```bash
python3 adl/tools/validate_planning_template.py ...
```

Expected artifacts:
- `Generated draft and validation output.`
- `Generated draft and validation output.`
- `Generated draft and validation output.`

Primary proof surface:
- `Generated draft and validation output.`

Secondary proof surfaces:
- `Generated draft and validation output.`
- `Generated draft and validation output.`

Expected success signals:
- Required sections present and placeholders resolved.
- Required sections present and placeholders resolved.

Determinism / replay notes:
- Same inputs produce same generated draft structure.
- Same inputs produce same generated draft structure.

Reviewer checks:
- Planning reviewer confirms no authoritative doc drift.
- Planning reviewer confirms no authoritative doc drift.

Known limits / caveats:
- caveat 1

---

### D-pilot - C-SDLC planning proof demo

Description:
- Demonstrates generated planning docs against reviewed milestone docs.

Milestone claims / work packages covered:
- Structural planning-template readiness.

Commands to run:

```bash
python3 adl/tools/validate_planning_template.py ...
```

Expected artifacts:
- `Generated draft and validation output.`
- `Generated draft and validation output.`

Primary proof surface:
- `Generated draft and validation output.`

Expected success signals:
- Required sections present and placeholders resolved.

Determinism / replay notes:
- Same inputs produce same generated draft structure.

Reviewer checks:
- Planning reviewer confirms no authoritative doc drift.

Known limits / caveats:
- caveat 2

---

### D-pilot - C-SDLC planning proof demo

Description:
- Demonstrates generated planning docs against reviewed milestone docs.

Milestone claims / work packages covered:
- Structural planning-template readiness.

Commands to run:

```bash
python3 adl/tools/validate_planning_template.py ...
```

Expected artifacts:
- `Generated draft and validation output.`

Primary proof surface:
- `Generated draft and validation output.`

Expected success signals:
- Required sections present and placeholders resolved.

Determinism / replay notes:
- Same inputs produce same generated draft structure.

Reviewer checks:
- Planning reviewer confirms no authoritative doc drift.

Known limits / caveats:
- caveat 3

## Cross-Demo Validation

Required baseline validation:

```bash
python3 adl/tools/validate_planning_template.py ...
```

Cross-demo checks:
- Compare generated output to existing v0.91.4 planning surface.
- Compare generated output to existing v0.91.4 planning surface.
- Compare generated output to existing v0.91.4 planning surface.

Failure policy:
- If one demo is blocked, record the blocker and say whether milestone review can proceed with an alternate proof surface.
- If deterministic behavior is expected but not observed, record the exact unstable artifact or command output.

## Determinism Evidence

Evidence directory / run root:
- `docs/milestones/v0.91.4/review/`

Repeatability approach:
- Same inputs produce same generated draft structure.
- Same inputs produce same generated draft structure.

Normalization rules:
- Same inputs produce same generated draft structure.
- Same inputs produce same generated draft structure.

Observed results summary:
- Same inputs produce same generated draft structure.
- Same inputs produce same generated draft structure.
- Same inputs produce same generated draft structure.

## Reviewer Sign-Off Surface

For each demo, the reviewer should be able to answer:
- What milestone claim does this demo prove?
- Which command should be run first?
- Which artifact or trace is the primary proof surface?
- What deterministic or replay guarantee is being claimed?
- What caveats or substitutions apply?

Review owners:
- Planning reviewer confirms no authoritative doc drift.
- Planning reviewer confirms no authoritative doc drift.

Review status:
- draft; requires reviewer sign-off

## Notes
- note 1
- note 2

## Exit Criteria
- The milestone’s major claims are mapped to bounded demos or explicit alternate proof surfaces.
- Each demo has runnable commands, expected artifacts, and a clear success signal.
- Determinism / replay expectations are explicit where required.
- A reviewer can inspect the matrix and locate the primary proof surface for each demo without extra reconstruction work.
