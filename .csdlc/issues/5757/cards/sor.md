# Structured Output Record

Template: 1.0.0

Issue: 5757

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Fix Runtime v3 Observatory trusted localhost origin validation, monotonic live completion ordering, and real shared-certificate authenticated-WSS proof.

## Artifacts

- .csdlc/evidence/5757/DESIGN.md
- .csdlc/evidence/5757/diagram.mmd
- .csdlc/evidence/5757/html-integrated-proof.log
- .csdlc/evidence/5757/shared-localhost-certificate/fingerprints.log
- adl/tools/test_v0917_html_observatory_integrated_proof.sh
- adl/tools/validate_v0917_html_observatory.py
- demos/html-observatory/app.js
- adl-runtime-kernel/tests/observatory.rs

## Execution

- Normalize Runtime v3 Observatory API bases through a strict HTTPS localhost:20997 origin gate before fetch, bearer login, or WebSocket construction.
- Guard live, retained, WSS, and fallback completions with a shared monotonic generation so stale async completions cannot overwrite current UI state.
- Extend the integrated Observatory proof with trusted-origin negative cases, stale completion races, authenticated WSS control evidence, and shared localhost certificate proof on ports 8765 and 20997.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Verify whitespace and conflict-marker hygiene for the ready PR diff.",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_v0917_html_observatory_integrated_proof.sh"
    ],
    "purpose": "Run the focused Observatory proof covering trusted localhost origin validation, stale completion ordering, shared localhost certificate evidence, authenticated WSS control, and focused Runtime v3 Rust tests.",
    "outcome": "passed",
    "evidence_ref": "observatory-integrated-proof.log"
  },
  {
    "command": [
      "node",
      "--check",
      "demos/html-observatory/app.js"
    ],
    "purpose": "Parse the HTML Observatory browser script after the trusted-origin and generation-ordering changes.",
    "outcome": "passed",
    "evidence_ref": "observatory-js-syntax.log"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
