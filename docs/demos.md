# ADL Demo Commands

This page lists the v0.3 demo flows with exact commands and expected outcomes.

## 1) v0.3 Fork/Join Plan-Only Demo

File:
- `swarm/examples/v0-3-concurrency-fork-join.adl.yaml`

Command (from repo root):

```bash
cargo run --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-concurrency-fork-join.adl.yaml --print-plan
```

What to expect:
- Command succeeds.
- Plan output includes: `fork.plan`, `fork.branch.alpha`, `fork.branch.beta`, `fork.join`.
- This is a historical v0.3 plan-shape demo. The current runtime does implement concurrent execution in v0.5.

## 2) v0.3 Remote Provider MVP Demo

Files:
- `swarm/examples/v0-3-remote-http-provider.adl.yaml`
- `swarm/examples/v0-3-remote-provider-demo.adl.yaml` (compat alias)

Plan command (from repo root):

```bash
cargo run --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-remote-http-provider.adl.yaml --print-plan
```

What to expect:
- Command succeeds without network dependency.
- Plan output includes step `remote_summary`.

Run command (requires local mock/endpoint):

```bash
SWARM_REMOTE_BEARER_TOKEN=demo-token \
cargo run --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-remote-http-provider.adl.yaml --run
```

What to expect:
- On reachable endpoint: step output + run summary.
- On failure: clear error for timeout, non-200 response, or missing auth env var.
