# Paper Sonata Expansion Demo

This `v0.90.2` demo expands Paper Sonata with a bounded manuscript,
review, and revision proof layer.

Canonical command:

```bash
bash adl/tools/demo_v0902_paper_sonata_expansion.sh
```

Validation command:

```bash
bash adl/tools/test_demo_v0902_paper_sonata_expansion.sh
```

## What Changed

- adds a copied `source_packet/` with a source manifest
- adds explicit role outputs for conductor, scholar, and analyst work
- adds a generated manuscript draft
- adds editor review notes
- adds machine-readable revision requests
- adds a revised manuscript that addresses the review requests
- adds a no-submission publication gate

## What This Proves

The demo proves a stronger public-facing Paper Sonata review surface: a reviewer
can trace source material into generated draft text, review feedback, revision
requests, revision output, and a blocked publication gate.

This is new proof on top of the delivered `v0.88` and `v0.89` Paper Sonata
baseline. It does not reopen or replace the baseline workflow.

## Boundaries

This demo does not claim:

- publication readiness
- live-web literature coverage
- journal-ready citation quality
- autonomous scientific discovery
- external posting or submission

The manuscript remains a bounded internal demo artifact unless a later issue
explicitly authorizes publication work.

## Generated Artifacts

By default, the command writes:

```text
artifacts/v0902/paper_sonata_expansion/
```

Key reviewer paths:

- `source_packet/source_manifest.json`
- `role_outputs/conductor_plan.json`
- `role_outputs/scholar_literature_review.md`
- `role_outputs/analyst_results_summary.md`
- `manuscript/draft.md`
- `review/editor_review_notes.md`
- `review/revision_requests.json`
- `revision/revised_manuscript.md`
- `publication_gate/no_submission.md`
- `run_manifest.json`

## Review Order

1. Inspect the source manifest and copied source packet.
2. Inspect the conductor plan and role outputs.
3. Read the draft manuscript.
4. Read the editor review notes and revision requests.
5. Compare the revised manuscript against the review requests.
6. Confirm the publication gate blocks external posting and submission.
7. Run the validation command.
