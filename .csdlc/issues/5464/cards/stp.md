# Structured Task Prompt

Template: 1.0.0

Issue: 5464

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Update only nextest installer steps and their static contract, then prove the warning absent on GitHub-hosted CI.

## Deliverables

- Reviewed immutable installer pin
- Fail-closed nextest install configuration
- Static regression contract
- Hosted warning-free proof

## Acceptance

1. AC1: Every nextest install uses an immutable installer manifest that contains nextest 0.9.140 for x86_64 Linux
2. AC2: Every nextest install sets fallback to none
3. AC3: Static validation rejects stale pins, floating pins, missing fallback policy, and version drift
4. AC4: GitHub-hosted CI emits no unsupported-binary or cargo-binstall fallback warning

## Dependencies

- GitHub-hosted Actions runner
- Official nextest release metadata
- Official taiki-e/install-action manifest

## Inputs

- .github/workflows/ci.yaml
- adl/tools/test_ci_runtime_contracts.sh
- GitHub run 29632957768

## Non Goals

- AWS execution
- AWS-backed validation
- Changing nextest version
- Unrelated test-lane changes
