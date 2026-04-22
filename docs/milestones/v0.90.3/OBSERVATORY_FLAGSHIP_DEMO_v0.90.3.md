# Observatory Flagship Demo - v0.90.3

## Status

Implemented runbook and reviewer entrypoint for the v0.90.3 inhabited CSM
Observatory flagship demo.

This runbook consumes the multimode UI architecture in
`OBSERVATORY_UI_ARCHITECTURE_v0.90.3.md`. The demo is not only an inhabited
story; it is the first planned traversal through the Observatory rooms, lenses,
memory dots, and Corporate Investor UI fallback.

WP-10 has landed the redacted private-state projection proof in
`REDACTED_OBSERVATORY_PROJECTIONS_v0.90.3.md`. WP-14 should consume that proof
as the private-state visibility substrate rather than inventing ad hoc
projection behavior inside the flagship demo.

Reviewer command:

```bash
adl runtime-v2 observatory-flagship-demo --out artifacts/v0903/demo-d12-observatory-flagship
```

The command emits `runtime_v2/observatory/flagship_proof_packet.json`,
`runtime_v2/observatory/flagship_operator_report.md`, and
`runtime_v2/observatory/flagship_walkthrough.jsonl` alongside the supporting
witness, receipt, projection, access-control, challenge, quarantine, and
operator-control artifacts.

## Purpose

The v0.90.3 demo matrix should not collapse into integration tests in disguise.
Feature proof rows remain necessary, but the flagship demo should make the CSM
polis feel inhabited and governed.

The demo principle is: no ship or walled town is anything if it is empty.

## Demo Story

The demo shows one bounded local CSM scenario:

- a citizen-like actor has private state, signed continuity evidence, witness,
  and receipt
- a guest enters the polis but cannot silently acquire citizen rights
- a service actor performs a bounded role without becoming a hidden social actor
- an operator sees a redacted Observatory projection
- an ambiguous or challenged transition enters sanctuary or quarantine
- the Freedom Gate docket records allow, deny, defer, or challenge decisions
- the timeline shows causal events without exposing raw private state

The point is not to claim personhood. The point is to show the substrate in
motion.

## Observatory Rooms

The demo should traverse rooms in this order:

1. World / Reality Mode
   - opens the demo on an inhabited polis rather than an artifact list
   - shows citizen, guest, service actor, sanctuary/quarantine boundary,
     resource weather, and trace routes
   - uses lenses to make projection boundaries explicit

2. Operator / Governance Mode
   - follows one trace route into a Freedom Gate docket case
   - shows allow, deny, defer, or challenge decisions as cases with evidence
   - shows disabled unsafe actions with reasons

3. Cognition / Internal State Mode
   - optional in v0.90.3
   - shows only bounded implemented signals such as coupling, coherence,
     degraded state, or anomaly markers when evidence exists
   - must not claim mature PHI, affect, instinct, or emotional substrate

4. Corporate Investor UI fallback
   - available through a visible operator control and keyboard shortcut
   - useful when the multimode UI is too complex for the room or demo continuity
     matters more than showing the full power-user surface
   - switches presentation only, not evidence, trace, redaction, or authority

The spaceship/art-deco design remains valuable as a power-user or fallback
surface, but it should not be the default investor-facing path.

## Walkthrough Sequence

| Step | Room | Lens / Memory Dot | What The Reviewer Sees | Proof Boundary |
| --- | --- | --- | --- | --- |
| 1 | World / Reality Mode | triage overview | inhabited polis with citizen, guest, service actor, and protected boundary | design/demo projection, not live production UI |
| 2 | World / Reality Mode | continuity lens | citizen worldline, witness, receipt, and trace route | continuity evidence without raw private state |
| 3 | World / Reality Mode | quarantine lens | challenged or ambiguous transition enters sanctuary/quarantine | unsafe activation is blocked |
| 4 | Operator / Governance Mode | quarantine review | Freedom Gate docket case with evidence, rationale, and next safe action | docket is audit surface, not unrestricted control |
| 5 | Operator / Governance Mode | continuity proofs | disabled unsafe actions and allowed read-only review affordances | command authority remains policy-bound |
| 6 | Cognition / Internal State Mode | anomaly watch | bounded coherence/coupling/degraded-state signal if implemented | no PHI, affect, or personhood claim |
| 7 | Corporate Investor UI fallback | corporate investor view | calm art-deco summary for boardroom or constrained demo context | presentation fallback only |

## Actors

| Actor | Role | What The Demo Shows |
| --- | --- | --- |
| Citizen-like actor | provisional governed identity | continuity, standing, private state projection, receipt |
| Guest | default human/external entry mode | no silent rights escalation |
| Service actor | bounded technical actor | authority without hidden citizenship |
| Operator | accountable human operator | redacted visibility and disabled unsafe actions |
| Challenged state or actor | ambiguity surface | quarantine/sanctuary rather than unsafe activation |

## Required Artifacts

- visibility or Observatory packet
- witness
- citizen receipt
- redacted projection
- private-state redaction policy and leakage/authority negative-case proof
- standing or communication event
- standing policy and communication-boundary negative-case proof
- access approval or denial event
- access authority matrix and denial-fixture proof
- challenge or quarantine artifact
- operator report
- feature proof coverage record
- multimode UI architecture reference
- lens and memory-dot sequence
- Corporate Investor UI fallback note

## Demo Matrix Relationship

D1 through D11 remain feature proof rows. They prove individual surfaces.

D12 is the flagship scenario. It ties the rows together so a reviewer can answer:

- who is inside the CSM?
- what state changed?
- what evidence proves continuity?
- who was allowed or denied access?
- what did the Observatory show?
- what did it intentionally hide?
- what was quarantined or challenged?

D13 remains the proof coverage record.

D14 is the UI architecture design artifact. The flagship demo must consume D14
but must not treat D14 as runtime proof.

## Lenses And Memory Dots

Minimum lenses:

- public lens
- operator lens
- reviewer lens
- continuity lens
- quarantine lens

Minimum memory dots:

- triage overview
- continuity proofs
- quarantine review
- anomaly watch if bounded evidence exists
- corporate investor view

Each lens must state what it shows, what it redacts, and what artifact supports
the projection. Each memory dot must restore an authorized view only.

## Non-Proving Boundaries

This demo does not prove:

- first true Gödel-agent birth
- full personhood
- full moral, emotional, kindness, or wellbeing substrate
- complete migration or cross-polis continuity
- full contract-market economics
- unrestricted operator control
- production security or privacy hardening
- mature PHI, affect, instinct, or emotional-substrate instrumentation
- production UI readiness

It proves a bounded local citizen-state safety and visibility story.
