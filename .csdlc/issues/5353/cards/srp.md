# Structured Review Prompt

Template: 1.0.0

Issue: 5353

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/5353

## Prompts

- Verify issue-local paths cannot create a false existing-record condition.
- Verify both design and diagram digests refresh atomically.
- Verify tests do not widen into ADL or Runtime code.

## Findings

[
  {
    "id": "5353-TR-P1-transient-terminal-projection",
    "severity": "p1",
    "summary": "Review recovery temporarily projects implemented state until typed terminal observation is recorded",
    "actionable": false,
    "in_scope": true,
    "disposition": "accepted_risk",
    "fix_revision": null,
    "route": "#5423"
  },
  {
    "id": "5353-TR-P2-version-identity",
    "severity": "p2",
    "summary": "All six retained card identities say v0.91.8 while live issue and PR truth say v0.91.7",
    "actionable": true,
    "in_scope": false,
    "disposition": "out_of_scope",
    "fix_revision": null,
    "route": "#5427"
  },
  {
    "id": "5353-TR-P2-plan-steps",
    "severity": "p2",
    "summary": "SPP steps remained pending after implementation, validation, review, and merge",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:21b4e1a9d46b9607f3fb29bb04e2084843590366:ccdc825a421b209ed8fa2651708bceca65fc577861703d97c139f83cfef27c52",
    "route": null
  },
  {
    "id": "5353-TR-P2-doctor-proof",
    "severity": "p2",
    "summary": "SOR did not retain the declared passing doctor acceptance proof",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:21b4e1a9d46b9607f3fb29bb04e2084843590366:ccdc825a421b209ed8fa2651708bceca65fc577861703d97c139f83cfef27c52",
    "route": null
  },
  {
    "id": "5353-TR-P2-focused-command",
    "severity": "p2",
    "summary": "The focused VPP lane used two positional Cargo test filters and was not executable",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:21b4e1a9d46b9607f3fb29bb04e2084843590366:ccdc825a421b209ed8fa2651708bceca65fc577861703d97c139f83cfef27c52",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Typed terminal closeout must replace the transient implemented projection
- Card identity version remains stale until #5427 applies the new typed repair

## Review Result

Revision: Some("git-blake3:21b4e1a9d46b9607f3fb29bb04e2084843590366:ccdc825a421b209ed8fa2651708bceca65fc577861703d97c139f83cfef27c52")

Reviewer: Some("codex-subagent-terminal-publication")

Result: pass
