# v0.91.5 First Internal Review Findings Register

## Metadata

- Milestone: `v0.91.5`
- Source review issue: `#3576`
- Register date: `2026-06-16`
- Register status: `routed_to_remediation_queue`
- Canonical remediation umbrella: `#3899`

## Summary

- The expected retained local first-internal-review findings register was
  missing during `#3899` readiness repair.
- This reconstructed register preserves routing truth from the live remediation
  issue set `#3891` through `#3898` without inventing missing severity or
  approval fields.
- `#3892` already has draft PR `#3900` in flight and should keep that state
  during execution.
- Use this register as the retained review-to-remediation bridge for the queued
  first internal-review mini-sprint.

## Reconstruction Note

This packet is reconstructed from the routed remediation issues created from the
first v0.91.5 internal review:

- `#3891` through `#3898`
- queue umbrella `#3899`

The available retained local packet set did not preserve severity rankings in a
standalone findings register at the expected path, so this register records
`severity: not_retained` rather than guessing.

## Findings

| Routed issue | Area | Severity | Finding summary | Current route |
| --- | --- | --- | --- | --- |
| `#3891` | validation | `not_retained` | Treat merged green PRs as successful in `pr validation` instead of misclassifying them as cancelled. | Execute as the first validation-truth remediation slice under `#3899`. |
| `#3892` | proof | `not_retained` | Extend small-binary delegation proof coverage to `finish`, `validation`, and `closeout`. | Preserve in-flight draft PR `#3900`; continue under `#3899`. |
| `#3893` | lifecycle | `not_retained` | Repair toolkit sprint closeout and card truth so retained sprint/card state matches GitHub closeout truth. | Execute after `#3891` / `#3892` under `#3899`. |
| `#3894` | docs | `not_retained` | Refresh proof coverage, quality-gate, handoff, and WP-tail docs so review-facing planning truth matches the current bridge state. | Parallel-safe docs remediation group under `#3899`. |
| `#3895` | evidence | `not_retained` | Redact private LAN endpoint details from tracked public proof packets without weakening evidence meaning. | Parallel-safe evidence hygiene group under `#3899`. |
| `#3896` | observability | `not_retained` | Preserve quiet stderr when compatibility log sink setup fails. | Parallel-safe tooling remediation group under `#3899`. |
| `#3897` | GitHub linkage | `not_retained` | Support standard GitHub autoclose syntax in fallback PR close-link detection. | Parallel-safe tooling remediation group under `#3899`. |
| `#3898` | markdown-ast | `not_retained` | Allow intentional section-local removals in `replace-section` while preserving unrelated-document safety. | Parallel-safe tooling remediation group under `#3899`. |

## Supplemental Operator-Reported Findings

These additional tooling/adapter findings were observed during live execution of
the remediation flow and should be treated as part of the same bounded
mini-sprint rather than as separate unscheduled residue.

| Route band | Area | Severity | Finding summary | Current route |
| --- | --- | --- | --- | --- |
| `#3896-#3898` tooling tranche | worktree bootstrap | `operator_reported` | New issue worktrees can miss required prompt-template scaffolding such as `docs/templates/prompts`, forcing manual bridge repair before execution. | Route into the tooling remediation tranche under `#3899`; likely owner is the worktree/bootstrap/control-plane surface rather than docs-only cleanup. |
| `#3896-#3898` tooling tranche | worktree bootstrap | `operator_reported` | New issue worktrees can miss repo-local `adl/tools` wrappers such as `adl/tools/pr.sh` and `adl/tools/validate_structured_prompt.sh`, forcing manual copy/bridge repair. | Route into the tooling remediation tranche under `#3899`; fix together with prompt-template bridging so bound worktrees preserve expected local execution surfaces. |
| `#3896-#3898` tooling tranche | execution assumptions | `operator_reported` | `pr.sh run` can bind a worktree that lacks repo-local helper-path assumptions required by the workflow, so binding succeeds but execution fails until hand-patched. | Route into the tooling remediation tranche under `#3899`; treat as a bootstrap/adapter correctness gap, not as operator error. |
| `#3896-#3898` tooling tranche | GitHub auth handoff | `operator_reported` | `adl/tools/pr.sh issue list/view/create` may require explicit `GITHUB_TOKEN` env wiring even when local GitHub auth already exists. | Route into the tooling remediation tranche under `#3899`; fix auth inheritance/handoff rather than teaching manual wrappers as normal operator practice. |
| `#3896-#3898` tooling tranche | issue-body validation UX | `operator_reported` | Create/run flow can fail on strict required issue-body sections such as `Issue-Graph Notes` without surfacing the canonical missing section early enough for smooth repair. | Route into the tooling remediation tranche under `#3899`; preserve strict validation but improve early missing-section/operator guidance. |
| `#3896-#3898` tooling tranche | stale baseline detection | `operator_reported` | `run` can bind a new issue worktree onto a baseline missing prerequisite in-flight issue state without surfacing an early warning. | Route into the tooling remediation tranche under `#3899`; add earlier stale-baseline/prerequisite-output warning rather than silently binding to an incomplete base. |
| `#3896-#3898` tooling tranche | repeatability | `operator_reported` | Current workflow still depends on manual knowledge of local bridge repairs for prompt-template and `adl/tools` surfaces. | Route into the tooling remediation tranche under `#3899`; encode the repair knowledge in the adapter/bootstrap path instead of relying on session memory. |
| `#3896-#3898` tooling tranche | finish validation routing | `operator_reported` | Repo-native `pr finish` does not classify `adl/src/cli/observability.rs` or `adl/src/cli/tooling_cmd/markdown_ast_edit.rs` plus its focused test surface into docs-only, small-binary focused, or larger-binary focused finish-validation lanes, which blocks normal publication for the quiet-stderr remediation even after focused proof passes. | Route into the tooling remediation tranche under `#3899`; fix finish-path classification truth instead of bypassing the workflow silently. |

## Queue Linkage

Ordered execution route retained by the queue umbrella:

1. `#3891` and `#3892`
2. `#3893`
3. `#3894` and `#3895`
4. `#3896`, `#3897`, and `#3898`
5. Close `#3899` only after child issues are closed or explicitly rerouted

## Residual Risks

- The original standalone local review register was absent at the expected
  retained path; this packet restores routing truth but does not reconstruct
  missing severity rankings.
- External review, final v0.92 preflight, next-milestone planning, and release
  ceremony remain downstream work and must not be inferred from this packet.
