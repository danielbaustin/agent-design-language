# Validation Planning Prompt

Template: 1.0.0

Issue: 5383

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: docs/milestones/v0.91.8/setup/5383/DESIGN.md

Diagram: docs/milestones/v0.91.8/setup/5383/DIAGRAM.mmd

## Selected Lanes

[
  {
    "lane": "issue-routing-verification",
    "proof_role": "Verify #4641 restored and WP-14A preservation issue exists via live issue routing evidence",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "gh",
      "issue",
      "view",
      "4641",
      "--json",
      "title,labels"
    ],
    "parallel_group": "routing",
    "defer_reason": null
  },
  {
    "lane": "planning-package-presence",
    "proof_role": "Verify the required v0.91.8 planning package exists",
    "acceptance_ids": [
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "test",
      "-f",
      "docs/milestones/v0.91.8/README.md"
    ],
    "parallel_group": "local-docs",
    "defer_reason": null
  },
  {
    "lane": "feature-doc-presence",
    "proof_role": "Verify v0.91.8 feature documentation surfaces exist and are reviewable",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "test",
      "-f",
      "docs/milestones/v0.91.8/features/README.md"
    ],
    "parallel_group": "local-docs",
    "defer_reason": null
  },
  {
    "lane": "docs-diff-check",
    "proof_role": "Whitespace and patch hygiene for docs/package edits",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "local-docs",
    "defer_reason": null
  },
  {
    "lane": "yaml-parse",
    "proof_role": "Parse v0.91.8 issue wave YAML",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      "-c",
      "import yaml, pathlib; yaml.safe_load(pathlib.Path('docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml').read_text())"
    ],
    "parallel_group": "local-docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `gh issue view 4641 --json title,labels`
- `test -f docs/milestones/v0.91.8/README.md`
- `test -f docs/milestones/v0.91.8/features/README.md`
- `git diff --check`
- `python3 -c import yaml, pathlib; yaml.safe_load(pathlib.Path('docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml').read_text())`

## Failure Semantics

Fail closed with explicit blocker rows and no readiness claim if issue routing, YAML validity, or planned-posture validation cannot be proven.

## Handoff

Retain typed evidence before convergence.
