# Demo Matrix - v0.90.5

## Status

Opened issue-wave proof matrix. The demo lane is allocated so WP-18 and WP-19
are not treated as generic release-tail cleanup. WP-19 is now landed, the core
Governed Tools v1.0 rows D1 through D12 each have an explicit proof home, and
the matching reviewer map lives in `FEATURE_PROOF_COVERAGE_v0.90.5.md`.

| ID | Demo | WP | Proof Claim | Required Artifacts | Status |
| --- | --- | --- | --- | --- | --- |
| D1 | Tool-call threat model proof | WP-02 | Tool calls are proposals, not execution | threat model and side-effect taxonomy | LANDED |
| D2 | UTS conformance suite | WP-03-WP-05 | JSON-compatible UTS has valid, invalid, extension, and dangerous-category fixtures | schema, fixture packet, conformance output | LANDED |
| D3 | ACC authority fixture | WP-06-WP-07 | Runtime authority, visibility, delegation, and redaction examples are explicit | ACC fixtures, visibility matrix, and redaction examples | LANDED |
| D4 | Tool registry binding proof | WP-08 | Unknown or unregistered tools cannot bind to execution | registry fixture and rejection tests | LANDED |
| D5 | UTS to ACC compiler proof | WP-09-WP-10 | Model-facing proposals compile deterministically, normalize safely, or fail closed before policy/execution | compiler output and normalization tests | LANDED |
| D6 | Policy and Freedom Gate proof | WP-11-WP-12 | Tool actions require authority and mediation before execution | policy fixtures and decision events | LANDED |
| D7 | Governed executor proof | WP-13 | Only approved ACC-backed actions execute | executor output and refusal records | LANDED |
| D8 | Trace/redaction proof | WP-14 | Tool evidence is reviewable without leaking private data | trace packet and redacted views | LANDED |
| D9 | Dangerous negative suite | WP-15 | Destructive, process, network, exfiltration, missing actor, hidden delegation, unsafe replay, unregistered adapter, and prompt/tool-argument leakage failures fail closed with redacted denial evidence | negative test report | LANDED |
| D10 | Simple local/Gemma proposal evaluation demo | WP-16-WP-17 | The bounded benchmark harness and a local/Gemma-focused evaluation together show proposal shape, authority humility, privacy, unsafe resistance, and any governed fixture-backed execution/refusal path without claiming the full v0.91 comparison report | `docs/milestones/v0.90.5/review/model-proposal-benchmark-report.json`, `docs/milestones/v0.90.5/review/local-gemma-model-evaluation-report.json`, small scorecard, failure notes, and governed fixture-backed demo evidence, or explicit model-availability skip | LANDED |
| D11 | Governed Tools v1.0 flagship demo | WP-18 | Reviewer can inspect one truthful governed-tools packet spanning proposal, validation, ACC, mediation context, execution or denial, trace, and redaction across four named cases | flagship proof packet and reviewer/public reports plus four named case artifacts; not blocked on full v0.91 model comparison | LANDED |
| D12 | Feature proof coverage record | WP-19 | Every governed-tools feature claim reaches review with proof, fixture, non-proving status, or explicit deferral | proof coverage record | LANDED |
| D13 | ACIP proof demo | Comms-08 | Reviewer can inspect one deterministic path from consultation through capability negotiation into governed coding invocation and back into redacted review/public evidence without requiring a live provider | `acip.proof.demo.v1` packet, coding proposal-ready outcome, trace bundle, and explicit non-proving statements | LANDED |

## Non-Proving Boundaries

- These demos do not prove public standardization of UTS.
- These demos do not prove production sandboxing or production secrets handling.
- These demos do not permit arbitrary shell execution by model output.
- These demos do not prove all future tool adapters.
- These demos do not prove full local-vs-remote Gemma comparison; that report is
  deferred to `v0.91`.
- The ACIP proof demo does not prove live transport, encrypted external
  transport, reputation systems, or cross-polis federation.
- These demos do not replace citizen standing, access control, or Freedom Gate.
- These demos do prove that approved fixture-backed actions and denied unsafe
  actions are distinguishable in review evidence.
- The exact proof home for each governed-tools row now lives in
  `FEATURE_PROOF_COVERAGE_v0.90.5.md`.
