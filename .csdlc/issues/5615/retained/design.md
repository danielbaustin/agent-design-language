# Issue #5615 CI Routing Design

## Purpose

Route C-SDLC-only changes through proof that matches their actual surface. The
change preserves the existing required check names while preventing lifecycle
metadata and standalone C-SDLC v2 Rust changes from launching unrelated ADL
workspace or Runtime coverage.

## Classification Contract

The existing path classifier gains one explicit output:
`csdlc_v2_standalone_required`.

- `.csdlc/**` lifecycle metadata selects focused path/tooling contracts and no
  Rust or coverage producer.
- `csdlc-v2/**` source, manifest, or test changes select the standalone C-SDLC
  v2 Rust job with tests, formatting, and strict Clippy. They do not select ADL
  workspace or Runtime coverage.
- Runtime, ADL workspace, and mixed changes retain their existing proving
  routes. A mixed change that includes `csdlc-v2/**` additionally requires the
  standalone C-SDLC v2 job.

The stable `adl-ci` aggregate includes the standalone job and fails closed when
the classifier requires it but the job does not complete successfully. The
stable `adl-coverage` aggregate remains truthful: it reports success when
coverage is not required by policy and continues to require its selected
producers when coverage is required.

## Portable Local Validation

One small command wrapper supplies writable Cargo state for local validation.
It accepts `ADL_CARGO_BUILD_ROOT`, otherwise uses `ADL_FASTWORK_ROOT` with
`/Volumes/FastWork` as the production default. It canonicalizes the selected
root, rejects the repository and its descendants, requires an existing
writable directory, and overwrites inherited `CARGO_HOME` and
`CARGO_TARGET_DIR` with directories beneath that external root. Tests inject a
temporary FastWork candidate, including a path containing spaces. Hosted CI
always declares its runner-temporary root, so CI does not depend on FastWork.

## Boundaries

- No Runtime behavior, Runtime v2, ADL-v2 semantics, AWS, or Spot execution.
- No validation-manager or C-SDLC lifecycle redesign.
- No new Rust crate or third-party dependency.
- Existing immutable GitHub Action pins and required-check names remain.

## Proof

Classifier fixtures cover metadata-only, `csdlc-v2` source/test (including
`csdlc-v2/tests/gate7_lifecycle.rs`), Runtime, ADL workspace, and mixed diffs.
Workflow contract tests prove standalone-job materialization and fail-closed
aggregate wiring. Wrapper tests prove explicit-root success, FastWork
preference, environment isolation, and clear refusal without a writable root.
