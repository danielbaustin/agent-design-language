# v0.8 Internal Readiness Review

Date: 2026-03-12
Scope: Internal readiness gate before external third-party review

## Recommendation

Recommendation: not yet ready for `#707`

The repository now presents a coherent version story for `main`, but the external review packet is still not complete enough for a serious third-party pass.

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
- runtime crate `Cargo.toml`
- runtime crate `README.md`
- runtime demo module
- runtime CLI usage module
- Gödel runtime status module
- Gödel stage-loop module
- Gödel experiment-record module
- card prompt generation script
- prompt-spec lint script

Validated by:

- from the runtime crate directory: `cargo test --workspace`
- `cd tools/transpiler_demo && cargo run --quiet`
- `rg -n '\{\{[^}]+\}\}' docs/milestones/v0.8 docs/tooling README.md */README.md`
- host-path leakage scan across milestone and tooling docs

## Blockers For External Review

### Blocker 1: Final third-party review packet artifact is still missing

Observed facts:

- `docs/milestones/v0.8/THIRD_PARTY_REVIEW_V0.8.md` is absent on this branch.
- Review-tail docs exist, but the final external-review artifact itself is still missing.

Inferred conclusion:

- The repo cannot honestly claim the full third-party review packet is ready while its explicit handoff artifact is absent.

Recommended action:

- Add or restore the final external-review handoff artifact before making review-readiness claims.

### Blocker 2: Review-readiness docs must stay aligned with the refreshed version story and current repo truth

Observed facts:

- `README.md` presents the latest tagged release as `v0.7.0` and active development as `v0.8`.
- `swarm/Cargo.toml` declares `version = "0.8.0"`.
- Reviewer-facing docs must explicitly frame this as unreleased `main`-branch v0.8 work rather than a shipped v0.8 release.

Inferred conclusion:

- Review packet truth now depends on keeping every reviewer-facing surface aligned to the same unreleased-v0.8 story.

Recommended action:

- Keep recovery/readiness docs synchronized with the README/manifest story and avoid slipping back into mixed-release wording.

### Blocker 3: Demo and review entry surfaces are still confusing

Observed facts:

- `docs/milestones/v0.8/DEMOS_V0.8.md` calls several required rows "demos" even when they are really doc/spec verification surfaces (`D8-01`, `D8-02`, `D8-03`, `D8-05`).
- Only the transpiler row (`D8-04`) presents a clearly runnable command.
- The runtime demo module exposes a real `demo-c-godel-runtime`, but that path is not surfaced as a canonical required demo in `DEMOS_V0.8.md`.

Inferred conclusion:

- An external reviewer will not get a clean answer to "what can I run?" versus "what is a schema/spec review surface?".

Recommended action:

- Separate runnable demos from doc/spec inspection rows and give each required review surface an explicit entry path.

## Architecture Findings

### A1. Demo ownership is too centralized in one file

Observed facts:

- The runtime demo module contains three unrelated demo families, inline artifact templates, file-writing logic, and test helpers.

Inferred conclusion:

- Demo logic has started to accumulate as a catch-all module rather than a set of clearly bounded demo surfaces.

Recommended action:

- Split by demo family or artifact responsibility before this grows further. This is not a pre-review blocker by itself, but it is visible structural drift.

### A2. `godel_runtime` is really a milestone-surface validator

Observed facts:

- The Gödel runtime status module loads milestone JSON artifacts from `docs/milestones/v0.8/` and validates stage order / cross-links.
- Actual bounded runtime behavior lives under the dedicated runtime Gödel modules.

Inferred conclusion:

- The `godel_runtime` name overstates what the module does today. It is a bounded validation/status surface, not the primary Gödel executor.

Recommended action:

- Either rename the module in a later cleanup pass or document the boundary more explicitly in reviewer-facing docs.

## Code Findings

### C1. CLI usage surface is carrying too much historical weight

Observed facts:

- The runtime CLI usage surface mixes current commands with multiple historical example families and compatibility notes in one long help surface.

Inferred conclusion:

- The CLI is still workable, but the presentation is more crowded than it should be for third-party review.

Recommended action:

- Tighten help text or split "current" versus "historical compatibility" examples in a follow-up pass.

### C2. Reviewer-tooling shell scripts are effective but brittle

Observed facts:

- The card prompt and prompt-spec lint scripts parse markdown/YAML structure with `awk` and regex-oriented shell logic.

Inferred conclusion:

- This is acceptable for bounded tooling, but it is a fragile surface if card format grows further.

Recommended action:

- Keep the current scripts for v0.8, but treat them as a future hardening target rather than a stable long-term parser architecture.

### C3. Runtime evidence is stronger than some planning docs admit

Observed facts:

- The stage-loop, experiment-record, and Gödel index modules contain real bounded runtime and persistence/indexing code.
- `cargo test --workspace` passes from the runtime crate directory, including deterministic tests around stage ordering, mutation/hypothesis selection, record persistence, and ObsMem indexing surfaces.

Inferred conclusion:

- The codebase has crossed from pure spec work into bounded executable runtime territory.

Recommended action:

- Make review docs describe this as "implemented bounded runtime surfaces" rather than "schema-only milestone" where appropriate.

## Documentation Findings

### D1. Recovery audit is stale

Observed facts:

- `docs/milestones/v0.8/RECOVERY_AUDIT_V0.8.md` began as an earlier recovery snapshot.
- Current repo state includes concrete runtime Gödel modules and direct test coverage for them, so the audit must stay refreshed if it remains part of the review packet.

Inferred conclusion:

- The audit is now useful historically but no longer accurate as a current-state briefing.

Recommended action:

- Update or supersede it before using it as a review input.

### D2. The convergence doc is too optimistic

Observed facts:

- `docs/milestones/v0.8/DOCS_CONVERGENCE_V0.8.md` previously said the docs were converged for handoff.
- The branch still lacks a `THIRD_PARTY_REVIEW_V0.8.md` artifact.

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

- `cargo test --workspace` passes from the runtime crate directory.
- `cd tools/transpiler_demo && cargo run --quiet` passes and emits a deterministic mapping result.
- The repo contains real bounded Gödel runtime code, deterministic tests, ObsMem integration surfaces, and a reviewer/tooling contract set under `docs/tooling/`.

Inferred conclusion:

- v0.8 is not empty or decorative. There is enough real implementation to justify external review soon, but not with the current milestone-truth packet.

## Ready / Not Ready Decision

Decision: not yet ready for `#707`

Minimum unblock set:

1. Add or restore the final third-party review handoff artifact.
2. Keep review-tail docs synchronized with the reconciled version story and current code/demo truth.
3. Clarify the external review entry path by separating runnable demos from doc/spec validation rows.

## Recommended Next Actions

1. Add or restore `THIRD_PARTY_REVIEW_V0.8.md`.
2. Refresh `RECOVERY_AUDIT_V0.8.md` and `DOCS_CONVERGENCE_V0.8.md` against current runtime truth whenever the review packet changes again.
3. Add or restore the missing third-party review packet artifact before calling the repo externally review-ready.
4. Simplify demo/review navigation so an external reviewer can answer "what do I run?" in one pass.
