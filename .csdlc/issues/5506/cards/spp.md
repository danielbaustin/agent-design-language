# Structured Planning Prompt

Template: 1.0.0

Issue: 5506

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Extract the reviewed coverage mapping, validate auth-only and mixed execution, review, publish, and merge before rebasing #5494.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Apply the four-file Runtime v3 auth coverage mapping",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Run focused coverage-tool contract tests",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Review, publish, and merge the tooling-only PR",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Mixed selectors retain ADL workspace coverage
- Auth-only selection resolves at least one Runtime v3 test
- No runtime source changes

## Risks

- Auth-only routing could accidentally suppress mixed ADL coverage
- The nextest expression could select no tests

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5506/retained/design.md

Digest: 20cf641e33b4974869160e064dff275dae0a0e07063cd6408ffa64f4d859cbf9

## Diagram

.csdlc/issues/5506/retained/diagram.mmd

Digest: e43e8aba7055ad219b77ed6b87bb3ccf433afa45a792b7f7d3dbab90782b5384

## Stop Conditions

- The mapping requires runtime source changes
- Focused contract tests cannot prove both execution modes

## Handoff

Proceed only after doctor readiness.
