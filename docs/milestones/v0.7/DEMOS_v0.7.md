# v0.7 Demo Matrix

Canonical runnable demos for milestone v0.7 (WP-13 / #474).

Execution assumptions:
- Run commands from repo root unless noted.
- Use canonical binaries: `adl` and `adl-remote`.
- Core demos are offline/local (loopback only, no external network).
- `run` commands use `--allow-unsigned` where needed so runtime signature enforcement for workflow docs does not block demo execution. This does not bypass remote request signing policy in D-11.

## Story-driven demo packs (user-facing)

### S-01 Determinism You Can Trust
- Purpose: Show the same workflow yields byte-identical outputs across repeated runs.
- Preconditions: `ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh`.
- Commands:
```bash
ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- swarm/examples/v0-6-hitl-no-pause.adl.yaml --run --trace --allow-unsigned --out .tmp/v07-s01-a
ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- swarm/examples/v0-6-hitl-no-pause.adl.yaml --run --trace --allow-unsigned --out .tmp/v07-s01-b
cmp .tmp/v07-s01-a/s1.txt .tmp/v07-s01-b/s1.txt
cmp .tmp/v07-s01-a/s2.txt .tmp/v07-s01-b/s2.txt
cmp .tmp/v07-s01-a/s3.txt .tmp/v07-s01-b/s3.txt
```
- Expected output: all `cmp` checks succeed; run artifacts exist for `v0-6-hitl-no-pause-demo`.
- Artifact paths: `.tmp/v07-s01-a/`, `.tmp/v07-s01-b/`, `.adl/runs/v0-6-hitl-no-pause-demo/`.

### S-02 From Failure to Clarity
- Purpose: Show deterministic failure surfaces with preserved run artifacts for diagnosis.
- Preconditions: none.
- Commands:
```bash
cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- swarm/examples/failure-missing-file.adl.yaml --run --allow-unsigned
```
- Expected output: deterministic failure with actionable missing-file message.
- Artifact paths: `.adl/runs/failure-missing-file-demo/run_status.json`, `.adl/runs/failure-missing-file-demo/run_summary.json`.

### S-03 Portable Learning (Exportable Intelligence)
- Purpose: Show learning export is deterministic and sanitized for sharing.
- Preconditions: at least one prior run in `.adl/runs` (for example S-01).
- Commands:
```bash
cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- learn export --format jsonl --runs-dir .adl/runs --out .tmp/v07-s03-a.jsonl
cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- learn export --format jsonl --runs-dir .adl/runs --out .tmp/v07-s03-b.jsonl
cmp .tmp/v07-s03-a.jsonl .tmp/v07-s03-b.jsonl
```
- Expected output: `cmp` succeeds and JSONL export is stable across reruns.
- Artifact paths: `.tmp/v07-s03-a.jsonl`, `.tmp/v07-s03-b.jsonl`.

### S-04 Enterprise Trust Boundary (Signed Remote Requests)
- Purpose: Demonstrate deterministic remote-signing trust failures and policy messaging.
- Preconditions:
  - `ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh`
  - loopback remote server via `adl-remote`
  - local keypair generated via `adl keygen`
- Commands:
```bash
tmpdir="$(mktemp -d)"
cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- keygen --out-dir "$tmpdir/.keys"
export ADL_REMOTE_REQUEST_SIGNING_PRIVATE_KEY_B64="$(tr -d '\n' < "$tmpdir/.keys/ed25519-private.b64")"
export ADL_REMOTE_REQUEST_SIGNING_KEY_ID="demo-key-1"
ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin adl-remote -- 127.0.0.1:8787 >/tmp/adl-remote-s04.log 2>&1 &
remote_pid=$!
ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- swarm/examples/v0-7-enterprise-signed-remote.adl.yaml --run --trace --allow-unsigned --out "$tmpdir/out"
kill "$remote_pid"
unset ADL_REMOTE_REQUEST_SIGNING_PRIVATE_KEY_B64
ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- swarm/examples/v0-7-enterprise-signed-remote.adl.yaml --run --trace --allow-unsigned
```
- Expected output:
  - signed-path succeeds deterministically (remote step executes under valid signed request + trust policy)
  - missing-signature path fails deterministically with `REMOTE_REQUEST_SIGNATURE_MISSING`
- Artifact paths: `.adl/runs/v0-7-enterprise-signed-remote/`, `/tmp/adl-remote-s04.log`, `"$tmpdir/out"/` (partial/failed run output).

### S-05 ADL is the Product Name (Compatibility Window)
- Purpose: Show canonical CLI naming plus deterministic legacy shim warning.
- Preconditions: none.
- Commands:
```bash
cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- --help
```
- Expected output:
  - `adl` help prints without deprecation noise
  - legacy compatibility shim behavior is deterministic and emits a stable deprecation warning (validated by CI + CLI compatibility tests)
- Artifact paths: stdout/stderr only.

## D-01 Basic Local Run
- Purpose: Validate baseline local execution and deterministic artifact emission.
- Preconditions: `ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh`.
- Commands:
```bash
ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- swarm/examples/v0-6-hitl-no-pause.adl.yaml --run --trace --allow-unsigned --out .tmp/v07-d01
```
- Expected output: run succeeds; trace shows deterministic step lifecycle.
- Artifact paths: `.tmp/v07-d01/s1.txt`, `.tmp/v07-d01/s2.txt`, `.tmp/v07-d01/s3.txt`, `.adl/runs/v0-6-hitl-no-pause-demo/`.

## D-02 Plan Determinism (Primitives)
- Purpose: Validate stable `--print-plan` output for canonical primitives fixture.
- Preconditions: none.
- Commands:
```bash
cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- swarm/examples/v0-5-primitives-minimal.adl.yaml --print-plan
```
- Expected output: deterministic plan text with stable IDs and ordering.
- Artifact paths: stdout only.

## D-03 Pattern Compiler (Fork/Join)
- Purpose: Validate deterministic pattern compilation and join dependencies.
- Preconditions: none.
- Commands:
```bash
cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- swarm/examples/v0-5-pattern-fork-join.adl.yaml --print-plan
```
- Expected output: canonical `p::<pattern_id>::...` IDs; deterministic branch ordering.
- Artifact paths: stdout only.

## D-04 Deterministic Concurrent Execution
- Purpose: Validate bounded concurrent execution with deterministic ordering.
- Preconditions: `ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh`.
- Commands:
```bash
ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- swarm/examples/v0-3-concurrency-fork-join.adl.yaml --run --trace --allow-unsigned --out .tmp/v07-d04
```
- Expected output: fork steps and join execute successfully; deterministic trace ordering.
- Artifact paths: `.tmp/v07-d04/fork/`, `.adl/runs/v0-3-concurrency-fork-join/`.

## D-05 Sandbox Failure Demo
- Purpose: Demonstrate safe, deterministic sandbox rejection/error reporting.
- Preconditions: none.
- Commands:
```bash
cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- swarm/examples/failure-missing-file.adl.yaml --run --allow-unsigned
```
- Expected output: run fails with deterministic, actionable error.
- Artifact paths: `.adl/runs/failure-missing-file-demo/` (failed-run artifacts).

## D-06 HITL Pause/Resume (Step-Boundary)
- Purpose: Validate pause/resume strictness and deterministic roundtrip.
- Preconditions: `ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh`.
- Commands:
```bash
ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- swarm/examples/v0-6-hitl-pause-resume.adl.yaml --run --trace --allow-unsigned --out .tmp/v07-d06-pause
ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- resume v0-6-hitl-pause-demo --out .tmp/v07-d06-resume
```
- Expected output: first command pauses at deterministic boundary; resume command completes.
- Artifact paths: `.adl/runs/v0-6-hitl-pause-demo/pause_state.json`, `.tmp/v07-d06-resume/`.

## D-07 Streaming Is Observational
- Purpose: Show streaming trace output does not change final artifacts.
- Preconditions: `ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh`.
- Commands:
```bash
ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- swarm/examples/v0-6-hitl-no-pause.adl.yaml --run --trace --allow-unsigned --out .tmp/v07-d07-stream
ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- swarm/examples/v0-6-hitl-no-pause.adl.yaml --run --quiet --allow-unsigned --out .tmp/v07-d07-quiet
```
- Expected output: streamed run emits chunk events; output artifacts are equivalent to quiet run.
- Artifact paths: `.tmp/v07-d07-stream/`, `.tmp/v07-d07-quiet/`, `.adl/runs/v0-6-hitl-no-pause-demo/`.

## D-08 Provider Profiles + Delegation Metadata
- Purpose: Validate profile resolution and delegation trace metadata emission.
- Preconditions: `ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh`.
- Commands:
```bash
ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- swarm/examples/v0-6-provider-profile-delegation.adl.yaml --run --trace --allow-unsigned --out .tmp/v07-d08
```
- Expected output: run succeeds; delegation metadata appears only when non-empty/canonicalized.
- Artifact paths: `.tmp/v07-d08/`, `.adl/runs/v0-6-provider-profile-delegation/`.

## D-09 Instrumentation: Graph + Replay + Diffs
- Purpose: Validate deterministic instrumentation surfaces.
- Preconditions: none.
- Commands:
```bash
cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- instrument graph swarm/examples/v0-5-pattern-fork-join.adl.yaml --format json
cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- instrument graph swarm/examples/v0-5-pattern-fork-join.adl.yaml --format dot
cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- instrument replay swarm/examples/v0-6-trace-sample-a.json
cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- instrument diff-plan swarm/examples/v0-5-pattern-linear.adl.yaml swarm/examples/v0-5-pattern-fork-join.adl.yaml
cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- instrument diff-trace swarm/examples/v0-6-trace-sample-a.json swarm/examples/v0-6-trace-sample-b.json
```
- Expected output: deterministic structured outputs suitable for repeat comparisons.
- Artifact paths: stdout only.

## D-10 Learning Export (JSONL)
- Purpose: Validate deterministic learning export surface for run artifacts.
- Preconditions: At least one local run completed (for example D-01).
- Commands:
```bash
cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- learn export --format jsonl --runs-dir .adl/runs --out .tmp/v07-d10-learning.jsonl
```
- Expected output: deterministic JSONL export with no secrets/absolute host paths.
- Artifact paths: `.tmp/v07-d10-learning.jsonl`.

## D-11 Enterprise Signed Remote Run (#497)
- Purpose: Exercise real remote signing/trust policy path with deterministic success/failure behavior.
- Preconditions:
  - `ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh`
  - no external network; loopback-only local remote process
- One-command harness (success):
```bash
swarm/tools/demo_d11_signed_remote.sh success
```
- Negative harness command (deterministic rejection):
```bash
swarm/tools/demo_d11_signed_remote.sh negative
```
- Expected output:
  - success command runs end-to-end through `local.first -> remote.mid -> local.last`
  - harness explicitly verifies remote-step execution by asserting `remote.mid` and `local.last` are `success` in `.adl/runs/v0-7-enterprise-signed-remote/steps.json`
  - negative command fails deterministically with `REMOTE_REQUEST_SIGNATURE_MISSING`
- Signature contract:
  - request signatures use canonical JSON bytes where object keys are recursively sorted and only `security.request_signature` is excluded before signing/verifying.
  - normative reference: [ADR-0006 Remote Request Signing Canonicalization](../../adr/0006-remote-signing-canonicalization.md)
- Artifact paths:
  - `.adl/runs/v0-7-enterprise-signed-remote/`
  - `.tmp/d11-remote.log`
  - `.tmp/d11-keys/` (ephemeral demo keys; do not commit, private key file is chmod 600 on unix)

## D-12 Hierarchical Planner Pattern (Issue #541)
- Purpose: Demonstrate canonical Planner -> Executor -> Aggregator orchestration with deterministic artifact layout.
- Preconditions: `ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh`.
- Command:
```bash
ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin adl -- swarm/examples/v0-7-hierarchical-planner.adl.yaml --run --allow-unsigned --out .tmp/v07-d12
```
- Expected output:
  - run succeeds through `planner.plan`, `executor.alpha`, `executor.beta`, `aggregator.final`
  - artifacts include structured stage files: `plan.json`, `exec-alpha.json`, `exec-beta.json`, `aggregate.json`
- Artifact paths:
  - `.tmp/v07-d12/plan.json`
  - `.tmp/v07-d12/exec-alpha.json`
  - `.tmp/v07-d12/exec-beta.json`
  - `.tmp/v07-d12/aggregate.json`
  - `.adl/runs/v0-7-hierarchical-planner/`
