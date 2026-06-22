# CSM Observatory Governed Prototype

## What This Is

This is a portable governed Observatory surface that extends the `v0.90.1`
static console line into a richer reviewer-facing and mobile-capable surface:

- rooms
- lenses
- memory dots
- trace ribbon
- Freedom Gate docket as cases
- proposal-only operator controls
- review/export links
- Corporate Investor UI fallback
- explicit proof / rehearsal / substrate / blocked / deferred posture
- explicit HTML versus Unity lane split

It consumes the bounded runtime visibility artifact at:

```text
adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json
```

and overlays governed prototype UI metadata for rooms, lenses, classification,
and review posture.

It is intentionally not a live Runtime v2 control room. It is a serious
portable prototype for how an operator/reviewer Observatory can feel once the
underlying governed tool and authority substrate exists.

## Open Locally

Open:

```text
demos/v0.90.4/csm_observatory_governed_prototype.html
```

The HTML records the current bounded runtime visibility packet:

```text
adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json
```

If browser file restrictions block `fetch`, the page falls back to a
deterministic embedded governed packet overlay sourced from the same bounded
artifact family and policy inputs.

## Truth Boundary

Demo classification: `fixture_backed_governed_mobile_surface`.

This prototype proves:

- the Observatory can feel like a civic instrument panel rather than a generic
  dashboard
- rooms, lenses, and memory dots can stay tied to explicit packet state
- a portable HTML surface can consume current bounded runtime visibility inputs
  without depending on Unity
- active-looking controls can remain proposal-only with disabled reasons,
  authority checks, trace anchors, and review exports
- proof / rehearsal / substrate / blocked / deferred posture can stay visible
  instead of being implied
- HTML and Unity lane responsibilities can stay explicit
- Corporate Investor mode can be a presentation fallback without changing
  evidence, authority, or redaction posture

This prototype does not prove:

- live Runtime v2 mutation from the UI
- Governed Tools v1.0 execution
- unrestricted operator authority
- raw private-state browsing
- production security hardening
- Unity lane completion

## Validation

Run:

```text
bash adl/tools/test_demo_v0904_csm_observatory_governed_prototype.sh
```

The validation checks that the prototype artifacts exist, reference the
current bounded runtime packet, keep proposal-only claim boundaries explicit,
render the required surface regions, avoid private path or secret leakage, and
pass a deterministic semantic/render smoke test.
