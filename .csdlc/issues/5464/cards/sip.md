# Structured Intent Prompt

Template: 1.0.0

Issue: 5464

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Install cargo-nextest 0.9.140 directly from a supported immutable manifest without fallback.

## Required Outcome

Every hosted nextest install uses the reviewed manifest, fails closed if unsupported, and emits no cargo-binstall fallback warning.

## Scope

- .github/workflows/ci.yaml
- adl/tools/test_ci_runtime_contracts.sh

## Authority

- GitHub run 29632957768 is warning evidence
- Official nextest release assets and install-action manifests are installation authority
- Repository CI runtime contracts preserve the canonical pin and fallback policy

## Assumptions

- none

## Operator Constraints

- none
