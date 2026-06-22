# HTML Mobile Governed Observatory Proof for #4341

## Scope

This packet records the bounded proof surface for the rebuilt HTML Observatory
lane in `#4341`.

It proves a portable operator/reviewer/mobile Observatory surface. It does not
claim Unity completion, live governed mutation, or WP-09 umbrella closeout.

## Source Evidence

- `demos/v0.90.4/csm_observatory_governed_prototype.html`
- `demos/v0.90.4/csm_observatory_governed_prototype.css`
- `demos/v0.90.4/csm_observatory_governed_prototype.js`
- `demos/v0.90.4/csm_observatory_governed_prototype.md`
- `adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json`
- `demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json`
- `docs/milestones/v0.91.6/features/OBSERVATORY_UNITY_CONSUMPTION_CLASSIFICATION_v0.91.6.md`
- `docs/milestones/v0.91.6/review/observatory/WP09_WORKING_UNITY_OBSERVATORY_CLOSEOUT_4035.md`

## What Changed

- The HTML Observatory now loads the current bounded runtime visibility packet
  rather than pointing only at the older governed prototype packet.
- The portable surface keeps an embedded governed fallback overlay so direct
  file-open and fetch-blocked cases still render truthfully.
- The UI now exposes proof, rehearsal, substrate, blocked, and deferred
  posture directly in the surface.
- The UI now exposes the HTML versus Unity lane split directly in the surface.
- Mobile/touch-safe affordances were tightened for phone and tablet widths.

## Proof Claims

This issue proves:

- the HTML Observatory can consume current bounded runtime visibility artifacts
  without depending on Unity;
- the portable surface keeps redaction and proposal-only boundaries explicit;
- mobile-oriented navigation remains readable and touch-safe in the authored
  layout contract;
- proof posture and HTML/Unity role split are explicit instead of implied.

This issue does not prove:

- live AWS, runtime, or governed-tool mutation;
- Unity Observatory readiness or completion;
- WP-09 umbrella closeout;
- production OTel/security closure beyond the cited bounded review inputs.

## Validation

Run:

```text
bash adl/tools/test_demo_v0904_csm_observatory_governed_prototype.sh
```

That focused proof checks:

- artifact existence;
- packet-schema validity for the fallback governed packet;
- JSON readability for the current runtime visibility packet consumed by the
  HTML surface;
- current runtime packet reference in the HTML;
- classification and lane-split surface rendering through the JS smoke test;
- authored mobile CSS guardrails and touch-target affordance tokens;
- secret/path leakage absence across the demo artifacts.

## Reviewer Takeaway

`#4341` should be accepted if reviewers agree that the HTML Observatory is now a
truthful portable lane:

- HTML Observatory = operator / reviewer / mobile surface
- Unity Observatory = richer inhabitant / interactive surface

and that the rebuilt surface leaves blocked and deferred WP-09 work explicit
instead of smearing it into false readiness.
