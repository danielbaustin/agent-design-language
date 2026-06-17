# Rust Refactor Mini-Sprint Closeout (#3751)

Sprint issue: `#3745`  
Closeout issue: `#3751`  
Captured: 2026-06-17  
Status: ready_for_pr_publication

## Summary

The v0.91.5 Rust refactoring mini-sprint completed its bounded semantic
refactor wave before Sprint 4. The sprint did not try to erase every large Rust
file from the tracker. It reduced review/change-path cost in the scoped
production surfaces, recorded explicit no-op/defer decisions for the large
surfaces that remained, and left Sprint 4 with a cleaner set of known follow-on
hotspots instead of generic "split big files later" debt.

This packet records the final sprint truth:

- the semantic audit in `#3746` refreshed the tracker evidence and reordered
  the wave by risk;
- `#3748`, `#3749`, and `#3747` each landed one bounded semantic refactor with
  merged PR proof;
- `#3750` captured the consolidation/no-op/defer decisions so the sprint does
  not silently repeat `*_parts` churn;
- the current-equivalent tracker surface is the deterministic
  `bash adl/tools/report_large_rust_modules.sh` report, not the missing ignored
  `.adl/reports/manual/rust_module_watch_list.md` path named in the issue body.

## Merged Child Outcomes

| Issue | PR | Outcome | Notes |
| --- | --- | --- | --- |
| `#3746` | `#3911` | audit landed | Refreshed hotspot evidence and set the safe execution order. |
| `#3748` | `#3912` | prompt-template boundary landed | Extracted rendered-card structure/schema behavior from `adl/src/csdlc_prompt_editor.rs` into `adl/src/csdlc_prompt_editor/structure.rs`. |
| `#3749` | `#3915` | run-artifact boundary landed | Extracted run/pause state artifact helpers and pause-path sanitization into `adl/src/cli/run_artifacts_types/state.rs` without changing public artifact semantics. |
| `#3747` | `#3917` | GitHub control-plane boundary landed | Extracted transport/test-support/test boundaries from `adl/src/cli/pr_cmd/github.rs`, but did not fully isolate every GitHub control-plane concern into a clean standalone transport layer. |
| `#3750` | `#3918` | consolidation review landed | Recorded which `*_parts` families should consolidate, defer, or stay as-is. |

## Refreshed Tracker Truth

The issue body names `.adl/reports/manual/rust_module_watch_list.md`, but that
path is operator-local, ignored by Git, and absent from the bound worktree. The
authoritative refreshed tracker for this closeout is therefore the deterministic
report generated from:

```bash
bash adl/tools/report_large_rust_modules.sh
```

### Scoped before/after snapshot

| Surface | Audit snapshot in `#3746` | Post-sprint snapshot | Sprint result |
| --- | ---: | ---: | --- |
| `adl/src/cli/pr_cmd/github.rs` | 4551 (`RATIONALE`) | 1491 (`REVIEW`) | Production control-plane file is materially smaller after the transport/test split, but retry policy, blocking Octocrab execution, issue mutation, and compatibility-path concerns still live in `github.rs`. |
| `adl/src/csdlc_prompt_editor.rs` | 2468 (`RATIONALE`) | 1812 (`RATIONALE`) | Large, but the structure/schema seam is now extracted and review scope is narrower. |
| `adl/src/cli/run_artifacts_types.rs` | 1550 (`RATIONALE`) | 1485 (`REVIEW`) | Now below the rationale threshold after the state-focused split into `state.rs`. |

### Current large-surface interpretation

- `adl/src/cli/pr_cmd/github/tests.rs` is now `1965` lines and sits at
  `RATIONALE`, but this is an intentional extracted proof surface rather than a
  hidden production hot path. It should still be routed as future
  test-structure cleanup before it becomes another hard-to-edit hotspot.
- `adl/src/cli/pr_cmd/finish_support.rs` remains `1952` lines at
  `RATIONALE`. This was explicitly routed out of scope in `#3746` and remains
  the clearest production follow-on after this sprint.
- `adl/src/csdlc_prompt_editor.rs` remains above the rationale threshold, but
  the sprint already consumed the next real internal seam. Do not reopen it
  just to chase line count without a new semantic boundary.
- `adl/src/cli/tests/pr_cmd_inline/basics.rs`,
  `adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs`, and
  `adl/src/cli/tests/pr_cmd_inline/lifecycle/start_ready.rs` remain explicit
  large proof surfaces. They were no-op/defer cases in this wave and should not
  be presented as unfinished production refactor work.

## What Improved

- The sprint replaced a generic "big files are bad" story with merged,
  evidence-backed semantic boundaries.
- `github.rs` is no longer a 4.5k-line mixed transport/test surface.
- The GitHub control-plane surface is smaller and more navigable, even though
  the extracted `transport.rs` boundary is still partial rather than fully
  clean.
- `run_artifacts_types.rs` is now below the rationale threshold while keeping
  artifact contract behavior stable.
- `csdlc_prompt_editor.rs` shed its rendered-card structure/schema machinery
  into a concept-named module rather than anonymous `parts`.
- The follow-on consolidation candidate is now explicit:
  `governed_tools_flagship_demo_parts` from `#3750`.

## What This Sprint Intentionally Did Not Do

- It did not claim the full Rust hotspot list is clean.
- It did not widen into `finish_support.rs`.
- It did not reorganize the large CLI inline proof files just because they are
  big.
- It did not convert success into line-count reduction without proof-lane
  benefits.

## Sprint 4 Handoff

Sprint 4 can proceed without waiting for another Rust refactor pass. The real
remaining Rust refactor risk is now concentrated and named:

1. `adl/src/cli/pr_cmd/finish_support.rs` remains the highest-priority
   production follow-on.
2. `governed_tools_flagship_demo_parts` remains the strongest consolidation
   candidate from the `#3750` review.
3. Large proof-only test files remain visible in the tracker but are not
   blockers for Sprint 4 by themselves.

This means Sprint 4 should consume the landed narrower boundaries as-is and
route future Rust hotspot work through explicit follow-on issues rather than
reopening this mini-sprint implicitly.

No Sprint 4 sequencing doc update was needed for this issue because the
currently tracked planning surfaces already place Sprint 4 behind the queued
review-remediation wave `#3899`:

- `docs/milestones/v0.91.5/SPRINT_v0.91.5.md`
- `docs/milestones/v0.91.5/WBS_v0.91.5.md`
- `docs/milestones/v0.91.5/WP_ISSUE_WAVE_v0.91.5.yaml`

Those surfaces already keep `#3574` as the canonical Sprint 4 umbrella and do
not require a further sequencing correction from this mini-sprint closeout.

## Key Evidence

- `docs/milestones/v0.91.5/RUST_REFACTOR_SEMANTIC_AUDIT_3746.md`
- `docs/milestones/v0.91.5/review/RUST_MODULE_CONSOLIDATION_DECISIONS_3750.md`
- `docs/milestones/v0.91.5/SPRINT_v0.91.5.md`
- `docs/milestones/v0.91.5/WBS_v0.91.5.md`
- `docs/milestones/v0.91.5/WP_ISSUE_WAVE_v0.91.5.yaml`
- ADL-native closeout truth recorded in the paired issue-local `sor.md` cards
  for `#3746`, `#3747`, `#3748`, `#3749`, and `#3750`
- Historical/manual GitHub provenance from `gh pr view`:
  - `gh pr view 3911 --json statusCheckRollup,files,mergedAt,mergeCommit`
  - `gh pr view 3912 --json statusCheckRollup,files,mergedAt,mergeCommit`
  - `gh pr view 3915 --json statusCheckRollup,files,mergedAt,mergeCommit`
  - `gh pr view 3917 --json statusCheckRollup,files,mergedAt,mergeCommit`
  - `gh pr view 3918 --json statusCheckRollup,files,mergedAt,mergeCommit`

## Residual Risks

- The issue body still names the missing ignored manual watch-list path. The
  repo should continue using the deterministic report helper as current tracker
  truth unless a tracked replacement file is introduced intentionally later.
- `finish_support.rs` remains a real production hotspot and should not be
  forgotten now that the mini-sprint is closing.
- `adl/src/cli/pr_cmd/github/transport.rs` is still a partial extraction rather
  than a clean standalone transport layer because `github.rs` continues to own
  retry policy, blocking Octocrab execution, issue mutation, and compatibility
  shims. Routed follow-on: `#3928`.
- The extracted `github/tests.rs` proof surface is large enough to watch and
  should be treated as a future test-structure cleanup candidate, even though
  it is not a production-boundary regression by itself. Routed follow-on:
  `#3929`.

## Validation Performed

```bash
bash adl/tools/report_large_rust_modules.sh
bash adl/tools/report_module_navigability.sh --top 12 --format tsv
rg -n "#3574|Sprint 4|3899" docs/milestones/v0.91.5/SPRINT_v0.91.5.md docs/milestones/v0.91.5/WBS_v0.91.5.md docs/milestones/v0.91.5/WP_ISSUE_WAVE_v0.91.5.yaml
# Historical/manual GitHub provenance only; not the preferred ADL-native control-plane proof path:
gh pr view 3911 --json statusCheckRollup,files,mergedAt,mergeCommit
gh pr view 3912 --json statusCheckRollup,files,mergedAt,mergeCommit
gh pr view 3915 --json statusCheckRollup,files,mergedAt,mergeCommit
gh pr view 3917 --json statusCheckRollup,files,mergedAt,mergeCommit
gh pr view 3918 --json statusCheckRollup,files,mergedAt,mergeCommit
```

Result: passed.

## Closeout Decision

`#3751` is ready for PR publication once the paired SRP and SOR record the same
review and execution truth captured above. After `#3751` merges and closes, the
parent sprint issue `#3745` should close with this packet as its final truth
surface.
