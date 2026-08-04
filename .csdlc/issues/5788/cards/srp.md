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
    "disposition": "fixed",
    "fix_revision": "git-blake3:a024ff02dcae6b0cd03c294af7f204e19a7c1723:1cef3165741451b5beb05d45c6d94fba295cbb90a403e56014e792c19a093d97",
    "route": null
  },
  {
    "id": "F-5788-2",
    "severity": "p1",
    "summary": "The SOR referenced a validation evidence path that was not retained.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:a024ff02dcae6b0cd03c294af7f204e19a7c1723:1cef3165741451b5beb05d45c6d94fba295cbb90a403e56014e792c19a093d97",
    "route": null
  },
  {
    "id": "F-5788-3",
    "severity": "p1",
    "summary": "The owner lane warm-cache step used a different target and wrote repo-local output.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:a024ff02dcae6b0cd03c294af7f204e19a7c1723:1cef3165741451b5beb05d45c6d94fba295cbb90a403e56014e792c19a093d97",
    "route": null
  },
  {
    "id": "F-5788-4",
    "severity": "p2",
    "summary": "The inventory test was hard-coded and did not reject a duplicate nested build.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:a024ff02dcae6b0cd03c294af7f204e19a7c1723:1cef3165741451b5beb05d45c6d94fba295cbb90a403e56014e792c19a093d97",
    "route": null
  },
  {
    "id": "F-5788-5",
    "severity": "p2",
    "summary": "The rereview worktree contained uncommitted lifecycle and review-guide metadata.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:a024ff02dcae6b0cd03c294af7f204e19a7c1723:1cef3165741451b5beb05d45c6d94fba295cbb90a403e56014e792c19a093d97",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:a024ff02dcae6b0cd03c294af7f204e19a7c1723:1cef3165741451b5beb05d45c6d94fba295cbb90a403e56014e792c19a093d97")

Reviewer: Some("subagent:019fc8ce-d14d-7783-a875-70ad800660a2")

Result: pass
