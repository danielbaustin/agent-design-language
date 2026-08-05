# Structured Output Record

Template: 1.0.0

Issue: 5684

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented split C-SDLC GitHub owner binaries, shared adl-resilience retry/backoff crate, runtime backoff wiring, install/coexistence manifest enforcement, current docs/skill/template updates, deleted structured-prompt wrapper bootstrap guidance repair, post-publication Opus/Hegel findings remediation, issue_create post-create confirmation race repair, and corrected exact proof that transient empty marker search occurs after successful issue creation.

## Artifacts

- git diff --check origin/main...HEAD: pass
- CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest cargo test --manifest-path adl-resilience/Cargo.toml: pass, 3 tests
- CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_github_actions: pass, 3 tests
- CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest cargo test --manifest-path csdlc-v2/Cargo.toml --test gate10a: pass, 14 tests
- CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest cargo check --manifest-path adl-runtime/Cargo.toml: pass
- CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest cargo test --manifest-path csdlc-v2/Cargo.toml --test gate10a current_bootstrap_guidance_does_not_call_deleted_prompt_wrapper: pass, 1 test
- rg "bash adl/tools/validate_structured_prompt\\.sh" docs/templates/prompts/1.0.3 csdlc-v2/operator/skills/csdlc-v2-init/SKILL.md csdlc-v2/operator/skills/csdlc-v2-validate/SKILL.md docs/default_workflow.md docs/tooling/README.md docs/tooling/structured-prompt-validator-binary-resolution.md csdlc-v2/AGENTS.md: no matches
- CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest csdlc-install install --repo /Volumes/FastWork/adl-csdlc-install-manifest --destination /Volumes/FastWork/adl-5684-install-proof-a0a270a/csdlc-v2: pass, source_revision git:a0a270adc67678af9d4f5cb4712e1b2d3d8264aa, 21 binaries including csdlc-github-issue, csdlc-github-pr, and csdlc-merge
- csdlc-install verify --repo /Volumes/FastWork/adl-csdlc-install-manifest --bin-dir /Volumes/FastWork/adl-5684-install-proof-a0a270a/csdlc-v2 --inventory csdlc-v2/operator/coexistence.json: pass true, no missing v2 binaries
- Opus-family review via OpenRouter anthropic/claude-opus-4.8 returned 4 findings; P1/P3/P3 fixed and stdout/stderr convention accepted as matching the existing csdlc-github contract.
- Direct Anthropic claude-opus-5 and OpenRouter anthropic/claude-opus-5 both failed closed with HTTP 200 but no usable final review text; no clean Opus-5 verdict was fabricated.
- Hegel exact delta review returned CLEAN after fixes.
- Pasteur PR janitor identified csdlc-v2-standalone format failure; local fmt check now passes.
- CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check: pass
- CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_github_actions issue_create_and_comment_reconcile_by_marker_with_exact_readback -- --nocapture: pass, 1 test
- CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_github_actions: pass, 3 tests
- Ohm exact-head review at 113a67dab found two P3s: over-broad retry for multiple exact markers and missing zero-search regression proof; both fixed in the current delta.
- Epicurus exact-head review at 1ab421541 found the zero-search regression proof was consumed before POST; the corrected test now arms zero-search from the POST handler and passes focused/full GitHub action tests.

## Execution

- Added adl-resilience shared crate with retry policy and capped exponential backoff primitives.
- Wired csdlc-v2 GitHub issue-create readback retry through adl-resilience.
- Wired adl-runtime guardian and supervision backoff through adl-resilience.
- Added csdlc-github-issue and csdlc-github-pr split owner binaries while retaining csdlc-github compatibility.
- Updated operator skills/coexistence inventory so csdlc-github-issue, csdlc-github-pr, csdlc-pr-state, csdlc-github, and csdlc-merge are required.
- Updated current GitHub boundary, owner-binary install, default workflow, C-SDLC v2 AGENTS, and operator skill docs.
- Replaced current structured-prompt validator wrapper guidance with typed csdlc-validate guidance and added Gate 10A regression coverage.
- Finalized #5684 implementation after focused validation of split GitHub binaries, stable install inventory, shared resilience wiring, and deleted bootstrap-wrapper guard.
- Updated active prompt-template 1.0.3 structure schemas so new/current bootstrap structure guidance no longer blesses bash adl/tools/validate_structured_prompt.sh.
- Refreshed stable owner-binary install proof at source revision a0a270adc67678af9d4f5cb4712e1b2d3d8264aa after active template-schema repair was committed.
- Added a shared PR-state request conversion used by both csdlc-github and csdlc-github-pr so split binaries cannot drift from the compatibility facade.
- Added retry-classifier support to adl-resilience and restricted issue-create readback retries to the known transient marker-lag reconciliation case.
- Documented and tested the adl-resilience exponential-backoff cap.
- Extended owner-binary dirty-source guarding to include the shared adl-resilience path dependency before cargo runs.
- Formatted csdlc-v2/tests/gate10a.rs to repair the failing csdlc-v2-standalone format check.
- Fixed issue_create post-create confirmation so direct marker readback falls back to idempotent marker search inside the same retry policy instead of falsely reporting reconciliation_required after a successful issue mutation.
- Deduplicated marker-search candidate issue numbers before exact packet verification so duplicate search results for the same created issue do not look like multiple acceptable matches.
- Extended the local GitHub regression to hide the marker on first direct read and return duplicate/noisy search results while still proving exactly one issue POST.
- Stopped retrying multiple distinct exact-marker matches during post-create confirmation so terminal ambiguity remains an actionable reconciliation_required error.
- Added a transient empty marker-search simulation before search recovery to prove the retryable zero-result path that happens after successful issue creation but before search consistency catches up.
- Moved the transient empty marker-search simulation so it is armed by successful POST creation, ensuring the first empty search occurs during post-create confirmation rather than the pre-create idempotency scan.

## Validation

[
  {
    "command": [
      "/usr/bin/git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Run git diff --check across the PR diff.",
    "outcome": "passed",
    "evidence_ref": "diff-check.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest",
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate10a"
    ],
    "purpose": "Run Gate 10A installer/coexistence/bootstrap guidance tests.",
    "outcome": "passed",
    "evidence_ref": "gate10a-install-bootstrap-tests.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest",
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions"
    ],
    "purpose": "Run csdlc-v2 GitHub action tests.",
    "outcome": "passed",
    "evidence_ref": "github-action-split-tests.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest",
      "cargo",
      "test",
      "--manifest-path",
      "adl-resilience/Cargo.toml"
    ],
    "purpose": "Run adl-resilience unit tests.",
    "outcome": "passed",
    "evidence_ref": "resilience-tests.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest",
      "cargo",
      "check",
      "--manifest-path",
      "adl-runtime/Cargo.toml"
    ],
    "purpose": "Run adl-runtime cargo check.",
    "outcome": "passed",
    "evidence_ref": "runtime-resilience-check.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest",
      "cargo",
      "test",
      "--manifest-path",
      "adl-resilience/Cargo.toml"
    ],
    "purpose": "Run adl-resilience unit tests after retry-classifier and backoff-cap fixes.",
    "outcome": "passed",
    "evidence_ref": "post-opus-resilience-tests.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest",
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions"
    ],
    "purpose": "Run csdlc-v2 GitHub action tests after split PR-state conversion and issue-create retry classifier fixes.",
    "outcome": "passed",
    "evidence_ref": "post-opus-github-action-tests.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest",
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate10a"
    ],
    "purpose": "Run Gate 10A install/coexistence tests after owner-source dirty guard and formatter repair.",
    "outcome": "passed",
    "evidence_ref": "post-opus-gate10a-tests.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "ADL_CARGO_BUILD_ROOT=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest-ci",
      "bash",
      "adl/tools/run_cargo_validation.sh",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "purpose": "Run full csdlc-v2 locked test suite through the repo cargo validation wrapper using FastWork build output.",
    "outcome": "passed",
    "evidence_ref": "post-opus-csdlc-v2-full-tests.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "ADL_CARGO_BUILD_ROOT=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest-ci",
      "bash",
      "adl/tools/run_cargo_validation.sh",
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all",
      "--",
      "--check"
    ],
    "purpose": "Run csdlc-v2 rustfmt check matching the failed GitHub standalone format step.",
    "outcome": "passed",
    "evidence_ref": "post-opus-csdlc-v2-fmt-check.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "ADL_CARGO_BUILD_ROOT=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest-ci",
      "bash",
      "adl/tools/run_cargo_validation.sh",
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run csdlc-v2 clippy with warnings denied through the repo cargo validation wrapper using FastWork build output.",
    "outcome": "passed",
    "evidence_ref": "post-opus-csdlc-v2-clippy.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest",
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all",
      "--",
      "--check"
    ],
    "purpose": "Run csdlc-v2 rustfmt check after issue_create marker-search fallback fix.",
    "outcome": "passed",
    "evidence_ref": "post-create-race-fmt-check.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest",
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions",
      "issue_create_and_comment_reconcile_by_marker_with_exact_readback",
      "--",
      "--nocapture"
    ],
    "purpose": "Run focused issue_create post-create confirmation race regression with marker lag and duplicate/noisy search results.",
    "outcome": "passed",
    "evidence_ref": "post-create-race-focused-test.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest",
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions"
    ],
    "purpose": "Run full csdlc-v2 GitHub action test target after issue_create marker-search fallback fix.",
    "outcome": "passed",
    "evidence_ref": "post-create-race-github-action-tests.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest",
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all",
      "--",
      "--check"
    ],
    "purpose": "Run csdlc-v2 rustfmt check after narrowing issue_create confirmation retry semantics.",
    "outcome": "passed",
    "evidence_ref": "post-create-race-p3-fmt-check.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest",
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions",
      "issue_create_and_comment_reconcile_by_marker_with_exact_readback",
      "--",
      "--nocapture"
    ],
    "purpose": "Run focused issue_create confirmation regression with stale direct readback, transient empty search, duplicate same-issue search rows, and noisy non-marker candidate.",
    "outcome": "passed",
    "evidence_ref": "post-create-race-p3-focused-test.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest",
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions"
    ],
    "purpose": "Run full csdlc-v2 GitHub action test target after narrowing post-create retry semantics.",
    "outcome": "passed",
    "evidence_ref": "post-create-race-p3-github-action-tests.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest",
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all",
      "--",
      "--check"
    ],
    "purpose": "Run csdlc-v2 rustfmt check after correcting post-create zero-search proof.",
    "outcome": "passed",
    "evidence_ref": "post-create-race-p3b-fmt-check.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest",
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions",
      "issue_create_and_comment_reconcile_by_marker_with_exact_readback",
      "--",
      "--nocapture"
    ],
    "purpose": "Run focused issue_create confirmation regression with transient empty marker search armed after successful POST.",
    "outcome": "passed",
    "evidence_ref": "post-create-race-p3b-focused-test.log"
  },
  {
    "command": [
      "/usr/bin/env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/cargo-targets/adl-csdlc-install-manifest",
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions"
    ],
    "purpose": "Run full csdlc-v2 GitHub action test target after correcting post-create zero-search proof.",
    "outcome": "passed",
    "evidence_ref": "post-create-race-p3b-github-action-tests.log"
  }
]

## Integration

pr_open

## Publication

Publication: draft

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
