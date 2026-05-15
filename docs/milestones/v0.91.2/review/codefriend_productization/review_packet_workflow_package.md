# CodeFriend Review Packet Workflow Package

## Purpose

Define the bounded `CodeFriend` review-packet workflow that `WP-06` promotes
from skill fragments into a coherent product surface.

This package describes:

- the packet-to-report workflow
- the required evidence surfaces
- the product-boundary and non-claim rules

It does not claim autonomous review authority, publication readiness, or
customer delivery by itself.

## Audience

- internal operators running CodeFriend review flows
- reviewers checking whether a CodeFriend packet is complete enough to hand to
  downstream specialist or product-report lanes
- future `WP-07` work aligning heuristics and demos to the same package

## Workflow Boundary

The intended bounded flow is:

1. build a repository review packet
2. route bounded specialist review lanes over that packet
3. synthesize findings without hiding disagreement or uncertainty
4. write a product report from the completed review artifacts
5. run redaction/evidence and review-quality checks before any external use

The workflow stops before:

- customer publication
- marketing claims
- automatic remediation
- automatic issue creation
- automatic merge or release approval

## Required Skill Surfaces

### Packet construction

- `repo-packet-builder`

Primary role:

- build a bounded review packet root
- record scope, exclusions, evidence index, and specialist assignments

Required packet artifacts:

- `run_manifest.json`
- `repo_scope.md`
- `repo_inventory.json`
- `evidence_index.json`
- `specialist_assignments.json`

### Specialist review lanes

Primary expected lanes:

- `repo-review-code`
- `repo-review-security`
- `repo-review-tests`
- `repo-review-docs`
- `repo-architecture-review`
- `repo-review-synthesis`

These lanes must preserve:

- evidence-backed findings
- explicit uncertainty
- explicit non-reviewed surfaces

### Product report packaging

- `product-report-writer`

Primary role:

- convert completed review artifacts into a customer-grade CodeFriend report

Required report artifacts:

- legacy compatibility filenames retained by the current product-report skill
  contract:
- `codebuddy_product_report.md`
- `codebuddy_product_report.json`

### Publication-boundary checks

Required downstream gates before external use:

- `redaction-and-evidence-auditor`
- `review-quality-evaluator`

These are required because a bounded internal report shape is not the same as
external publication readiness.

## Workflow Stages

### Stage 1: Packet build

Entry condition:

- bounded repo scope exists

Exit condition:

- packet root exists with scope, inventory, evidence, and specialist
  assignments

### Stage 2: Specialist review

Entry condition:

- packet root exists

Exit condition:

- specialist findings or explicit no-findings surfaces exist

### Stage 3: Synthesis

Entry condition:

- specialist review artifacts exist

Exit condition:

- one synthesis surface separates findings, non-findings, unresolved
  questions, and residual risk

### Stage 4: Product report

Entry condition:

- synthesis and packet artifacts exist

Exit condition:

- one customer-grade report exists without hiding severity or disagreement

### Stage 5: Publication boundary

Entry condition:

- product report exists

Exit condition:

- external use is either:
  - explicitly still blocked
  - or separately cleared by redaction/evidence and quality review

`WP-06` only packages this stage boundary. It does not clear it.

## Product Boundary Rules

CodeFriend may describe itself as:

- a bounded review-packet workflow
- a repository intelligence and review surface
- a packet-to-report system grounded in explicit evidence

CodeFriend may not describe itself here as:

- an autonomous code-review authority
- a replacement for human reviewer judgment
- a compliance certifier
- an automatic remediation or merge engine

## Proof Surfaces For WP-06

The `WP-06` proof package consists of:

- this workflow package
- `product_report_template.md`
- `evidence_requirements.md`
- `skill_demo_alignment.md`

Together they prove that the CodeFriend product lane has:

- one bounded packet workflow
- one customer-grade report shape
- one explicit evidence policy
- one truthful alignment to current skill surfaces

## Non-Claims

- This package does not prove the review heuristics themselves. `WP-07` owns
  that layer.
- This package does not prove a live Google Workspace bridge. `WP-08` owns that
  lane.
- This package does not authorize customer publication.
- This package does not claim that every repo review lane is already perfect.
