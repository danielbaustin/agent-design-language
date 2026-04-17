# `pr run` Demo

This is the bounded historical proof surface for the supported `pr run` path.

What it proves:

- `adl/tools/pr.sh` exposes a real `pr run` command
- the command delegates to the existing Rust `adl` runtime rather than embedding feature-specific logic
- the command resolves and executes a bounded ADL workflow over the runtime primitives
- the command leaves behind canonical run artifacts that can be inspected deterministically

Current limitation:

- `pr run` is the supported control-plane run surface today
- browser/editor direct invocation remains follow-on work

Demo command:

```bash
adl/tools/demo_five_command_run.sh
```

Expected proof surface:

- `<runs_root>/v0-4-demo-deterministic-replay/run.json`
- `<runs_root>/v0-4-demo-deterministic-replay/run_status.json`
- `<runs_root>/v0-4-demo-deterministic-replay/run_summary.json`

Demo note:

- the demo uses an isolated temporary `--runs-root` so it does not leave behind repo-local run artifacts
- `pr run` defaults to the canonical repo-local `.adl/runs/` root when `--runs-root` is not supplied

Expected command behavior:

- prints the underlying bounded ADL run
- prints a final `PR RUN ok` summary
- reports the run id, workflow id, and canonical proof artifact paths
