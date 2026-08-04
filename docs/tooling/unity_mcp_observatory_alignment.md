# Unity-MCP Observatory Alignment

This runbook proves that one local Unity-MCP endpoint is attached to the
intended Observatory project before any scene result is accepted as evidence.
It does not prove runtime integration, scene quality, or investor readiness.

## Prerequisites

- Open exactly one Unity editor for the intended Observatory project.
- Install the project-compatible AI Game Developer package.
- Start its local HTTP MCP server. A random or deterministic project port is
  acceptable; do not prescribe a global port.
- Persist the project connection mode as `Custom`. An explicit `--url` does
  not override a persisted `Cloud` configuration for this proof.
- Use the repository Unity-MCP CLI and stable `.adl/bin/adl` binary.
- Keep cloud and externally hosted endpoints out of this proof lane.

## Probe

When Unity-MCP process discovery recognizes the editor:

```bash
ADL_UNITY_PROJECT_PATH=/path/to/unity-observatory
bash adl/tools/probe_unity_mcp_observatory_alignment.sh \
  --project "$ADL_UNITY_PROJECT_PATH" \
  --url "$DISCOVERED_LOCAL_MCP_URL"
```

When the CLI reports a known false negative, pass the exact editor PID only
after obtaining it from the editor launch receipt:

```bash
bash adl/tools/probe_unity_mcp_observatory_alignment.sh \
  --project "$ADL_UNITY_PROJECT_PATH" \
  --url "$DISCOVERED_LOCAL_MCP_URL" \
  --editor-pid "$UNITY_EDITOR_PID"
```

The recovery path uses `adl process status --pid`, then requires the
project-local `Logs/Editor.log` to bind that PID to the canonical project. It
never performs a broad host process scan.

## Pass Boundary

A pass requires all of the following:

1. status echoes the canonical intended project;
2. persisted project configuration reports `Custom`, not `Cloud`;
3. one discovered loopback endpoint is selected explicitly, or exactly one
   loopback endpoint is resolved without an explicit discovery result;
4. the intended editor is proven by CLI status or PID plus project-local log;
5. the endpoint returns the canonical `<project>/Assets` path from
   `UnityEngine.Application.dataPath`;
6. `adl process status --port` reports a bound loopback port;
7. `scene-list-opened` returns a successful read-only MCP response.

Project mismatch, missing editor proof, cloud or external fallback, malformed
status, endpoint ambiguity without an explicit discovery result, MCP-reported
project identity mismatch, and read-only tool failure are `FAIL_CLOSED`.
Output is sanitized for URL userinfo, authorization values, bearer tokens,
credential fields, access and refresh tokens, client secrets, environment
secret assignments, passwords, and API keys.

## Current Tooling Findings

- Unity-MCP CLI `0.82.2` can report zero Unity processes for an editor it
  launched successfully. The probe permits a bounded PID-plus-project-log
  corroboration path and retains the false negative visibly.
- CLI status reports both the project-derived local URL and the config or
  explicit URL when they differ. A discovered explicit loopback URL therefore
  selects that exact reported candidate; two unselected candidates still fail
  closed as ambiguous.
- A project without `com.ivanmurzak.unity.mcp` can load the Observatory scene
  while no MCP server exists. Scene visibility is not MCP readiness.
- An in-editor update from plugin `0.82.3` to `0.86.1` was observed restoring
  the old `McpPlugin 6.10.0` dependency before compiling code requiring the
  newer credential-provider contract. The resulting `CS0115` is a plugin
  update-order defect; do not patch the immutable package cache. Recover with
  the supported CLI remove/install route and a known-compatible project pin.
- The plugin's server download is not ready until its pinned archive passes
  SHA256 verification and is atomically published. Connection refusals during
  that bounded download interval are not proof of a permanent endpoint fault.

These findings belong to #4739. Batch editor liveness remains owned by #4741,
ILPP diagnostics by #5332, and Observatory visual/runtime polish by #5354.
