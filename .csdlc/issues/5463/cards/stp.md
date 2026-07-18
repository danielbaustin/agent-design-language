# Structured Task Prompt

Template: 1.0.0

Issue: 5463

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Replace only annotated Node 20 action pins, strengthen their static contract, retain inventory, and prove hosted annotations absent.

## Deliverables

- Reviewed Node 24 immutable pins
- Canonical static pin contract
- Retained pin inventory
- Hosted annotation-free proof

## Acceptance

1. AC1: All checkout occurrences use reviewed checkout v7 Node 24 commit
2. AC2: All upload-artifact occurrences use reviewed v7 Node 24 commit
3. AC3: All rust-cache occurrences use reviewed v2.9.1 Node 24 commit
4. AC4: Deprecated SHAs are absent and immutable full-SHA policy remains enforced
5. AC5: A GitHub-hosted PR run emits no Node.js 20 deprecation annotation

## Dependencies

- GitHub-hosted Actions runner and official action release metadata

## Inputs

- .github/workflows
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_ci_path_policy.sh
- GitHub run 29632957768

## Non Goals

- AWS execution
- AWS-backed validation
- Floating action tags
- Unrelated CI policy changes
