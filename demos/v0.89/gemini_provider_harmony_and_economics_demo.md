# Gemini Provider Harmony And Economics Demo

Canonical command:

```bash
bash adl/tools/demo_v089_gemini_provider_harmony_and_economics.sh
```

## What It Does

This demo extends the earlier bounded Gemini proof by adding an explicit
provider-selection layer.

ADL prepares one bounded packet, records why Gemini is the right participant
for that packet, invokes Gemini through the existing bounded runtime path, and
then emits both the resulting artifact and the provider-fit rationale.

## Why It Matters

The point is not that Gemini beats every other provider.

The point is that ADL can make provider choice legible:

- capability fit
- cost class
- latency class
- reviewability
- bounded-scope match

That gives the repo a calmer, more reviewer-friendly provider-harmony story.

## Primary Proof Surfaces

- `provider_selection/provider_selection_manifest.json`
- `provider_selection/capability_and_cost_reasoning.md`
- `review_artifacts/gemini_artifact.md`
- `runtime/runs/v0-89-gemini-provider-harmony-and-economics/run_summary.json`
- `runtime/runs/v0-89-gemini-provider-harmony-and-economics/logs/trace_v1.json`
