# v0.3 Fork/Join Sequential Run Example

Files:
- `swarm/examples/v0-3-fork-join-seq-run.adl.yaml`

## Runtime behavior (v0.3)

- `workflow.kind: concurrent` executes in deterministic declared step order.
- Fork branches run sequentially in this single-threaded runtime.
- Join runs only after prior fork steps succeed.
- Join reads saved fork outputs via `@state:<save_as_key>`.

## Run

```bash
cargo run -- examples/v0-3-fork-join-seq-run.adl.yaml --run --trace --out out
```

Expected artifacts:
- `out/fork_join/join.txt`
