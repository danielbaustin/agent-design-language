# Validation Planning Prompt

Template: 1.0.0

Issue: 5824

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5824/design.md

Diagram: .csdlc/prepared/issues/5824/diagram.mmd

## Selected Lanes

[
  {
    "lane": "enum-inventory-contract",
    "proof_role": "Validate complete field ownership and one disposition per restricted current-v2 field.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      "-rjson",
      "-e",
      "r=JSON.parse(File.read('.csdlc/evidence/5824/enum-inventory.json')); abort('empty') unless r.is_a?(Array)&&!r.empty?; allowed=%w[typed_complete finite_gap intentionally_extensible]; abort('bad row') unless r.all?{|x|x['field'].to_s!=''&&allowed.include?(x['disposition'])&&x['stored_string']&&x['owners'].is_a?(Hash)}; abort('duplicate') unless r.map{|x|x['field']}.uniq.length==r.length"
    ],
    "parallel_group": "inventory",
    "defer_reason": null
  },
  {
    "lane": "typed-card-roundtrip-and-schema",
    "proof_role": "Prove parse/display/serde, public schema, semantic editor, Markdown importer, renderer, and existing-card parity.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2",
      "enum"
    ],
    "parallel_group": "csdlc-v2",
    "defer_reason": null
  },
  {
    "lane": "invalid-value-and-legacy-negative",
    "proof_role": "Reject unknown finite values and prove any supported alias normalization is explicit and lossless.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2",
      "invalid"
    ],
    "parallel_group": "csdlc-v2",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace and unintended template or sunset-v1 changes and support exact-revision review.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
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

Seconds: 3600

Tokens: 25000

## Commands

- `ruby -rjson -e r=JSON.parse(File.read('.csdlc/evidence/5824/enum-inventory.json')); abort('empty') unless r.is_a?(Array)&&!r.empty?; allowed=%w[typed_complete finite_gap intentionally_extensible]; abort('bad row') unless r.all?{|x|x['field'].to_s!=''&&allowed.include?(x['disposition'])&&x['stored_string']&&x['owners'].is_a?(Hash)}; abort('duplicate') unless r.map{|x|x['field']}.uniq.length==r.length`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate2 enum`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate2 invalid`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
