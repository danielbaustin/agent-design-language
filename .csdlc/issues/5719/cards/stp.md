# Structured Task Prompt

Template: 1.0.0

Issue: 5719

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Fix CI selector policy and focused contract tests only; do not change podcast page content or unrelated CI behavior.

## Deliverables

- path-policy classifier update
- focused tests for podcast/demo static path selection
- regression tests proving Rust/tooling changes still select full or required lanes
- truthful lifecycle and publication evidence

## Acceptance

1. AC-1: A #5716-like podcast studio/static demo path set reports full_coverage_required=false.
2. AC-2: The same path set does not schedule both hosted runtime and hosted workspace producer coverage.
3. AC-3: The stable adl-coverage-hosted aggregator/check remains present and truthful.
4. AC-4: Rust/runtime/provider/tooling policy path changes retain their existing full/focused coverage requirements.
5. AC-5: Focused CI path-policy and workflow contract tests pass locally.

## Dependencies

- #5716 observed duplicate hosted producer coverage
- Existing ci_path_policy contract suite

## Inputs

- adl/tools/ci_path_policy.sh
- adl/tools/test_ci_path_policy.sh
- .github/workflows/ci.yaml
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_validation_manager.sh

## Non Goals

- removing required stable check aggregation
- rewriting the hosted coverage workflow
- changing podcast page content
- weakening Rust/runtime/provider/tooling coverage
