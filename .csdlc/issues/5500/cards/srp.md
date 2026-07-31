# Structured Review Prompt

Template: 1.0.0

Issue: 5500

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

## Scope

docs/tooling/milestone-dashboard
adl/tools/test_milestone_dashboard.sh

## Prompts

- Does the design extend the existing dashboard and Observatory contracts without creating a second framework or authority?
- Are retained, live, stale, unknown, blocked, and non-authoritative states complete and fail closed?
- Can untrusted snapshot or Runtime values cause XSS, unsafe navigation, credential exposure, unbounded work, or authority confusion?
- Are future paths disjoint from #5502 and every other WP-10A child?
- Are COTS choices, zero-new-dependency posture, budgets, and deterministic fixtures small and executable?
- Does preparation preserve exact #5498 and #5349 terminal gates?

## Findings

[
  {
    "id": "R-1-runtime-origin-allowlist",
    "severity": "p1",
    "summary": "The Runtime v3 Observatory adapter must fail closed when the snapshot declares no allowed origins; an empty allowlist must not permit arbitrary HTTPS Runtime API bases.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:a24992cfaecfb6adaa2f82ea1b780dd7d1cc6803:e20c0116b8738017024b693a5d3136929d45f2910b53a07a1ada73a7ec3d2e02",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- GPT-5.5-specific reviewer tooling was not callable in this environment after tool discovery; this record preserves the single available typed pre-PR review boundary.

## Review Result

Revision: Some("git-blake3:a24992cfaecfb6adaa2f82ea1b780dd7d1cc6803:e20c0116b8738017024b693a5d3136929d45f2910b53a07a1ada73a7ec3d2e02")

Reviewer: Some("codex:single-pre-pr-review")

Result: pass
