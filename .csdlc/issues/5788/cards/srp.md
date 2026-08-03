# Structured Review Prompt

Template: 1.0.0

Issue: 5788

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl/tools/install_owner_binaries.sh
adl/tools/run_cargo_validation.sh
adl/tools/run_owner_validation_lane.sh
adl/tools/test_owner_binary_install.sh
adl/tools/test_owner_validation_lane.sh
adl/tools/test_run_cargo_validation.sh

## Prompts

- Can any failure leave Cargo.lock changed?
- Are all requested default bins current?
- Are pre-existing dirty bytes preserved?

## Findings

[
  {
    "id": "F-5788-1",
    "severity": "p1",
    "summary": "Exact-head SRP scope omitted the Cargo validation wrapper and its test.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5788-2",
    "severity": "p1",
    "summary": "The SOR referenced a validation evidence path that was not retained.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5788-3",
    "severity": "p1",
    "summary": "The owner lane warm-cache step used a different target and wrote repo-local output.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5788-4",
    "severity": "p2",
    "summary": "The inventory test was hard-coded and did not reject a duplicate nested build.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:05f424c6a6e9b5e60a09a6b4f290b9c49dd83681:e16d11a0c894cb216554a75aa52866b5fdeed70e90ab8dc6432f5d123baff592")

Reviewer: Some("subagent:019fc8ce-d14d-7783-a875-70ad800660a2")

Result: changes_required
