# Validation Planning Prompt

Template: 1.0.0

Issue: 5825

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5825/design.md

Diagram: .csdlc/prepared/issues/5825/diagram.mmd

## Selected Lanes

[
  {
    "lane": "birthday-runtime-v3",
    "proof_role": "Run the exact Runtime v3 integration target and fail when the selected target contains no tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "birthday",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "parallel_group": "5825-core",
    "defer_reason": null
  },
  {
    "lane": "birthday-native-platform-receipts",
    "proof_role": "Require passed native macOS and Linux receipts with a nonzero test count and identical fixture digest.",
    "acceptance_ids": [
      "AC-4",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      "-rjson",
      "-e",
      "receipts=ARGV.map{|p| JSON.parse(File.read(p))};\nabort \"native platform receipts must cover macos and linux\" unless receipts.map{|r| r[\"platform\"]}.sort==%w[linux macos];\nabort \"native proof failed\" unless receipts.all?{|r| r[\"status\"]==\"passed\" && Integer(r[\"tests_run\"])>0};\nabort \"fixture digest mismatch\" unless receipts.map{|r| r[\"fixture_digest\"]}.uniq.length==1",
      ".csdlc/evidence/5825/native-platform/macos.json",
      ".csdlc/evidence/5825/native-platform/linux.json"
    ],
    "parallel_group": "5825-platform",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo nextest run --manifest-path adl-runtime-kernel/Cargo.toml --test birthday --no-tests=fail --status-level all`
- `ruby -rjson -e receipts=ARGV.map{|p| JSON.parse(File.read(p))};
abort "native platform receipts must cover macos and linux" unless receipts.map{|r| r["platform"]}.sort==%w[linux macos];
abort "native proof failed" unless receipts.all?{|r| r["status"]=="passed" && Integer(r["tests_run"])>0};
abort "fixture digest mismatch" unless receipts.map{|r| r["fixture_digest"]}.uniq.length==1 .csdlc/evidence/5825/native-platform/macos.json .csdlc/evidence/5825/native-platform/linux.json`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
