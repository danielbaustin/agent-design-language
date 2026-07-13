# Unified Runtime Kernel Integration Proof for #5097

Issue: `#5097 [v0.91.7][runtime][unification] Implement unified runtime kernel integration path`

## Outcome

#5097 adds a bounded unified runtime-kernel proof path that composes the live CSM current-runtime substrate with the Runtime v2 contract surfaces already used for lifecycle, standing, scheduler/provider selection, ACIP, memory, resilience, continuity, and observability.

The executable entrypoint is:

```sh
adl runtime-v2 unified-runtime-kernel --out docs/milestones/v0.91.7/review/runtime/unified_kernel_5097/evidence
```

For PR evidence, the retained artifact root is:

```text
docs/milestones/v0.91.7/review/runtime/unified_kernel_5097/evidence
```

The ignored `artifacts/` path is useful for local scratch runs only. The tracked milestone evidence root above is the durable review surface for this issue.

## Implementation Surface

- `adl/src/runtime_v2/unified_runtime_kernel.rs`
- `adl/src/runtime_v2/tests/unified_runtime_kernel.rs`
- `adl/src/runtime_v2/mod.rs`
- `adl/src/runtime_v2/contracts.rs`
- `adl/src/runtime_v2/tests.rs`
- `adl/src/cli/runtime_v2_cmd/commands.rs`
- `adl/src/cli/runtime_v2_cmd/helpers.rs`
- `adl/src/cli/usage.rs`

## Participants

The unified proof requires and validates these runtime participants:

- `daemon_tick`: bounded live current-runtime tick and stop/final status artifacts
- `lifecycle_standing`: Runtime v2 lifecycle and standing policy
- `scheduler_provider`: scheduler/provider/local-agent selection
- `memory_obsmem`: AEE trace handoff to ObsMem write, ack, and retrieval
- `acip_boundary`: ACIP runtime stream readiness and hardening packet
- `resilience_continuity`: stop, lease, failed-cycle, checkpoint, recovery, replay, quarantine, and hardening evidence
- `observability`: synthetic correlation index with one participant event per required runtime boundary
- `external_signals`: disabled local AWS signal shell recorded fail-closed

## Retained Evidence

Generated under `docs/milestones/v0.91.7/review/runtime/unified_kernel_5097/evidence`:

- `issue_5097/unified_runtime_kernel_summary.json`
- `issue_5097/unified_runtime_kernel_events.jsonl`
- `issue_5097/unified_runtime_kernel_negative_cases.json`
- `issue_5097/aws_signal_config_disabled.json`
- `issue_5097/current_runtime/agent.yaml`
- `issue_5097/current_runtime/initial_status.json`
- `issue_5097/current_runtime/run_status.json`
- `issue_5097/current_runtime/stop_status.json`
- `issue_5097/current_runtime/final_status.json`

The proof also materializes the Runtime v2 contract evidence it consumes, including:

- `runtime_v2/csm_run/integrated_first_run_proof_packet.json`
- `runtime_v2/csm_run/integrated_first_run_transcript.jsonl`
- `runtime_v2/standing/standing_policy.json`
- `runtime_v2/standing/standing_negative_cases.json`
- `issue_4697/obsmem_memory_write_request.json`
- `issue_4697/obsmem_memory_write_ack.json`
- `issue_4697/obsmem_retrieval_result.json`
- `runtime_v2/acip/acip_hardening_packet.json`
- `runtime_v2/memory_identity/memory_identity_architecture.json`
- `runtime_v2/godel_agent_runtime/godel_agent_runtime.json`

## Negative Cases

The generated negative-case packet records these boundaries:

- `invalid_lifecycle_standing_transition`
- `provider_scheduler_mismatch`
- `failed_tick_recoverable_cycle`
- `stop_request`
- `missing_disabled_external_signal_config`

## Validation Run

Validation was run from the bound #5097 worktree with Rust build output directed to `/Volumes/FastWork/adl-target-5097`:

```sh
CARGO_TARGET_DIR=/Volumes/FastWork/adl-target-5097 cargo check --manifest-path adl/Cargo.toml --lib
CARGO_TARGET_DIR=/Volumes/FastWork/adl-target-5097 cargo fmt --manifest-path adl/Cargo.toml
CARGO_TARGET_DIR=/Volumes/FastWork/adl-target-5097 cargo test --manifest-path adl/Cargo.toml runtime_v2_unified_runtime_kernel -- --nocapture
CARGO_TARGET_DIR=/Volumes/FastWork/adl-target-5097 cargo run --manifest-path adl/Cargo.toml -- runtime-v2 unified-runtime-kernel --out docs/milestones/v0.91.7/review/runtime/unified_kernel_5097/evidence
git diff --check
```

Observed focused Rust test result:

```text
5 passed; 0 failed; 1898 filtered out
```

CSM loopback liveness was checked from the main checkout with the permission-safe process helper:

```sh
./adl/target/debug/adl process status --port 19997 --host 127.0.0.1 --json
```

Observed result: `bound_port` on `127.0.0.1:19997`.

## Truth Boundary

This proof runs a bounded local current-runtime tick and materializes local contract evidence. It does not claim live AWS credentials, paid cloud resources, remote providers, or long-soak completion. The external signal participant records the disabled AWS signal shell as an explicit fail-closed local configuration.

The bounded tick is a local proof fixture only. It is not a daemon lifetime budget and does not reintroduce a public runtime stop budget.

The observability participant proves a synthetic correlation index over the composed local proof participants. It does not claim that the live daemon, scheduler, memory, ACIP, and cloud signal packets already share one cross-artifact trace id.

## Downstream Use

The unified kernel boundary gives #5096 a concrete consumption surface for scheduler/provider, reasoning/loop, lifecycle/standing, memory, ACIP, continuity, resilience, and observability without redefining runtime architecture.
