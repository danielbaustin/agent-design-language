# C-SDLC v2 playbook for v1-origin PR tails

This playbook applies when a pull request was opened before the v1 command
surface was sunset. The PR remains valid; only its lifecycle authority changes.

## Authority

Use the typed v2 binaries for all new observations and mutations:

| Tail state | Authoritative operation | Result |
| --- | --- | --- |
| Need current issue/PR facts | `csdlc-pr-state` or `csdlc-github-pr`, then `csdlc-shepherd` | Read-only observation/classification |
| Branch is behind `main` | Rebase or merge in the bound worktree | New substantive revision; review must be reassigned |
| Checks are pending | Shepherd state `waiting` | Wait; do not publish or close |
| Checks failed | Shepherd state `repair_required` or `retryable` | Repair or retry, then revalidate |
| Review is required | Typed v2 review assignment/recording | Preserve exact reviewed revision |
| PR is ready | `csdlc-publish`, then `csdlc-finish` after terminal GitHub truth | Publish/readiness/terminal evidence |

Do not invoke `pr.sh`, `workflow-conductor`, v1 prompt-template wrappers, or
`csdlc-import` for a new lifecycle mutation. Historical records may mention
those commands; those mentions are evidence, not executable instructions.

## Stale-base repair

1. Keep the issue claim and bound worktree.
2. Fetch the intended base and rebase or merge it in the issue worktree.
3. Run the smallest proving validation for the changed paths.
4. Treat the resulting commit as a new substantive revision. A prior review is
   stale unless a valid typed non-substantive proof covers only metadata paths.
5. Reassign and record review before updating the PR.
6. Re-run shepherd classification and wait for required checks.

Never solve a stale base by editing `.csdlc` state directly or publishing a
dirty tree.

## Direct GitHub fallback

The current live provider observation path is the typed `csdlc-pr-state` or
`csdlc-github-pr` route. A direct GitHub query is permitted only when the typed
route cannot reach the provider or when diagnosing it. Use the shared approved token resolver, never
print token contents, and record the sanitized result (PR number, state, draft
flag, base/head refs and SHA, review decision, checks, and merge state) in the
issue's evidence packet. The fallback does not grant merge or close authority.

## Merge and closeout

Only a green, non-draft PR with current review truth may proceed to readiness.
Record the exact head SHA and required-check outcomes. After GitHub reports the
terminal state, use `csdlc-finish` to preserve derived terminal evidence, then
use `csdlc-clean cleanup` only after cleanup eligibility passes. No v2 binary
grants merge authority; merging is an explicit operator/GitHub action, and v2
records the observed result.
