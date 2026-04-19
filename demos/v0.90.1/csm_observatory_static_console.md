# CSM Observatory Static Console Prototype

## What This Is

This is the first read-only CSM Observatory console prototype. It renders the
fixture-backed proto-csm-01 visibility packet into a reviewer-facing control
room surface:

- manifold header
- citizen constellation
- kernel pulse
- Freedom Gate docket
- trace ribbon
- operator action rail

The visual direction is an illuminated chartroom for a small governed world:
professional, advanced, and memorable without pretending the runtime is live.

## Open Locally

Open:

```text
demos/v0.90.1/csm_observatory_static_console.html
```

No web server is required for inspection. When served from a static file server,
the JavaScript reads the canonical packet reference. When opened directly from a
local file and browser file restrictions block fetch, it falls back to a
deterministic embedded packet projection sourced from the same fixture. The HTML
records the canonical packet reference:

```text
demos/fixtures/csm_observatory/proto-csm-01-visibility-packet.json
```

## Truth Boundary

Demo classification: fixture_backed.

This prototype proves the Observatory mental model and visual language. It does
not prove a live CSM run, live Runtime v2 capture, live operator mutation,
v0.92 identity rebinding, migration, or a first Gödel-agent birthday.

All operator actions are read-only or disabled. Live mutation remains deferred
until command packets route through the Runtime v2 kernel/control plane.

## Validation

Run:

```text
bash adl/tools/test_demo_v0901_csm_observatory_static_console.sh
```

The validation checks that the console artifacts exist, remain fixture-labeled,
reference the canonical packet, include the required first-screen regions, avoid
private path or endpoint leakage, and keep operator mutation disabled.
