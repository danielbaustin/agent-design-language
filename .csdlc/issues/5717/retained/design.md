# Issue 5717 design

Update the integrated podcast studio page copy and small layout details now
that #5715 preserved and routed the operator-provided HTML export.

This issue intentionally edits the studio page text for launch readiness. The
visual structure, script/runtime wiring, clean `podcast-studio.html` filename,
audio artifact, RSS feed, and podcast landing route remain intact. The source
of truth for the studio page remains the committed reference bundle under
`demos/podcast/studio-reference/`; generated served output under
`demos/podcast/studio/` is refreshed from that reference.

Required operator changes:

- Use the correct Agent Logic logo.
- Use the name `Synthetic Minds Podcast`.
- Replace "New guests drop in most weeks" with "Special guests join us occasionally."
- Correct podcast episode numbers to start at 1.
- Reduce the gap below the "Listen now" button.
- Use `podcast@agent-logic.ai` as the contact email link.
- Answer "Is there video?" with no.
- Keep FAQ capitalized.
- Replace fake past episodes with proposed episode topics starting at episode 1.
- Add a line break after the copyright line.

The work must not claim production deployment, rewrite the audio/RSS foundation,
or broaden the studio redesign beyond these copy, logo, and tiny spacing fixes.
