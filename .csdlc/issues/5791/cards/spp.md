# Structured Planning Prompt

Template: 1.0.0

Issue: 5791

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Bind #5791 in FastWork, inventory current revision and recently closed issues, run review-skill lanes over actual code and evidence, synthesize findings, fix or route accepted findings, re-review, validate, publish, and close out after merge.

## Plan

Revision 1

## Steps

[
  {
    "id": "step-1",
    "action": "Initialize and bind #5791 lifecycle state in the FastWork worktree.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "step-2",
    "action": "Record exact revision, issue/PR truth, closed-since-prior-review inventory, and changed code surfaces.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "step-3",
    "action": "Review code, tests, CI, docs, lifecycle, security, operability, and evidence.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "step-4",
    "action": "Deduplicate findings and fix or route accepted in-scope issues.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "step-5",
    "action": "Record exact-head review and validation truth, publish PR, merge when green, and close out.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Root main checkout remains clean.
- Review claims are pinned to exact revisions.
- Findings are evidence-bound and severity ranked.
- No release-readiness claim exceeds the evidence.

## Risks

- Prior review artifacts may be stale relative to newly merged issues.
- Issue closure state may not imply lifecycle closeout truth.
- Code changes may have landed after docs or review summaries.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.adl/local-artifacts/5791-bootstrap/design.md

Digest: dc6e935fef0e5809fb3a7ca88b8ada8d7b589a98772303e767c3ef8ec2a518b6

## Diagram

.adl/local-artifacts/5791-bootstrap/diagram.mmd

Digest: 34b7221b8b44934caedcf76d8806a42d5e863201c149efff01d014b5942fdf68

## Stop Conditions

- Block if residual coding issues required by #5791 are still open or unmerged.
- Block if exact current revision cannot be established.
- Block if review artifacts would require root main mutation.
- Block if publication lacks current exact-head review truth.

## Handoff

Proceed only after doctor readiness.
