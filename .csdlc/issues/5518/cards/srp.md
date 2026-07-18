# Structured Review Prompt

Template: 1.0.0

Issue: 5518

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/5516
.csdlc/issues/5518
.csdlc/prepared/issues/5518
csdlc-v2/src/bin/csdlc-closeout.rs
csdlc-v2/src/lib.rs
csdlc-v2/src/model.rs
csdlc-v2/src/schema.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/gate7_lifecycle.rs

## Prompts

- Is the operation narrower than terminal design repair?
- Are all CAS and receipt checks fail closed?
- Can a failed refresh leave split truth?
- Does #5516 S3 match its SOR after repair?

## Findings

[
  {
    "id": "F-5518-1",
    "severity": "p1",
    "summary": "Initial cross-issue repair authority did not require exact target-path ownership.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:45a6348f44e80dcddf8bef7c13f88002dfcece20:27cdf49e79bb3f544adecaff36c373e61effbaa2f2dc153d2026a359d993962c",
    "route": null
  },
  {
    "id": "F-5518-2",
    "severity": "p1",
    "summary": "Initial terminal receipt compare-and-swap was not serialized across worktrees.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:45a6348f44e80dcddf8bef7c13f88002dfcece20:27cdf49e79bb3f544adecaff36c373e61effbaa2f2dc153d2026a359d993962c",
    "route": null
  },
  {
    "id": "F-5518-3",
    "severity": "p2",
    "summary": "Initial repair lacked repeatable Store-level atomicity and stale-CAS coverage.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:45a6348f44e80dcddf8bef7c13f88002dfcece20:27cdf49e79bb3f544adecaff36c373e61effbaa2f2dc153d2026a359d993962c",
    "route": null
  },
  {
    "id": "F-5518-4",
    "severity": "p1",
    "summary": "Terminal reconciliation and generic recovery could write common receipts outside the global terminal lock.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:45a6348f44e80dcddf8bef7c13f88002dfcece20:27cdf49e79bb3f544adecaff36c373e61effbaa2f2dc153d2026a359d993962c",
    "route": null
  },
  {
    "id": "F-5518-5",
    "severity": "p2",
    "summary": "Terminal reconciliation exposed a projection swap before its recovery journal was durable.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:45a6348f44e80dcddf8bef7c13f88002dfcece20:27cdf49e79bb3f544adecaff36c373e61effbaa2f2dc153d2026a359d993962c",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:45a6348f44e80dcddf8bef7c13f88002dfcece20:27cdf49e79bb3f544adecaff36c373e61effbaa2f2dc153d2026a359d993962c")

Reviewer: Some("subagent:019f7581-a4bf-7fb3-a900-3d71dfea4abc")

Result: pass
