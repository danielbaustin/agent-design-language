# Structured Review Prompt

Template: 1.0.0

Issue: 5467

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.github/workflows/ci.yaml
adl/tools/resolve_ci_backend.sh
adl/tools/setup_aws_spot_remote_validation_github_resources.sh
adl/tools/test_run_aws_spot_ci_profile.sh
.csdlc/issues/5467

## Prompts

- Can the new assertions remain unreachable?
- Do fixtures prove behavior rather than grep only?
- Can any case invoke AWS?

## Findings

[
  {
    "id": "F-5467-1",
    "severity": "p1",
    "summary": "Removing all stale setup assertions weakened live SSM, EBS attachment, and IAM cleanup policy coverage.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:cf96c76f32ec7c4e3c05df660ba259f96730023a:b292c08fe883633bcc7664151eecebf86c4f8ec94a11fe499054b4301875e2bc",
    "route": null
  },
  {
    "id": "F-5467-2",
    "severity": "p3",
    "summary": "Invalid backend proof accepted any nonzero status while lifecycle truth claimed exact status 2.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:cf96c76f32ec7c4e3c05df660ba259f96730023a:b292c08fe883633bcc7664151eecebf86c4f8ec94a11fe499054b4301875e2bc",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- AWS deployment and live permission exercise remain explicitly outside this issue; proof is source-to-policy local contract plus GitHub-hosted CI.

## Review Result

Revision: Some("git-blake3:cf96c76f32ec7c4e3c05df660ba259f96730023a:b292c08fe883633bcc7664151eecebf86c4f8ec94a11fe499054b4301875e2bc")

Reviewer: Some("bounded-subagent-review-5467")

Result: pass
