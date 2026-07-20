# C-SDLC v2 Post-Readiness Design Reapproval

Issue #5440 adds one audited recovery operation to the existing typed editor.
`approve-design` may refresh design and diagram references while an issue is
`bound` or `implemented`, provided the active claim, generation, and digest all
match. The operation preserves the lifecycle phase and transition history,
increments generation, refreshes SPP and VPP digests, and appends audit evidence.

The route remains unavailable after exact-revision review. Reviewed, published,
merge-ready, merged, and closed-out records must use their existing typed review
or terminal recovery semantics so design changes cannot silently retain stale
review authority.

No card Markdown is edited directly and no review guard is weakened.
