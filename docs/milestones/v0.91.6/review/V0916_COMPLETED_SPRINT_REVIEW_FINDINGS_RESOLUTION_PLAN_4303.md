# v0.91.6 Completed Sprint Review Findings Resolution Plan

Status: `findings_resolution_plan`
Date: 2026-06-20
Owner issue: `#4303`

This plan groups the remaining findings from the retained sprint-review pass
after adding packets for `#4160`, `#4237`, and `#4250`.

## Findings Register

| Finding | Severity | Source review | Resolution route | Status |
| --- | --- | --- | --- | --- |
| `#4163` was closed from issue-comment-only evidence rather than a merged PR. | P2 | `V0916_ACIP_RUNTIME_MINI_SPRINT_REVIEW_4160.md` | Keep caveat in retained review; later ACIP/runtime code audit may open a repair issue only if the schema surface is missing or untested. | recorded_no_immediate_issue |
| `#4252`, `#4254`, and `#4256` were closed as folded issues but have no issue-local fold comments. | P2 | `V0916_COMPLETED_SPRINTS_REVIEW_REMEDIATION_MINI_SPRINT_REVIEW_4250.md` | Add issue comments pointing to `#4251` or `#4255` as replacement owners. | resolved_in_4305 |
| `#4237` closed while its issue body still contains bootstrap `status: "draft"` metadata. | P3 | `V0916_SESSION_GOAL_WORKFLOW_HARDENING_MINI_SPRINT_REVIEW_4237.md` | Leave caveat in retained review unless a future typed issue-body editor pass can normalize closed umbrella frontmatter safely. | retained_caveat_confirmed_in_4305 |
| Completed-sprint matrix still routed work to `#4253` and `#4255` after those issues closed. | P3 | `#4303` matrix review | Update matrix follow-up section to classify `#4253` and `#4255` as closed remediation lanes. | fixed_in_4303 |
| `pr.sh run` / Rust delegate dirties `adl/Cargo.lock` before the clean-root guard when `main` has a stale lockfile relative to `adl/Cargo.toml`. | P1 | `#4303` execution observation | Open or route a tooling bug to make the delegate fail closed before modifying the lockfile, or require `--locked`/fresh-bin delegation for lifecycle checks. | needs_tooling_issue |

## Immediate Actions

1. Keep `#4163` as a retained-review caveat rather than reopening it now.
2. Record that `#4305` added issue-comment hygiene for `#4252`, `#4254`, and
   `#4256`, pointing readers to `#4251` or `#4255` as the replacement owners.
3. Open or route a tooling bug for the Cargo.lock side effect before more
   lifecycle commands run from root `main`.
4. Keep `#4237` issue-body frontmatter as a recorded historical caveat until a
   later typed issue-body editor pass can normalize it safely.

## Non-Goals

- Do not re-execute closed sprint implementation work.
- Do not claim `#4163` was PR-merged.
- Do not edit issue bodies by hand.
- Do not hide the Cargo.lock side effect as operator error.
