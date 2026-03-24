# Rust Module Watch List

This document is the canonical watch list and maintainability guardrail for large Rust implementation modules in ADL.

It is derived from:
- `.adl/docs/v0.85planning/RUST_MODULE_SIZE_REVIEW_v0.85.md`

This is a governance surface, not a mandatory refactor queue. Files stay on this watch list unless current work materially touches them.

## Guardrail

Default review thresholds:

- below `800` LoC: usually acceptable if cohesive
- `800` to `999` LoC: watch when scope is already broad or still growing
- `1000` to `1499` LoC: review candidate; new growth should be deliberate
- `1500` LoC and above: strong decomposition candidate; PRs should either improve structure or justify deferral in the output card

Default team rule:

- do not refactor watched modules opportunistically when unrelated work lands nearby
- do keep watched modules on this list until they are split or materially reduced
- if a PR materially modifies a watched module, it should either:
  - improve structure as part of the change, or
  - record a brief deferral rationale in the output card
- use responsibility boundaries, not arbitrary line-count quotas, to drive splits
- use `adl/tools/report_large_rust_modules.sh` as a non-blocking review aid; it must not fail the build by default

## Current Watch List

Measured from the current `adl/src/**/*.rs` tree after the v0.85 B2 refactors for `adl/src/adl.rs` and `adl/src/execute/mod.rs`.

| Path | Approx. LoC | Responsibility summary | Suggested future split boundaries | Watch level |
|---|---:|---|---|---|
| `adl/src/cli/run_artifacts.rs` | 1674 | CLI artifact assembly, path handling, emission logic, and export/report shaping | split path/schema helpers from emit/export flows; separate CLI presentation from artifact persistence | High |
| `adl/src/remote_exec.rs` | 1587 | remote execution planning, transfer/setup logic, and remote lifecycle coordination | split transport/session setup from execution orchestration and artifact retrieval | High |
| `adl/src/instrumentation.rs` | 1327 | instrumentation/event capture, shaping, and persistence/report helpers | split event/schema definitions from emitters, formatting, and persistence helpers | Medium |
| `adl/src/learning_export.rs` | 1137 | learning export assembly, schema shaping, and serialization/report helpers | split export model/schema code from collection/build logic and output writing | Medium |
| `adl/src/demo.rs` | 1047 | demo catalog/configuration plus demo execution/helpers | split demo definitions/catalog from execution helpers and reporting/output shaping | Medium |
| `adl/src/execute/runner.rs` | 995 | execution runner orchestration, scheduling, and policy handling | split policy/scheduler helpers from runner lifecycle and error/report shaping | Watch |
| `adl/src/trace.rs` | 936 | trace model, persistence, and rendering/query helpers | split trace record/schema logic from output formatting and IO helpers | Watch |
| `adl/src/signing.rs` | 934 | signing material handling, envelope creation, and verification helpers | split key/material utilities from signing/verification flows and report shaping | Watch |
| `adl/src/godel/stage_loop.rs` | 916 | stage progression, orchestration, and artifact/report linkage for the Godel loop | split stage state transitions from artifact/report assembly and CLI-facing summaries | Watch |
| `adl/src/sandbox.rs` | 887 | sandbox policy/configuration and execution boundary helpers | split policy/config parsing from sandbox command/runtime helpers | Watch |
| `adl/src/provider.rs` | 865 | provider configuration, request assembly, and provider-facing orchestration | split provider model/schema code from request execution and normalization helpers | Watch |
| `adl/src/cli/godel_cmd.rs` | 815 | CLI argument handling and command dispatch for Godel features | split command parsing/dispatch from artifact inspection/rendering helpers | Watch |

## Modules Removed From Immediate Watch Priority

The following large modules were the first B2 refactor targets and are no longer the primary watch-list focus:

- `adl/src/adl.rs`
- `adl/src/execute/mod.rs`

They should still be reviewed like any other growing module, but they no longer anchor the immediate large-module governance problem that motivated this document.

## Operating Rule For Future PRs

When a PR touches a watched module:

1. confirm whether the change expands scope, only edits a narrow existing responsibility, or improves structure
2. if the module grows materially and no refactor is performed, explain the deferral briefly in the output card
3. if the change introduces a clear new responsibility boundary, prefer extracting that boundary rather than extending the large file again

When a PR does not touch a watched module:

- do not refactor it opportunistically just because it is nearby

## Review Procedure

Use the report script during review or preflight:

```bash
./adl/tools/report_large_rust_modules.sh
```

Optional narrower checks:

```bash
./adl/tools/report_large_rust_modules.sh --threshold-watch 800 --threshold-review 1000 --threshold-rationale 1500
./adl/tools/report_large_rust_modules.sh --format tsv
```

The script is intentionally non-blocking and exits successfully unless the invocation itself is invalid.
