# Structured Output Record

Template: 1.0.0

Issue: 5675

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented typed Kimi/Moonshot and MiniMax hosted routes with bounded budgets, current MiniMax endpoint, billing envelope classification, and redacted shared extraction.

## Artifacts

- .csdlc/prepared/issues/5675/design.md
- .csdlc/prepared/issues/5675/diagram.mmd
- .adl/provider-adapter-focused-tests.log
- .adl/live-provider-probe-disposition.md

## Execution

- adl/src/provider_adapter.rs
- adl/src/provider/profiles.rs

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "provider_adapter::tests",
      "--lib"
    ],
    "purpose": "Focused adapter routing, budget, envelope, and redaction proof",
    "outcome": "passed",
    "evidence_ref": ".adl/provider-adapter-focused-tests.log"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
