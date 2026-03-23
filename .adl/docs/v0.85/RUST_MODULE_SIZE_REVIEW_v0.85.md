# Rust Module Size Review — v0.85

## Metadata
- Milestone: `v0.85`
- Version: `0.85`
- Date: `2026-03-23`
- Scope: Rust source and test files under `adl/`

## Purpose
Capture the current Rust file-size distribution at the close of v0.85 and document practical recommendations for keeping modules and test surfaces manageable.

This review is intended as a maintainability checkpoint, not as a hard policy that every file must satisfy.

## Method
The current repository tree was scanned using `rg --files -g '*.rs'`, then physical lines were counted per file.

The review reports two views:

- all Rust files in the repo
- implementation-focused files, excluding `adl/tests/` and generated demo output under `demos/`

LoC here means physical line count, not logical statements or non-blank lines.

## Summary
The largest Rust files in the repo are still dominated by large test surfaces plus a small number of very large implementation modules.

The current pattern suggests two different cleanup tracks:

- split very large test suites by feature or scenario family
- refactor very large implementation modules when they are carrying multiple responsibilities

## Repo-Wide Top Files

| Rank | File | LoC | Notes |
|---|---|---:|---|
| 1 | `adl/tests/execute_tests.rs` | 5929 | Largest file in repo; integration-style execution coverage |
| 2 | `adl/src/cli/tests.rs` | 2544 | Test-heavy source-adjacent module |
| 3 | `adl/src/adl.rs` | 2103 | Largest implementation module |
| 4 | `adl/tests/cli_smoke.rs` | 2028 | Large CLI scenario/smoke suite |
| 5 | `adl/src/execute/mod.rs` | 1768 | Large execution implementation module |
| 6 | `adl/src/cli/run_artifacts.rs` | 1674 | Large implementation module |
| 7 | `adl/src/remote_exec.rs` | 1587 | Large implementation module |
| 8 | `adl/src/instrumentation.rs` | 1327 | Large implementation module |
| 9 | `adl/src/learning_export.rs` | 1137 | Large implementation module |
| 10 | `adl/src/demo.rs` | 1047 | Large implementation module |

## Largest Implementation Modules

These rankings exclude `adl/tests/` and generated demo output under `demos/`.

| Rank | File | LoC | Recommendation Priority |
|---|---|---:|---|
| 1 | `adl/src/adl.rs` | 2103 | High |
| 2 | `adl/src/execute/mod.rs` | 1768 | High |
| 3 | `adl/src/cli/run_artifacts.rs` | 1674 | Medium |
| 4 | `adl/src/remote_exec.rs` | 1587 | Medium |
| 5 | `adl/src/instrumentation.rs` | 1327 | Medium |
| 6 | `adl/src/learning_export.rs` | 1137 | Medium |
| 7 | `adl/src/demo.rs` | 1047 | Low to medium |
| 8 | `adl/src/execute/runner.rs` | 995 | Watch |
| 9 | `adl/src/trace.rs` | 936 | Watch |
| 10 | `adl/src/signing.rs` | 934 | Watch |

## Interpretation
Rust practice generally optimizes for cohesion and navigability rather than a strict file-size ceiling.

Large files are not automatically wrong. They become a problem when they:

- mix several domains or workflows
- make reviews noisy because unrelated changes land in the same module
- require excessive scrolling to understand one behavior
- accumulate helper code that clearly wants to live in submodules
- blur public API boundaries and internal implementation details

The current repo profile indicates that test organization is the easiest near-term maintainability win, while a smaller set of implementation modules should be candidates for responsibility-based refactors.

## Guidance On Tests
For Rust, small `#[cfg(test)]` modules co-located with the code are still a good fit when tests need private access and remain tightly scoped.

Once tests grow into scenario-heavy, fixture-heavy, or CLI-heavy suites, the more maintainable pattern is to move them into dedicated test modules or integration files grouped by behavior.

Recommended policy:

- keep small unit tests close to the implementation
- move broad behavior, smoke, end-to-end, and fixture-heavy tests into `adl/tests/`
- split giant test files by feature family rather than by arbitrary size
- avoid keeping very large test suites inside parent implementation files unless private access is essential

## Guidance On Large Modules
The right trigger for refactoring is not only LoC. The stronger signal is whether a file still has one clear responsibility.

Recommended soft threshold:

- below about `800` LoC: usually fine if cohesive
- around `1000` to `1200` LoC: review for split opportunities
- above `1500` LoC: expect an explicit reason to keep it unified
- above `2000` LoC: strong candidate for decomposition unless the module is unusually cohesive

Recommended decomposition pattern:

- convert `foo.rs` into `foo/mod.rs` when the module naturally contains multiple sub-areas
- move coherent slices into files such as `foo/parser.rs`, `foo/runner.rs`, `foo/schema.rs`, `foo/tests.rs`, or similar domain-specific names
- keep `mod.rs` thin and focused on orchestration, exports, and top-level flow

## Recommendations For v0.85 Follow-Up

### 1. Split the largest test suites first
Primary candidates:

- `adl/tests/execute_tests.rs`
- `adl/tests/cli_smoke.rs`
- `adl/src/cli/tests.rs`

Suggested split strategy:

- group execution tests by execution mode, artifact handling, failure behavior, or replay surface
- group CLI smoke tests by command family or user workflow
- evaluate whether `adl/src/cli/tests.rs` should remain source-adjacent or move toward dedicated CLI test modules

This work should improve readability and reviewability with relatively low architectural risk.

### 2. Refactor the two largest implementation modules by responsibility
Primary candidates:

- `adl/src/adl.rs`
- `adl/src/execute/mod.rs`

These files should be reviewed for separable concerns such as:

- public API vs. internal orchestration
- data/model definitions vs. execution logic
- planning or validation helpers vs. runtime behavior
- command handling vs. artifact production

The goal is not to create many tiny files. The goal is to give each submodule a stable conceptual boundary.

### 3. Put the remaining large implementation files on a watch list
Files to monitor:

- `adl/src/cli/run_artifacts.rs`
- `adl/src/remote_exec.rs`
- `adl/src/instrumentation.rs`
- `adl/src/learning_export.rs`
- `adl/src/demo.rs`

These do not necessarily require immediate decomposition, but they should receive extra scrutiny when future changes expand their scope.

### 4. Adopt a lightweight maintainability guardrail
Recommended team rule:

- do not enforce a hard maximum file size in CI
- do flag files above roughly `1000` LoC for human review
- require a brief rationale when files exceed roughly `1500` LoC and continue growing
- prefer responsibility-driven refactors over size-driven churn

## Proposed Working Policy
The following policy is a reasonable default for this codebase:

1. Keep small private unit tests close to the implementation.
2. Move broad or scenario-heavy tests into dedicated integration-style files.
3. Treat files above `1000` LoC as review candidates, not automatic violations.
4. Treat files above `1500` to `2000` LoC as strong decomposition candidates unless they remain clearly cohesive.
5. Split modules by responsibility and navigability, not by arbitrary line-count quotas.

## Suggested Next Review Targets
If maintainability work is scheduled after v0.85, the best first-pass targets are:

1. `adl/tests/execute_tests.rs`
2. `adl/src/cli/tests.rs`
3. `adl/tests/cli_smoke.rs`
4. `adl/src/adl.rs`
5. `adl/src/execute/mod.rs`

## Command Reference
The analysis was produced from the current checkout using:

```sh
rg --files -g '*.rs'
```

Line counts were then computed from the resulting file list.
