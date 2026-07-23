# Issue 5613 preparation review

Reviewed revision: `d3f0c0eb8d1169822bf7795b1894f010ed3f469c`

Reviewer: `subagent:019f8654-f522-7ef3-8d22-6d4b8d9a643b`

Result: changes required; four blockers found.

## Dispositions

1. Retained portability proof now scans the values card, rendered SOR, and
   shared retained receipt.
2. Issue 5339 and 5591 terminal commit inputs now use full 40-character SHAs in
   authoritative design, typed cards, and executable proof.
3. The VPP now includes the promised complete locked C-SDLC v2 test lane.
4. A dedicated exact-scope validator rejects every path outside the declared
   repair surface and rejects all dependency-manifest, Runtime, ADL-v2, infra,
   CI, AWS/provider-enabling scope by construction.

The Clippy budget mismatch is also corrected to 600 seconds. Fresh-worktree
doctor invocations use the current `--repo` interface.
