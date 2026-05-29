# Browser Proof Runbook v0.91.4

## Purpose

Browser proof should be boring. Demo issues should not rediscover how to open a local page every time they need to prove a static HTML artifact.

This runbook records the preferred browser-proof order for v0.91.4 demo work.

## Preferred Order

1. Use the Codex in-app browser when running inside Codex.
   - It can load `localhost` and `127.0.0.1` pages.
   - It has a Playwright-backed evaluation surface.
   - Use it for proof capture when the demo needs DOM/gameplay assertions.
2. Use the installed Chromium app from an operator terminal when the in-app browser is unavailable.
   - Known operator command:
     ```bash
     open -a chromium http://127.0.0.1:43191/demos/v0.91.3/starharvest_five_minute_sprint_demo.html
     ```
   - This may work in the operator shell even if `open -a chromium` does not resolve from a sandboxed Codex process.
3. Use Safari as a manual fallback only when automation is not required.
4. Use `curl` only for HTTP reachability, not for gameplay proof.

## Starharvest Local Server

```bash
PORT=43191 bash adl/tools/demo_v0913_starharvest_html_game.sh
```

Quick URL check:

```bash
bash adl/tools/demo_v0913_starharvest_html_game.sh --print-url
curl -I --max-time 2 http://127.0.0.1:43191/demos/v0.91.3/starharvest_five_minute_sprint_demo.html
```

## Proof Expectations

A real browser/gameplay proof should verify at least:

- page title or hero text loads
- ship exists
- asteroid elements render
- hazard elements render
- HUD exists
- pressing `Space` triggers a valid interaction
- pressing `R` restarts the run

## Known Pitfalls

- Do not treat `curl` success as gameplay proof.
- Do not report `partial` if a browser proof path is available.
- Do not assume app lookup behaves the same in the operator shell and Codex sandbox.
- If a browser app is installed but cannot be found from Codex, record the failing command and use the in-app browser or ask the operator to run the explicit `open -a chromium ...` command.

## Current v0.91.4 Finding

During `#3458`, `curl` confirmed the Starharvest page returned HTTP 200 from the local server. The operator confirmed `open -a chromium http://www.google.com` works from their shell, while the Codex sandbox could not resolve `open -a chromium`. Future proof tooling should preserve both routes instead of treating one failed launcher as evidence that browsers are unavailable.
