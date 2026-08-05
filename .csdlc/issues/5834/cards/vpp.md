# Validation Planning Prompt

Template: 1.0.0

Issue: 5834

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5834/design.md

Diagram: .csdlc/prepared/issues/5834/diagram.mmd

## Selected Lanes

[
  {
    "lane": "birthday-review-packet",
    "proof_role": "Parse the packet manifest/schema, recompute referenced digests, resolve links, and require the exact WP-08 through WP-15 roster including WP-13A and WP-14. [preexec_rejection exit=1 diagnostic_sha256=342d4c821b9ac863c110874f50eddf925201e0474dcc4d02afc4477325a2e469]",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5834/validate-review-packet.rb",
      "--packet",
      "docs/milestones/v0.92/review/FIRST_BIRTHDAY_REVIEW_PACKET_v0.92.md",
      "--manifest",
      "docs/milestones/v0.92/review/first-birthday-review-evidence.v1.json",
      "--schema",
      "docs/milestones/v0.92/review/first-birthday-review-packet.schema.json"
    ],
    "parallel_group": "5834-packet",
    "defer_reason": null
  },
  {
    "lane": "birthday-review-packet-negative",
    "proof_role": "Run one-at-a-time negative fixtures for stale digests, missing roster entries, private paths, contradictory status, and forbidden public claims.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5834/validate-review-packet.rb",
      "--negative-fixtures",
      ".csdlc/evidence/5834/negative-fixtures/"
    ],
    "parallel_group": "5834-negative",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/prepared/issues/5834/validate-review-packet.rb --packet docs/milestones/v0.92/review/FIRST_BIRTHDAY_REVIEW_PACKET_v0.92.md --manifest docs/milestones/v0.92/review/first-birthday-review-evidence.v1.json --schema docs/milestones/v0.92/review/first-birthday-review-packet.schema.json`
- `ruby .csdlc/prepared/issues/5834/validate-review-packet.rb --negative-fixtures .csdlc/evidence/5834/negative-fixtures/`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
