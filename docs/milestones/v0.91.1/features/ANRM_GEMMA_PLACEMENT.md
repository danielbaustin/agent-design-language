# ANRM And Gemma Placement

## Metadata

- Feature Name: ANRM/Gemma Placement And Trace Dataset
- Milestone Target: `v0.91.1`
- Status: planned
- Planned WP Home: WP-11
- Source Docs: `.adl/docs/TBD/anrm/`
- Proof Modes: dataset, fixtures, review

## Purpose

Place ANRM/Gemma work as a bounded local-model evidence lane: trace extraction,
dataset mapping, evaluator gates, and placement decisions before any training
or capability claim.

## Scope

In scope:

- ANRM/Gemma placement package.
- Trace extractor spec.
- Trace-to-dataset mapping.
- Fixture dataset and limitations report.

Out of scope:

- Training success claims.
- House-model selection without evaluator evidence.
- Unreviewed local-model adaptation.

## Acceptance Criteria

- Dataset generation is deterministic in fixture mode.
- Model claims are separated from evidence.
- Future training/evaluator work has a clean source packet.
