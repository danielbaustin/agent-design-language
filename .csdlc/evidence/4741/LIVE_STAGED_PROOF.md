# Issue 4741 exact-head live-attempt evidence

The reviewed liveness implementation was committed at
`2fb6fa86fee8b96457e0c179646a0c5109861e4a`. Focused unit, contract,
selector, syntax, JSON, and diff checks passed, and independent review reported
no actionable implementation findings.

The exact-head live command selected `skipped_fail_closed` before launching
Unity. The repository-installed owner binary exists, but its recorded source
hash does not match the current declared source root:

- binary SHA-256:
  `03696d2f54b64dabcc7b4c445fd90fdcff2807cd17abd5a925800beae49ad5fb`
- installed source hash:
  `c837322454e3973c728335d561a4b0179b57678b7caff09c21d0bd5887b28b16`
- current source hash:
  `0db8351b2a7fa9170494b31818e4e382033937a680594a622dec520994a4ffed`

The wrapper returned exit 2 with
`terminal_outcome=owner_binary_provenance_invalid`. It did not create a staged
project, inspect broad process state, launch Unity, diagnose IL post-processing,
touch MCP alignment, or claim demo readiness.

This is the current #4741 tooling blocker. Per operator instruction, no binary
was built or replaced during this issue.

## Tooling truth captured

The first pre-commit attempt had already been appended to the SOR before
independent review rejected its revision identity. The nonterminal
`csdlc-edit` surface has no typed replace/remove operation for an existing
validation result. The correction therefore uses the supported append-only
route twice:

- an authoritative blocked result points to `live-staged-proof.json`;
- a later blocked result repeats the exact command, purpose, and evidence
  reference of the stale entry so C-SDLC v2 latest-result semantics prevent
  that historical identity from remaining passed.

The audit remains immutable. A future typed nonterminal validation-replacement
operation would make this repair less indirect.
