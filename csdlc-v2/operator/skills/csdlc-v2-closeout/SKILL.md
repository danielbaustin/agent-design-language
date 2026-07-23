---
name: csdlc-v2-closeout
description: Record terminal truth and safely close out a green integrated issue.
---
Invoke `csdlc-closeout`. Fail closed on incomplete readiness, stale generation,
missing terminal evidence, a conflicting terminal receipt, or unsafe prune
scope.

`closeout` atomically retains the closed issue record and all six typed cards
under the repository's Git common directory before reporting success. This
receipt is the immediate terminal authority after an implementation PR merges.
Use `reconcile-terminal --request <request.json>` from a dedicated closeout
branch to materialize that authority into the tracked `.csdlc/issues/<issue>`
projection. Never patch the primary checkout or card Markdown directly.

`repair-sor-validation --request <request.json>` atomically replaces one exact
terminal SOR validation result under a distinct active repair authority. The
target must remain closed-out and claim-free, the authority must protect the
target issue path, and authority, target, receipt, and old-result identities
must all match. Replacement commands and evidence references must be portable;
the operation regenerates the tracked projection and retained receipt together
or rolls both back.

`prune` requires closed-out canonical state and revalidates the same retained
receipt before removing the issue worktree.
