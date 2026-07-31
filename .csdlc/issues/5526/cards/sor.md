# Structured Output Record

Template: 1.0.0

Issue: 5526

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Added deterministic first-class provider profiles and preserved vendor identity across the shared HTTP transport for current Kimi, MiniMax, hosted Qwen, xAI/Grok, Mistral, Cohere, DeepSeek v4, Z.ai GLM-5, and Gemini 3.1 model lanes.

## Artifacts

- adl/src/provider/profiles.rs
- adl/src/provider_substrate.rs
- adl/src/provider/mod.rs
- .csdlc/evidence/5526/implementation/provider-expansion.log

## Execution

- Added stable provider profile names, endpoints, and provider model identifiers for each expanded vendor lane.
- Extended provider substrate vendor inference so shared transport does not collapse distinct vendor identities.
- Added focused registry and substrate tests for deterministic identity and model selection.
- Kept credentials, live calls, AWS/Bedrock, and OpenRouter routing out of the deterministic implementation lane.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "provider_"
    ],
    "purpose": "Prove expanded provider identities, deterministic model identifiers, vendor inference, adapter redaction, and provider regression behavior.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5526/implementation/provider-expansion.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "--test",
      "provider_tests",
      "provider_"
    ],
    "purpose": "Prove expanded provider identities, deterministic model identifiers, vendor inference, chat adapter request/response shapes, and legacy generic HTTP compatibility.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5526/implementation/provider-expansion.log"
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
