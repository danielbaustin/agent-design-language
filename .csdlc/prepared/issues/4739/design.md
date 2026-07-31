# Issue 4739 design

## Decision

Provide one repository-owned Unity-MCP alignment probe that accepts the intended
Unity project path, resolves the endpoint associated with that project, and
performs a read-only MCP call only after project and endpoint identity agree.

The probe must not assume a fixed port. It must report the endpoint source,
canonical project path, permission-safe liveness result, read-only tool result,
and one terminal `PASS` or `FAIL_CLOSED` classification.

## Ownership boundary

Issue #4739 owns Unity project and MCP endpoint identity, bounded parsing,
redaction, read-only proof, and its focused tests and runbook.

Issue #4741 owns editor liveness, open-editor versus batch-editor selection, and
batch watchdog behavior. Issue #5332 owns ILPP retry-loop diagnosis and
classification. Scene staging, fallback geometry, runtime contract generation,
and investor rendering are outside #4739.

## Preserved predecessor work

An older dirty #4739 worktree contains candidate changes across several Unity
ownership boundaries. It remains untouched. Execution should selectively port
only the MCP alignment probe, its focused tests, and bounded documentation after
reviewing the current base.

## Proof design

1. Canonicalize and validate the intended Unity project path.
2. Ask the repository-approved Unity-MCP CLI for project-specific status.
3. Parse the project and endpoint from structured or explicitly bounded output.
4. Use `adl process status --port <resolved-port> --json` when a local port is
   available.
5. Refuse endpoint/project disagreement, missing editor identity, cloud fallback,
   malformed output, or ambiguous multiple-project state.
6. Invoke one documented read-only MCP scene/tool operation.
7. Redact URL userinfo, tokens, authorization values, and machine-local secret
   paths from retained output.

## Test design

- Deterministic fixtures cover matching project/endpoint, mismatched project,
  missing Unity editor, cloud/external fallback, malformed status, and failed
  read-only tool calls.
- Shell syntax and contract tests prove the repository command shape.
- The validation selector maps the probe, its dedicated unit test, and runbook
  to one focused non-Unity alignment lane; selector behavior has a focused test.
- A live probe is retained only when the intended project is running; otherwise
  the exact fail-closed result remains truthful blocker evidence.
