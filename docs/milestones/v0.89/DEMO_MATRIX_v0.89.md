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

## Demo Taxonomy

Use these categories consistently during `v0.89`:

- Ordinary demos:
  bounded runnable proof rows intended for milestone demo sweeps.
- Heavyweight proof packages:
  integrated reviewer, quality-gate, or release-tail surfaces that may still be canonical proof, but should not be treated like quick demos.

For `v0.89`, rows `D1` through `D6` are expected to behave like ordinary demo rows.
Rows `D7` and `D11` are heavier reviewer-facing proof rows and may remain artifact or document driven even when they are complete.

Future quality-gate or release-review packages for `v0.89` should be classified as heavyweight proof packages, not ordinary demos.

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
| D1 | AEE convergence walkthrough | `WP-02` bounded convergence and stop conditions | `cargo test --manifest-path adl/Cargo.toml write_run_state_artifacts_projects_execute_owned_runtime_control_state -- --nocapture` | `control_path/convergence.json` + `control_path/summary.txt` | reviewer can see converge / stall / bounded-out behavior and named stop-family reasoning | use deterministic runtime-control fixtures; D1 shares the bounded runtime-control proof run with D2 and D3 | LANDED |
| D2 | Freedom Gate v2 judgment demo | `WP-03` richer allow / defer / refuse / escalate behavior | `cargo test --manifest-path adl/Cargo.toml write_run_state_artifacts_projects_execute_owned_runtime_control_state -- --nocapture` | `learning/freedom_gate.v1.json` + `control_path/final_result.json` | reviewer can distinguish decision outcome, judgment boundary, and follow-up | stable fixtures should replay to the same outcome class and escalation path | LANDED |
| D3 | Decision + action mediation proof | `WP-04` - `WP-05` explicit choice and authorization boundary | `cargo test --manifest-path adl/Cargo.toml write_run_state_artifacts_projects_execute_owned_runtime_control_state -- --nocapture` | `control_path/decisions.json` + `control_path/action_proposals.json` + `control_path/mediation.json` | reviewer can see route selection, reframing, and commitment-gate outcomes separated from authorized runtime action | deterministic fixtures should preserve outcome classes and mediation follow-up from the shared runtime-control proof run | LANDED |
| D4 | Skill invocation contract demo | `WP-06` bounded skill execution protocol | `cargo test --manifest-path adl/Cargo.toml cli_artifact_validate_control_path_ -- --nocapture` | `control_path/skill_model.json` + `control_path/skill_execution_protocol.json` + `control_path/summary.txt` | reviewer can distinguish a selected governed skill from other action kinds and inspect the pre-execution authorization lifecycle end to end | deterministic fixture replay should preserve lifecycle state, authorization outcome, and trace expectation | LANDED |
| D5 | Godel experiment package demo | `WP-07` governed adopt / reject improvement behavior | `cargo run --manifest-path adl/Cargo.toml -- godel run ...` then `cargo run --manifest-path adl/Cargo.toml -- godel inspect ...` | `runs/<run-id>/godel/experiment_record.v1.json` + `evaluation_plan.v1.json` | reviewer can inspect baseline / variant pairing, canonical evidence, bounded mutation, and adopt / reject decision from one bounded summary | identical bounded inputs should preserve stage order, canonical artifact paths, and decision class | LANDED |
| D6 | ObsMem evidence and ranking walkthrough | `WP-08` explainable retrieval and ranking | `cargo run --manifest-path adl/Cargo.toml -- demo demo-f-obsmem-retrieval --run --trace --out ./out` | `obsmem_retrieval_result.json` | ranking cites evidence families, status signals, and deterministic tie-break values | identical demo inputs should preserve result order and explanation shape | LANDED |
| D7 | Security / trust / posture walkthrough | `WP-09` main-band security contract | `cargo test --manifest-path adl/Cargo.toml write_run_state_artifacts_projects_execute_owned_runtime_control_state -- --nocapture`, then review `docs/milestones/v0.89/features/SECURITY_AND_THREAT_MODELING.md`, `docs/milestones/v0.89/features/ADL_SECURITY_POSTURE_MODEL.md`, and `docs/milestones/v0.89/features/ADL_TRUST_MODEL_UNDER_ADVERSARY.md` together | `control_path/security_review.json` + `control_path/summary.txt` + reviewer-facing threat/posture/trust document set | reviewer can see explicit trust boundaries, declared posture, contested trust state, and contested-operation trust assumptions without overclaiming `v0.89.1` adversarial runtime work | proof row is artifact-led with supporting reviewer docs and should replay to the same posture / trust classification for the same bounded fixture | LANDED |
| D11 | Quality-gate walkthrough | `WP-14` reviewer-facing quality / coverage / proof-package gate | `bash adl/tools/demo_v089_quality_gate.sh` | `artifacts/v089/quality_gate/quality_gate_record.json` + `artifacts/v089/quality_gate/README.md` | reviewer can inspect one bounded record that captures the local quality suite, coverage gate posture, proof-package checks, and maintainability-watch output with per-check logs | repeated runs should preserve the same check inventory and explicit PASS / FAIL structure for the same repo state | PLANNED |

Status guidance:
- `PLANNED` = intended but not yet validated
- `READY` = runnable and locally validated
- `BLOCKED` = known dependency or missing proof surface
- `LANDED` = milestone evidence exists and is ready for review

Current convergence note:
- rows `D1` through `D7` now have landed milestone evidence from `WP-02` - `WP-09`
- row `D11` is the explicit quality-gate walkthrough proof row for `WP-14`
- the remaining proof-wave work is now about convergence, packaging, quality, and review rather than a missing main-band security row

Heavyweight proof-package rule:
- if a proof surface mainly exists to bundle review, release, or quality-gate evidence, classify it as a heavyweight proof package even if it is script-driven
- do not silently fold heavyweight proof packages into ordinary demo sweeps without saying so explicitly

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
cargo test --manifest-path adl/Cargo.toml write_run_state_artifacts_projects_execute_owned_runtime_control_state -- --nocapture
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
- use deterministic runtime-control fixtures; the same fixture should preserve the same convergence-state class

Reviewer checks:
- look for explicit convergence or stall reasoning
- verify that budget/policy stop states are visible

Known limits / caveats:
- this row proves bounded convergence artifacts, not the later adversarial runtime band

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

---

### D5) Godel experiment package demo

Description:
- run the bounded Godel stage loop and inspect the resulting canonical experiment package
- show that adopt / reject behavior is recorded as explicit experiment evidence rather than
  narrative inference

Milestone claims / work packages covered:
- `WP-07`

Commands to run:

```bash
tmp_root="$(mktemp -d)"
cargo run --manifest-path adl/Cargo.toml -- godel run --run-id run-745-a --workflow-id wf-godel-loop --failure-code tool_failure --failure-summary "step failed with deterministic parse error" --evidence-ref runs/run-745-a/run_status.json --evidence-ref runs/run-745-a/logs/activation_log.json --runs-dir "$tmp_root"
cargo run --manifest-path adl/Cargo.toml -- godel inspect --run-id run-745-a --runs-dir "$tmp_root"
```

Expected artifacts:
- `runs/run-745-a/godel/evaluation_plan.v1.json`
- `runs/run-745-a/godel/mutation.v1.json`
- `runs/run-745-a/godel/canonical_evidence_view.v1.json`
- `runs/run-745-a/godel/experiment_record.v1.json`
- `runs/run-745-a/godel/experiment_record.runtime.v1.json`

Primary proof surface:
- `godel inspect` summary paired with `runs/run-745-a/godel/experiment_record.v1.json`

Expected success signals:
- reviewer can inspect canonical experiment ID, evidence view ID, mutation ID, and evaluation plan
  ID directly from the bounded inspect output
- reviewer can see baseline / variant pairing and final canonical decision without reconstructing
  the package by hand

Determinism / replay notes:
- identical bounded inputs should preserve the same stage order, artifact paths, and canonical
  decision class

Reviewer checks:
- verify that the canonical package is visible alongside the runtime stage-loop artifacts
- verify that the inspect surface exposes the canonical decision and paired run context directly

Known limits / caveats:
- this slice proves bounded experiment packaging and decision reviewability, not full later-band
  multi-run optimization or open-ended self-modification

---

### D6) ObsMem evidence and ranking walkthrough

Description:
- run the bounded ObsMem retrieval demo and inspect the resulting explanation-bearing retrieval
  artifact
- show that retrieval ordering is not just deterministic, but reviewer-legible through explicit
  provenance, status, and tie-break signals

Milestone claims / work packages covered:
- `WP-08`

Commands to run:

```bash
cargo run --manifest-path adl/Cargo.toml -- demo demo-f-obsmem-retrieval --run --trace --out ./out
```

Expected artifacts:
- `runs/_shared/obsmem_store.v1.json`
- `obsmem_retrieval_result.json`
- `runs/demo-f-run-a/godel/experiment_record.runtime.v1.json`
- `runs/demo-f-run-a/godel/obsmem_index_entry.runtime.v1.json`
- `runs/demo-f-run-b/godel/experiment_record.runtime.v1.json`
- `runs/demo-f-run-b/godel/obsmem_index_entry.runtime.v1.json`
- `runs/demo-f-run-c/godel/experiment_record.runtime.v1.json`
- `runs/demo-f-run-c/godel/obsmem_index_entry.runtime.v1.json`

Primary proof surface:
- `obsmem_retrieval_result.json`

Expected success signals:
- reviewer can inspect the normalized query, policy order, and returned hit set directly
- each explained result records prior vs effective score, provenance families, and deterministic
  tie-break values
- equal-ranked successful hits preserve lexical tie-break order while failed hits are explicitly
  penalized

Determinism / replay notes:
- identical demo inputs should preserve the same run ordering and explanation shape across repeated
  runs

Reviewer checks:
- verify that provenance is surfaced as named families rather than buried inside raw citations
- verify that `status_success_boost` / `status_failure_penalty` and query-tag overlap signals are
  visible in the explanation surface
- verify that the tie-break values explain why equal-ranked hits remain ordered predictably

Known limits / caveats:
- this slice proves bounded retrieval explanation and ranking legibility, not the later full
  memory-architecture or identity-linked semantics work

## Cross-Demo Validation

Required baseline validation:

```bash
bash adl/tools/demo_v089_proof_entrypoints.sh
bash adl/tools/demo_v089_review_surface.sh
```

Cross-demo checks:
- convergence claims use the same stop-state vocabulary as the feature docs and WBS
- gate / decision / action demos agree on outcome classes and authority boundaries
- security/trust/posture proof rows do not overclaim adversarial runtime work that belongs to `v0.89.1`
- heavyweight proof packages remain clearly separated from ordinary demos in milestone guidance and review notes
- the integrated review surface preserves the same D1-D7 claim mapping as the lighter proof-entrypoint manifest

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
