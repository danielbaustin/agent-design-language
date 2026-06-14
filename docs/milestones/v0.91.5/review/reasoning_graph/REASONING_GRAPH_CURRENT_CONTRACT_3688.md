# Reasoning Graph Current Contract

## Status

`contract_defined_for_follow_on_implementation`

## Purpose

This packet reconnects ADL's older reasoning-graph work to the current C-SDLC runtime, trace, governance, and proof model.

The main conclusion is deliberately narrow: reasoning graphs already exist as ADL artifacts, but their current role must be treated as an evidence/provenance projection over traceable events, not as private chain-of-thought exposure and not as a completed signed/queryable trace system.

## Source Evidence

| Source | Current meaning |
| --- | --- |
| Issue `#876` `[v0.85][WP-16] Reasoning graph and affect integration` | Closed implementation issue for the first reasoning graph + affect integration slice. |
| PR `#962` `[v0.85][WP-16] Reasoning graph and affect integration` | Merged implementation PR for the older reasoning graph integration. |
| `docs/milestones/v0.85/REASONING_GRAPH_SCHEMA_V0.85.md` | Historical conceptual schema direction. Useful as prior art, not the current runtime contract by itself. |
| `docs/milestones/v0.85/DEMO_MATRIX_v0.85.md` | Records the v0.85 demo/proof status for reasoning graph + affect integration. |
| `adl/tools/demo_reasoning_graph_affect.sh` | Historical demo route that prints initial/adapted `learning/reasoning_graph.v1.json` artifacts. |
| `adl/tools/demo_affect_godel_vertical_slice.sh` | Historical vertical slice that refreshes affect and reasoning graph artifacts before Gödel mapping. |
| `adl/src/artifacts.rs` | Current run-path accessor includes `learning/reasoning_graph.v1.json`. |
| `adl/src/cli/run_artifacts_types.rs` | Current Rust types include `ReasoningGraphArtifact`, `ReasoningGraphRecord`, node, edge, and selected-path structures. |
| `adl/src/cli/run_artifacts/cognitive/state_artifacts/decision_artifacts.rs` | Current builder creates reasoning graph artifacts from execution/decision state. |
| `adl/src/cli/run_artifacts/runtime/writer.rs` | Current runtime writer serializes and writes `reasoning_graph.v1.json`. |
| `docs/planning/ADL_FEATURE_LIST.md` | Schedules signed/queryable trace and reasoning/provenance closure later, especially `v0.94`. |
| `docs/milestones/v0.94/features/REASONING_GRAPH_BASELINE_v0.94.md` | Future baseline location for reasoning graph plus signed trace/query completion. |

## Current Contract Boundary

`ReasoningGraphV1` is a reviewable artifact-level representation of explicit reasoning/provenance structure derived from ADL-visible events.

There are two relevant states:

- **Current emitted artifact:** today's Rust path writes `learning/reasoning_graph.v1.json` using `ReasoningGraphArtifact`, `ReasoningGraphRecord`, `ReasoningGraphNode`, `ReasoningGraphEdge`, and `ReasoningGraphSelection`. That artifact includes fields such as `reasoning_graph_version`, `run_id`, `generated_from`, `graph_id`, `dominant_affect_mode`, `ranking_rule`, `selected_path`, `nodes`, and `edges`.
- **Current contract target:** the public C-SDLC reasoning-graph contract below defines the review/governance boundary that follow-on work must implement or map to. It is not a claim that every existing emitted artifact already carries every target field.

It is not:

- private chain-of-thought
- hidden model scratchpad text
- unredacted prompt internals
- a signed/queryable trace guarantee by itself
- a replacement for ACC authority checks
- an upstream delegation contract

## Current Emitted Artifact Compatibility

The existing writer output is the compatibility baseline for `#3690` and `#3691`.

Current emitted graph artifacts are **legacy-compatible inputs** to the new contract when:

- `reasoning_graph_version` and `run_id` are present.
- `generated_from` identifies the artifact sources used to build the graph.
- `selected_path`, `nodes`, and `edges` are present.
- node `rationale` and edge `rationale` are treated as public bounded summaries, not hidden chain-of-thought.
- current node kinds such as `affect` and edge relations such as `updates` and `prioritizes` are either preserved as legacy relation kinds or mapped into the target taxonomy by explicit migration logic.

Follow-on implementation must not reject current `reasoning_graph.v1.json` artifacts merely because they lack target fields like `artifact_ref`, `source_trace_ref`, `redaction_policy_ref`, `evidence_refs`, or `private_reasoning_exposed`. Those fields are target-contract surfaces that must be added, derived, or validated through an explicit compatibility adapter.

## Target Artifact Shape

A public reasoning graph artifact that claims the refreshed C-SDLC contract must preserve these fields or their validated equivalents:

```yaml
schema: reasoning_graph.v1
artifact_ref: repo_or_run_relative_path
source_trace_ref: trace_or_run_ref
reasoning_graph_version: integer
redaction_policy_ref: optional policy ref
graph:
  selected_path:
    node_ids: ordered node ids
    rationale_ref: public evidence/ref, not private reasoning text
  nodes:
    - node_id: stable id
      node_kind: observation | hypothesis | decision | action | evidence | constraint | review | delegation_ref
      summary: public bounded summary
      evidence_refs: list of repo/run-relative refs
      private_reasoning_exposed: false
  edges:
    - from: node id
      to: node id
      edge_kind: supports | contradicts | refines | selects | gates | delegates_to | reviewed_by
      evidence_refs: list of repo/run-relative refs
```

## Required Invariants

- `artifact_ref` and all evidence refs must be repo-relative, run-relative, or explicit external issue/PR URLs when those fields are present or derived.
- `private_reasoning_exposed` must be false for every public artifact that carries the refreshed contract surface.
- A selected path may cite a public rationale ref or public selected-path summary, but must not embed hidden chain-of-thought.
- Graph nodes must be summaries of reviewable evidence, not opaque private model self-report.
- Graph edges must carry meaningful relationship kinds. Legacy `relation` values such as `updates` and `prioritizes` remain acceptable only when explicitly classified as legacy-compatible or mapped to the refreshed taxonomy.
- Delegation edges are references only unless the upstream delegation contract is also present.
- Signed/queryable trace semantics are out of scope until the v0.94 trace closure work lands.

## Relationship To Current ADL Surfaces

| Surface | Relationship |
| --- | --- |
| Trace substrate | Current writer output lives beside `trace_v1.json`; refreshed reasoning graphs should reference current run/artifact identifiers first, then signed/queryable trace IDs only after v0.94 trace work exists. Reasoning graphs are not the trace authority by themselves. |
| ObsMem | Intended integration constraint: ObsMem may store or index reasoning graph refs when public provenance is useful. It must not store private reasoning text through this path. |
| PVF | Intended integration constraint: PVF may treat reasoning graph checks as one proof lane when the lane has explicit input refs, status, and failure semantics. |
| Review provider | Intended integration constraint: a review-provider result may cite reasoning graph refs as evidence, but provider output remains advisory and not authoritative. |
| Runtime v2 | Intended integration constraint: Runtime v2 may emit or consume reasoning graph refs as part of public proof packets, especially when decision provenance matters. |
| ACC | Authority boundary: ACC remains the authority layer. Reasoning graph edges may cite authority evidence, but cannot grant authority. |
| Upstream delegation | Reasoning graphs may include future `delegation_ref` nodes or `delegates_to` edges, but upstream delegation itself requires the separate contract to be defined by follow-on issue `#3689`; `UpstreamDelegationV1` is a planned follow-on contract name, not an implemented contract claim in this issue. |

## Trace Reference Rule For Follow-On Implementation

Until signed/queryable trace work lands, `#3690` should use the strongest current trace reference available in this order:

1. current run/artifact identifiers already present in the artifact, especially `run_id` and `generated_from`
2. repo-relative or run-relative artifact paths such as `trace_v1.json` and `learning/reasoning_graph.v1.json`
3. explicit event identifiers only when the current artifact model already exposes them

`#3690` must not invent signed trace IDs, query IDs, or cryptographic trace authority before the v0.94 trace closure work provides that substrate.

## Follow-On Contract Requirements

The next implementation issue should add or verify:

- Rust-visible validation for the required artifact shape.
- Negative tests rejecting private reasoning leakage.
- Negative tests rejecting absolute host-path refs in public graph artifacts.
- A compatibility adapter or migration rule that classifies current and historical `learning/reasoning_graph.v1.json` examples as legacy-compatible, migrated, or unsupported without accidentally rejecting the current writer's own output.
- A small proof packet linking this contract to the upstream-delegation contract without claiming full v0.94 signed/queryable trace completion.

## Non-Claims

This issue does not claim:

- the historical v0.85 reasoning graph schema is already the final current contract
- all trace archive reasoning graph artifacts satisfy this contract
- signed/queryable trace is complete
- upstream delegation is implemented
- private chain-of-thought can be exposed as a reasoning graph
- reasoning graph evidence can override ACC, review, or operator authority

## Validation Expectations

A reviewer should be able to verify this packet with focused checks:

- issue `#876` exists and is closed
- PR `#962` exists and is merged
- `adl/tools/demo_reasoning_graph_affect.sh` exists
- `adl/tools/demo_affect_godel_vertical_slice.sh` exists
- `adl/src/artifacts.rs` has the reasoning graph path accessor
- `adl/src/cli/run_artifacts_types.rs` has the reasoning graph artifact types
- `adl/src/cli/run_artifacts/runtime/writer.rs` writes `reasoning_graph.v1.json`
- planning docs still route signed/queryable trace and reasoning/provenance closure to later milestone work

## Disposition

`#3688` should close only when this current contract is reviewed and accepted as the design input for:

- `#3689` upstream delegation contract
- `#3690` trace-record implementation
- `#3691` proof/demo flow
