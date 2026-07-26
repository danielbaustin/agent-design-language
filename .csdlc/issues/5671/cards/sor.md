# Structured Output Record

Template: 1.0.0

Issue: 5671

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented Claude Opus 5 profile, setup template, and focused provider proof.

## Artifacts

- .csdlc/prepared/issues/5671/validation-evidence/provider-profile-focused.log
- .csdlc/prepared/issues/5671/validation-evidence/provider-build-focused.log

## Execution

- adl/src/provider/profiles.rs
- adl/src/provider/mod.rs
- adl/src/provider/http_family/tests.rs
- adl/src/cli/provider_cmd.rs
- adl/src/cli/usage.rs
- adl/tests/provider_tests/profiles.rs

## Validation

[
  {
    "command": [
      "cargo",
      "check",
      "--manifest-path",
      "adl/Cargo.toml"
    ],
    "purpose": "Compile the ADL crate after the provider change",
    "outcome": "passed",
    "evidence_ref": "provider-build-focused.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "provider_"
    ],
    "purpose": "Run the focused provider test suite for Opus 5",
    "outcome": "passed",
    "evidence_ref": "provider-profile-focused.log"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
