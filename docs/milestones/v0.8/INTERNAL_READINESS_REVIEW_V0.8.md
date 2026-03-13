# v0.8 Internal Readiness Review

Date: 2026-03-12
Scope: Internal readiness gate before external third-party review

## Recommendation

Recommendation: not yet ready for `#707`

The repository contains real bounded v0.8 implementation work, especially in the Gödel runtime scaffolding and transpiler demo thread, but the external review surface is still not coherent enough for a serious third-party pass.

## Evidence Base

Observed directly from:

- `docs/milestones/v0.8/README.md`
- `docs/milestones/v0.8/RECOVERY_AUDIT_V0.8.md`
- `docs/milestones/v0.8/DEMOS_V0.8.md`
- `docs/milestones/v0.8/DOCS_CONVERGENCE_V0.8.md`
- `docs/milestones/v0.8/QUALITY_GATE_V0.8.md`
- `docs/milestones/v0.8/RUST_TRANSPILER_DEMO.md`
- `docs/milestones/v0.8/RUST_TRANSPILER_VERIFICATION_V0.8.md`
- `README.md`
- `swarm/README.md`
- `swarm/Cargo.toml`
- `swarm/src/demo.rs`
- `swarm/src/cli/usage.rs`
- `swarm/src/godel_runtime.rs`
- `swarm/src/godel/stage_loop.rs`
- `swarm/src/godel/experiment_record.rs`
- `swarm/tools/card_prompt.sh`
- `swarm/tools/lint_prompt_spec.sh`

Validated by:

- `cd swarm && cargo test --workspace`
- `cd tools/transpiler_demo && cargo run --quiet`
- `rg -n '\{\{[^}]+\}\}' docs/milestones/v0.8 docs/tooling README.md swarm/README.md`
- `rg -n '\.adl/docs/v08planning|/Users/|/home/' docs/milestones/v0.8 docs/tooling README.md swarm/README.md`

## Blockers For External Review

### Blocker 1: Version story is internally inconsistent

Observed facts:

- `swarm/Cargo.toml` declares `version = "0.8.0"`.
- `README.md` says latest released milestone is `v0.7.0`.
- `swarm/README.md` says current runtime release is `v0.7.0`.

Inferred conclusion:

- An external reviewer cannot tell whether `main` is presenting a released `0.7.0` runtime, an unreleased `0.8.0` runtime, or a mixed milestone branch.

Recommended action:

- Reconcile manifest versioning and release-status language before external review claims are made.

### Blocker 2: Review-readiness docs overstate convergence

Observed facts:

- `docs/milestones/v0.8/DOCS_CONVERGENCE_V0.8.md` says `Status: Converged for review handoff`.
- `docs/milestones/v0.8/THIRD_PARTY_REVIEW_V0.8.md` is absent on this branch.
- `docs/milestones/v0.8/RECOVERY_AUDIT_V0.8.md` still claims large portions of Gödel runtime work are unimplemented, which is now stale relative to `swarm/src/godel/` and related tests.

Inferred conclusion:

- The review-tail documents are no longer aligned with repository truth and currently over-claim readiness while also under-reporting newly landed runtime work.

Recommended action:

- Refresh the recovery/review packet so it matches the actual current repo before asking a third party to evaluate it.

### Blocker 3: Demo and review entry surfaces are still confusing

Observed facts:

- `docs/milestones/v0.8/DEMOS_V0.8.md` calls several required rows "demos" even when they are really doc/spec verification surfaces (`D8-01`, `D8-02`, `D8-03`, `D8-05`).
- Only the transpiler row (`D8-04`) presents a clearly runnable command.
- `swarm/src/demo.rs` exposes a real `demo-c-godel-runtime`, but that path is not surfaced as a canonical required demo in `DEMOS_V0.8.md`.

Inferred conclusion:

- An external reviewer will not get a clean answer to "what can I run?" versus "what is a schema/spec review surface?".

Recommended action:

- Separate runnable demos from doc/spec inspection rows and give each required review surface an explicit entry path.

## Architecture Findings

### A1. Demo ownership is too centralized in one file

Observed facts:

- `swarm/src/demo.rs` contains three unrelated demo families, inline artifact templates, file-writing logic, and test helpers.

Inferred conclusion:

- Demo logic has started to accumulate as a catch-all module rather than a set of clearly bounded demo surfaces.

Recommended action:

- Split by demo family or artifact responsibility before this grows further. This is not a pre-review blocker by itself, but it is visible structural drift.

### A2. `godel_runtime` is really a milestone-surface validator

Observed facts:

- `swarm/src/godel_runtime.rs` loads milestone JSON artifacts from `docs/milestones/v0.8/` and validates stage order / cross-links.
- Actual bounded runtime behavior lives under `swarm/src/godel/`.

Inferred conclusion:

- The `godel_runtime` name overstates what the module does today. It is a bounded validation/status surface, not the primary Gödel executor.

Recommended action:

- Either rename the module in a later cleanup pass or document the boundary more explicitly in reviewer-facing docs.

## Code Findings

### C1. CLI usage surface is carrying too much historical weight

Observed facts:

- `swarm/src/cli/usage.rs` mixes current commands with multiple historical example families and compatibility notes in one long help surface.

Inferred conclusion:

- The CLI is still workable, but the presentation is more crowded than it should be for third-party review.

Recommended action:

- Tighten help text or split "current" versus "historical compatibility" examples in a follow-up pass.

### C2. Reviewer-tooling shell scripts are effective but brittle

Observed facts:

- `swarm/tools/card_prompt.sh` and `swarm/tools/lint_prompt_spec.sh` parse markdown/YAML structure with `awk` and regex-oriented shell logic.

Inferred conclusion:

- This is acceptable for bounded tooling, but it is a fragile surface if card format grows further.

Recommended action:

- Keep the current scripts for v0.8, but treat them as a future hardening target rather than a stable long-term parser architecture.

### C3. Runtime evidence is stronger than some planning docs admit

Observed facts:

- `swarm/src/godel/stage_loop.rs`, `swarm/src/godel/experiment_record.rs`, and `swarm/src/godel/obsmem_index.rs` contain real bounded runtime and persistence/indexing code.
- `cargo test --workspace` passes, including deterministic tests around stage ordering, mutation/hypothesis selection, record persistence, and ObsMem indexing surfaces.

Inferred conclusion:

- The codebase has crossed from pure spec work into bounded executable runtime territory.

Recommended action:

- Make review docs describe this as "implemented bounded runtime surfaces" rather than "schema-only milestone" where appropriate.

## Documentation Findings

### D1. Recovery audit is stale

Observed facts:

- `docs/milestones/v0.8/RECOVERY_AUDIT_V0.8.md` still says large portions of v0.8 runtime integration are missing.
- Current repo state includes concrete runtime files under `swarm/src/godel/`, `swarm/src/godel_runtime.rs`, and test coverage for them.

Inferred conclusion:

- The audit is now useful historically but no longer accurate as a current-state briefing.

Recommended action:

- Update or supersede it before using it as a review input.

### D2. The convergence doc is too optimistic

Observed facts:

- `docs/milestones/v0.8/DOCS_CONVERGENCE_V0.8.md` says the docs are converged for handoff.
- The branch still lacks a `THIRD_PARTY_REVIEW_V0.8.md` artifact and still has a broken version story.

Inferred conclusion:

- The convergence claim is premature.

Recommended action:

- Downgrade the claim or refresh the handoff packet after blockers are resolved.

### D3. Demo matrix terminology is misleading

Observed facts:

- `docs/milestones/v0.8/DEMOS_V0.8.md` uses "demo" for both runnable proof surfaces and schema/doc inspection rows.

Inferred conclusion:

- The matrix is useful internally, but reviewer-facing terminology is not yet crisp.

Recommended action:

- Rename rows or add a column that distinguishes `runnable_demo` from `doc_review_surface`.

## Positive Signals

Observed facts:

- `cd swarm && cargo test --workspace` passes.
- `cd tools/transpiler_demo && cargo run --quiet` passes and emits a deterministic mapping result.
- The repo contains real bounded Gödel runtime code, deterministic tests, ObsMem integration surfaces, and a reviewer/tooling contract set under `docs/tooling/`.

Inferred conclusion:

- v0.8 is not empty or decorative. There is enough real implementation to justify external review soon, but not with the current milestone-truth packet.

## Ready / Not Ready Decision

Decision: not yet ready for `#707`

Minimum unblock set:

1. Reconcile version-bearing manifests and README/release-status language.
2. Refresh review-tail docs so recovery/convergence claims match current code and missing artifacts.
3. Clarify the external review entry path by separating runnable demos from doc/spec validation rows.

## Recommended Next Actions

1. Fix the version story first; it is the fastest high-confidence blocker to remove.
2. Refresh `RECOVERY_AUDIT_V0.8.md` and `DOCS_CONVERGENCE_V0.8.md` against current runtime truth.
3. Add or restore the missing third-party review packet artifact before calling the repo externally review-ready.
4. Simplify demo/review navigation so an external reviewer can answer "what do I run?" in one pass.
