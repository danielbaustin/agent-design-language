# adl/examples

Runnable ADL example documents for the `adl` runtime.
Use these files for local smoke checks, deterministic demo runs, and CLI validation.

This directory is the crate-local fixture inventory, not the primary user-facing demo index.
For demo discovery from repo root, start with `../../demos/README.md`.

## Run An Example

From `adl/`:

```bash
cargo run -q --bin adl -- examples/<file>.adl.yaml --print-plan
cargo run -q --bin adl -- examples/<file>.adl.yaml --run --trace
```

No-network mock-provider pattern:

```bash
ADL_OLLAMA_BIN=tools/mock_ollama_v0_4.sh cargo run -q --bin adl -- examples/v0-6-hitl-no-pause.adl.yaml --run --trace --out ./out
```

## Example Families (Short Index)

- v0.3: concurrency, retry, bounded AEE recovery, remote provider (`v0-3-*`)
- v0.5: primitives, patterns, remote execution MVP (`v0-5-*`)
- v0.6: HITL pause/resume, profiles, delegation, instrumentation demos
- v0.7: provider portability / HTTP profile compatibility proof surface
- v0.87.1: no-network mock provider family proof (`v0-87-1-provider-mock-demo.adl.yaml`)
- v0.87.1: bounded HTTP family proof (`v0-87-1-provider-http-demo.adl.yaml`)
- v0.87.1: local Ollama family proof (`v0-87-1-provider-local-ollama-demo.adl.yaml`)
- v0.87.1: ChatGPT family proof (`v0-87-1-provider-chatgpt-demo.adl.yaml`)

## See Also / Canonical Docs

- Canonical demo index: `../../demos/README.md`
- Provider demo proof governance: `../../docs/tooling/PROVIDER_DEMO_SURFACES.md`
- v0.7 demo matrix (canonical): `../../docs/milestones/v0.7/DEMOS_v0.7.md`
- v0.6 release notes: `../../docs/milestones/v0.6/RELEASE_NOTES_v0.6.md`
- Runtime/CLI usage: `../README.md`
- ADRs: `../../docs/adr/`
- Root project entrypoint: `../../README.md`
