# Reviewer Output Provenance

This document defines the first bounded provenance-verification surface for deterministic review output artifacts.

## Purpose

WP-08 needs a concrete evidence/provenance-linked output surface in the repo.

This slice focuses on deterministic review outputs because they already:

- carry explicit evidence pointers
- use constrained enums and ordering rules
- represent a serious claim about what was reviewed and why

## Verification Entry Point

Use:

```bash
adl tooling verify-review-output-provenance \
  --review docs/tooling/examples/reviewer-provenance/good_review_output_661.yaml
```

## What It Verifies

- `review_target.input_card_path` and `review_target.output_card_path` are repo-relative and exist
- finding `evidence` pointers use supported pointer classes
- evidence pointers are ordered canonically
- path/artifact pointers are repo-relative
- validation command entries do not leak absolute host paths
- `absolute_host_paths_present: false` is not contradicted by the artifact text

## Deterministic Fixture

Use:

```bash
bash adl/tools/test_review_output_provenance.sh
```

The fixture compares:

- a valid provenance-linked review output artifact
- an intentionally bad artifact that should be refused

## Scope Notes

This does not solve the full long-term provenance system for ADL.

It is the first bounded verification path proving that at least one evidence-linked output surface can be checked deterministically inside the repo.
