# Starharvest Refresh Note v0.91.4

## Status

`passed_with_fresh_browser_proof`

## Demo Identity

- Demo: `Starharvest`
- Demo id: `ct_demo_002_starharvest`
- Refresh issue: `#3458`
- Umbrella issue: `#3455`
- Primary artifact: `demos/v0.91.3/starharvest_five_minute_sprint_demo.html`
- Existing proof packet: `docs/milestones/v0.91.3/review/five_minute_html_game/FIVE_MINUTE_HTML_GAME_PACKET_v0.91.3.md`

## Refresh Decision

Starharvest remains the first visible browser-game proof surface for the demo showcase. The v0.91.4 refresh does not redesign or rebuild the game. It carries forward the v0.91.3 artifact and makes the proof boundary explicit for reviewers.

## Current Proof Status

| Surface | Status | Evidence |
| --- | --- | --- |
| Artifact exists | `passed` | `demos/v0.91.3/starharvest_five_minute_sprint_demo.html` |
| Helper launch path | `passed` | `bash adl/tools/demo_v0913_starharvest_html_game.sh --print-path`; `bash adl/tools/demo_v0913_starharvest_html_game.sh --print-url` |
| Prior packet proof | `passed` | `docs/milestones/v0.91.3/review/five_minute_html_game/FIVE_MINUTE_HTML_GAME_PACKET_v0.91.3.md` |
| Browser/gameplay proof | `passed` | `docs/milestones/v0.91.4/review/demo_showcase/STARHARVEST_BROWSER_PROOF_v0.91.4.md` records a fresh localhost browser proof for load, render, Space interaction, and restart. |

## What Reviewers Should Do

1. Start with the HTML artifact for the quickest visual payoff.
2. Use the v0.91.3 proof packet to inspect the original design, implementation, QA, and proof claims.
3. Treat Starharvest as evidence that a bounded C-SDLC creative sprint can produce a visible browser artifact.
4. Use the v0.91.4 browser proof for the current load, render, interaction, and restart evidence.

## Claims Preserved

- Starharvest is a concrete, inspectable static browser artifact.
- The prior proof packet remains linked and reviewable.
- The launch helper remains the canonical run entry point for this artifact.
- The v0.91.4 browser proof closes the prior representative browser/gameplay gap for load, render, interaction, and restart.

## Non-Claims

- This refresh does not claim a new v0.91.4 implementation sprint.
- This refresh does not claim production game quality.
- This refresh does not claim exhaustive browser/gameplay validation beyond the recorded representative proof path.
- This refresh does not claim that every C-SDLC issue completes in five minutes.

## Validation

Focused commands for this refresh:

```bash
bash adl/tools/demo_v0913_starharvest_html_game.sh --print-path
bash adl/tools/demo_v0913_starharvest_html_game.sh --print-url
curl -I --max-time 2 http://127.0.0.1:43191/demos/v0.91.3/starharvest_five_minute_sprint_demo.html
test -f demos/v0.91.3/starharvest_five_minute_sprint_demo.html
test -f docs/milestones/v0.91.3/review/five_minute_html_game/FIVE_MINUTE_HTML_GAME_PACKET_v0.91.3.md
```

## Follow-On Route

If WP-13 needs exhaustive win/loss path coverage, route that as a bounded follow-on against Starharvest or the showcase package. Do not widen this refresh beyond the recorded representative browser proof path.
