# Structured Output Record

Template: 1.0.0

Issue: 5756

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Implemented MiniMax-scoped billing classification for code 1008 and cross-provider regressions.

## Artifacts

- adl/src/provider_adapter.rs
- .csdlc/evidence/5756

## Execution

- Refactored MiniMax base_resp error classification into a reusable MiniMax-only helper.
- Parse MiniMax non-success HTTP base_resp envelopes before falling back to shared HTTP classification.
- Removed the shared bare 1008 substring billing shortcut.
- Added positive MiniMax and negative OpenAI, Anthropic, DeepSeek, and generic hosted provider regressions.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--offline",
      "--manifest-path",
      "adl/Cargo.toml",
      "provider_adapter::tests::minimax",
      "--lib"
    ],
    "purpose": "Prove MiniMax status_code 1008 remains non-retryable ProviderBillingBlocked.",
    "outcome": "passed",
    "evidence_ref": "provider-minimax-tests.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--offline",
      "--manifest-path",
      "adl/Cargo.toml",
      "provider_adapter::tests::non_minimax",
      "--lib"
    ],
    "purpose": "Prove non-MiniMax hosted providers do not inherit MiniMax billing classification from bare 1008 text.",
    "outcome": "passed",
    "evidence_ref": "provider-non-minimax-tests.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--offline",
      "--manifest-path",
      "adl/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove the provider adapter changes are warning-clean under strict Clippy.",
    "outcome": "passed",
    "evidence_ref": "strict-clippy.log"
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
