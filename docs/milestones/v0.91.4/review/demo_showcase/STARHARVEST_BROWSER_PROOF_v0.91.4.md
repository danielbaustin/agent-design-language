# Starharvest Browser Proof v0.91.4

## Status

`passed`

## Issue

- Refresh issue: `#3458`
- Umbrella issue: `#3455`
- Demo artifact: `demos/v0.91.3/starharvest_five_minute_sprint_demo.html`
- Local URL tested: `http://127.0.0.1:43191/demos/v0.91.3/starharvest_five_minute_sprint_demo.html`

## Browser Proof Method

The proof used the Codex in-app browser against the local Starharvest HTTP server. It drove browser-side keyboard input through the browser automation surface and read the resulting DOM state.

Local server:

```bash
PORT=43191 bash adl/tools/demo_v0913_starharvest_html_game.sh
```

Reachability check:

```bash
curl -I --max-time 2 http://127.0.0.1:43191/demos/v0.91.3/starharvest_five_minute_sprint_demo.html
```

Observed HTTP status: `200 OK`.

## Browser Evidence

```json
{
  "before": {
    "asteroidCount": 6,
    "hazardCount": 3,
    "score": "0 / 180",
    "seeds": "4",
    "status": "Glide to an asteroid, then press Space to plant, tend, or harvest."
  },
  "afterSpace": {
    "score": "0 / 180",
    "seeds": "3",
    "status": "Planted Ember Hollow. Stay mobile while the crop warms up."
  },
  "afterR": {
    "score": "0 / 180",
    "seeds": "4",
    "status": "Glide to an asteroid, then press Space to plant, tend, or harvest."
  }
}
```

## Checks Passed

- Page loaded through localhost in a real browser surface.
- HUD rendered with score and seed state.
- Six asteroid elements rendered.
- Three hazard elements rendered.
- Pressing `Space` performed a valid game interaction and changed seeds from `4` to `3`.
- Pressing `R` restarted the run and reset seeds to `4`.
- Status banner changed after interaction and returned to the initial instruction after restart.

## Claim Boundary

This proof upgrades the v0.91.4 Starharvest browser/gameplay status to `passed` for the representative load, render, interaction, and restart path above. It does not claim exhaustive playthrough coverage, win/loss path coverage, production game quality, or universal five-minute delivery.
