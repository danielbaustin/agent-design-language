# Structured Review Prompt

Template: 1.0.0

Issue: 5757

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

## Scope

adl/tools/test_v0917_html_observatory_integrated_proof.sh
adl/tools/validate_v0917_html_observatory.py
demos/html-observatory/
infra/runtime-v3/runtime-init.toml

## Prompts

- Verify untrusted origins are rejected before bearer/WSS use.
- Verify monotonic generation ordering covers live, retained, WSS, and fallback completions.
- Verify shared-certificate/browser-control/authenticated-WSS proof is real and not fixture-only.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:21536d308bfb0648c395490b621a388f9be191bc:5b4f0b7a078af58462c36b9470113e867b7fb5e1e37ec1ac1cbeaa785fcb15a4")

Reviewer: Some("codex-bounded-review")

Result: pass
