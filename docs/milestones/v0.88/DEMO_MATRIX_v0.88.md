# Demo Matrix - v0.88

## Metadata
- Milestone: `v0.88`
- Version: `v0.88`
- Date: `2026-04-11`
- Owner: `Daniel Austin`

## Current State

`v0.88` now has a real reviewer package.

Primary integrated review command:

```bash
bash adl/tools/demo_v088_review_surface.sh
```

Primary integrated review artifact:
- `artifacts/v088/review_surface/demo_manifest.json`

## Proof Rows

| Demo ID | Focus | Command | Primary artifact | Reviewer claim | Status |
|---|---|---|---|---|---|
| D1 | temporal schema and anchors | `bash adl/tools/demo_v088_temporal_review_surface.sh` | `artifacts/v088/temporal_review_surface/state/temporal_schema_v1.json` | ADL exposes temporal schema and bounded chronosense anchors explicitly enough for direct reviewer inspection | implemented |
| D2 | continuity and identity | `bash adl/tools/demo_v088_temporal_review_surface.sh` | `artifacts/v088/temporal_review_surface/state/continuity_semantics_v1.json` | ADL makes continuity and identity semantics reviewable rather than implicit | implemented |
| D3 | temporal retrieval and commitments | `bash adl/tools/demo_v088_temporal_review_surface.sh` | `artifacts/v088/temporal_review_surface/state/temporal_query_retrieval_v1.json` | ADL exposes retrieval and commitment state through explicit reviewer artifacts | implemented |
| D4 | execution policy and cost | `bash adl/tools/demo_v088_temporal_review_surface.sh` | `artifacts/v088/temporal_review_surface/state/execution_policy_cost_model_v1.json` | ADL shows policy posture, cost, and temporal explanation as one bounded review surface | implemented |
| D5 | PHI-style integration metrics | `bash adl/tools/demo_v088_phi_review_surface.sh` | `artifacts/v088/phi_review_surface/state/phi_integration_metrics_v1.json` | ADL can compare low / medium / high integration profiles without metaphysical overclaim | implemented |
| D6 | instinct declaration and influence | `bash adl/tools/demo_v088_instinct_review_surface.sh` | `artifacts/v088/instinct_review_surface/state/instinct_model_v1.json` | ADL declares instinct explicitly as a bounded runtime input | implemented |
| D7 | bounded agency under instinct | `bash adl/tools/demo_v088_instinct_review_surface.sh` | `artifacts/v088/instinct_review_surface/state/instinct_runtime_surface_v1.json` | ADL can show a deterministic bounded case where instinct changes candidate selection without escaping policy limits | implemented |
| D8 | Paper Sonata flagship demo | `bash adl/tools/demo_v088_paper_sonata.sh` | `artifacts/v088/paper_sonata/demo_manifest.json` | ADL can orchestrate a bounded manuscript workflow with durable role artifacts and truthful runtime evidence | implemented |
| D9 | deep-agents comparative proof | `bash adl/tools/demo_v088_deep_agents_comparative_proof.sh` | `artifacts/v088/deep_agents_comparative_proof/comparative_manifest.json` | ADL turns a visible file-based deep-agent packet into a reviewer-auditable proof surface with provenance and reference mapping | implemented |

## Reviewer Rule

No row is complete until a reviewer can answer all three:
- what command do I run?
- what artifact do I inspect?
- what claim does this row prove?
