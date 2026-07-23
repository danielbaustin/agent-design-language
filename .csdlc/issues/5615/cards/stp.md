# Structured Task Prompt

Template: 1.0.0

Issue: 5615

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Add the smallest explicit standalone C-SDLC v2 route, preserve truthful aggregates, and provide portable external Cargo cache/target execution.

## Deliverables

- Metadata-only and C-SDLC v2 source/test classifier behavior
- Standalone C-SDLC v2 test/fmt/Clippy job with fail-closed stable aggregation
- Portable external Cargo home/target wrapper
- Exact focused regressions and hosted proof

## Acceptance

1. AC-1: `.csdlc/**` metadata-only diffs run focused lifecycle/path/tooling proof without ADL workspace or Runtime coverage
2. AC-2: `csdlc-v2/**` source, manifest, and test diffs require a standalone C-SDLC v2 test/fmt/strict-Clippy job without unrelated ADL workspace or Runtime coverage
3. AC-3: `csdlc-v2/tests/gate7_lifecycle.rs` is an exact regression fixture for the standalone route
4. AC-4: Runtime, ADL workspace, and mixed diffs preserve their intended existing lanes; mixed C-SDLC v2 diffs also require standalone C-SDLC proof
5. AC-5: Stable `adl-ci` fails closed if selected standalone C-SDLC proof is absent, skipped, cancelled, or failed
6. AC-6: Stable `adl-coverage` remains truthful and existing required-check names do not change
7. AC-7: The local wrapper exports writable external `CARGO_HOME` and `CARGO_TARGET_DIR`, honors a declared root, otherwise prefers writable FastWork, and fails clearly without either
8. AC-8: Hosted CI declares its own writable runner-temporary root and has no dependency on local FastWork
9. AC-9: Focused tests, exact-revision review, hosted checks, post-merge proof, and typed closeout pass with no unresolved findings

## Dependencies

- Issue 5613 merged terminal projections at origin/main 09c0bd1784216dbce1ad4cdebfe2d453af6e3d9d
- Existing CI classifier, required aggregates, and pinned GitHub Actions

## Inputs

- .github/workflows/ci.yaml
- adl/tools/ci_path_policy.sh
- adl/tools/test_ci_path_policy.sh
- adl/tools/test_ci_runtime_contracts.sh
- Bash, Cargo, jq, git, and existing pinned GitHub Actions

## Non Goals

- No CI architecture or validation-manager redesign
- No C-SDLC v2 lifecycle implementation change
- No Runtime behavior, Runtime v2, or ADL-v2 semantic change
- No AWS, Spot execution, credentials, or provider access
- No new Rust crate or third-party dependency
