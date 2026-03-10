# v0.8 Recovery Audit (Repository Truth)

Date: 2026-03-10
Scope: Repository-state audit and recovery planning only.

## Current Repository State

### Observed facts
- Canonical v0.8 docs directory is populated under `docs/milestones/v0.8/` with schema/spec docs, planning docs, and release-tail docs.
- Root README still presents active development as v0.75 (`README.md`).
- Runtime crate manifest is `0.7.0` (`swarm/Cargo.toml`).
- Runtime README still documents current runtime as v0.6 (`swarm/README.md`).
- `docs/milestones/v0.8/MILESTONE_CHECKLIST_V0.8.md` is still template content with unresolved `{{...}}` placeholders.
- `docs/milestones/v0.8/RELEASE_NOTES_V0.8.md` also still contains unresolved `{{...}}` placeholders.
- Issue/PR state in the review tail is not fully converged:
  - `#707` open (PR open)
  - `#708` open (no PR)
  - `#668` open (no PR)

### Inferred conclusions
- v0.8 documentation surfaces exist, but the milestone is not in a release-ready state.
- repository-level status/version messaging is internally inconsistent across README/manifests/docs.

## Implemented Features

### Observed facts
- Issues `#609`-`#616`, `#664`-`#669` (except `#668`), `#677`, `#683`, `#704`-`#706` were merged primarily as docs/spec artifacts.
- `#702` and `#703` added real code artifacts for the transpiler demo scaffold:
  - `tools/transpiler_demo/Cargo.toml`
  - `tools/transpiler_demo/src/main.rs`
  - `examples/workflows/rust_transpiler_demo.yaml`
  - `demos/rust_output/workflow_runtime.rs`
  - `demos/rust_output/transpiler_verification.v0.8.json`

### Inferred conclusions
- The only concrete v0.8 code addition in the audited issue set is the bounded transpiler-demo scaffold/evidence path.
- Most other v0.8 merges delivered specification/documentation contracts, not executable runtime behavior.

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
- No runtime parser/validator/executor references were found for v0.8 schema artifacts such as:
  - `experiment_record.v1`
  - `canonical_evidence_view.v1`
  - `mutation.v1`
  - `evaluation_plan.v1`
  - `godel_experiment_workflow.template.v1`
  - `experiment_index_entry.v1`
  - `tool_result.v1`
  (Search across `swarm/src`, `swarm/tests`, `swarm/schemas` returned no direct schema integration references.)
- `docs/milestones/v0.8/DEMOS_V0.8.md` and `docs/milestones/v0.8/QUALITY_GATE_V0.8.md` are planning docs; CI workflows do not currently reference these v0.8 gate/demo surfaces.
- Third-party review artifact file `docs/milestones/v0.8/THIRD_PARTY_REVIEW_V0.8.md` is absent on current branch state; issue `#707` remains open.

### Inferred conclusions
- v0.8 runtime integration WPs are largely unimplemented in executable code.
- Demo and quality-gate surfaces are documented, but not yet enforced as deterministic runtime/CI gates.

## Version Inconsistencies

### Observed facts
- `swarm/Cargo.toml` declares `version = "0.7.0"`.
- Root `README.md` says:
  - latest released milestone: v0.7.0
  - active development milestone: v0.75
- `swarm/README.md` says current runtime release is v0.6.
- v0.8 docs are present and active in repository planning/execution artifacts.

### Inferred conclusions
- Version messaging is split across three narratives (v0.6 runtime README, v0.7 crate/release, v0.75 active milestone, and ongoing v0.8 execution).
- This creates milestone drift and review ambiguity.

## Recommended Recovery Plan to Complete v0.8

### Guiding principle
Repository truth is authoritative. Keep recovery deterministic, bounded, and sequence-driven.

### Minimal deterministic recovery sequence
1. **Stabilize milestone truth surfaces (docs-only, short cycle)**
   - Resolve placeholder docs:
     - `MILESTONE_CHECKLIST_V0.8.md`
     - `RELEASE_NOTES_V0.8.md`
   - Align root/runtime README status statements with actual active milestone plan.

2. **Close review-tail gaps before new feature expansion**
   - Merge/complete `#707` and `#708` with explicit fixed-vs-deferred findings.
   - Complete missing planning integration issue `#668` or defer explicitly with rationale.

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

