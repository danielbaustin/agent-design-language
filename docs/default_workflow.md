# Default C-SDLC v2 workflow

C-SDLC work is independent of the sunset ADL wrappers. Use the typed Rust
binaries and operator skills under `csdlc-v2/`:

1. `csdlc-issue create` records draft source intent without a claim.
2. `csdlc-prepare sync` creates a complete immutable generation and
   `csdlc-prepare seal` publishes digest-pinned execution readiness. The
   convenience `csdlc-prepare run` performs both while preserving a synced
   generation when seal fails.
3. `csdlc-bind run` derives internal claim, branch, worktree, and protected
   paths from the issue, sealed receipt, and governed session identity.
4. `csdlc-edit` applies typed card edits; `csdlc-validate` validates values,
   Markdown AST structure, and schemas.
5. Implement in that worktree, then run the focused Rust/PVF validation lane.
6. `csdlc-review` records current review truth before `csdlc-publish`.
7. GitHub issue operations use `csdlc-github-issue`; PR observation uses
   `csdlc-github-pr` or `csdlc-pr-state`.
8. `csdlc-finish` validates the exact reviewed green head, merges it when
   needed, and derives terminal authority from live GitHub state. It is
   idempotent and never creates a second closeout PR or rewrites tracked cards
   after merge.

The coupled `csdlc-init` command is deleted. New binding uses only
`csdlc-bind run` and must not require the operator to create or copy a claim
ID. See
`docs/tooling/C_SDLC_V2_ISSUE_PREPARATION_AND_BINDING_RUNBOOK.md`.

There is no separate closeout writer or terminal-reconciliation command. Safe
worktree cleanup is a separate operation and is never a side effect of finish.

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
