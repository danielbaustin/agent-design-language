# Artifact Model v1 (v0.7)

Artifact Model v1 defines deterministic, canonical run artifact locations under:

`<repo>/.adl/runs/<run_id>/`

This model is intentionally path-stable (no timestamps/UUIDs in path names by default)
and is centralized in runtime code via `swarm/src/artifacts.rs`.

## Layout

For each run id:

```
.adl/runs/<run_id>/
  run.json
  steps.json
  pause_state.json            # present only for paused runs
  run_summary.json            # reserved for WP #482
  outputs/                    # canonical output subtree
  logs/                       # reserved deterministic log subtree
  learning/
    scores.json               # reserved for WP #483
    suggestions.json          # reserved for WP #484
    overlays/                 # reserved for WP #485
  meta/
    ARTIFACT_MODEL.json       # {"artifact_model_version": 1}
```

Run summary schema reference:
- `docs/milestones/v0.7/RUN_SUMMARY_v1.md`

## Versioning

- Marker file: `meta/ARTIFACT_MODEL.json`
- Field: `artifact_model_version: 1`

This marker is written by default whenever run state artifacts are materialized.

## Determinism rules

- Path generation is centralized and deterministic (`RunArtifactPaths`).
- Canonical run root is stable: `.adl/runs/<run_id>`.
- No wall-clock timestamp or UUID components are added to artifact paths by default.
- Reserved filenames are stable and fixed to support replay/export tooling.

## Backwards compatibility

- Existing run state files remain in place:
  - `run.json`
  - `steps.json`
  - `pause_state.json` (paused only)
- Model v1 is additive:
  - centralized path ownership
  - reserved subtrees/files for learning WPs
  - `meta/ARTIFACT_MODEL.json` marker
