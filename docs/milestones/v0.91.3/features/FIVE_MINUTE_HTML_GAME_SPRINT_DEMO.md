# Five-Minute HTML Game Sprint Demo

## Status

In flight under demo `WP-02` / `#3221`.

## Purpose

Build the first high-visibility C-SDLC creative-production demo: a playable
browser game that is visibly authored, reviewable, and runnable without hidden
setup.

## Feature Surface

The demo artifact is:

- `demos/v0.91.3/starharvest_five_minute_sprint_demo.html`
- `demos/v0.91.3/starharvest_five_minute_sprint_demo.css`
- `demos/v0.91.3/starharvest_five_minute_sprint_demo.js`

Supporting run and proof surfaces are:

- `adl/tools/demo_v0913_starharvest_html_game.sh`
- `docs/milestones/v0.91.3/review/five_minute_html_game/`

## Expected Outcome

The demo should show that a bounded C-SDLC slice can produce:

- a compact but real game loop
- intentional art direction
- explicit scope cuts
- reviewable proof surfaces

## Gameplay Contract

The current game loop is intentionally small:

- one ship
- six asteroid gardens
- planting, tending, and harvesting actions
- drifting hazards
- score target and win/loss state
- a small upgrade rail

## Proof Boundary

This demo is meant to prove:

- C-SDLC can produce a small but real visible artifact
- the artifact can be run locally without special environment setup
- the artifact can carry an explicit proof packet and QA surface

This demo is not meant to prove:

- universal five-minute software delivery
- general game-engine replacement
- production-ready game content or balancing

## Linked Review Packet

- `docs/milestones/v0.91.3/review/five_minute_html_game/FIVE_MINUTE_HTML_GAME_PACKET_v0.91.3.md`

