# C-SDLC v2 agent contract

- This workspace is clean-room and independent of ADL Runtime and incumbent C-SDLC implementation crates, schemas, tests, and fixtures.
- Use only the typed Rust owners and the eleven thin contracts under `operator/skills/` for v2 lifecycle work.
- Cards are generated projections. Never edit Markdown/state directly; use `csdlc-edit` and markdown.rs AST validation.
- Read current authority only from `operator/generation-selector.json`. Gate 10C cutover is complete and Gate 10D2 records exact parity approval and final v1 sunset; historical coexistence and rollback proofs remain immutable evidence.
- Route through `csdlc-install resolve`; it consumes the tracked selector as the sole default/override authority.
- Install only into `.adl/bin/csdlc-v2/`; never target shared `.adl/bin/`. `csdlc-install verify` is fail closed on missing, symlinked, or non-executable binaries, invalid selector state, or provenance failures; the reviewed inventory explicitly records `v1_sunset:true`.
- GitHub issue actions route through `csdlc-github-issue`; PR observation routes through `csdlc-github-pr` or `csdlc-pr-state`; `csdlc-github` is compatibility only; `csdlc-finish` is the sole exact-head merge and derived-terminal authority.
- Default switching is owned only by `csdlc-cutover`; v1 deletion is never authorized by that command.
