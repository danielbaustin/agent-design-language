# Structured Review Prompt

Template: 1.0.0

Issue: 5791

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

README.md
docs/milestones/v0.91.8/CANONICAL_DOC_INVENTORY_v0.91.8.md
docs/milestones/v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md
adl/Cargo.toml
adl/Cargo.lock
adl-runtime/Cargo.toml
adl-runtime/Cargo.lock
tools/aws_remote_validation/Cargo.toml
tools/aws_remote_validation/Cargo.lock

## Prompts

- Does the review corpus include issues closed since the prior WP-18 review?
- Does the review inspect actual code and validation surfaces?
- Are findings deduplicated and evidence-bound?
- Are release-readiness claims supported by exact current evidence?

## Findings

[
  {
    "id": "IR5791-07",
    "severity": "p1",
    "summary": "WP-17 update-list truth left current release-tail docs saying WP-17 was active and root README reporting crate version 0.91.7.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:b99de0df70874a14bb9ea9b10170bdd8d1a447b7:5285d13cab55c9847db6c33b078cc5faeb9294ff915997a72d43cce394d7250b",
    "route": null
  },
  {
    "id": "IR5791-08",
    "severity": "p2",
    "summary": "REVIEW.md still says WP-17 documentation truth alignment is active, but typed claim ownership blocks #5791 from editing REVIEW.md because #5357 owns that path.",
    "actionable": true,
    "in_scope": false,
    "disposition": "out_of_scope",
    "fix_revision": null,
    "route": "#5357 protected REVIEW.md; do not edit in #5791"
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- REVIEW.md remains a known stale current-truth line under #5357 ownership and must be corrected by the owning issue/worktree.
- #5801 tracks broader CI/lifecycle simplification separately.

## Review Result

Revision: Some("git-blake3:b99de0df70874a14bb9ea9b10170bdd8d1a447b7:5285d13cab55c9847db6c33b078cc5faeb9294ff915997a72d43cce394d7250b")

Reviewer: Some("codex-stale-wp17-doc-truth-review")

Result: pass
