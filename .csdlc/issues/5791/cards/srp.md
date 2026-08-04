# Structured Review Prompt

Template: 1.0.0

Issue: 5791

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

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
    "id": "IR5791-06",
    "severity": "p1",
    "summary": "Active Rust package manifests and lockfiles still reported 0.91.7 during v0.91.8 WP-18 review.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:e4ee565c98a5cce14afc6ec3b4e5cf3f9c394863:0cb36cefd88fbc7363e2b5bb78e634bef28127bae2696dc275b2b4d65ba6e4d4",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- WP-17 package-version alignment was missed before WP-18; #5801 tracks CI/lifecycle simplification separately.

## Review Result

Revision: Some("git-blake3:e4ee565c98a5cce14afc6ec3b4e5cf3f9c394863:0cb36cefd88fbc7363e2b5bb78e634bef28127bae2696dc275b2b4d65ba6e4d4")

Reviewer: Some("codex-version-truth-review")

Result: pass
