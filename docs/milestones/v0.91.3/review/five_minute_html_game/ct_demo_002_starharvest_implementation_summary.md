# Starharvest Implementation Summary

## Artifact Layout

- HTML shell for the full game surface
- CSS for atmosphere, HUD, playfield, and panel styling
- plain JavaScript runtime for movement, hazards, crop states, upgrades, and
  win/loss logic

## Key Runtime Pieces

- ship movement with soft drift and boost
- six asteroid gardens with `empty`, `growing`, and `ripe` states
- interactive beam actions on `Space`
- drifting shard hazards that chip hull
- score target, timer, oxygen drain, and combo logic
- small upgrade store for beam, growth, and hull recovery

## Run Path

The demo is intentionally local-first:

- open `demos/v0.91.3/starharvest_five_minute_sprint_demo.html`
- or serve it with `bash adl/tools/demo_v0913_starharvest_html_game.sh`

## Validation Support

- packet validator: `adl/tools/validate_five_minute_html_game_packet.py`
- packet smoke test: `adl/tools/test_five_minute_html_game_packet.sh`

## Current Verification Boundary

The artifact, run helper, and packet validator/test lane are present and pass.

Full browser automation evidence for this exact environment is still partial:
the sandbox blocked the local headless browser runtime during screenshot
capture, so the packet should not overclaim a completed browser-run proof.
