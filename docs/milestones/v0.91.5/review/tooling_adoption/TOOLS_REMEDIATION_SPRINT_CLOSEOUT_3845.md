# Tools Remediation Sprint Closeout (#3845)

Sprint issue: #3845  
Captured: 2026-06-16  
Status: ready_for_final_umbrella_closeout

## Summary

The #3845 tools remediation sprint completed the remaining active tooling
remediation issues in the #3802-#3835 band and the final sprint-tail issue
#3797. Toolkit simplification and Rust refactoring remained outside this
sprint, as intended.

This closeout packet records live GitHub state observed during final review.
It replaces the earlier local-only readiness note that said #3797 / PR #3858
was still blocking closeout.

## Ordered Child Outcomes

| Issue | Final state | Closeout note |
| --- | --- | --- |
| `#3802` | CLOSED | Parallel prompt-card validation hang diagnostics completed. |
| `#3803` | CLOSED | Prompt-card enum diagnostics aligned with lifecycle states. |
| `#3804` | CLOSED | Lifecycle-card absolute-path leakage diagnostics refined. |
| `#3805` | CLOSED | Octocrab token preflight diagnostics added. |
| `#3809` | CLOSED | Redacted provider artifact refs preserve bounded uniqueness. |
| `#3816` | CLOSED | Governed issue-body repair command exposed. |
| `#3819` | CLOSED | Nested worktree binding from issue worktrees prevented. |
| `#3822` | CLOSED | PR merge false-negative on local branch deletion repaired. |
| `#3823` | CLOSED | Transient GitHub checks API failures classified during PR watch. |
| `#3828` | CLOSED | Default Rust validation narrowed for focused `pr_cmd` changes. |
| `#3829` | CLOSED | `pr_cmd` risky-file coverage guidance made more specific. |
| `#3831` | CLOSED | Local URL opener failure no longer fails or alarms PR creation. |
| `#3834` | CLOSED | PR validation moved off `gh` and onto the ADL platform path. |
| `#3835` | CLOSED | Structured logging added for PR validation waits. |

## Sprint-Tail Resolution

The earlier #3845 readiness note correctly held the sprint open while #3797 /
PR #3858 was still active.

Final live state:

- `#3797` is CLOSED.
- PR `#3858` is MERGED.
- PR `#3858` checks completed with `adl-ci` success, `adl-coverage` success,
  and `adl-slow-proof` skipped.

No #3845 sprint-tail issue remains open.

## Validation Performed

Final review used live GitHub and repo workflow state:

- `bash adl/tools/pr.sh doctor 3845 --mode full --json`
  - returned `PASS`, with no open PR wave blockers for the #3845 queue.
- `gh issue view` over #3797, #3802, #3803, #3804, #3805, #3809, #3816, #3819,
  #3822, #3823, #3828, #3829, #3831, #3834, and #3835
  - confirmed every sprint child and tail issue is CLOSED.
- `gh pr view 3858`
  - confirmed PR #3858 is MERGED and its required checks succeeded or were
    intentionally skipped.

## Residual Boundaries

The sprint does not close or absorb:

- toolkit simplification work
- Rust refactoring work
- v0.91.6 planning-doc updates that consume the completed tool fixes
- future ergonomics around issue-body validation wording

Those are separate work queues. They should not keep #3845 open.

## Closeout Decision

#3845 is ready for final umbrella closeout.

