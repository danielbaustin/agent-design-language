# Structured Task Prompt

Template: 1.0.0

Issue: 5467

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Repair and prove only the local backend-snapshot CI contract.

## Deliverables

- Repaired contract assertion
- Reachability proof for all snapshot checks
- Three local backend behavior cases

## Acceptance

1. AC-1: Repair the stale builder-script assertion
2. AC-2: Prove every backend-snapshot assertion executes
3. AC-3: Locally prove hosted and Spot-selected routing
4. AC-4: Invalid backend fails closed
5. AC-5: No AWS execution or inspection

## Dependencies

- #5412 final-head review finding routed to #5467

## Inputs

- adl/tools/test_run_aws_spot_ci_profile.sh
- .github/workflows/ci.yaml
- adl/tools/run_aws_spot_builder_image_validation.sh
- adl/tools/verify_ci_backend_route.py

## Non Goals

- No AWS execution, inspection, cleanup, or credentials
- No Runtime v3 product changes
- No remote workflow invocation
