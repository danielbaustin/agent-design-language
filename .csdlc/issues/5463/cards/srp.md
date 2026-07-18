# Structured Review Prompt

Template: 1.0.0

Issue: 5463

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

.github/workflows/aws-codefriend-build.yaml
.github/workflows/aws-spot-remote-validation.yaml
.github/workflows/ci.yaml
.github/workflows/nightly-coverage-ratchet.yaml
.github/workflows/v0871_milestone_closeout_gate.yaml
adl/tools/test_ci_runtime_contracts.sh
adl/tools/test_ci_path_policy.sh
docs/tooling/GITHUB_ACTIONS_RUNTIME_PIN_INVENTORY.md

## Prompts

- Are all annotated occurrences replaced?
- Do major upgrades preserve used inputs and outputs?
- Does the static contract reject deprecated or floating pins?
- Are hosted annotations genuinely absent?

## Findings

[
  {
    "id": "F-5463-1",
    "severity": "p2",
    "summary": "Valid quoted YAML uses scalars bypass canonical and deprecated action pin enforcement.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:7c19d5461d71c79e8a4b9c4ba1a3f7d4f7c991ac:b785ae5945c851b62e70638a0713b7a9d0773a531e0329c20f6035ee19667fa2",
    "route": null
  },
  {
    "id": "F-5463-2",
    "severity": "p2",
    "summary": "Rejected negative fixtures increment real workflow occurrence counts and can mask action disappearance.",
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

Revision: Some("git-blake3:7c19d5461d71c79e8a4b9c4ba1a3f7d4f7c991ac:b785ae5945c851b62e70638a0713b7a9d0773a531e0329c20f6035ee19667fa2")

Reviewer: Some("bounded-subagent-review-5463")

Result: changes_required
