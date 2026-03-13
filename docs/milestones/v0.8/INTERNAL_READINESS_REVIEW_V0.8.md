# v0.8 Internal Readiness Review

Date: 2026-03-13
Scope: Internal readiness gate before external third-party review

This document reflects the current repository truth after the recovery-tail refresh. It is an internal blocker-oriented review artifact, not a claim that v0.8 is ready for external handoff.

## Recommendation

Recommendation: not yet ready for `#707`

The repository now contains real bounded v0.8 runtime/demo work, and the main branch version story is mostly coherent, but the external review packet still has two material blockers: mixed reviewer-facing version language and the absence of the final third-party review artifact.

## Evidence Base

Observed directly from:
- `docs/milestones/v0.8/README.md`
- `docs/milestones/v0.8/RECOVERY_AUDIT_V0.8.md`
- `docs/milestones/v0.8/DEMOS_V0.8.md`
- `docs/milestones/v0.8/DOCS_CONVERGENCE_V0.8.md`
- `docs/milestones/v0.8/RUST_TRANSPILER_DEMO.md`
- `docs/milestones/v0.8/RUST_TRANSPILER_VERIFICATION_V0.8.md`
- `README.md`
- `swarm/Cargo.toml`
- `swarm/README.md`
- runtime/demo surfaces under `swarm/src/godel/`, `swarm/src/demo.rs`, `tools/transpiler_demo/`, `examples/`, and `demos/`

Validated by:
- `cargo test --manifest-path swarm/Cargo.toml --workspace`
- host-path and stale planning-path leakage scan across milestone and README surfaces
- file existence/path checks for current review-packet docs and runnable demo surfaces

- from the runtime crate directory: `cargo test --workspace`
- `cd tools/transpiler_demo && cargo run --quiet`
- `rg -n '\{\{[^}]+\}\}' docs/milestones/v0.8 docs/tooling README.md */README.md`
- host-path leakage scan across milestone and tooling docs
## Current Positive Signals

## Current Positive Signals

### Observed facts
- `cargo test --manifest-path swarm/Cargo.toml --workspace` passes on this branch.
- The repo contains real bounded Gödel runtime code, deterministic tests, ObsMem integration surfaces, and runnable demo paths.
- The transpiler scaffold remains runnable under `tools/transpiler_demo/`.

### Inferred conclusion
- v0.8 has enough real implementation to justify serious external review soon.

## Remaining Blockers For External Review

### Blocker 1: Final third-party review packet artifact is still missing

Observed facts:
- `swarm/Cargo.toml` declares `version = "0.8.0"`.
- Root `README.md` presents active development milestone `v0.8` and latest released milestone `v0.7.0`.
- `swarm/README.md` still presents the runtime as `v0.7.0`.

Inferred conclusion:
- An external reviewer still cannot read the runtime/version story cleanly from all reader-facing surfaces.

Recommended action:
- Reconcile `swarm/README.md` with the current `0.8.0` development/runtime story before review-readiness claims are made.

### Blocker 2: Final third-party handoff artifact is still missing

Observed facts:
- `docs/milestones/v0.8/THIRD_PARTY_REVIEW_V0.8.md` is absent on this branch.
- Review-tail docs exist, but the final external-review artifact itself is still missing.

Inferred conclusion:
- The external review packet is still incomplete even after the recovery-tail refresh.

Recommended action:
- Add the final third-party review artifact after the packet and version story are aligned.

## Non-Blocker Findings

### Finding 1: Demo/review packet is clearer than the earlier recovery state

Observed facts:
- The review packet now distinguishes runnable demos from inspect-only surfaces.
- `DEMOS_V0.8.md` and `README.md` provide explicit reviewer entry points.

Inferred conclusion:
- The earlier ambiguity about "what do I run?" versus "what do I inspect?" is substantially reduced.

### Finding 2: The repo is no longer accurately described as mostly docs-only

Observed facts:
- Runtime modules, deterministic tests, and runnable bounded demos are present now.

Inferred conclusion:
- Review-tail docs should describe v0.8 as a mixed milestone with real bounded runtime/demo code plus contract/spec surfaces.

## Additional Internal Findings

### Finding 3: Runtime evidence is stronger than some planning docs admit

Observed facts:
- The stage-loop, experiment-record, and Gödel index modules contain real bounded runtime and persistence/indexing code.
- `cargo test --workspace` passes from the runtime crate directory, including deterministic tests around stage ordering, mutation/hypothesis selection, record persistence, and ObsMem indexing surfaces.

Inferred conclusion:
- The codebase has crossed from pure spec work into bounded executable runtime territory.

Recommended action:
- Keep review docs describing this as "implemented bounded runtime surfaces" rather than "schema-only milestone."

### Finding 4: Demo matrix terminology is still somewhat misleading

Observed facts:
- `docs/milestones/v0.8/DEMOS_V0.8.md` uses "demo" for both runnable proof surfaces and schema/doc inspection rows.

Inferred conclusion:
- The matrix is useful internally, but reviewer-facing terminology is not yet crisp.

Recommended action:
- Rename rows or add a column that distinguishes `runnable_demo` from `doc_review_surface`.

## Ready / Not Ready Decision

Decision: not yet ready for `#707`

Minimum unblock set:

1. Add or restore the final third-party review handoff artifact.
2. Keep review-tail docs synchronized with the reconciled version story and current code/demo truth.
3. Clarify the external review entry path by separating runnable demos from doc/spec validation rows.

## Recommended Next Actions

1. Reconcile `swarm/README.md` with the current `0.8.0` unreleased-development story.
2. Add or restore `THIRD_PARTY_REVIEW_V0.8.md`.
3. Refresh `RECOVERY_AUDIT_V0.8.md` and `DOCS_CONVERGENCE_V0.8.md` whenever the review packet changes again.
4. Keep the reviewer entry path explicit so an external reviewer can answer "what do I run?" in one pass.
