# v0.8 Internal Readiness Review

Date: 2026-03-14
Scope: Internal readiness gate before external third-party review

This document reflects the current repository truth after the recovery-tail refresh. It is an internal blocker-oriented review artifact, not a claim that v0.8 is ready for external handoff.

## Recommendation

Recommendation: not yet ready for `#707`

The repository now contains real bounded v0.8 runtime/demo work, the main-branch version story is coherent, and the reviewer demo entry path is materially better than it was during recovery. The external review packet still has two material blockers: the absence of the final third-party review artifact and the lack of a simplified final handoff surface for an outside reviewer.

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
- runtime/demo surfaces under `swarm/src/godel/`, `swarm/src/demo.rs`, `tools/transpiler_demo/`, `examples/`, and `demos/`
- `demos/godel_failure_hypothesis_experiment.md`
- `demos/aee-recovery/README.md`

Validated by:
- `cargo test --manifest-path swarm/Cargo.toml --workspace`
- host-path and stale planning-path leakage scan across milestone and README surfaces
- file existence/path checks for current review-packet docs and runnable demo surfaces

- from the runtime crate directory: `cargo test --workspace`
- `cd tools/transpiler_demo && cargo run --quiet`
- `rg -n '\{\{[^}]+\}\}' docs/milestones/v0.8 docs/tooling README.md */README.md`
- host-path leakage scan across milestone and tooling docs

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
- `docs/milestones/v0.8/THIRD_PARTY_REVIEW_V0.8.md` is absent on this branch.
- Review-tail docs exist, but the final external-review artifact itself is still missing.

Inferred conclusion:
- The external review packet is still incomplete even after the catch-up implementation wave and docs refresh.

Recommended action:
- Add the final third-party review artifact after the packet and reviewer path are aligned.

### Blocker 2: Final external entry flow still needs one simplified handoff surface

Observed facts:
- Runnable demos now exist under `demos/`, including:
  - `demos/godel_failure_hypothesis_experiment.md`
  - `demos/aee-recovery/README.md`
- The packet still spans runnable demos, inspect-only schema surfaces, and planning docs across several files.

Inferred conclusion:
- An external reviewer can now find the real runtime/demo surfaces, but the final handoff still requires a compact, opinionated entry artifact rather than a multi-doc packet alone.

Recommended action:
- Add the final third-party handoff artifact and use it to separate "run this" from "inspect this" in one place.

## Non-Blocker Findings

### Finding 1: Demo/review packet is clearer than the earlier recovery state

Observed facts:
- The review packet now distinguishes runnable demos from inspect-only surfaces.
- `DEMOS_V0.8.md` and `README.md` provide explicit reviewer entry points.
- `demos/README.md` now acts as the canonical user-facing demo index, with dedicated runbooks for the bounded Gödel CLI and AEE recovery paths.

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

### Finding 4: Demo matrix still needs final external-facing simplification

Observed facts:
- `docs/milestones/v0.8/DEMOS_V0.8.md` intentionally mixes runnable demos with inspect-only review surfaces.
- That is useful for internal review, but it is still denser than an outside reviewer ideally needs.

Inferred conclusion:
- The matrix is useful internally, but the final external handoff should still flatten the path further.

Recommended action:
- Keep the matrix as the canonical internal review surface, but create a simpler external handoff doc for `#707`.

## Ready / Not Ready Decision

Decision: not yet ready for `#707`

Minimum unblock set:

1. Add or restore the final third-party review handoff artifact.
2. Keep review-tail docs synchronized with the current code/demo truth.
3. Clarify the external review entry path by separating runnable demos from doc/spec validation rows in the final handoff artifact.

## Recommended Next Actions

1. Add or restore `THIRD_PARTY_REVIEW_V0.8.md`.
2. Refresh `RECOVERY_AUDIT_V0.8.md` and `DOCS_CONVERGENCE_V0.8.md` whenever the review packet changes again.
3. Keep the reviewer entry path explicit so an external reviewer can answer "what do I run?" in one pass.
