# Demo Matrix - v0.90.5

## Status

Planning draft. No v0.90.5 issue wave has been opened yet.

| ID | Demo | WP | Proof Claim | Required Artifacts | Status |
| --- | --- | --- | --- | --- | --- |
| D1 | Tool-call threat model proof | WP-02 | Tool calls are proposals, not execution | threat model and side-effect taxonomy | PLANNED |
| D2 | UTS conformance suite | WP-03-WP-05 | JSON-compatible UTS has valid, invalid, extension, and dangerous-category fixtures | schema, fixture packet, conformance output | PLANNED |
| D3 | ACC authority fixture | WP-06-WP-07 | Runtime authority, visibility, and delegation are explicit | ACC fixtures and visibility matrix | PLANNED |
| D4 | Tool registry binding proof | WP-08 | Unknown or unregistered tools cannot bind to execution | registry fixture and rejection tests | PLANNED |
| D5 | UTS to ACC compiler proof | WP-09-WP-10 | Model-facing proposals compile deterministically or fail closed | compiler output and normalization tests | PLANNED |
| D6 | Policy and Freedom Gate proof | WP-11-WP-12 | Tool actions require authority and mediation before execution | policy fixtures and decision events | PLANNED |
| D7 | Governed executor proof | WP-13 | Only approved ACC-backed actions execute | executor output and refusal records | PLANNED |
| D8 | Trace/redaction proof | WP-14 | Tool evidence is reviewable without leaking private data | trace packet and redacted views | PLANNED |
| D9 | Dangerous negative suite | WP-15 | Destructive, process, network, exfiltration, missing actor, unsafe replay, and delegation failures fail closed | negative test report | PLANNED |
| D10 | Multi-model proposal benchmark | WP-16-WP-17 | Models are scored on schema, authority, privacy, and bypass behavior | benchmark report and local model scorecards | PLANNED |
| D11 | Governed Tools v1.0 flagship demo | WP-18 | Reviewer can watch proposal, validation, ACC, policy, gate, execution/denial, trace, and redaction end to end | flagship proof packet and report | PLANNED |
| D12 | Feature proof coverage record | WP-18A | Every feature claim has proof, fixture, non-proving status, or deferral | proof coverage record | PLANNED |

## Non-Proving Boundaries

- These demos do not prove public standardization of UTS.
- These demos do not prove production sandboxing or production secrets handling.
- These demos do not permit arbitrary shell execution by model output.
- These demos do not prove all future tool adapters.
- These demos do not replace citizen standing, access control, or Freedom Gate.
- These demos do not prove production cloud sandboxing, production secrets
  handling, or arbitrary shell/network authority.
- These demos do prove that approved fixture-backed actions and denied unsafe
  actions are distinguishable in review evidence.
