# ANRM And Gemma Placement

## Metadata

- Feature Name: ANRM/Gemma Placement And Trace Dataset
- Milestone Target: `v0.91.1`
- Status: landed
- Planned WP Home: WP-12
- Source Docs: `.adl/docs/TBD/anrm/`
- Proof Modes: dataset, fixtures, review
- Review Artifacts:
  - `docs/milestones/v0.91.1/review/anrm_gemma_trace_dataset/anrm_trace_dataset.json`
  - `docs/milestones/v0.91.1/review/anrm_gemma_trace_dataset/anrm_trace_extractor_spec.json`
  - `docs/milestones/v0.91.1/review/anrm_gemma_trace_dataset/anrm_gemma_placement_package.json`
  - `docs/milestones/v0.91.1/review/anrm_gemma_trace_dataset/anrm_trace_dataset_limitations.md`

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

## Landed Surface

WP-12 lands one bounded fixture-mode placement package rather than live-model
training or benchmark work.

Landed outputs:

- deterministic extractor tool:
  - `adl/tools/build_v0911_anrm_trace_dataset.py`
- focused proof:
  - `adl/tools/test_build_v0911_anrm_trace_dataset.sh`
- tracked review artifacts:
  - `docs/milestones/v0.91.1/review/anrm_gemma_trace_dataset/anrm_trace_dataset.json`
  - `docs/milestones/v0.91.1/review/anrm_gemma_trace_dataset/anrm_trace_extractor_spec.json`
  - `docs/milestones/v0.91.1/review/anrm_gemma_trace_dataset/anrm_gemma_placement_package.json`
  - `docs/milestones/v0.91.1/review/anrm_gemma_trace_dataset/anrm_trace_dataset_limitations.md`

## Non-Claims

- This feature does not prove training success.
- This feature does not prove benchmark superiority.
- This feature does not approve Gemma-family promotion to a runtime dependency.
