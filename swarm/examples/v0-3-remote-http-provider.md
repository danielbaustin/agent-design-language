# v0.3 Remote HTTP Provider Demo

Files:
- `swarm/examples/v0-3-remote-http-provider.adl.yaml`
- `swarm/examples/v0-3-remote-provider-demo.adl.yaml` (compat alias)

## What it demonstrates

- ADL v0.3 provider selection for `type: "http"`.
- Deterministic blocking request/response contract:
  - request JSON body: `{"prompt":"..."}`
  - response JSON body: `{"output":"..."}`
- Optional bearer auth header from env var.

## One-obvious commands

From repo root:

```bash
cargo run --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-remote-http-provider.adl.yaml --print-plan
```

For a real execution, run a local mock server first, then:

```bash
SWARM_REMOTE_BEARER_TOKEN=demo-token \
cargo run --manifest-path swarm/Cargo.toml -- swarm/examples/v0-3-remote-http-provider.adl.yaml --run
```

## Expected behavior

- `--print-plan` succeeds without network access and prints `remote_summary`.
- `--run` succeeds only when the configured endpoint is reachable.
- Runtime failures are explicit for timeout, non-200 status, or missing auth env var.
