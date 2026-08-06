# C-SDLC v2 Issue Creation and Binding

This is the active operator path for creating an execution-ready C-SDLC v2
issue and binding it to a worktree. It has no preparation generation, claim,
lease, heartbeat, protected-path ledger, or claim-recovery step.

## 1. Create

Start from a clean primary checkout on the declared base branch. Prepare one
typed creation request containing the issue identity, repository, design and
diagram paths, and complete initial values for all six cards.

```bash
csdlc-issue --root <repo> create --request <create-request.json>
```

Creation validates the request before committing issue state. A successful
result creates the canonical issue record, six cards, design, and diagram. An
invalid or conflicting request exits nonzero and must not leave partial files.
Repeating the identical request is idempotent; conflicting initialization truth
is rejected.

## 2. Validate

```bash
csdlc-validate --root <repo> issue --issue <issue-number>
csdlc-doctor --repo <repo> --issue <issue-number>
```

Both commands must report a passing, execution-ready issue. Validation exits
nonzero for blocked or corrupt state. Readiness requires approved design truth,
repository-relative affected areas, and concrete proving commands.

## 3. Bind

Prepare one typed bind request:

```json
{
  "issue": 1234,
  "base_branch": "main",
  "branch": "codex/1234-short-description",
  "worktree": ".worktrees/adl-wp-1234"
}
```

Then run:

```bash
csdlc-bind --root <repo> --request <bind-request.json>
```

Bind revalidates the issue under the issue and Git-topology locks before Git
mutation. It creates or reuses the exact requested branch/worktree, projects
the canonical issue state atomically, and records that topology in the issue
index. Repeating the same binding returns `created: false`. Contradictory issue,
branch, worktree, readiness, or record truth exits nonzero.

If binding fails after creating a branch or worktree, the command removes only
the topology it created. If projection into an existing worktree fails, files
created by that projection are removed. Repair the reported source conflict and
rerun the same request.

## Authority Boundary

The bound issue record plus live Git branch/worktree topology are the execution
authority. Operators do not create, copy, renew, recover, or release a claim ID.
Session coordination may prevent humans or agents from choosing the same work,
but it is not C-SDLC lifecycle authority and is not an input to either command.
