# Structured Output Record

Template: 1.0.0

Issue: 5815

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Finalized the executable migration plan for five repositories moving to the Agent Logic organization and incorporated the bounded Gemini 3.1 Pro review findings.

## Artifacts

- .adl/docs/TBD/AGENT_LOGIC_ACCOUNT_REPO_MIGRATION_PLAN.md
- .csdlc/evidence/5815/gemini-3.1-pro-review-result.json

## Execution

- Defined the five-repository transfer inventory, order, gates, rollback, and verification
- Excluded asksifu and Horust with explicit dispositions
- Included agent-logic.ai link cutover and destination GitHub, GHCR, LFS, Pages, OIDC, secret, team, and App handling
- Recorded and dispositioned Gemini 3.1 Pro review findings

## Validation

[
  {
    "command": [
      "git diff --no-index --check /dev/null .adl/docs/TBD/AGENT_LOGIC_ACCOUNT_REPO_MIGRATION_PLAN.md",
      "jq assertions over Gemini result",
      "rg assertions over required migration topics",
      "csdlc-doctor --repo . --issue 5815"
    ],
    "purpose": "Prove document hygiene, exact migration inventory coverage, valid Gemini 3.1 Pro evidence, and healthy typed issue state.",
    "outcome": "passed",
    "evidence_ref": "Focused validation passed: no whitespace errors; Gemini route google/gemini-3.1-pro-preview returned status ok; all required repositories, exclusions, website cutover, GHCR, LFS, and OIDC topics are present; typed doctor passed with no findings."
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
