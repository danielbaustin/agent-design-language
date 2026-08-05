# Validation Planning Prompt

Template: 1.0.0

Issue: 5818

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5818/design.md

Diagram: .csdlc/prepared/issues/5818/diagram.mmd

## Selected Lanes

[
  {
    "lane": "canonical-inventory-contract",
    "proof_role": "Validate the retained inventory has unique paths, allowed dispositions, owners, and complete current-surface coverage.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      "-rjson",
      "-e",
      "p='.csdlc/evidence/5818/canonical-surface-inventory.json'; rows=JSON.parse(File.read(p)); abort('empty inventory') unless rows.is_a?(Array)&&!rows.empty?; allowed=%w[update already_current historical_preserve not_authoritative]; abort('invalid inventory') unless rows.all?{|r| r['path'].to_s!=''&&allowed.include?(r['disposition'])&&r['owner'].to_s!=''}; abort('duplicate path') unless rows.map{|r|r['path']}.uniq.length==rows.length"
    ],
    "parallel_group": "docs-contract",
    "defer_reason": null
  },
  {
    "lane": "version-and-structure-parity",
    "proof_role": "Prove Cargo metadata, current version parity, and structured YAML/JSON/Markdown entrypoints are consistent.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "metadata",
      "--locked",
      "--format-version",
      "1"
    ],
    "parallel_group": "metadata",
    "defer_reason": null
  },
  {
    "lane": "historical-preservation-negative",
    "proof_role": "Reject edits that rewrite historical milestone, release, review, migration, or evidence claims.",
    "acceptance_ids": [
      "AC-4",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--exit-code",
      "origin/main",
      "--",
      "docs/milestones/v0.91.8",
      "docs/releases",
      ".csdlc/evidence"
    ],
    "parallel_group": "negative",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace errors and support exact-revision bounded review.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby -rjson -e p='.csdlc/evidence/5818/canonical-surface-inventory.json'; rows=JSON.parse(File.read(p)); abort('empty inventory') unless rows.is_a?(Array)&&!rows.empty?; allowed=%w[update already_current historical_preserve not_authoritative]; abort('invalid inventory') unless rows.all?{|r| r['path'].to_s!=''&&allowed.include?(r['disposition'])&&r['owner'].to_s!=''}; abort('duplicate path') unless rows.map{|r|r['path']}.uniq.length==rows.length`
- `cargo metadata --locked --format-version 1`
- `git diff --exit-code origin/main -- docs/milestones/v0.91.8 docs/releases .csdlc/evidence`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
