# C-SDLC v2 operator playbook

Use the independent Rust binaries and typed skills under `csdlc-v2/`. Keep
the primary checkout on clean `main`, bind a tracked issue with `csdlc-bind`,
and make implementation changes only in the bound worktree. Edit cards through
`csdlc-edit`/Markdown ASTs, validate with `csdlc-validate`, review with
`csdlc-review`, publish only with current review evidence, and close out with
`csdlc-closeout`.

Historical v1 commands are retained in
`docs/legacy/CODEX_PLAYBOOK_V1.md` for migration evidence only.
