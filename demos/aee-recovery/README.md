# Bounded AEE Recovery Demo

This runbook demonstrates one bounded Adaptive Execution Engine recovery path
using only current v0.8 repository behavior:

1. a first run fails on a retryable transient provider error
2. the run artifacts emit a retry-budget recommendation
3. an explicit bounded overlay is applied
4. a second run succeeds because the retry budget increased

This is a reviewer-facing demo of bounded recovery, not a claim of autonomous
policy learning or open-ended self-healing.

## Demo Assets

- Initial workflow: `swarm/examples/v0-3-aee-recovery-initial.adl.yaml`
- Adapted workflow: `swarm/examples/v0-3-aee-recovery-adapted.adl.yaml`
- Bounded recovery overlay: `demos/aee-recovery/retry-budget.overlay.json`
- Deterministic no-network provider mock: `swarm/tools/mock_ollama_fail_once.sh`

## Run From Repository Root

### 1. Start clean

```bash
rm -rf .adl/runs/v0-3-aee-recovery-initial .adl/runs/v0-3-aee-recovery-adapted ./out/aee-recovery
mkdir -p ./out/aee-recovery
```

### 2. Run the initial failing case

```bash
ADL_OLLAMA_BIN=swarm/tools/mock_ollama_fail_once.sh \
ADL_AEE_DEMO_STATE_DIR=./out/aee-recovery/state-initial \
cargo run --manifest-path swarm/Cargo.toml --bin adl -- \
  swarm/examples/v0-3-aee-recovery-initial.adl.yaml \
  --run \
  --trace \
  --out ./out/aee-recovery/initial
```

Expected outcome:
- command exits non-zero
- `.adl/runs/v0-3-aee-recovery-initial/learning/suggestions.json` contains a
  retry-budget recommendation

### 3. Inspect the recovery recommendation and replay the failed trace

```bash
cat .adl/runs/v0-3-aee-recovery-initial/learning/suggestions.json
cargo run --manifest-path swarm/Cargo.toml --bin adl -- \
  instrument replay .adl/runs/v0-3-aee-recovery-initial/logs/activation_log.json
```

Look for:
- `intent: "increase_step_retry_budget"` in `suggestions.json`
- a replay summary that reflects the failed first run

### 4. Apply the bounded retry overlay and rerun

```bash
ADL_OLLAMA_BIN=swarm/tools/mock_ollama_fail_once.sh \
ADL_AEE_DEMO_STATE_DIR=./out/aee-recovery/state-adapted \
cargo run --manifest-path swarm/Cargo.toml --bin adl -- \
  swarm/examples/v0-3-aee-recovery-adapted.adl.yaml \
  --run \
  --trace \
  --overlay demos/aee-recovery/retry-budget.overlay.json \
  --out ./out/aee-recovery/adapted
```

Expected outcome:
- command exits successfully
- the first attempt fails, then retry succeeds under the raised retry budget
- `.adl/runs/v0-3-aee-recovery-adapted/learning/overlays/` contains overlay
  audit artifacts

### 5. Inspect the recovered run and replay it

```bash
cat .adl/runs/v0-3-aee-recovery-adapted/learning/overlays/applied_overlay.json
cat .adl/runs/v0-3-aee-recovery-adapted/run_summary.json
cargo run --manifest-path swarm/Cargo.toml --bin adl -- \
  instrument replay .adl/runs/v0-3-aee-recovery-adapted/logs/activation_log.json
```

Key files:
- `.adl/runs/v0-3-aee-recovery-initial/learning/suggestions.json`
- `.adl/runs/v0-3-aee-recovery-adapted/learning/overlays/source_overlay.json`
- `.adl/runs/v0-3-aee-recovery-adapted/learning/overlays/applied_overlay.json`
- `.adl/runs/v0-3-aee-recovery-adapted/run_summary.json`
- `.adl/runs/v0-3-aee-recovery-adapted/logs/activation_log.json`

## What This Demonstrates

- bounded retry-policy adaptation
- explicit evidence for why the adaptive step was chosen
- explicit overlay application rather than hidden policy mutation
- inspectable and replayable artifact trail for both failed and recovered runs

## What This Does Not Claim

- autonomous retry-strategy learning
- cross-run persistent strategy state
- open-ended policy mutation
- v0.85-scale AEE completion
