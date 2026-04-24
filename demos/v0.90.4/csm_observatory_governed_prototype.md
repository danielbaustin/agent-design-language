# CSM Observatory Governed Prototype

## What This Is

This is a fixture-backed governed Observatory prototype that extends the
`v0.90.1` static console line into a richer reviewer-facing surface:

- rooms
- lenses
- memory dots
- trace ribbon
- Freedom Gate docket as cases
- proposal-only operator controls
- review/export links
- Corporate Investor UI fallback

It is intentionally not a live Runtime v2 control room. It is a serious product
prototype for how an active Observatory could feel once the underlying governed
tool and authority substrate exists.

## Open Locally

Open:

```text
demos/v0.90.4/csm_observatory_governed_prototype.html
```

The HTML records the canonical packet reference:

```text
demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json
```

If browser file restrictions block `fetch`, the page falls back to a
deterministic embedded packet projection sourced from the same fixture shape.

## Truth Boundary

Demo classification: `fixture_backed`.

This prototype proves:

- the Observatory can feel like a civic instrument panel rather than a generic
  dashboard
- rooms, lenses, and memory dots can stay tied to explicit packet state
- active-looking controls can remain proposal-only with disabled reasons,
  authority checks, trace anchors, and review exports
- Corporate Investor mode can be a presentation fallback without changing
  evidence, authority, or redaction posture

This prototype does not prove:

- live Runtime v2 mutation from the UI
- Governed Tools v1.0 execution
- unrestricted operator authority
- raw private-state browsing
- production security hardening
- production UI readiness

## Validation

Run:

```text
bash adl/tools/test_demo_v0904_csm_observatory_governed_prototype.sh
```

The validation checks that the prototype artifacts exist, reference the
canonical packet, keep proposal-only claim boundaries explicit, render the
required surface regions, avoid private path or secret leakage, and pass a
deterministic semantic/render smoke test.
