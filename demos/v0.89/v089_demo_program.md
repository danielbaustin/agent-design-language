# v0.89 Demo Program

`v0.89` uses a mixed proof model:
- bounded runnable proof rows where the runtime already exposes stable entrypoints
- reviewer-legible document rows where the milestone claim is deliberately design-contract work

The canonical scaffolding command is:

```bash
bash adl/tools/demo_v089_proof_entrypoints.sh
```

That command writes:
- `artifacts/v089/proof_entrypoints/demo_manifest.json`
- `artifacts/v089/proof_entrypoints/README.md`
- `artifacts/v089/proof_entrypoints/index.txt`

Use those artifacts as the reviewer entry surface for D1 through D7.

If you want the integrated reviewer package instead of the row-by-row entry map:

```bash
bash adl/tools/demo_v089_review_surface.sh
```

That review surface assembles:
- `artifacts/v089/review_surface/demo_manifest.json`
- `artifacts/v089/review_surface/claim_matrix.md`
- `artifacts/v089/review_surface/proof_entrypoints/demo_manifest.json`

## Demo Order

### D1) AEE convergence walkthrough

Command:

```bash
cargo test --manifest-path adl/Cargo.toml write_run_state_artifacts_projects_execute_owned_runtime_control_state -- --nocapture
```

Primary proof:
- `control_path/convergence.json`

Focus:
- named convergence state
- stop-condition family
- progress signal

### D2) Freedom Gate v2 judgment demo

Command:

```bash
cargo test --manifest-path adl/Cargo.toml write_run_state_artifacts_projects_execute_owned_runtime_control_state -- --nocapture
```

Primary proof:
- `learning/freedom_gate.v1.json`

Focus:
- allow / defer / refuse / escalate boundary
- required follow-up and blocked commitment semantics

### D3) Decision + action mediation proof

Command:

```bash
cargo test --manifest-path adl/Cargo.toml write_run_state_artifacts_projects_execute_owned_runtime_control_state -- --nocapture
```

Primary proof:
- `control_path/decisions.json`

Supporting proofs:
- `control_path/action_proposals.json`
- `control_path/mediation.json`

Focus:
- route selection
- reframing
- commitment gate
- model-proposes/runtime-decides boundary

### D4) Skill invocation contract demo

Command:

```bash
cargo test --manifest-path adl/Cargo.toml cli_artifact_validate_control_path_ -- --nocapture
```

Primary proof:
- `control_path/skill_model.json`

Supporting proofs:
- `control_path/skill_execution_protocol.json`

Focus:
- selected skill identity
- lifecycle state before execution
- authorization and trace expectations

### D5) Godel experiment package demo

Command shape:

```bash
tmp_root="$(mktemp -d)"
cargo run --manifest-path adl/Cargo.toml -- godel run --run-id run-745-a --workflow-id wf-godel-loop --failure-code tool_failure --failure-summary "step failed with deterministic parse error" --evidence-ref runs/run-745-a/run_status.json --evidence-ref runs/run-745-a/logs/activation_log.json --runs-dir "$tmp_root"
cargo run --manifest-path adl/Cargo.toml -- godel inspect --run-id run-745-a --runs-dir "$tmp_root"
```

Primary proof:
- `runs/<run-id>/godel/experiment_record.v1.json`

Focus:
- canonical experiment record
- baseline / variant pairing
- explicit adopt / reject decision semantics

### D6) ObsMem evidence and ranking walkthrough

Command:

```bash
cargo run --manifest-path adl/Cargo.toml -- demo demo-f-obsmem-retrieval --run --trace --out ./out
```

Primary proof:
- `obsmem_retrieval_result.json`

Focus:
- explicit ranking explanation
- provenance families
- deterministic tie-break behavior

### D7) Security / trust / posture walkthrough

Review surfaces:
- `docs/milestones/v0.89/features/SECURITY_AND_THREAT_MODELING.md`
- `docs/milestones/v0.89/features/ADL_SECURITY_POSTURE_MODEL.md`
- `docs/milestones/v0.89/features/ADL_TRUST_MODEL_UNDER_ADVERSARY.md`

Focus:
- threat boundaries
- declared posture classes
- trust assumptions under contest

Important boundary:
- D7 is a bounded reviewer-facing documentation proof row for `v0.89`
- it is not the live adversarial runtime/demo package planned for `v0.89.1`

## Reviewer Notes

- D1, D2, and D3 intentionally share one bounded runtime-control artifact set.
- D4 validates the skill-contract layer on top of that control-path family.
- D5 and D6 are the main runnable reviewer rows for experiment evidence and retrieval explanation.
- D7 is the correct stop point for the main-band security story before `v0.89.1`.
