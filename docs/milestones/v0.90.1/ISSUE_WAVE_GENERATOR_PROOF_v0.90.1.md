# Issue-Wave Generator Proof - v0.90.1

## Purpose

WP-02 aligns the reusable issue-wave generator with the current v0.90.1
milestone package shape. The proof target is mechanical: the generator must read
the tracked WBS and sprint docs without hand repair and emit the queue, outcome,
dependency, and sprint metadata needed by later issue-wave creation.

## Source Inputs

- `docs/milestones/v0.90.1/WBS_v0.90.1.md`
- `docs/milestones/v0.90.1/SPRINT_v0.90.1.md`
- `docs/milestones/v0.90.1/WP_ISSUE_WAVE_v0.90.1.yaml`
- `docs/tooling/WP_ISSUE_WAVE_GENERATION.md`

## Proof Command

```bash
cargo run --manifest-path adl/Cargo.toml -- tooling generate-wp-issue-wave --version v0.90.1
```

Expected result: command exits successfully and emits `schema:
adl.wp_issue_wave.v1` with entries for WP-02 through WP-20 plus the
supplemental WP-15A third-party review slot.

## Alignment Checks

The v0.90.1 package intentionally differs from the older generator fixture:

- The WBS table lives under `## Work Package Shape`.
- The WBS table records issue numbers in the second column.
- The sprint plan uses sprint sections instead of a `## Sprint Overview` table.
- WP-02 through WP-04 use fine-grained queues before Runtime v2 coding starts.
- WP-15A uses an alphanumeric WP id so the parser preserves supplemental review
  work without confusing it with WP-15 or WP-16.

The generator now accepts those shapes and preserves these review-critical
values:

| WP | Expected queue | Expected outcome | Expected sprint |
| --- | --- | --- | --- |
| WP-02 | tools | docs | Sprint 1 |
| WP-03 | tools | code | Sprint 1 |
| WP-04 | docs | docs | Sprint 1 |
| WP-12 | demo | demo | Sprint 3 |
| WP-15A | review | review | Sprint 4 |
| WP-17 | release | docs | Release Tail |

## Regression Coverage

The focused regression is:

```bash
cargo test --manifest-path adl/Cargo.toml cli::tooling_cmd::wp_issue_wave::tests -- --nocapture
```

It covers:

- legacy v0.88 `## Sprint Overview` and six-column WBS generation
- current v0.90.1 `## Work Package Shape` generation
- sprint-section parsing for Sprint 1 through Release Tail
- alphanumeric supplemental WP ids such as WP-15A
- queue and outcome inference for tools, docs, demo, release, and runtime rows
- deterministic WP range expansion

## Truth Boundary

This proof does not create GitHub issues, mutate tracker state, or replace the
opened v0.90.1 issue map. `WP_ISSUE_WAVE_v0.90.1.yaml` remains the issue-number
source of truth after WP-01 opened issues #2141 through #2160 and WP-15A was
added as #2215.
