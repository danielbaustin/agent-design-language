# Integrated C-SDLC Timing Proof For #3716

Date: 2026-06-14
Milestone: v0.91.5
Sprint: #3717
Proof issue: #3716
Measured issue: #3715
Measured PR: #3731
Status: source-backed timing proof for one real C-SDLC issue run

## Summary

This proof uses #3715 as the measured end-to-end run for the checklist sprint.
It was not a toy path: #3715 implemented the AST-backed Markdown editing
substrate, opened PR #3731, passed GitHub CI, merged, and completed closeout.

The run proves that the refactored command surface, octocrab-backed GitHub
transport, prompt-template card surfaces, Markdown AST editing policy, review,
publication, and closeout can operate together on one real issue.

The run also shows remaining friction. In particular, merge-time validation still
reran broad local proof after GitHub checks had already passed, and the octocrab
merge operation failed against a draft PR with a generic GitHub error. The PR was
then marked ready and merged with an explicit `gh` fallback. That fallback is
recorded here rather than hidden.

## Measured Issue Identity

| Field | Value |
| --- | --- |
| Issue | #3715 |
| Issue title | `[v0.91.5][tools][markdown] Implement AST-backed Markdown editing substrate` |
| Issue URL | `https://github.com/danielbaustin/agent-design-language/issues/3715` |
| Branch | `codex/3715-v0-91-5-tools-markdown-implement-ast-backed-markdown-editing-substrate` |
| PR | #3731 |
| PR URL | `https://github.com/danielbaustin/agent-design-language/pull/3731` |
| Base branch | `main` |
| Merge commit | `d85b08a72802a1f267406b7ca1d44d1bc7fffe16` |
| Implementation commit | `577ead1254dce402a2ffd649ea9539df9345126f` |
| Formatting fix commit | `2473df4eae8a4e000a076745ececc170e96460fe` |

## Timing

| Interval | Start | End | Elapsed |
| --- | --- | --- | --- |
| Issue opened to PR opened | 2026-06-15T01:40:52Z | 2026-06-15T02:55:50Z | 1h 14m 58s |
| PR opened to PR merged | 2026-06-15T02:55:50Z | 2026-06-15T03:14:20Z | 18m 30s |
| Issue opened to issue closed | 2026-06-15T01:40:52Z | 2026-06-15T03:14:21Z | 1h 33m 29s |

These timings include implementation, local focused validation, subagent review,
review finding repair, PR publication, CI, a formatting-failure repair, broad
local merge validation, fallback merge handling, and closeout.

## Command Path Evidence

| Surface | Evidence |
| --- | --- |
| Issue binding | `bash adl/tools/pr.sh run 3715` created the issue worktree and branch. |
| GitHub transport | The workflow emitted `github_octocrab` events for issue and PR lookups/edits. |
| Prompt cards | #3715 cards existed under the canonical `SIP -> STP -> SPP -> SRP -> SOR` lifecycle and validated before finish. |
| AST editing | #3715 added `adl tooling markdown-ast-edit replace-section` and focused Rust tests for the new substrate. |
| Review | Bounded subagent review found lifecycle-card output bypass risk, ungoverned escape hatch risk, overbroad proof claims, and fixture-coverage overclaim. All were fixed before PR publication. |
| PR publication | `bash adl/tools/pr.sh finish 3715 ...` created PR #3731 and committed the implementation branch. |
| CI repair | GitHub CI caught `cargo fmt --all -- --check`; the branch was repaired with commit `2473df4eae8a4e000a076745ececc170e96460fe`. |
| Merge validation | `bash adl/tools/pr.sh finish 3715 --merge ...` ran broad local proof before attempting merge. |
| Closeout | `bash adl/tools/pr.sh closeout 3715` pruned the issue worktree after merge and card validation. |

## Source Pointers

This proof is based on the following inspectable sources:

- Issue and PR state: `https://github.com/danielbaustin/agent-design-language/issues/3715`
  and `https://github.com/danielbaustin/agent-design-language/pull/3731`.
- CI proof: GitHub Actions check runs attached to PR #3731:
  `adl-ci` at
  `https://github.com/danielbaustin/agent-design-language/actions/runs/27521376249/job/81339830142`,
  `adl-coverage` at
  `https://github.com/danielbaustin/agent-design-language/actions/runs/27521376249/job/81339830220`,
  and `adl-slow-proof` at
  `https://github.com/danielbaustin/agent-design-language/actions/runs/27521376249/job/81339830514`.
- Merge commit: `d85b08a72802a1f267406b7ca1d44d1bc7fffe16` on `main`.
- Issue-local lifecycle records for #3715:
  `.adl/v0.91.5/tasks/issue-3715__v0-91-5-tools-markdown-implement-ast-backed-markdown-editing-substrate/`.
- Review evidence: #3715 pre-PR subagent review reported lifecycle-card output
  bypass risk, ungoverned card-path escape hatch risk, overbroad proof claims,
  and fixture-coverage overclaim; the implementation was revised before PR
  publication.
- Local command transcript evidence was observed during #3715 execution and is
  summarized here; it is not claimed as a durable replay log.

## Validation Evidence

| Check | Result | Notes |
| --- | --- | --- |
| Focused Rust proof | PASS | `cargo test --manifest-path adl/Cargo.toml markdown_ast_edit -- --nocapture` passed 5 focused tests. |
| Diff whitespace | PASS | `git diff --check` passed. |
| Coverage-impact preflight | PASS | Broader `tooling_cmd` coverage summary passed coverage-impact preflight for touched Rust command files. |
| GitHub `adl-ci` | PASS | Completed successfully at 2026-06-15T03:06:19Z. |
| GitHub `adl-coverage` | PASS | Completed successfully at 2026-06-15T03:03:35Z. |
| GitHub `adl-slow-proof` | SKIPPED | Expected skipped lane for this PR. |
| Merge-time broad local proof | PASS | `cargo check`, full nextest run, and doc tests passed before merge attempt. Full nextest reported `2852 passed, 3 skipped` in 113.841s. |

## Automation / Delegation / Manual Split

| Step | Classification | Notes |
| --- | --- | --- |
| Issue worktree binding | Automated | `pr.sh run` delegated to the Rust `adl` command and octocrab issue lookups. |
| Card bootstrap/validation | Automated | Structured prompt validators ran during lifecycle commands. |
| Implementation | Manual/Codex-authored | Code and docs were produced inside the issue worktree. |
| Focused test selection | Manual/Codex-authored | Focused Rust, diff, and coverage-impact proof were selected based on touched surface. |
| Subagent review | Delegated | Subagent review produced actionable findings before publication. |
| Review finding repair | Manual/Codex-authored | Findings were repaired before PR publication. |
| PR creation/update | Automated with wrapper delegate | `pr.sh finish` used the Rust path and emitted octocrab events, but still exposed retired helper friction. |
| CI repair | Manual/Codex-authored | Formatting failure was repaired and pushed. |
| Merge | Automated attempt plus fallback | Octocrab merge operation failed while PR was draft; explicit `gh` fallback marked ready and merged. |
| Closeout | Automated | `pr.sh closeout` validated cards and pruned the worktree. |

## Fallback Usage

Fallbacks were used and are part of the proof result:

- `pr.sh finish` still reported retired `attach_post_merge_closeout.sh` behavior during publication.
- Octocrab merge failed with a generic GitHub error while the PR was still draft.
- `gh pr ready` and `gh pr merge --repo ...` were used explicitly to complete the merge.
- Running `gh pr merge` inside the issue worktree failed because local `main` was already checked out in another worktree; rerunning from outside the repo context succeeded and reported the PR already merged.

These are not blockers to the measured issue outcome, but they are evidence that
follow-up tooling work remains necessary.

## What This Proves

- A real issue can move from issue start to PR open and closeout while using the
  integrated C-SDLC surfaces developed in this mini-sprint.
- Octocrab-backed GitHub operations are active on the covered issue and PR paths.
- Prompt-template lifecycle cards and structured validators remain in the loop.
- Markdown AST editing is now a real command substrate with focused tests and
  lifecycle-card guardrails.
- Subagent review found meaningful defects before PR publication and improved the
  result.

## What This Does Not Prove

- It does not prove the workflow is fully fast yet.
- It does not prove `gh` has been eliminated from every operational path.
- It does not prove merge-time validation reuse is solved.
- It does not prove release/watcher paths are octocrab-native.
- It does not prove card closeout can be fully automated without editor/validator
  friction in every case.

## Findings From The Proof

- `P1` Merge-time validation still duplicates proof already available from GitHub
  CI in common cases. The measured run reran broad local proof after GitHub
  checks were green. This directly motivates PVF equivalence/reuse work.
- `P1` The octocrab merge path should fail with a clear draft-state error or mark
  the PR ready before merge when policy allows it. A generic GitHub error forced
  manual diagnosis.
- `P2` `pr.sh finish` still exposes retired helper friction. This is survivable,
  but it weakens the “single clean command path” story.
- `P2` The workflow can still need explicit fallback to `gh` for edge cases,
  which should remain logged until replaced.

## Checklist Disposition

The cross-system proof checklist is satisfied for one real issue run, with the
fallbacks and residual risks above. The result is useful evidence, not a claim
that the workflow is finished.
