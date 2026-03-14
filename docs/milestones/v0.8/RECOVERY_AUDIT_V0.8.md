# v0.8 Recovery Audit (Repository Truth)

Date: 2026-03-13
Scope: Repository-state audit and recovery-truth refresh for the current v0.8 review packet.

This document supersedes earlier recovery assumptions that predated the current bounded runtime and demo work. Repository truth is authoritative.

## Current Repository State

### Observed facts
- Canonical v0.8 docs directory is populated under `docs/milestones/v0.8/` with schema/spec docs, planning docs, and release-tail docs.
- Root `README.md` presents the latest tagged release as `v0.7.0` and the active development milestone as `v0.8`.
- Runtime crate manifest is `0.8.0` (`swarm/Cargo.toml`).
- Runtime README is a reviewer-facing surface for current runtime behavior, but some wording still lags the unreleased-v0.8 story.
- The repository contains real bounded v0.8 runtime/demo surfaces under `swarm/src/godel/`, `swarm/src/demo.rs`, `tools/transpiler_demo/`, `examples/`, and `demos/`.
- `cargo test --manifest-path swarm/Cargo.toml --workspace` passes on this branch.
- `docs/milestones/v0.8/THIRD_PARTY_REVIEW_V0.8.md` is still absent.
- v0.8 review-tail docs are present but still include pre-refresh recovery/readiness language in several places.
- Issue/PR state in the review tail is not fully converged:
  - `#707` open (PR open)
  - `#708` open (no PR)
  - `#668` open (no PR)

### Inferred conclusions
- v0.8 is no longer a docs-only milestone surface. The repository now contains bounded executable runtime/demo behavior in addition to schema/spec docs.
- v0.8 documentation surfaces exist and now sit on top of a real bounded runtime/demo base, but the milestone is not yet in a release-ready or external-review-ready state.
- The correct repository story is: latest tagged release `v0.7.0`, active development milestone `v0.8`, current `main`-branch runtime crate version `0.8.0`.
- The main remaining reviewer-facing inconsistency is not "missing runtime everywhere" but a mixed review-tail packet: some docs still describe earlier recovery state while others reflect newer bounded implementation.

## Implemented Features

### Observed facts
The following surfaces are implemented in executable code on this branch:
- Bounded Gödel runtime surfaces under `swarm/src/godel/`:
  - `stage_loop.rs`
  - `hypothesis.rs`
  - `mutation.rs`
  - `evaluation.rs`
  - `experiment_record.rs`
  - `obsmem_index.rs`
- A bounded milestone-surface validator in `swarm/src/godel_runtime.rs`.
- Runnable bounded demo surfaces in `swarm/src/demo.rs`:
  - `demo-c-godel-runtime`
  - `demo-d-godel-obsmem-loop`
  - `demo-e-multi-agent-card-pipeline`
  - `demo-f-obsmem-retrieval`
- Rust transpiler scaffold surfaces:
  - `tools/transpiler_demo/Cargo.toml`
  - `tools/transpiler_demo/src/main.rs`
  - `demos/rust-transpiler/workflow/rust_transpiler_demo.yaml`
  - `demos/rust-transpiler/output/workflow_runtime.rs`
  - `demos/rust-transpiler/output/transpiler_verification.v0.8.json`
- Runtime/demo tests covering bounded v0.8 behavior, including:
  - `swarm/tests/demo_tests.rs`
  - `swarm/tests/obsmem_validation_tests.rs`
  - `godel::*` unit tests in the runtime crate.

### Inferred conclusions
- The repository contains enough real v0.8 runtime/demo behavior to justify external review soon.
- The correct reviewer description is now "bounded implemented runtime plus spec/docs surfaces," not "mostly unimplemented runtime."
- The milestone still mixes implemented bounded runtime/demo surfaces with spec-only contracts and planning docs.

## Documentation/Spec-Heavy Surfaces

### Observed facts
The following remain primarily schema/spec or planning surfaces:
- Schema spine and canonical examples under `docs/milestones/v0.8/`:
  - `CANONICAL_EVIDENCE_VIEW_V1.md`
  - `MUTATION_FORMAT_V1.md`
  - `EVALUATION_PLAN_V1.md`
  - `EXPERIMENT_RECORD_V1.md`
  - `OBSMEM_INDEXING_SURFACES_V1.md`
  - associated `*.json` schema/example files
- Review/planning/order docs such as:
  - `EXECUTION_ORDER_V0.8.md`
  - `GODEL_SCHEMA_DELIVERY_ORDER_V0.8.md`
  - `AUTHORING_DELIVERY_ORDER_V0.8.md`
  - `BOUNDED_AEE_V1_SCOPE_V0.8.md`
  - `QUALITY_GATE_V0.8.md`

### Inferred conclusions
- v0.8 is still a mixed milestone: implemented bounded code paths plus contract/spec/planning surfaces.
- Review docs must preserve that distinction explicitly instead of collapsing everything into either "implemented" or "missing."

## Remaining Gaps / Missing Review-Tail Work

### Observed facts
- Some v0.8 schema/spec artifacts still do not have full runtime ingestion/execution wiring, including:
  - `experiment_record.v1`
  - `canonical_evidence_view.v1`
  - `mutation.v1`
  - `evaluation_plan.v1`
  - `godel_experiment_workflow.template.v1`
  - `experiment_index_entry.v1`
  - `tool_result.v1`
- `docs/milestones/v0.8/DEMOS_V0.8.md` and `docs/milestones/v0.8/QUALITY_GATE_V0.8.md` are still reviewer/planning surfaces rather than fully enforced CI/release gates.
- Third-party review artifact file `docs/milestones/v0.8/THIRD_PARTY_REVIEW_V0.8.md` is absent on current branch state; issue `#707` remains open.
- `swarm/README.md` still presents the runtime as a `v0.7.0` release surface while `swarm/Cargo.toml` declares `0.8.0`.
- Reviewer-entry docs require explicit run-vs-inspect guidance to avoid confusion about which surfaces are runnable demos versus inspect-only review artifacts.

### Inferred conclusions
- Remaining v0.8 gaps are about completing and hardening integration, not about proving the milestone has zero executable runtime value.
- Demo and quality-gate surfaces are documented, but not yet fully enforced as deterministic runtime/CI gates.
- The primary remaining gaps are review-packet clarity and version-truth alignment, not wholesale absence of runtime work.
- v0.8 should not be presented as release-ready yet, but it also should not be described as mostly unimplemented.

## Version Inconsistencies

### Observed facts
- `swarm/Cargo.toml` declares `version = "0.8.0"`.
- Root `README.md` says:
  - latest released milestone: v0.7.0
  - active development milestone: v0.8
- `swarm/README.md` still says `Current runtime release: v0.7.0`.
- Reviewer-facing v0.8 docs should describe `main` as unreleased v0.8 work on top of the latest tagged v0.7.0 release.
- v0.8 docs are present and active in repository planning/execution artifacts.

### Inferred conclusions
- The coherent version story is available, but every reviewer-facing surface must keep using it consistently.
- The root README and manifest together describe an unreleased `v0.8` development branch reasonably clearly.
- The runtime README lags behind that story and remains a direct reviewer-facing inconsistency.

## Recommended Recovery Plan to Complete v0.8

### Guiding principle
Repository truth is authoritative. Review-tail docs should describe the bounded runtime/demo work that exists now while remaining explicit about deferred surfaces and unresolved blockers.

### Minimal deterministic recovery sequence
1. **Stabilize milestone truth surfaces (docs-only, short cycle)**
   - Keep README/manifests/reviewer-facing docs aligned on one story:
     - latest tagged release: `v0.7.0`
     - active milestone on `main`: `v0.8`
     - runtime crate version on `main`: `0.8.0`
   - Avoid release wording that implies v0.8 is already shipped.

2. **Close review-tail gaps before new feature expansion**
   - Merge/complete `#707` and `#708` with explicit fixed-vs-deferred findings.
   - Complete missing planning integration issue `#668` or defer explicitly with rationale.
   - Reconcile `swarm/README.md` with the current `0.8.0` development/runtime story before third-party review claims are made.
   - Restore or create the final `THIRD_PARTY_REVIEW_V0.8.md` handoff artifact.

3. **Restore / prepare the external review artifact**
   - Add or restore `THIRD_PARTY_REVIEW_V0.8.md` as the final handoff packet once the review-tail docs and version story are aligned.

4. **Preserve implemented-versus-deferred honesty**
   - Keep bounded implemented runtime/demo surfaces explicit.
   - Keep broader autonomy / learning / future authoring ambitions clearly deferred.

## Evidence Commands Used
- `gh` issue/PR metadata queries for issue/merge state and changed-file classification.
- Repository content checks:
  - `rg` scans across README/manifests/workflows/runtime/docs
  - directory/file existence checks under `docs/milestones/v0.8/`
- `cargo test --manifest-path swarm/Cargo.toml --workspace`
- `rg` scans across `swarm/`, `demos/`, `examples/`, `tools/`, and `docs/milestones/v0.8/`
- file existence checks for review-tail packet artifacts
