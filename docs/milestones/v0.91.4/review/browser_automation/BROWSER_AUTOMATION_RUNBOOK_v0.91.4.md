# Browser Automation Runbook v0.91.4

## Status

`canonical_agent_route_defined`

## Purpose

ADL agents need a boring, repeatable browser proof path for demos, localhost checks, and web-facing review work. Browser proof should not be rediscovered during every demo issue.

## Canonical Recommendation

For ADL agent proof, use the Codex in-app browser first. The repo-cached Playwright Chromium executable should be diagnosed and recorded when present, but direct shell-driven headless Chromium is not the default proof route unless its smoke test passes in the current environment.

Why:

- it can load `localhost` and `127.0.0.1` pages from the Codex app context
- it supports screenshot/DOM proof
- it supports real browser-side CUA input such as clicks and keypresses
- it avoids relying on macOS LaunchServices app-name lookup from the sandboxed shell
- the primary-checkout or worktree Playwright Chromium cache path can be diagnosed directly without relying on `open -a`
- a cached Chromium version check can prove which executable exists, while a headless smoke check proves whether direct shell execution is usable

Use operator-installed Chromium/Safari/Chrome as operator-visible fallback paths, not as the primary automated agent path. Use direct cached Chromium headless only after the diagnostic smoke says it works. In the current environment, the cached Chromium executable can report its version, but direct shell headless launch fails under crashpad permission constraints.

## Route Types

| Route | What it proves | What it does not prove | Status |
| --- | --- | --- | --- |
| `curl` / HTTP HEAD | URL reachability and HTTP status | rendering, script execution, keyboard/mouse interaction | useful but insufficient |
| Codex in-app browser | page load, rendering, screenshot, DOM state, CUA input | operator's external browser app state | canonical for agent proof |
| Codex Browser MCP route | local HTTP page load, Playwright-backed DOM inspection, click/type/key proof through the in-app browser | direct control of the operator's visible Chromium app | canonical automation substrate inside Codex |
| Computer Use visible Chromium route | control of the real visible Chromium app, address-bar navigation, accessibility-tree proof, screenshots of what the operator sees | Playwright DOM APIs or headless automation | canonical for human-visible Chromium demos |
| `open -a chromium ...` from operator shell | operator can open a URL in local Chromium | Codex shell can resolve the same app name | valid manual/operator route |
| `open -Ra <app>` from Codex shell | whether LaunchServices resolves an app for the Codex process | whether the operator can open it from another shell | diagnostic only |
| repo-cached Playwright Chromium executable | shell-visible Chromium executable under `.adl/.cache/diagram-renderers/playwright/...`; version check | app-name lookup or successful headless execution | diagnostic route; direct proof only if smoke passes |
| direct Chrome/Chromium executable headless | possible headless proof if stable | may abort under macOS/sandbox constraints | optional fallback |
| standalone Playwright install outside repo | package/runtime availability and browser-cache metadata | successful browser launch from the Codex shell | diagnostic only unless launch smoke passes |

## Why Chromium Can Be Up But Not Usable From Codex Shell

The operator shell and Codex tool process do not necessarily share the same app lookup behavior. During `#3458`, the operator reported:

```bash
open -a chromium http://www.google.com
```

worked locally. In the Codex shell, app lookup for `chromium` failed even though browser proof was still possible through the Codex in-app browser. Treat this as an environment boundary:

- operator shell app lookup: useful manual route
- Codex shell app lookup: diagnostic route only
- Codex in-app browser: canonical automated proof route
- Computer Use app control: canonical route when the requirement is the visible Chromium window

Do not conclude that a browser is unavailable merely because one route fails.

## Visible Chromium Proof Pattern

When the requirement is specifically "open it in Chromium" or "show the demo in the real browser window," use Computer Use against the running Chromium app instead of shell `open -a` or direct headless Playwright.

Known working route from `#3497`:

- Computer Use resolved the running Chromium app:
  - app bundle: `.adl/.cache/diagram-renderers/playwright/chromium-1134/chrome-mac/Chromium.app/`
  - bundle ID: `org.chromium.Chromium`
- Computer Use set the Chromium address bar to:
  - `http://127.0.0.1:43191/demos/v0.91.3/starharvest_five_minute_sprint_demo.html`
- Chromium loaded the Starharvest page.
- The accessibility tree verified:
  - window title: `Starharvest / Five-Minute Sprint Demo`
  - URL: `127.0.0.1:43191/demos/v0.91.3/starharvest_five_minute_sprint_demo.html`
  - heading: `Starharvest`
  - timer, stardust, seeds, hull, objective, asteroid field, selected asteroid, and restart button were present.

Use this route for human-visible demo playback. Use the Browser MCP route for automated DOM/gameplay proof where Playwright selectors are more precise.

## In-App Browser Proof Pattern

Use the Browser skill and the in-app browser. For interaction proof, prefer Playwright-backed DOM inspection for stable selectors, CUA input for game keys/clicks when needed, then inspect DOM state.

Minimal local HTTP smoke pattern:

```js
await tab.goto('http://127.0.0.1:43219/');
await tab.playwright.waitForLoadState({ state: 'load', timeoutMs: 10000 });
await tab.playwright.locator('#btn').click();
const result = await tab.playwright.evaluate(() => ({
  title: document.title,
  heading: document.querySelector('#ok')?.textContent,
  clicked: document.body.dataset.clicked || null,
}));
```

Known working Starharvest pattern:

```js
await tab.goto('http://127.0.0.1:43191/demos/v0.91.3/starharvest_five_minute_sprint_demo.html');
await tab.playwright.waitForLoadState({ state: 'load', timeoutMs: 10000 });
await tab.cua.click({ x: 500, y: 400 });
await tab.cua.keypress({ keys: ['Space'] });
const afterSpace = await tab.playwright.evaluate(() => ({
  score: document.querySelector('#hud-score')?.textContent,
  seeds: document.querySelector('#hud-seeds')?.textContent,
  status: document.querySelector('#status-banner')?.textContent,
}));
await tab.cua.keypress({ keys: ['r'] });
const afterRestart = await tab.playwright.evaluate(() => ({
  score: document.querySelector('#hud-score')?.textContent,
  seeds: document.querySelector('#hud-seeds')?.textContent,
  status: document.querySelector('#status-banner')?.textContent,
}));
```

Important: do not rely on constructing `KeyboardEvent` inside the evaluated page context unless you have verified that event constructors exist there. During `#3458`, CUA keypress was the reliable route.

## Diagnostic Script

Use the browser route diagnostic to record what the current environment exposes:

```bash
python3 adl/tools/diagnose_browser_routes.py --json
```

With a local URL reachability check:

```bash
python3 adl/tools/diagnose_browser_routes.py --url http://127.0.0.1:43191/demos/v0.91.3/starharvest_five_minute_sprint_demo.html --json
```

The diagnostic separates:

- `app_routes`: app lookup visible to the Codex shell
- `known_executable_routes`: known macOS executable paths
- `version_smoke`: direct executable identity check for Chrome/Chromium-family routes
- `headless_smoke`: optional direct shell headless execution check
- `path_routes`: PATH-resolved browser commands
- `http_check`: HTTP reachability only
- `codex_in_app_browser_route`: documented canonical route that must be exercised through the Browser skill, not the shell

## Demo Issue Rules

- Record browser proof as `passed` only when a browser route loads the page and verifies the required DOM or interaction behavior.
- Record `curl` as reachability proof only.
- If operator Chromium works but Codex shell app lookup fails, record both facts.
- Prefer representative proof paths for demo readiness and route exhaustive coverage as separate follow-on work.
- Do not bury browser setup discoveries inside chat; update this runbook or the issue proof record.

## Starharvest Reference Result

The Starharvest proof route that worked in `#3458` was:

- local server on `127.0.0.1:43191`
- Codex in-app browser opened the demo URL
- CUA click focused the page
- CUA `Space` changed seeds from `4` to `3`
- CUA `r` restarted the game and reset seeds to `4`

This is the model for future lightweight demo browser proof.

## Current Diagnostic Result

Focused diagnostic command:

```bash
python3 adl/tools/diagnose_browser_routes.py --headless-smoke --json
```

Observed in the Codex shell during `#3497`:

- Browser MCP / Codex in-app browser route: passed a local HTTP proof against `127.0.0.1`, including page load, selector click, and DOM mutation inspection.
- Computer Use visible Chromium route: passed against the actual Chromium window. It loaded Starharvest, verified the visible URL/title, and exposed the game state through the accessibility tree.
- Standalone Playwright installed outside the repo at `<operator-local-playwright-tools>` reports Playwright `1.60.0` and browser cache metadata, but `chromium.launch({ headless: true })` fails from the Codex shell with a macOS MachPort permission error. Treat this as proof that the package is installed, not proof that direct shell browser launch is usable.
- `open -Ra chromium`: failed with `Unable to find application named 'chromium'`.
- `open -Ra Chromium`: failed with `Unable to find application named 'Chromium'`.
- `open -Ra Google Chrome`: failed with `Unable to find application named 'Google Chrome'`.
- `open -Ra Safari`: failed with `Unable to find application named 'Safari'`.
- macOS Chrome binary pattern, `<macos-app-bundle>/Contents/MacOS/Google Chrome`: exists in the reviewed environment and is executable.
- macOS Safari binary pattern, `<macos-app-bundle>/Contents/MacOS/Safari`: exists in the reviewed environment and is executable.
- `<operator-local-chromium-profile>` and `<operator-local-chromium-cache>`: exist as Chromium profile/cache directories; this is evidence of browser use, not an executable route.
- Alternate Chrome bundle pattern, `<alternate-macos-app-bundle>/Contents/MacOS/Google Chrome`: exists in this environment and should be diagnosed separately from the primary Chrome app when needed.
- primary-checkout Playwright Chromium executable: discovered at `.adl/.cache/diagram-renderers/playwright/chromium-1134/chrome-mac/Chromium.app/Contents/MacOS/Chromium` and reports `Chromium 129.0.6668.29`; issue worktrees may not have their own `.adl/.cache` copy.
- direct primary-checkout Playwright Chromium headless smoke from the Codex shell: attempted and failed with crashpad permission/handshake errors; do not use direct cached Chromium headless as the proof route unless a fresh smoke passes.
- direct Google Chrome headless smoke from the Codex shell: attempted and failed with signal-style return code `-6`; do not use direct Chrome headless as the default proof route here.
- Chromium operator route: operator reports `open -a chromium ...` works from their shell, but Codex shell cannot resolve `chromium`; preserve that distinction.
- Codex in-app browser route: worked during Starharvest proof, worked during local HTTP smoke, and remains the canonical agent route.

Conclusion: the issue is not simply “no browser is installed.” It is a boundary between shell app lookup, direct macOS app execution, standalone Playwright shell launch, visible-app control, and the Codex in-app browser automation surface. ADL agents should use the in-app browser / Browser MCP route for automated proof, use Computer Use for visible Chromium demos, record Playwright and Chromium executable paths as diagnostic routes, preserve operator `open -a chromium ...` as a manual fallback, and avoid direct cached Chromium, Chrome, or standalone Playwright headless launch unless a fresh diagnostic proves it works.
