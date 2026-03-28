# Demo Matrix — v0.86

## Metadata
- Milestone: `v0.86`
- Version: `0.86`
- Date: `2026-03-27`
- Owner: `adl`
- Related issues / work packages: `#882`, `WP-10`, `WP-11`

## Purpose
Define the canonical demo program for v0.86: which bounded local demos exist, which milestone claims they prove, how to run them, and which artifacts or proof surfaces reviewers should inspect.

This matrix is not a brainstorming surface. It is the bounded proof plan for the first working bounded cognitive system.

## How To Use
- Use this document for runnable milestone evidence, not for feature ideation.
- Keep the summary table and per-demo sections aligned so a reviewer can move from overview -> execution -> proof surface without reconstruction.
- Prefer bounded, local, replayable, copy/paste-friendly commands over aspirational demo language.
- If a demo is not yet implemented, mark it `PLANNED` and identify the substitute proof surface honestly.
- Keep names stable so later milestones can compare behavior against v0.86.

## Scope

In scope for `v0.86`:
- canonical bounded cognitive path demos
- arbitration and fast/slow routing demos
- bounded agency / candidate-selection demos
- bounded execution (AEE-lite) demos
- evaluation and termination demos
- frame adequacy and reframing demos
- memory participation (ObsMem-lite) demos
- Freedom Gate enforcement demos
- local review surfaces and artifact traces proving the full cognitive loop

Out of scope for `v0.86`:
- PHI / Φ_ADL metric demos
- AEE convergence demos
- richer affect modeling/demos, identity, governance, or signed-trace demos
- multi-agent social/governance demonstrations beyond bounded local review surfaces

## Runtime Preconditions

Working directory:

```bash
cd /Users/daniel/git/agent-design-language
```

Deterministic runtime / provider assumptions:

```bash
# expected local provider surface for v0.86 demos
ollama serve
# use fixed local model names/configuration documented by the demo scripts
```

Additional environment / fixture requirements:
- Ollama (or equivalent local inference provider) is installed and reachable.
- Demo scripts must use milestone-approved local models and fixed runtime defaults.
- Artifact output paths must be stable and documented by the scripts.

## Related Docs
- Design contract: `docs/milestones/v0.86/DESIGN_v0.86.md`
- WBS / milestone mapping: `docs/milestones/v0.86/WBS_v0.86.md`
- Sprint / execution plan: `docs/milestones/v0.86/SPRINT_v0.86.md`
- Release / checklist context: `docs/milestones/v0.86/RELEASE_PLAN_v0.86.md`, `docs/milestones/v0.86/MILESTONE_CHECKLIST_v0.86.md`
- Other proof-surface docs: `docs/milestones/v0.86/features/LOCAL_AGENT_DEMOS.md`

## Demo Coverage Summary

Use this table as the fast review surface for milestone coverage.

| Demo ID | Demo title | Milestone claim / WP proved | Command entry point | Primary proof surface | Success signal | Determinism / replay note | Status |
|---|---|---|---|---|---|---|---|
| D1 | Canonical Bounded Cognitive Path | `WP-13` canonical bounded cognitive path; integrated stack + loop + signals + arbitration + execution + evaluation + reframing + memory + Freedom Gate | `adl/tools/demo_v086_control_path.sh` | control-path artifact set + summary output | One run traverses signals -> candidate selection -> arbitration -> fast/slow path -> bounded execution -> evaluation -> reframing -> memory participation -> Freedom Gate -> final action/refusal | Fixed local model + stable artifact naming; reruns should preserve the same control-path shape | PLANNED |
| D2 | Fast vs Slow Routing | `WP-05`, `WP-06` arbitration and fast/slow reasoning paths | `adl/tools/demo_v086_fast_slow.sh` | routing decision artifacts for two scenarios | Simple task routes to fast path; complex/ambiguous task routes to slow path | Same scenarios and fixed local config should preserve route choice or explainable justification | PLANNED |
| D3 | Agency / Candidate Selection | `WP-07` bounded agency via candidate selection | `adl/tools/demo_v086_candidate_selection.sh` | candidate-set artifact + selected candidate record | Multiple candidates are generated and one is explicitly selected with rationale | Candidate count and selected candidate should be stable enough for review with fixed input | PLANNED |
| D4 | Freedom Gate Enforcement | `WP-12` Freedom Gate decision control | `adl/tools/demo_v086_freedom_gate.sh` | Freedom Gate decision event | At least one case is allowed and at least one case is deferred or refused | Same blocked/allowed scenarios should preserve gate outcome under fixed inputs | PLANNED |
| D5 | Full Review Surface Walkthrough | `WP-15`, `WP-16` local demo program + review surface | `adl/tools/demo_v086_review_surface.sh` | combined demo manifest / review guide / artifact directory | Reviewer can run one command and locate all primary proof surfaces | Artifact directory layout and manifest names must remain stable | PLANNED |

Status guidance:
- `PLANNED` = intended but not yet validated
- `READY` = runnable and locally validated
- `BLOCKED` = known dependency or missing proof surface
- `LANDED` = milestone evidence exists and is ready for review

## Coverage Rules
- Every major milestone claim should map to a runnable demo or an explicit alternate proof surface.
- Every demo should name one primary proof surface that a reviewer can inspect directly.
- Commands should be copy/paste-ready and should not require private local state beyond the documented local provider setup.
- Success signals should say what to check, not just “command exits 0”.
- Determinism / replay notes should explain how stability is judged.
- There must be at least one demo proving the full bounded cognitive loop end-to-end.

## Demo Details

### D1) Canonical Bounded Cognitive Path

Description:
- Proves that the first v0.86 bounded cognitive system executes end-to-end.
- Demonstrates the canonical path rather than isolated subsystem behavior.

Milestone claims / work packages covered:
- WP-02 Cognitive Stack Canonicalization
- WP-03 Cognitive Loop Canonicalization
- WP-04 Cognitive Signals
- WP-05 Cognitive Arbitration
- WP-06 Fast / Slow Thinking Paths
- WP-07 Agency and Candidate Selection
- WP-08 Bounded Execution
- WP-09 Evaluation Signals and Termination
- WP-10 Frame Adequacy and Reframing
- WP-11 Memory Participation
- WP-12 Freedom Gate
- WP-13 Canonical Bounded Cognitive Path

Commands to run:

```bash
./adl/tools/demo_v086_control_path.sh
```

Expected artifacts:
- `artifacts/v086/control_path/candidate_selection.json`
- `artifacts/v086/control_path/arbitration.json`
- `artifacts/v086/control_path/freedom_gate.json`
- `artifacts/v086/control_path/final_result.json`
- `artifacts/v086/control_path/summary.txt`
- artifacts/v086/control_path/signals.json
- artifacts/v086/control_path/execution_iterations.json
- artifacts/v086/control_path/evaluation.json
- artifacts/v086/control_path/reframing.json
- artifacts/v086/control_path/memory.json

Primary proof surface:
- `artifacts/v086/control_path/summary.txt`

Secondary proof surfaces:
- `artifacts/v086/control_path/arbitration.json`
- `artifacts/v086/control_path/freedom_gate.json`

Expected success signals:
- Signals (instinct/affect) are emitted and visible
- Candidate selection occurs before execution
- Arbitration selects a reasoning path
- Bounded execution performs at least one iteration
- Evaluation signals affect behavior or termination
- Reframing/adaptation occurs in at least one case
- Memory participation is visible
- Freedom Gate decision occurs before final action

Determinism / replay notes:
- Replay stability is judged by control-path shape and artifact structure, not byte-for-byte natural-language identity.
- If route choice or gate decision changes under fixed inputs, the instability must be documented.

Reviewer checks:
- Is the candidate-selection step explicit and inspectable?
- Does arbitration clearly choose a path?
- Does the Freedom Gate clearly allow, defer, or refuse?

Known limits / caveats:
- This demo proves the v0.86 bounded cognitive system only; it does not prove later-milestone affect, PHI, or identity behavior.

---

### D2) Fast vs Slow Routing

Description:
- Demonstrates that arbitration meaningfully distinguishes between fast-path and slow-path reasoning.
- Uses bounded local tasks rather than open-ended workloads.

Milestone claims / work packages covered:
- `WP-05` Cognitive Arbitration
- `WP-06` Fast / Slow Thinking Paths

Commands to run:

```bash
./adl/tools/demo_v086_fast_slow.sh
```

Expected artifacts:
- `artifacts/v086/fast_slow/simple_case.json`
- `artifacts/v086/fast_slow/complex_case.json`
- `artifacts/v086/fast_slow/comparison.txt`

Primary proof surface:
- `artifacts/v086/fast_slow/comparison.txt`

Secondary proof surfaces:
- `artifacts/v086/fast_slow/simple_case.json`
- `artifacts/v086/fast_slow/complex_case.json`

Expected success signals:
- The simple case selects the fast path.
- The complex or ambiguous case selects the slow path.
- The comparison output explains the difference in control behavior.

Determinism / replay notes:
- Route choice should be stable under fixed tasks, model, and prompt contract.
- If the same task flips between fast and slow across reruns, the demo is not review-ready.

Reviewer checks:
- Are both execution modes genuinely distinct?
- Is the route decision visible and justified?

Known limits / caveats:
- This demo proves bounded routing, not full performance optimization.

---

### D3) Agency / Candidate Selection

Description:
- Shows that the system generates and selects among bounded alternatives.
- Demonstrates agency as structured selection, not as free-form rhetoric.

Milestone claims / work packages covered:
- `WP-07` Agency and Candidate Selection

Commands to run:

```bash
./adl/tools/demo_v086_candidate_selection.sh
```

Expected artifacts:
- `artifacts/v086/candidate_selection/candidates.json`
- `artifacts/v086/candidate_selection/selection.json`
- `artifacts/v086/candidate_selection/summary.txt`

Primary proof surface:
- `artifacts/v086/candidate_selection/selection.json`

Secondary proof surfaces:
- `artifacts/v086/candidate_selection/candidates.json`
- `artifacts/v086/candidate_selection/summary.txt`

Expected success signals:
- More than one candidate is produced.
- One candidate is selected explicitly with rationale.

Determinism / replay notes:
- Reviewability depends on stable candidate-set structure and stable winner selection for fixed input.
- If only one candidate appears, this demo fails its purpose even if the command exits successfully.

Reviewer checks:
- Is there a real alternative set?
- Is the selection step explicit rather than implied?

Known limits / caveats:
- Candidate scoring may be heuristic in v0.86; the milestone claim is bounded inspectable choice, not perfect optimization.

---

### D4) Freedom Gate Enforcement

Description:
- Proves that the Freedom Gate can constrain action after arbitration and before execution.
- Demonstrates at least one allowed case and one blocked/deferred case.

Milestone claims / work packages covered:
- `WP-12` Freedom Gate (v0.86 minimal)

Commands to run:

```bash
./adl/tools/demo_v086_freedom_gate.sh
```

Expected artifacts:
- `artifacts/v086/freedom_gate/allowed_case.json`
- `artifacts/v086/freedom_gate/blocked_case.json`
- `artifacts/v086/freedom_gate/summary.txt`

Primary proof surface:
- `artifacts/v086/freedom_gate/blocked_case.json`

Secondary proof surfaces:
- `artifacts/v086/freedom_gate/allowed_case.json`
- `artifacts/v086/freedom_gate/summary.txt`

Expected success signals:
- At least one candidate action is allowed.
- At least one candidate action is deferred or refused.
- The gate decision includes an explicit reason.

Determinism / replay notes:
- The same fixed blocked/allowed scenarios must preserve their gate outcomes under replay.
- If the gate flips under fixed conditions, the policy surface is not stable enough for review.

Reviewer checks:
- Is the gate decision explicit and structured?
- Is there a meaningful blocked/deferred case, not only a happy-path allow?

Known limits / caveats:
- v0.86 proves the minimal decision boundary only; richer policy systems are deferred.

---

### D5) Full Review Surface Walkthrough

Description:
- Provides one obvious reviewer entry point for the milestone.
- Bundles the canonical demo surfaces and points to the primary artifacts.

Milestone claims / work packages covered:
- WP-15 Local Agent Demo Program
- WP-16 Demo Matrix and Review Surface

Commands to run:

```bash
./adl/tools/demo_v086_review_surface.sh
```

Expected artifacts:
- `artifacts/v086/review_surface/demo_manifest.json`
- `artifacts/v086/review_surface/README.txt`
- `artifacts/v086/review_surface/index.txt`

Primary proof surface:
- `artifacts/v086/review_surface/demo_manifest.json`

Secondary proof surfaces:
- `artifacts/v086/review_surface/README.txt`
- `docs/milestones/v0.86/DEMO_MATRIX_v0.86.md`

Expected success signals:
- One command gives the reviewer a stable starting point.
- The manifest points to all primary demo artifacts.

Determinism / replay notes:
- The manifest layout and referenced artifact names must remain stable.
- This demo is review-oriented; it does not replace subsystem correctness checks.

Reviewer checks:
- Can a reviewer find the right demo and proof artifact quickly?
- Does the manifest match the matrix and the actual artifacts?

Known limits / caveats:
- This is a review-surface demo, not a separate cognitive subsystem.

## Cross-Demo Validation

Required baseline validation:

```bash
./adl/tools/demo_v086_control_path.sh
./adl/tools/demo_v086_fast_slow.sh
./adl/tools/demo_v086_candidate_selection.sh
./adl/tools/demo_v086_freedom_gate.sh
./adl/tools/demo_v086_review_surface.sh
```

Cross-demo checks:
- Artifact names and directory layout are consistent across demos.
- Route naming and gate-decision vocabulary match the design and WBS docs.
- The review-surface manifest points to real artifacts, not placeholders.

Failure policy:
- If one demo is blocked, record the blocker and say whether milestone review can proceed with an alternate proof surface.
- If deterministic behavior is expected but not observed, record the exact unstable artifact or command output.
- If the integrated control-path demo is blocked, milestone review cannot claim full v0.86 bounded cognitive system proof.

## Determinism Evidence

Evidence directory / run root:
- `artifacts/v086/`

Repeatability approach:
- Run demos with fixed local model names and documented provider configuration.
- Judge repeatability primarily by control-path shape, artifact structure, and decision outputs.

Normalization rules:
- Ignore nondeterministic natural-language wording when the control path and artifact structure remain stable.
- Treat path selection, gate outcomes, and missing artifacts as non-normalizable failures.

Observed results summary:
- `PLANNED` until local validation is complete.
- Upgrade to `READY` only after bounded reruns preserve the claimed proof surface.
- Upgrade to `LANDED` only when the milestone evidence is ready for review.

## Reviewer Sign-Off Surface

For each demo, the reviewer should be able to answer:
- What milestone claim does this demo prove?
- Which command should be run first?
- Which artifact or trace is the primary proof surface?
- What deterministic or replay guarantee is being claimed?
- What caveats or substitutions apply?

Review owners:
- `Daniel Austin`
- `Internal / external review owners to be assigned during review kickoff`

Review status:
- Planned for v0.86 implementation and review cycle.

## Notes
- The local demo program is a first-class feature for v0.86, not optional polish.
- This matrix must remain faithful to the corrected v0.86 planning docs and must not silently reintroduce moved work from later milestones.

## Exit Criteria
- The milestone’s major claims are mapped to bounded demos or explicit alternate proof surfaces.
- Each demo has runnable commands, expected artifacts, and a clear success signal.
- Determinism / replay expectations are explicit where required.
- A reviewer can inspect the matrix and locate the primary proof surface for each demo without extra reconstruction work.
- The matrix remains aligned with the design, WBS, sprint plan, and local demo feature doc.
