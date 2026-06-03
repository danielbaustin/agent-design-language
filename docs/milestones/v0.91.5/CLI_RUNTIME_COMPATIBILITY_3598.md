# CLI Runtime Compatibility Boundary (#3598)

## Summary

Issue `#3598` introduces `adl-runtime` as the runtime compatibility binary for
the first v0.91.5 CLI ownership split. This is a behavior-preserving boundary:
runtime command ownership becomes explicit, but the existing `adl <adl.yaml>`
shortcut and runtime command semantics remain intact.

## Ownership Contract

`adl-runtime` owns runtime-facing command families during this split:

- `adl-runtime run <adl.yaml> ...`
- `adl-runtime resume <run_id> --adl <path> ...`
- `adl-runtime agent ...`
- `adl-runtime artifact ...`
- `adl-runtime csm observatory ...`
- `adl-runtime demo ...`
- `adl-runtime godel ...`
- `adl-runtime identity ...`
- `adl-runtime instrument ...`
- `adl-runtime learn export ...`
- `adl-runtime provider setup ...`
- `adl-runtime runtime-v2 ...`
- `adl-runtime keygen`, `adl-runtime sign`, and `adl-runtime verify`

The compatibility binary does not move implementation modules yet. It routes to
the existing runtime functions so command behavior can be proven equivalent
before deeper module or crate decomposition begins.

## Compatibility Rules

- `adl <adl.yaml>` remains a compatibility shortcut.
- `adl-runtime run <adl.yaml>` must preserve the legacy YAML shortcut behavior.
- `adl-runtime run <issue>` and `adl-runtime run #<issue>` must fail closed and direct issue work to
  `adl/tools/pr.sh run <issue>`.
- `adl-runtime pr ...` and `adl-runtime tooling ...` must fail closed because
  C-SDLC issue and tooling commands remain under the C-SDLC wrapper/migration
  spine.
- `adl/tools/pr.sh` remains the canonical agent-facing issue-work entrypoint
  under the wrapper migration contract from `#3597`.

## Runtime Design Notes

The runtime binary is intentionally conservative:

- no runtime execution semantics are changed;
- no provider, agent, demo, identity, Gödel, signing, learning, or
  instrumentation module is moved;
- no new runtime configuration state is introduced;
- no external-repo adapter command truth is changed;
- help text names the runtime boundary and the C-SDLC handoff explicitly.

This keeps the centerpiece runtime stable while still giving future issues a
clear binary boundary to test against.

## Validation Surface

Focused validation should prove:

- `adl-runtime --help` and `adl-runtime --version` work;
- `adl-runtime run <adl.yaml> --print-plan` matches
  `adl <adl.yaml> --print-plan`;
- `adl-runtime run <adl.yaml> --run` executes the existing runtime path with a
  mock provider and writes expected output artifacts;
- `adl-runtime run <issue>` and `adl-runtime run #<issue>` reject issue IDs
  with a C-SDLC handoff;
- `adl-runtime pr ...` and `adl-runtime tooling ...` reject C-SDLC ownership
  drift;
- the old `adl <adl.yaml>` shortcut remains available.

The fast shell proof is:

```bash
bash adl/tools/test_adl_runtime_compatibility.sh
```

The focused Rust proof is:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_dispatch -- --nocapture
cargo test --manifest-path adl/Cargo.toml --test cli_smoke adl_runtime -- --nocapture
```

## Non-Claims

- This does not split the Rust workspace into multiple crates.
- This does not remove or deprecate the `adl` binary.
- This does not change generated-card issue-work commands.
- This does not make `adl-runtime` the C-SDLC issue executor.
- This does not complete later observability or OpenTelemetry work.
