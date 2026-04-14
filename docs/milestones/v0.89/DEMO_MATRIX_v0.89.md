# Demo Matrix - v0.89

## Metadata
- Milestone: `v0.89`
- Version: `v0.89`
- Date: `2026-04-13`
- Owner: `Daniel Austin`
- Related issues / work packages: `WP-02` - `WP-13`

## Purpose

Define the canonical `v0.89` demo and proof program.

This matrix names the canonical proof rows, their milestone claims, and the expected proof surfaces so implementation can target them directly.

## Scope

In scope for `v0.89`:
- convergence behavior
- gate / decision / action proof surfaces
- experiment evidence and ObsMem explanation surfaces
- security / trust / posture walkthroughs

Out of scope for `v0.89`:
- the full `v0.89.1` adversarial runtime/demo package
- later signed-trace and reasoning-graph proof surfaces

## Runtime Preconditions

Working directory:

```bash
cd adl
```

Deterministic runtime / provider assumptions:

```bash
Prefer deterministic fixtures or bounded local/provider shims where possible.
Do not require private credentials for the main v0.89 proof rows unless the row is explicitly marked as live-only.
```

Additional environment / fixture requirements:
- use stable test fixtures where possible for convergence and gate behavior
- keep private-key or live-provider prerequisites out of the main proof row set unless explicitly justified

## Related Docs
- Design contract: `DESIGN_v0.89.md`
- WBS / milestone mapping: `WBS_v0.89.md`
- Sprint / execution plan: `SPRINT_v0.89.md`
- Release / checklist context: `MILESTONE_CHECKLIST_v0.89.md`
- Other proof-surface docs: `FEATURE_DOCS_v0.89.md`

## Demo Coverage Summary

| Demo ID | Demo title | Milestone claim / WP proved | Command entry point | Primary proof surface | Success signal | Determinism / replay note | Status |
|---|---|---|---|---|---|---|---|
| D1 | AEE convergence walkthrough | `WP-02` bounded convergence and stop conditions | planned `WP-02` entry point | convergence artifact + output record | reviewer can see converge / stall / bounded-out behavior | use deterministic fixtures for repeated stop-state verification | PLANNED |
| D2 | Freedom Gate v2 judgment demo | `WP-03` richer allow / defer / refuse / escalate behavior | `cargo test --manifest-path adl/Cargo.toml write_run_state_artifacts_projects_execute_owned_runtime_control_state -- --nocapture` | `learning/freedom_gate.v1.json` + `control_path/final_result.json` | reviewer can distinguish decision outcome, judgment boundary, and follow-up | stable fixtures should replay to the same outcome class and escalation path | READY |
| D3 | Decision + action mediation proof | `WP-04` - `WP-05` explicit choice and authorization boundary | planned `WP-04` / `WP-05` entry points | decision record + mediation artifact | reviewer can see model intent separated from authorized action | deterministic fixtures should preserve approval / rejection path | PLANNED |
| D4 | Skill invocation contract demo | `WP-06` bounded skill execution protocol | planned `WP-06` entry point | invocation artifact + trace | invocation lifecycle is reviewer-legible end to end | replay should preserve lifecycle structure | PLANNED |
| D5 | Experiment record demo | `WP-07` governed adopt / reject improvement behavior | planned `WP-07` entry point | experiment record artifact | reviewer can inspect baseline, variant, evidence, and decision | paired fixture runs should be stably comparable | PLANNED |
| D6 | ObsMem evidence and ranking walkthrough | `WP-08` explainable retrieval and ranking | planned `WP-08` entry point | retrieval explanation artifact | ranking cites evidence families and provenance | tie-break behavior should be stable under replay | PLANNED |
| D7 | Security / trust / posture walkthrough | `WP-09` main-band security contract | planned `WP-09` / `WP-11` review surface | reviewer-facing threat/posture/trust artifact set | reviewer can see explicit trust boundaries and declared posture | proof row may be document/artifact driven rather than fully executable | PLANNED |

Status guidance:
- `PLANNED` = intended but not yet validated
- `READY` = runnable and locally validated
- `BLOCKED` = known dependency or missing proof surface
- `LANDED` = milestone evidence exists and is ready for review

## Coverage Rules
- every major milestone claim should map to a runnable demo or an explicit alternate proof surface
- every demo should name one primary proof surface that a reviewer can inspect directly
- commands should become copy/paste-ready once the official issue wave lands
- success signals should describe what to inspect, not only process exit codes

## Demo Details

### D1) AEE convergence walkthrough

Description:
- demonstrate bounded convergence, stall, and bounded-out semantics
- show why another iteration is or is not justified

Milestone claims / work packages covered:
- `WP-02`
- bounded adaptive execution is a real surface, not a retry story

Commands to run:

```bash
Defined when the official `WP-02` issue opens and lands.
```

Expected artifacts:
- `control_path/convergence.json`
- `control_path/summary.txt`

Primary proof surface:
- `control_path/convergence.json` and the linked output record

Expected success signals:
- reviewer can distinguish progress vs repetition
- stop condition is explicit and justified

Determinism / replay notes:
- the same fixture should preserve the same convergence-state class

Reviewer checks:
- look for explicit convergence or stall reasoning
- verify that budget/policy stop states are visible

Known limits / caveats:
- command surface will be established by the official `WP-02` issue

---

### D2) Freedom Gate v2 judgment demo

Description:
- show a bounded governed judgment sequence with richer outcome classes

Milestone claims / work packages covered:
- `WP-03`

Commands to run:

```bash
cargo test --manifest-path adl/Cargo.toml freedom_gate -- --nocapture
cargo test --manifest-path adl/Cargo.toml write_run_state_artifacts_projects_execute_owned_runtime_control_state -- --nocapture
```

Expected artifacts:
- `learning/freedom_gate.v1.json`
- `control_path/decisions.json`
- `control_path/final_result.json`
- `control_path/summary.txt`

Primary proof surface:
- gate artifact and decision record pair, centered on `control_path/decisions.json`

Expected success signals:
- reviewer can see allow / defer / refuse / escalate distinctions
- reviewer can inspect `judgment_boundary`, `required_follow_up`, and deterministic rationale fields

Determinism / replay notes:
- stable fixtures should preserve outcome class, rationale shape, and escalation follow-up

Reviewer checks:
- verify the gate is a substrate boundary, not just prompt rhetoric

Known limits / caveats:
- richer moral/constitutional layers remain later-band work

## Cross-Demo Validation

Required baseline validation:

```bash
Defined by `WP-11` through `WP-13` as the demo and integration surfaces land.
```

Cross-demo checks:
- convergence claims use the same stop-state vocabulary as the feature docs and WBS
- gate / decision / action demos agree on outcome classes and authority boundaries
- security/trust/posture proof rows do not overclaim adversarial runtime work that belongs to `v0.89.1`

Failure policy:
- If one demo is blocked, record the blocker and say whether milestone review can proceed with an alternate proof surface.
- If deterministic behavior is expected but not observed, record the exact unstable artifact or command output.

## Determinism Evidence

Evidence directory / run root:
- established by the landed outputs of the official `WP-02` through `WP-13` issue wave (`#1789` through `#1800`)

Repeatability approach:
- prefer stable fixtures or bounded local/provider shims
- explain whether determinism means byte stability, stable ordering, or stable outcome class

Normalization rules:
- normalize timestamps, generated IDs, or provider-specific volatile fields when needed
- keep normalization explicit and reviewable rather than hidden in prose

Observed results summary:
- not yet available in this planning pass
- must be filled only from actual demo validation tied to the official implementation issue wave
- should distinguish runnable proof from alternate document/artifact proof

## Reviewer Sign-Off Surface

For each demo, the reviewer should be able to answer:
- What milestone claim does this demo prove?
- Which command should be run first?
- Which artifact or trace is the primary proof surface?
- What deterministic or replay guarantee is being claimed?
- What caveats or substitutions apply?

Review owners:
- Daniel Austin
- later third-party reviewer(s) as appropriate

Review status:
- planning pass complete; implementation and validation pending

## Notes
- this matrix is specific enough to seed demo issues without pretending the demos already exist
- `v0.89.1` adversarial runtime demos should not be silently folded into this matrix without an explicit scope decision

## Exit Criteria
- The milestone’s major claims are mapped to bounded demos or explicit alternate proof surfaces.
- Each demo has runnable commands, expected artifacts, and a clear success signal.
- Determinism / replay expectations are explicit where required.
- A reviewer can inspect the matrix and locate the primary proof surface for each demo without extra reconstruction work.
