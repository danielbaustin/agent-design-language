# Structured Review Prompt

Template: 1.0.0

Issue: 5463

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

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
    "fix_revision": "git-blake3:e3829a5a18db93b18ba0940fdd8e30fd2ff727cf:ac535a46baee767e7444ff08f7125b34c58be12d22a7d38865641bcbfb06c44d",
    "route": null
  },
  {
    "id": "F-5463-2",
    "severity": "p2",
    "summary": "Rejected negative fixtures increment real workflow occurrence counts and can mask action disappearance.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:e3829a5a18db93b18ba0940fdd8e30fd2ff727cf:ac535a46baee767e7444ff08f7125b34c58be12d22a7d38865641bcbfb06c44d",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:e3829a5a18db93b18ba0940fdd8e30fd2ff727cf:ac535a46baee767e7444ff08f7125b34c58be12d22a7d38865641bcbfb06c44d")

Reviewer: Some("bounded-subagent-review-5463")

Result: pass
