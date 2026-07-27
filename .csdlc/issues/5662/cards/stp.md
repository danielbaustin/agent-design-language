# Structured Task Prompt

Template: 1.0.0

Issue: 5662

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Polish and prove only the repository-owned Observatory scene, controller, UI, contract, and proof surfaces while authoring against the operator-provisioned FastWork Unity project.

## Deliverables

- Composed flagship Observatory environment and stable hero camera
- Controlled lighting and material hierarchy
- Fixed readable operator shell with clear icons and internal scrolling
- Truthful live, degraded, disconnected, and demo runtime states
- Supported agent, event, governance, evidence, and communication interactions
- Retained intended-project Play Mode proof at both target resolutions
- Durable routing for every tooling anomaly encountered

## Acceptance

1. AC-1: the hero view has a clear focal Observatory structure, grounded foreground, readable middle ground and background, and a deliberate route into the environment
2. AC-2: the camera does not clip major architecture or reveal accidental voids, floating terrain edges, disconnected ramps, or unfinished staging geometry
3. AC-3: lighting preserves material detail and depth, uses cyan as a controlled accent, and keeps dark regions intentionally readable
4. AC-4: UI text and icons are crisp, aligned, non-overlapping, and legible at 1920x1080 and 2560x1440 with a fixed dashboard and only internal scrolling
5. AC-5: the shell explicitly identifies live, degraded, disconnected, and demo states and never presents fixture data as live Polis truth
6. AC-6: live mode consumes repository-owned runtime contracts for supported agent, event, health, governance, evidence, and communication fields while unsupported fields remain explicit non-claims
7. AC-7: an operator can select an agent or subsystem, inspect current state, observe event flow, and send a bounded communication through the supported contract or receive an exact fail-closed reason
8. AC-8: direct Unity proof identifies the intended FastWork project, loaded FlagshipObservatoryStage, clean Play Mode result, and retained 1920x1080 and 2560x1440 captures
9. AC-9: focused validation passes without building a player binary or replacement owner binary and without copying licensed payloads into Git
10. AC-10: every encountered Unity-MCP, editor, runtime, asset, or proof-tool anomaly is retained in #5662 or routed to a named follow-up

## Dependencies

- Typed closed_out prerequisites #4739, #4741, and #5332
- Operator-provisioned FastWork Unity project and licensed local asset packs
- Repository-installed Unity-MCP CLI and #4739 alignment probe
- Repository-owned Observatory runtime contract
- WP-15 #5354 consumes the final proof

## Inputs

- GitHub issue #5662
- GitHub issue #5354
- demos/v0.91.6/unity-observatory
- adl/tools/probe_unity_mcp_observatory_alignment.sh
- adl/tools/test_v0916_unity_observatory_contract.sh
- .adl/docs/TBD/csm_observatory/UNITY_OBSERVATORY_DEMO.md
- operator-provided current-stage screenshot and Observatory dashboard visual direction

## Non Goals

- Do not build standalone player binaries
- Do not build replacement ADL, C-SDLC, Unity-MCP, or other owner binaries
- Do not republish licensed asset packs or add multi-gigabyte asset payloads to Git
- Do not absorb WP-14A platform acceptance or unrelated release-tail work
- Do not claim production cloud integration, complete investor readiness, or unsupported runtime semantics
- Do not treat static source checks as Unity visual or runtime proof
