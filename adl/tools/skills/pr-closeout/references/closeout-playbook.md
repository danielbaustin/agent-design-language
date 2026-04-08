# PR Closeout Playbook

Use this skill only after the PR outcome is known.

Check in this order:
1. PR merged, intentionally closed, or not required for the final disposition
2. issue closed
3. supersession, duplicate, or deferral references recorded when relevant
4. final STP/SIP/SOR truth
5. root/worktree card reconciliation
6. no required artifacts left only in the worktree
7. safe worktree prune

Safe actions:
- normalize final card truth
- sync final output-card truth back to the root bundle
- record supersession, duplicate, or no-PR disposition links in the closeout record
- prune the issue worktree after verification
- optionally delete the local branch only when policy explicitly allows it

Unsafe actions:
- merging or closing the PR from this skill
- cleaning unrelated worktrees
- deleting local branch state without explicit policy
- inventing final validation or integration facts
