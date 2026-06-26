# Cognitive Scheduler Runtime Advisory Proof for `#4544`

This packet records the bounded proof that `#4544` made the Cognitive
Scheduler operational for the v0.91.6 runtime lane without promoting it into
autonomous authority.

## What this proves

- `adl scheduler plan` is a first-class operator-facing CLI surface.
- the command accepts the tracked economics fixture and emits one deterministic
  scheduler plan artifact with schema `adl.scheduler.plan.v1`
- malformed fixture input fails non-zero with useful stderr diagnostics and no
  machine-readable stdout residue
- the scheduler remains advisory: it produces a plan artifact only and does not
  mutate GitHub, worktrees, providers, PRs, branches, or cloud resources
- the Soak #1 runtime packet consumes the scheduler artifact through
  `scheduler/scheduler_plan.json`

## Consumption handoff to Soak #1

The runtime soak generator now builds the scheduler plan from the tracked
economics fixture and writes it to:

- `docs/milestones/v0.91.6/review/runtime/v0916_integrated_runtime_soak_4543/scheduler/scheduler_plan.json`

The retained soak packet also cites that artifact in:

- `integrated_runtime_soak_proof.json`
- `integrated_runtime_soak_evidence_index.json`
- `completion_classification.json`

This is sufficient issue-scope proof that the scheduler artifact is no longer
component-only; the runtime umbrella can consume it as advisory evidence.

## Commands run

```bash
cargo test --manifest-path adl/Cargo.toml scheduler_plan -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_dispatch_exposes_help_and_version_without_csdlc_dispatch -- --nocapture
cargo test --manifest-path adl/Cargo.toml usage_mentions_v0_4_and_legacy_examples -- --nocapture
cargo run --quiet --manifest-path adl/Cargo.toml -- \
  scheduler plan --input adl/tests/fixtures/scheduler/economics_inputs_v1.json --json \
  >"$TMP/scheduler.stdout" 2>"$TMP/scheduler.stderr"
cargo run --quiet --manifest-path adl/Cargo.toml -- \
  scheduler plan --input "$TMP/bad.json" \
  >"$TMP/scheduler-bad.stdout" 2>"$TMP/scheduler-bad.stderr"
ADL_AWS_SIGNAL_MODE=mock ADL_AWS_REGION=us-west-2 ADL_AWS_SIGNAL_APPROVED=1 \
  ADL_AWS_HEARTBEAT_LOG_GROUP=/adl/mock \
  ADL_AWS_HEARTBEAT_LOG_STREAM=runtime-soak \
  cargo run --manifest-path adl/Cargo.toml --bin run_v0916_integrated_runtime_soak -- \
  --out docs/milestones/v0.91.6/review/runtime/v0916_integrated_runtime_soak_4543
```

## Output-channel proof

Successful scheduler execution wrote exactly one JSON line to stdout and wrote
only observability to stderr.

Malformed scheduler input wrote no stdout and returned the parse failure on
stderr, including the fixture validation detail:

- `failed to parse scheduler economics bundle`
- `source_doc_ref is required`

## Non-claims

- This packet does not claim the scheduler is an autonomous runtime conductor.
- This packet does not claim provider, GitHub, branch, or cloud mutation
  authority.
- This packet does not claim full v0.91.7 scheduling authority.
