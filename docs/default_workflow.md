# Default C-SDLC v2 workflow

C-SDLC work is independent of the sunset ADL wrappers. Use the typed Rust
binaries and operator skills under `csdlc-v2/`:

1. `csdlc-issue --root <repo> create --request <json>` creates the issue-local
   state, six cards, design, and diagram from one typed request.
2. `csdlc-edit` applies typed card edits; `csdlc-validate` validates values,
   Markdown AST structure, and schemas.
3. `csdlc-bind --root <repo> --request <json>` validates readiness and binds
   the issue to the requested Git branch and worktree. Git topology is the
   ownership authority; no claim ID, lease, heartbeat, or protected-path ledger
   is created.
4. Implement in that worktree, then run the focused Rust/PVF validation lane.
5. `csdlc-review` records current review truth before `csdlc-publish`.
6. GitHub issue operations use `csdlc-github-issue`; PR observation uses
   `csdlc-github-pr` or `csdlc-pr-state`.
7. `csdlc-finish` validates the exact reviewed green head, merges it when
   needed, and derives terminal authority from live GitHub state. It is
   idempotent and never creates a second closeout PR or rewrites tracked cards
   after merge.

There is no separate closeout writer or terminal-reconciliation command. Safe
worktree cleanup is a separate operation and is never a side effect of finish.

See
`docs/tooling/C_SDLC_V2_ISSUE_CREATION_AND_BINDING_RUNBOOK.md` for the bounded
creation and binding contract.

Use `csdlc-clean cleanup` with a typed request to classify or non-forcibly
remove one exact registered issue worktree. Dirty, missing, relocated, primary,
or identity-drifted worktrees are reported without deletion. Use
`compatibility-index` and `validate-census` for read-only legacy terminal
inspection; retained receipts are optional evidence and are not delivery
authority.

Cross-session ownership and waiting-state semantics remain documented in
`docs/tooling/ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md`.
Use `issue-watcher` for healthy waiting states and through `pr-janitor` only when
an actionable PR-tail blocker appears.

The former workflow is preserved only as historical evidence in
`docs/legacy/DEFAULT_WORKFLOW_V1.md`. It is not an operational route.
