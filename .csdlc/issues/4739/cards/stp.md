# Structured Task Prompt

Template: 1.0.0

Issue: 4739

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Selectively port and finish only the MCP alignment surface from preserved predecessor work onto the current v0.91.8 base.

## Deliverables

- Repository-owned Unity-MCP Observatory alignment probe
- Focused deterministic alignment classifier tests
- Permission-safe live endpoint and read-only MCP proof or exact fail-closed blocker
- Operator alignment runbook
- Truthful downstream WP-15 routing note

## Acceptance

1. AC-1: the probe canonicalizes and reports the intended Unity project path and derives the active MCP endpoint without assuming a fixed port
2. AC-2: a read-only MCP scene or tool call is attempted only after project and endpoint identity agree
3. AC-3: project mismatch, missing editor, cloud or external fallback, malformed status, ambiguous endpoint, and read-only tool failure each produce deterministic FAIL_CLOSED output
4. AC-4: retained output redacts URL userinfo, tokens, authorization values, credentials, and secret-bearing Unity settings
5. AC-5: local liveness uses adl process status and no broad ps, pgrep, or lsof scan
6. AC-6: a dedicated no-Unity unit script exercises matching status, project mismatch, missing editor, cloud fallback, malformed status, endpoint ambiguity, redaction, and read-only tool failure
7. AC-7: live proof succeeds against the intended Observatory project or records the exact current blocker and owner without claiming runtime, visual, or investor readiness
8. AC-8: the final diff excludes #4741 batch-liveness, #5332 ILPP, scene-fallback, runtime-contract, asset, and walkthrough implementation
9. AC-9: validation selector registration maps the probe, dedicated unit test, and runbook to one focused non-Unity alignment lane and its selector test passes

## Dependencies

- Current repository-approved Unity-MCP CLI contract
- Permission-safe adl process status binary
- #4741 only when editor liveness prevents the optional live alignment proof
- WP-15 #5354 consumes the final proof or explicit blocker

## Inputs

- GitHub issue #4739
- adl/tools/test_v0916_unity_observatory_contract.sh
- demos/v0.91.6/unity-observatory/README.md
- demos/v0.91.6/unity-observatory/PROOF_PACKET.md
- docs/tooling/PERMISSION_SAFE_PROCESS_STATUS.md
- preserved predecessor #4739 candidate diff

## Non Goals

- Do not rebuild or polish the Unity scene
- Do not change batch-editor launch or watchdog behavior
- Do not diagnose or fix ILPP startup
- Do not implement runtime shell integration or walkthrough capture
- Do not require one hard-coded MCP port
- Do not expose credentials or use cloud fallback
