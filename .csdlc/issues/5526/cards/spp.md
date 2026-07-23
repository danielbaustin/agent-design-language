# Structured Planning Prompt

Template: 1.0.0

Issue: 5526

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare issue-local cards, design, diagram, and prep validation now; execute only after live #5349 merge plus ancestry is true; implement deterministic provider/model contracts; run focused validation and one exact pre-PR review; publish only after implementation review passes.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Initialize six cards, design, diagram, and bound preparation claim",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Verify live WP-09 #5349 merge plus ancestry on current origin/main before product execution",
    "acceptance_ids": [
      "AC-7",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement provider identities, capability truth, model snapshots, deterministic fixtures, and redaction-safe configuration",
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
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused deterministic validation and optional credential-gated live smoke disposition",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run one exact pre-PR review, fix actionable findings, and publish only after review passes",
    "acceptance_ids": [
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- Secrets never enter cards, logs, traces, fixtures, stdout, or retained artifacts
- Provider output is data, not authority
- Resolved provider/model/version identity is retained for execution
- Direct-provider and OpenRouter proof are distinct
- Execution waits on live WP-09 merge and ancestry, not receipts
- Exactly one bounded pre-PR review is required after implementation

## Risks

- Provider identity could be collapsed into generic OpenAI-compatible routing
- Model aliases could drift after retained execution
- Provider discovery could introduce nondeterministic replay
- Credential or provider output could leak into retained artifacts
- Receipt evidence could be mistaken for live merge ancestry
- Provider scores could be misread as workflow authority

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5526/design.md

Digest: f45c0aac551edc5b0e065f9d923f3425bc74ef7368bb9b0015cc7c1bd765a30b

## Diagram

.csdlc/prepared/issues/5526/diagram.mmd

Digest: dabc037c0ce74efb2bd1ec39037e58c57881d964bd8055c4b90c7a84313374fc

## Stop Conditions

- Live WP-09 #5349 merge plus ancestry is absent or stale
- Any need for AWS, Bedrock, raw credentials, provider calls during preparation, or root-main writes
- Protected-path collision with an active unreleased issue
- Provider identity, alias, credential, or deterministic replay truth is ambiguous
- A live smoke lane would require retaining secrets or unredacted provider output

## Handoff

Proceed only after doctor readiness.
