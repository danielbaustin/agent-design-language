# WP-14A Platform Acceptance

Issue #5384 accepts the integrated v0.91.8 platform at baseline
`11151e0beab02b1667f6505b7f8992bfd47d2f8f`.

## Accepted Inputs

| Product | Issue | PR head | Accepted merge |
| --- | ---: | --- | --- |
| C-SDLC v2 | #5358 | `e048230245b1ad101c8056678123a2747faa4b60` | `fc75f4fc697262f89f99461679a406be0b4b3775` |
| Runtime v3 | #5361 | `f7fc71421f4bcf70039b910c9b88b538bb111400` | `f7258b07e9da414bfee518f0c89a76071bc03ee8` |
| ADL v2 soak and rollback | #5344 | `141dfa20ccc3753060687259ad933397331df9c7` | `d4825d4be9ed14ed6060dd33cbdafe5eaa5efcd2` |
| ADL v2 reversible default | #5343 | `e4bbc988cad682cbb2ff8d24085e1a99bccec1ce` | `e1b6a34e4763a79d1c40c641e64c0c061a0aa96c` |

All four issues are closed, all four PRs are merged with successful required
checks, and every accepted merge is an ancestor of the baseline. The executable
gate is `.csdlc/prepared/issues/5384/validate_dependency_gate.rb`; the complete
machine-readable register is
`.csdlc/evidence/5384/platform-acceptance-ledger.v1.json`.

## Consumer And Operations Proof

- `csdlc-install resolve` selects v2, and the stable operator manifest exposes
  the typed v2 skill set.
- Runtime v3 retained proof covers the bounded external guardian, authenticated
  HTTPS, WSS Observatory, rollback restore, and continuity restore.
- WP-12 retained proof covers ADL v2 opt-in soak, rollback, and Linux, macOS,
  and Windows Runtime v3 acceptance.
- The cutover report selects ADL v2 while preserving a rollback window through
  2026-08-12; deletion is not authorized by this issue.

## Boundaries

WP-14A does not claim Unity, Memory Palace, v0.92 execution, AWS deployment,
GPU operation, or credentialed remote-provider deployment. WP-13 deletion
remains deferred until immediately before #5356. Compatibility binaries still
report `0.91.7`; this acceptance record preserves that branding truth rather
than relabeling it.
