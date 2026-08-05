# Structured Planning Prompt

Template: 1.0.0

Issue: 5356

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Render six cards; freeze exact WP-17 gating, corpus, specialist matrix, identity, finding, budgets/PVF/no-deferral/rollback/publication contracts; review and fix preparation; typed bind and push; wait fail-closed for #5360.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Complete six cards, reviewed design/diagram, exact preparation claim, corpus/matrix, executable gates, budgets/PVF, typed bind/doctor, commit and push",
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
    "status": "in_progress"
  },
  {
    "id": "S2",
    "action": "Wait read-only for #5360 terminal receipt, claim release, and ancestry",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Amend exact review-output paths, freeze corpus identity, run six specialist lanes, and synthesize findings-first packet",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run review quality, typed review, exact/redaction/provenance gates, publish, shepherd CI, serialize authorized merge, post-merge proof, and closeout",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- preparation owns only four exact #5356 issue-local lifecycle/evidence paths
- all six specialist lanes consume one immutable exact corpus identity
- findings-first review never grants remediation, merge, release, or external-review authority
- unsupported and missing evidence remains explicit and blocks downstream review
- Runtime v2, AWS, credentials, host-absolute retained paths, private prompts, and untracked evidence are forbidden

## Risks

- a docs-only packet could be mistaken for a complete implementation review
- specialist lanes could review different revisions or silently omit owner surfaces
- live issue, PR, CI, and receipt truth could drift after corpus freeze
- finding severity or disposition could hide release blockers
- review evidence could leak credentials, private prompts, host paths, or unsupported claims
- review orchestration could become a duplicate lifecycle authority or exceed budget

## Estimates

{
  "elapsed_seconds": 86400,
  "total_tokens": 240000,
  "validation_seconds": 21600
}

## Design

.csdlc/prepared/issues/5356/design.md

Digest: 0d3b9927f587596993405ad5a14380207149769ebeb497b1acd89e63ff98e6b8

## Diagram

.csdlc/prepared/issues/5356/diagram.mmd

Digest: 04dc65f04a0ca9309fe6154c67205527c86171f0b426292099d72dfea28ef28e

## Stop Conditions

- #5360 lacks actual merge, typed closed_out, claim release, retained receipt, or ancestry
- the exact corpus is incomplete, mutable, untracked, private, host-bound, secret-bearing, or cannot be digested deterministically
- a required specialist, synthesis, review-quality, exact, redaction, provenance, CI, post-merge, or closeout lane would be skipped or deferred
- specialist revision identities differ or a finding cannot be retained and routed truthfully
- the work would use Runtime v2, AWS, raw gh, credentials, paid services, hard-coded addresses, out-of-claim writes, or product changes
- a protected-path collision, stale review, unsupported claim, budget breach, or publication-boundary violation occurs

## Handoff

Proceed only after doctor readiness.
