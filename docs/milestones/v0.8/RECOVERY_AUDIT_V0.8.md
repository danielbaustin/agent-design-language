# v0.8 Recovery Audit (Repository Truth)

Date: 2026-03-10
Scope: Repository-state audit and recovery planning only.

## Current Repository State

### Observed facts
- Canonical v0.8 docs directory is populated under `docs/milestones/v0.8/` with schema/spec docs, planning docs, and release-tail docs.
- Root `README.md` presents the latest tagged release as `v0.7.0` and the active development milestone as `v0.8`.
- Runtime crate manifest is `0.8.0` (`swarm/Cargo.toml`).
- Runtime README is intended to describe current `main`-branch runtime surfaces, not a separate shipped release line.
- v0.8 review-tail docs are present but still include pre-refresh recovery/readiness language in several places.
- Issue/PR state in the review tail is not fully converged:
  - `#707` open (PR open)
  - `#708` open (no PR)
  - `#668` open (no PR)

### Inferred conclusions
- v0.8 documentation surfaces exist and now sit on top of a real bounded runtime/demo base, but the milestone is not yet in a release-ready or external-review-ready state.
- The correct repository story is: latest tagged release `v0.7.0`, active development milestone `v0.8`, current `main`-branch runtime crate version `0.8.0`.

## Implemented Features

### Observed facts
- Issues `#609`-`#616`, `#664`-`#669` (except `#668`), `#677`, `#683`, `#704`-`#706` were merged primarily as docs/spec artifacts.
- `#702` and `#703` added real code artifacts for the transpiler demo scaffold:
  - `tools/transpiler_demo/Cargo.toml`
  - `tools/transpiler_demo/src/main.rs`
  - `examples/workflows/rust_transpiler_demo.yaml`
  - `demos/rust_output/workflow_runtime.rs`
  - `demos/rust_output/transpiler_verification.v0.8.json`
- The repository now also contains bounded Gödel runtime/demo surfaces under `swarm/src/godel/` and corresponding runtime demos exercised through the `adl demo` entrypoints.

### Inferred conclusions
- v0.8 is no longer purely a design-stage milestone in code terms.
- The milestone still mixes implemented bounded runtime/demo surfaces with spec-only contracts and planning docs.

## Documentation/Spec-Only Features

### Observed facts
Merged issue set delivered these as docs/spec-only surfaces:
- Gödel schema spine: `#609`, `#610`, `#611`, `#612`, `#683`
- Gödel workflow/integration docs: `#613`, `#615`, `#616`
- ObsMem indexing surfaces docs: `#614`
- Planning/order/boundaries: `#664`, `#665`, `#666`, `#667`, `#669`, `#677`
- Release-tail docs: `#704`, `#705`, `#706`

### Inferred conclusions
- “Implemented” in many issue closeouts currently means “specified in docs,” not “wired into runtime/tested behavior.”

## Missing Runtime Work

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

### Inferred conclusions
- Remaining v0.8 gaps are about completing and hardening integration, not about proving the milestone has zero executable runtime value.
- Demo and quality-gate surfaces are documented, but not yet fully enforced as deterministic runtime/CI gates.

## Version Inconsistencies

### Observed facts
- `swarm/Cargo.toml` declares `version = "0.8.0"`.
- Root `README.md` says:
  - latest released milestone: v0.7.0
  - active development milestone: v0.8
- Reviewer-facing v0.8 docs should describe `main` as unreleased v0.8 work on top of the latest tagged v0.7.0 release.
- v0.8 docs are present and active in repository planning/execution artifacts.

### Inferred conclusions
- The coherent version story is available, but every reviewer-facing surface must keep using it consistently.
- The external-review blocker is no longer “what version are we?” but “is the refreshed review packet complete and current?”

## Recommended Recovery Plan to Complete v0.8

### Guiding principle
Repository truth is authoritative. Keep recovery deterministic, bounded, and sequence-driven.

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
   - Restore or create the final `THIRD_PARTY_REVIEW_V0.8.md` handoff artifact.

3. **Convert v0.8 schema spine from spec to executable surfaces (bounded code WPs)**
   - Implement runtime ingestion/validation boundaries for core artifacts:
     - ExperimentRecord, Evidence View, Mutation, EvaluationPlan.
   - Add deterministic tests for ordering, required fields, and replay-safe behavior.

4. **Implement workflow/indexing integration path (bounded, test-first)**
   - Wire Gödel workflow template into runnable demo harness paths.
   - Implement deterministic indexing path for `run_summary` + `experiment_index_entry` consumption in runtime flows.

5. **Promote demo and quality surfaces from planning docs to enforced gates**
   - Add CI-smoke coverage for required v0.8 demo rows.
   - Add explicit quality gate checks tied to v0.8 release criteria (or document intentional divergence).

6. **Run a final convergence audit before ceremony**
   - Re-run this recovery audit checklist against repository state.
   - Proceed to release ceremony only after runtime-vs-doc parity is explicitly demonstrated.

## Evidence Commands Used
- `gh` issue/PR metadata queries for issue/merge state and changed-file classification.
- Repository content checks:
  - `rg` scans across README/manifests/workflows/runtime/docs
  - directory/file existence checks under `docs/milestones/v0.8/`
