# Structured Task Prompt

Template: 1.0.0

Issue: 5506

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Extract and prove the four-file coverage mapping required by #5494.

## Deliverables

- Runtime v3 auth risk mapping
- Independent-crate auth coverage route
- Auth-only and mixed-selection contract tests

## Acceptance

1. AC1: runtime_api_auth.rs selects runtime_v3_auth
2. AC2: auth-only coverage runs against adl-runtime/Cargo.toml
3. AC3: mixed selections run both Runtime v3 and ADL workspace tests
4. AC4: focused shell contract tests pass
5. AC5: exact-revision review and required checks pass before merge

## Dependencies

- Existing PR-fast coverage tooling
- Existing runtime_api_auth unit tests

## Inputs

- PR #5504 failed hosted coverage run 29638880423
- Issue #5494

## Non Goals

- Runtime source changes
- Runtime v3 weather changes
- Legacy full-coverage flake repair
- AWS execution
