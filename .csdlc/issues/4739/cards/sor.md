# Structured Output Record

Template: 1.0.0

Issue: 4739

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Added a fail-closed Unity-MCP Observatory alignment lane that binds the live endpoint to the canonical Unity project before accepting read-only scene evidence.

## Artifacts

- adl/tools/probe_unity_mcp_observatory_alignment.sh
- adl/tools/test_v0916_unity_mcp_alignment_unit.sh
- adl/config/validation_lane_selector.v0.91.6.json
- adl/tools/test_select_validation_lanes.sh
- docs/tooling/unity_mcp_observatory_alignment.md
- revision 3eb503273

## Execution

- Added the repository-owned alignment probe with persisted Custom-mode, loopback, editor, MCP Application.dataPath, port, and scene checks.
- Added deterministic no-Unity fixtures for success, project and identity mismatch, editor liveness, string and numeric Cloud mode, external fallback, random-port selection, ambiguity, malformed status, tool failure, redaction, and tool-call ordering.
- Registered one focused validation-selector lane and its selector fixture.
- Documented the bounded operator route and the observed Unity-MCP CLI process-discovery and plugin update-order defects under #4739.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_v0916_unity_mcp_alignment_unit.sh"
    ],
    "purpose": "Prove aligned and fail-closed project, endpoint, Cloud, liveness, ordering, and redaction classifiers.",
    "outcome": "passed",
    "evidence_ref": "Local exit 0 at revision 3eb503273: Unity-MCP alignment classifier fixtures passed."
  },
  {
    "command": [
      "bash",
      "adl/tools/test_v0916_unity_observatory_contract.sh"
    ],
    "purpose": "Prove the bounded Unity Observatory repository contract remains valid.",
    "outcome": "passed",
    "evidence_ref": "Local exit 0 at revision 3eb503273: v0.91.6 Unity Observatory contract guardrails passed."
  },
  {
    "command": [
      "bash",
      "adl/tools/test_select_validation_lanes.sh"
    ],
    "purpose": "Prove issue-owned paths select the dedicated Unity-MCP alignment lane.",
    "outcome": "passed",
    "evidence_ref": "Local exit 0 at revision 3eb503273: PASS test_select_validation_lanes."
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "74fb9904b..3eb503273"
    ],
    "purpose": "Prove the committed implementation range has no whitespace errors.",
    "outcome": "passed",
    "evidence_ref": "Local exit 0 with no output for exact committed range 74fb9904b..3eb503273."
  },
  {
    "command": [
      "env",
      "UNITY_MCP_CLI=/Users/daniel/git/Unity-MCP/cli/dist/cli.js",
      "ADL_PROCESS_BIN=/Users/daniel/git/agent-design-language/.adl/bin/adl",
      "bash",
      "adl/tools/probe_unity_mcp_observatory_alignment.sh",
      "--project",
      "/Volumes/FastWork/adl-unity-observatory/operator-provisioned-5332/unity-observatory",
      "--url",
      "http://localhost:23011",
      "--editor-pid",
      "9344"
    ],
    "purpose": "Prove the loopback MCP endpoint is attached to the intended live Observatory editor and loaded scene.",
    "outcome": "passed",
    "evidence_ref": "Local PASS: persisted Custom mode; PID 9344 bound by project-local log; MCP Application.dataPath matched canonical project; port 23011 bound without broad scan; FlagshipObservatoryStage loaded, clean, valid, RootCount 12."
  },
  {
    "command": [
      "bash",
      "-c",
      "UNITY_MCP_CLI=/Users/daniel/git/Unity-MCP/cli/dist/cli.js ADL_PROCESS_BIN=/Users/daniel/git/agent-design-language/.adl/bin/adl bash adl/tools/probe_unity_mcp_observatory_alignment.sh --project /Volumes/FastWork/adl-unity-observatory/operator-provisioned-5332/unity-observatory --url http://localhost:23011 --editor-pid 9344 > .csdlc/issues/4739/evidence/unity-mcp-alignment-live.txt"
    ],
    "purpose": "Retain sanitized, reviewable live endpoint, project identity, permission-safe port, and read-only scene output.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/issues/4739/evidence/unity-mcp-alignment-live.txt sha256:2ec84c325ec8864e531fd1e6b0a881c439418daf6e246c302927de8d09452a95; focused PCRE2 secret-pattern scan returned no matches."
  },
  {
    "command": [
      "bash",
      "-c",
      "UNITY_MCP_CLI=/Users/daniel/git/Unity-MCP/cli/dist/cli.js ADL_PROCESS_BIN=/Users/daniel/git/agent-design-language/.adl/bin/adl bash adl/tools/probe_unity_mcp_observatory_alignment.sh --project /Volumes/FastWork/adl-unity-observatory/operator-provisioned-5332/unity-observatory --url http://localhost:23011 --editor-pid 9344 > .csdlc/prepared/issues/4739/unity-mcp-alignment-live.txt"
    ],
    "purpose": "Retain sanitized live proof at the protected prepared-artifact path that survives typed card projections.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/prepared/issues/4739/unity-mcp-alignment-live.txt sha256:2ec84c325ec8864e531fd1e6b0a881c439418daf6e246c302927de8d09452a95; supersedes the non-retained issue-local path from the prior validation entry; focused PCRE2 secret-pattern scan returned no matches."
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
