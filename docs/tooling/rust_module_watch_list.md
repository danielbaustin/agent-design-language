# Rust Module Watch List

This document is the canonical watch list and maintainability guardrail for large Rust implementation modules in ADL.

It is derived from:
- `.adl/docs/v0.85planning/RUST_MODULE_SIZE_REVIEW_v0.85.md`
- `.adl/docs/v0.86/THIRD_PARTY_REVIEW_ISSUE_LIST.md`

This is a governance surface, not a mandatory refactor queue. Files stay on this watch list unless current work materially touches them.

It also captures the deferred large-file follow-up from the external `v0.86`
review. That review explicitly named:

- `adl/src/demo.rs`
- `adl/src/remote_exec.rs`
- `adl/src/cli/pr_cmd.rs`

and treated them as justified by current complexity, not as release blockers.
This watch list exists so that recommendation is tracked canonically rather than
remaining an informal review note.

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

Measured from the current `adl/src/**/*.rs` tree using:

```bash
./adl/tools/report_large_rust_modules.sh --format tsv
```

The three `RATIONALE` entries are the highest-priority review surfaces because
they exceed the strongest decomposition threshold and were also called out by
the external `v0.86` review.

| Path | Approx. LoC | Responsibility summary | Suggested future split boundaries | Watch level |
|---|---:|---|---|---|
| `adl/src/cli/pr_cmd.rs` | 2105 | PR control-plane orchestration, lifecycle transitions, bootstrap/finish flows, and CLI command ownership | split lifecycle families by responsibility (`create/init/run/ready/finish`), move shared repair/state helpers into focused modules, and keep shell wrappers thin | Rationale |
| `adl/src/demo.rs` | 1824 | demo catalog, scenario config, execution plumbing, and report/display helpers | split demo catalog/definitions from execution harnesses and reporting helpers | Rationale |
| `adl/src/remote_exec.rs` | 1740 | remote execution planning, transfer/setup logic, remote lifecycle coordination, and artifact retrieval | split transport/session setup from execution orchestration and retrieval/report helpers | Rationale |
| `adl/src/execute/state.rs` | 1417 | execution state modeling, persistence helpers, and state transition/report logic | split state model/schema code from persistence/loading helpers and derived-report helpers | Review |
| `adl/src/instrumentation.rs` | 1327 | instrumentation/event capture, shaping, and persistence/report helpers | split event/schema definitions from emitters, formatting, and persistence helpers | Review |
| `adl/src/cli/tests/pr_cmd_inline/finish.rs` | 1319 | inline `pr finish` test matrix covering workflow closeout and edge conditions | split finish-path test cases by concern (`validation`, `integration`, `error paths`) and centralize shared fixtures | Review |
| `adl/src/cli/run_artifacts/cognitive.rs` | 1287 | cognitive artifact assembly, shaping, and export helpers | split schema/model builders from export and report formatting helpers | Review |
| `adl/src/learning_export.rs` | 1149 | learning export assembly, schema shaping, and serialization/report helpers | split export model/schema code from collection/build logic and output writing | Review |
| `adl/src/adl/tests.rs` | 1118 | ADL parser/validator regression coverage across many behaviors | split tests by behavior family and move shared builders into test helpers | Review |
| `adl/src/cli/run_artifacts/runtime.rs` | 1089 | runtime artifact assembly, trace export, and persistence/report shaping | split path/schema helpers from emit/export flows and separate persistence from presentation helpers | Review |
| `adl/src/execute/runner.rs` | 1001 | execution runner orchestration, scheduling, and policy handling | split policy/scheduler helpers from runner lifecycle and error/report shaping | Review |
| `adl/src/cli/tests/artifact_builders/learning_runtime.rs` | 946 | learning/runtime artifact-builder regression coverage | split learning-vs-runtime assertions and centralize shared fixture setup | Watch |
| `adl/src/provider.rs` | 936 | provider configuration, request assembly, and provider-facing orchestration | split provider model/schema code from request execution and normalization helpers | Watch |
| `adl/src/trace.rs` | 936 | trace model, persistence, and rendering/query helpers | split trace record/schema logic from output formatting and IO helpers | Watch |
| `adl/src/signing.rs` | 929 | signing material handling, envelope creation, and verification helpers | split key/material utilities from signing/verification flows and report shaping | Watch |
| `adl/src/godel/stage_loop.rs` | 916 | stage progression, orchestration, and artifact/report linkage for the Godel loop | split stage state transitions from artifact/report assembly and CLI-facing summaries | Watch |
| `adl/src/sandbox.rs` | 887 | sandbox policy/configuration and execution boundary helpers | split policy/config parsing from sandbox command/runtime helpers | Watch |
| `adl/src/execute/tests.rs` | 839 | execution regression coverage across multiple behaviors | split tests by behavior family and move shared setup into helpers | Watch |
| `adl/src/cli/godel_cmd.rs` | 815 | CLI argument handling and command dispatch for Godel features | split command parsing/dispatch from artifact inspection/rendering helpers | Watch |

## v0.86 External Review Follow-up

The external `v0.86` review specifically named the following three files as
oversized but justified by current complexity:

| Path | Current LoC | External review posture | 1289 disposition |
|---|---:|---|---|
| `adl/src/cli/pr_cmd.rs` | 2105 | non-blocking maintainability concern | keep on watch list at `Rationale`; require explicit deferral or structural improvement when materially changed |
| `adl/src/demo.rs` | 1824 | non-blocking maintainability concern | keep on watch list at `Rationale`; prefer separating catalog/definitions from execution/reporting helpers |
| `adl/src/remote_exec.rs` | 1740 | non-blocking maintainability concern | keep on watch list at `Rationale`; prefer separating transport/session setup from orchestration/retrieval |

This issue does not force immediate refactors for those files. It records them
as explicit future split candidates with named responsibility boundaries.

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
4. if the module is in the `Rationale` band, treat the deferral note as required rather than optional

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
