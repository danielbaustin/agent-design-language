# Issue 5683 Tooling Anomalies

These anomalies were observed against the live project and are retained here for routing:

1. `probe_unity_mcp_observatory_alignment.sh` defaults to a worktree-local `.adl/bin/adl`, but a fresh bound worktree does not contain that generated binary. The probe passed when `ADL_PROCESS_BIN` named the stable repo owner binary.
2. Unity-MCP process discovery reported zero Unity processes while the repository process helper, project-local editor log, MCP project identity, and opened scene all corroborated the live editor.
3. `gameobject-modify` accepted `jsonPatchesPerGameObject` only as JSON-encoded strings, and `activeSelf` could not be assigned because the reflected property is read-only.
4. Disabling nested prefab particle children did not persist as a prefab override into Play Mode. Root-level suppression of the incompatible particle-heavy imported objects was required.
5. `screenshot-game-view` captures the current Game View dimensions but exposes no resolution argument. A deterministic Unity editor helper was required to select Full HD and QHD before capture.
6. Copying a revised C# source into the operator-provisioned project did not trigger import or compilation until `UnityEditor.AssetDatabase.Refresh()` was invoked through Unity-MCP.
7. Unity-MCP CLI `0.82.2` advertises an available update on every call. No update or replacement binary was installed during this issue.

No secret-bearing output is retained.
