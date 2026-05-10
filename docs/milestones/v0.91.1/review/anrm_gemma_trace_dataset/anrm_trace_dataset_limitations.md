# ANRM/Gemma Trace Dataset Limitations

## Status

Bounded fixture-mode placement package. This artifact is reviewable, but it is not a training or benchmark result.

## What This Package Proves

- The ANRM/Gemma lane has one deterministic trace-to-dataset extractor surface.
- The fixture cases, expected decisions, and prompt modes can be serialized into a reviewable dataset.
- Later evaluator or training work has one clean source packet to consume.

## What This Package Does Not Prove

- model training success
- benchmark superiority
- Gemma-family promotion to a runtime dependency
- ANRM placement approval beyond a bounded evidence-prep lane

## Review Notes

- The source packet is the CSM shepherd event classification family from `v0.90.1`.
- `raw_gemma` and `scaffolded_gemma` remain subject lanes only; they are not performance claims.
- Training eligibility remains `not_approved` for every generated dataset record in this slice.

## Next Truthful Follow-on

- Add evaluator-side scoring consumption for this fixture dataset.
- Expand trap cases only after the extractor and dataset surface are accepted.
- Keep any live model or benchmark work in a separate later issue.
