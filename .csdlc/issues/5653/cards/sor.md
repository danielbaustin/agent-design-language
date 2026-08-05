# Structured Output Record

Template: 1.0.0

Issue: 5653

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Updated the root README milestone badge and status to v0.91.8 release-tail truth, added the ADL homepage link, and documented that later milestones are not hosted releases.

## Artifacts

- .csdlc/evidence/5653/readme-focused.txt

## Execution

- README.md

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Prove README wording, homepage link, badge branch, and documentation whitespace/link boundary",
    "outcome": "passed",
    "evidence_ref": "Focused README assertions and git diff check passed; https://agent-logic.ai returned HTTP 200."
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Verify published-release links, homepage link, canonical main badges, and documentation whitespace",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5653/readme-focused.txt"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
