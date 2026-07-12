# Runtime v3 Entrypoint Switch

## Status

As of v0.91.7, Runtime v3 is selectable through an explicit CLI compatibility
boundary. Runtime v2 remains the default runtime until the cutover proof gate
authorizes a default switch.

## Covered Entrypoints

- `adl runtime-v3 select [--runtime v2|v3] [--json]`
- `adl-runtime runtime-v3 select [--runtime v2|v3] [--json]`

These entrypoints report the selected runtime and the Runtime v3 control API
policy without launching a daemon or changing global defaults.

The selector reports `DEFAULT_CHANGED=false` for both Runtime v2 and Runtime v3
selection. `SELECTION_DIFFERS_FROM_DEFAULT=true` is the reversible selection
signal for explicit Runtime v3 use while Runtime v2 remains the default.

## Selection Rules

- No selector: Runtime v2 remains selected.
- `--runtime v3`: Runtime v3 is selected explicitly.
- `--runtime v2`: Runtime v2 fallback is selected explicitly.
- `ADL_RUNTIME_SELECTION=v3`: Runtime v3 is selected when `--runtime` is
  omitted.
- Unknown values fail closed.

`--runtime` takes precedence over `ADL_RUNTIME_SELECTION`.

## Runtime v3 Control Policy

Runtime v3 uses the local control API endpoint:

```text
http://127.0.0.1:20997
```

The Runtime v3 kernel launch command reported by the selector is:

```text
adl-runtime-kernel serve
```

## Non-Covered Surfaces

This issue does not:

- make Runtime v3 the default;
- delete or decommission Runtime v2;
- migrate every Runtime v2 demo command;
- introduce a custom supervisor;
- start the Runtime v3 daemon implicitly.

Runtime v2 decommission remains gated by the aggregate Runtime v3 cutover proof
and an explicit default-switch decision.

## v0.91.7 Decision

#5254 records the final v0.91.7 default-switch decision: Runtime v2 remains the
default runtime, Runtime v3 remains explicit opt-in only, and Runtime v2
decommission is not authorized. See
`docs/architecture/RUNTIME_V3_CUTOVER_DECISION_5254.md`.
