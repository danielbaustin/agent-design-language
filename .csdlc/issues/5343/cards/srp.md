# Structured Review Prompt

Template: 1.0.0

Issue: 5343

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/5343
.csdlc/prepared/issues/5343
docs/milestones/v0.91.8/evidence/wp12/cutover-5343/report.json
docs/milestones/v0.91.8/evidence/wp12/cutover-handoff-5344.v1.json
adl-v2/tools/install-adl-v2.sh
adl-v2/crates/adl-cli/src/main.rs

## Prompts

- Can any missing, stale, malformed, contradictory, non-ancestral, or metadata-only #5344/#5345 fact bypass the execution gate?
- Can any argument, environment value, path, symlink, stale writer, lock race, interruption, or malformed receipt bypass exact installation verification or alter prior selector bytes outside the #5345 transaction?
- Are fresh-install selection, explicit v1 override, rollback-window checkpoints, exact restoration, and every failure class deterministic and fail-closed?
- Does #5343 own only cutover evidence and avoid selector implementation, Runtime v2 edits, legacy deletion, AWS, credentials, hidden network, and production overclaim?
- Are COTS, protected paths, LoC/test/module/time budgets, PVF classification, no-deferral, CI, exact review, and post-merge proof complete and executable?

## Findings

[
  {
    "id": "P1-malformed-selector-transaction-not-proven",
    "severity": "p1",
    "summary": "The malformed-selector case now executes a real selector transaction against copied active state and verifies both malformed and primary selector bytes remain unchanged.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:26457888a1c13abee54a8db7cd30ba15e25bbbbc:a36a58255d3f58d9583c6716786ca633e80257ddcd7c037e9f1ffbfc74a74af9",
    "route": null
  },
  {
    "id": "P1-host-specific-default-target-path",
    "severity": "p1",
    "summary": "The proof target now defaults portably through issue or Cargo configuration and then the repository-relative target directory.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:26457888a1c13abee54a8db7cd30ba15e25bbbbc:a36a58255d3f58d9583c6716786ca633e80257ddcd7c037e9f1ffbfc74a74af9",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The reviewer did not independently rerun the mutating transaction fault matrix; the conductor reran it at repair commit 8f5590dcd96fa173a84669b98c814a7d060c6882 and retained the passing exact report.

## Review Result

Revision: Some("git-blake3:26457888a1c13abee54a8db7cd30ba15e25bbbbc:a36a58255d3f58d9583c6716786ca633e80257ddcd7c037e9f1ffbfc74a74af9")

Reviewer: Some("subagent:gpt-5.5:Anscombe")

Result: pass
