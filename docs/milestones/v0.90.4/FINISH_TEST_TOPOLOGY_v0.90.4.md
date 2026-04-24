# Finish Test Topology v0.90.4

## Purpose

Record how the `cli::pr_cmd::tests::finish` tranche is split so ordinary Rust
validation stays fast without dropping publication and guardrail proof.

## Problem

The original finish tranche put almost every `pr finish` proof in the default
test lane. Most of those tests:

- create a temp repo
- initialize git metadata and a bare `origin`
- seed issue prompt / STP / SIP / SOR bundle fixtures
- shell through the full `real_pr finish` control path

That topology made a small number of finish tests dominate wall-clock time even
when the changed surface had nothing to do with finish publication.

## New Split

Ordinary lane:

- `adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs`
- one bounded integration guardrail in
  `adl/src/cli/tests/pr_cmd_inline/finish/guardrails/branch_and_gitignore.rs`
  that proves finish refuses the wrong checkout when the issue branch is bound
  elsewhere

Slow finish lane behind `slow-finish-tests`:

- `finish/publication.rs`
- `finish/guardrails/canonical_surfaces.rs`
- `finish/guardrails/foreign_bundle.rs`
- `finish/guardrails/output_truth.rs`
- `finish/guardrails/sync_and_prompt.rs`
- the heavier `branch_and_gitignore` publication-style temp-repo cases

## Runner Placement

Ordinary `cargo test` / `cargo nextest`:

- does not enable `slow-finish-tests`
- keeps fast argument rendering and one representative bound-checkout guardrail

Authoritative lanes:

- use `--all-features`
- continue to run the full slow finish tranche
- preserve publication, canonical-surface, foreign-bundle, sync, and stale-SOR
  proofs

## Why This Shape

This keeps one real finish integration proof in the fast lane so finish does
not become purely unit-tested there, while moving the expensive temp-repo PR
publication matrix into the same class of authoritative-only coverage used for
other slow proof surfaces.

## Non-claims

- This split does not claim the slow finish tranche is fully optimized.
- It only stops the ordinary lane from paying for every temp-repo finish
  publication scenario by default.
- Follow-on work can still convert individual slow finish cases into lighter
  helper-level tests when that can be done without losing behavioral coverage.
