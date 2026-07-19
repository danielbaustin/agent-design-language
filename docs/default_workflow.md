# Default C-SDLC v2 workflow

C-SDLC work is independent of the sunset ADL wrappers. Use the typed Rust
binaries and operator skills under `csdlc-v2/`:

1. `csdlc-init` creates the issue-local state and six cards.
2. `csdlc-edit` applies typed card edits; `csdlc-validate` validates values,
   Markdown AST structure, and schemas.
3. `csdlc-bind` claims the issue and creates the bound worktree.
4. Implement in that worktree, then run the focused Rust/PVF validation lane.
5. `csdlc-review` records current review truth before `csdlc-publish`.
6. `csdlc-closeout` records integration and terminal evidence.

The former workflow is preserved only as historical evidence in
`docs/legacy/DEFAULT_WORKFLOW_V1.md`. It is not an operational route.
