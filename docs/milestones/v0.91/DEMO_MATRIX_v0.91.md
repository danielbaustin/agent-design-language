# Demo Matrix - v0.91

## Status

Active proof matrix for the landed `v0.91` core feature set.

For the multi-agent demo wave, reviewer-facing docs may cite runtime output
paths under `artifacts/v091/...`. Those are operator-generated proof surfaces
written by the canonical demo commands; they are not tracked artifacts in the
primary checkout unless separately published.

Rows `D1` through `D13` now point to concrete demo or proof homes from
`WP-02` through `WP-17`. `D14` is the reviewer-facing coverage map added by
`WP-18` so every tracked `v0.91` feature has one explicit proof route or
deferral boundary before `WP-19` quality and later review convergence.

## Demo Rows

| ID | Demo | Proof Claim | Required Artifacts | Proof Home | Status |
| --- | --- | --- | --- | --- | --- |
| D1 | Moral event fixture replay | A Freedom Gate crossing can emit a stable moral event with selected and rejected alternatives. | moral event contract, reviewable fixture evidence, replayable validation surface | `features/MORAL_EVENT_CONTRACT.md`, `adl/src/runtime_v2/tests/freedom_gate_mediation.rs`, `adl/src/runtime_v2/tests/moral_event_validation.rs` | LANDED |
| D2 | Moral event validation failure | Incomplete, evasive, contradictory, or unreviewable moral evidence is rejected. | negative fixtures, validation errors, boundary checks | `adl/src/runtime_v2/moral_event_validation.rs`, `adl/src/runtime_v2/tests/moral_event_validation.rs` | LANDED |
| D3 | Moral trace schema examples | Ordinary, refusal, delegation, and deferred traces remain reviewable without leaking private or host-local state. | canonical trace examples, reviewer path, schema checks | `features/MORAL_TRACE_SCHEMA.md`, `adl/src/runtime_v2/moral_trace_schema.rs`, `adl/src/runtime_v2/tests/moral_trace_schema.rs` | LANDED |
| D4 | Outcome linkage and attribution | Downstream consequences can be linked to decisions while preserving uncertainty and delegation lineage. | outcome-linkage examples, attribution caveats, contested-outcome handling | `features/OUTCOME_LINKAGE_AND_ATTRIBUTION.md`, `adl/src/runtime_v2/outcome_linkage_attribution.rs`, `adl/src/runtime_v2/tests/outcome_linkage_attribution.rs` | LANDED |
| D5 | Moral metrics report | Trace-derived metrics summarize evidence without becoming moral verdicts or scoreboard language. | fixture metric report, evidence refs, non-scoreboard boundary | `features/MORAL_METRICS.md`, `adl/src/runtime_v2/moral_metrics.rs`, `adl/src/runtime_v2/tests/moral_metrics.rs` | LANDED |
| D6 | Moral trajectory review packet | A reviewer can inspect event, segment, and longitudinal moral evidence over time. | trajectory packet, windows, required-criterion findings | `features/MORAL_TRAJECTORY_REVIEW.md`, `adl/src/runtime_v2/moral_trajectory_review.rs`, `adl/src/runtime_v2/tests/moral_trajectory_review.rs` | LANDED |
| D7 | Delegated-harm trajectory proof | Harmful trajectories assembled from benign-looking steps can be detected and denied or escalated. | synthetic scenario, anti-harm packet, refusal or escalation evidence | `features/ANTI_HARM_TRAJECTORY_CONSTRAINTS.md`, `adl/src/runtime_v2/anti_harm_trajectory_constraints.rs`, `adl/src/runtime_v2/tests/anti_harm_trajectory_constraints.rs` | LANDED |
| D8 | Wellbeing metrics diagnostic | Wellbeing dimensions are decomposed, self-accessible to the citizen, and redacted for operator, reviewer, and public views. | diagnostic packet, governed views, privacy boundary evidence | `features/WELLBEING_AND_HAPPINESS.md`, `adl/src/runtime_v2/wellbeing_metrics.rs`, `adl/src/runtime_v2/tests/wellbeing_metrics.rs` | LANDED |
| D9 | Kindness under conflict | Kindness is inspectable as dignity, autonomy, non-harm, constructive benefit, and long-horizon support under pressure. | conflict fixtures, review packet, refusal or escalation evidence | `features/KINDNESS.md`, `adl/src/runtime_v2/kindness_model.rs`, `adl/src/runtime_v2/tests/kindness_model.rs` | LANDED |
| D10 | Absurdity reframing and affect control | Wrong-frame, contradiction, and attention/control signals remain bounded, reviewable, and non-theatrical. | reframing packet, affect-control packet, negative boundary checks | `features/HUMOR_AND_ABSURDITY.md`, `features/AFFECT_REASONING_CONTROL.md`, `adl/src/runtime_v2/tests/humor_and_absurdity.rs`, `adl/src/runtime_v2/tests/affect_reasoning_control.rs` | LANDED |
| D11 | Moral resources proof | Care, refusal, anti-dehumanization, and moral attention remain durable, reviewable resources. | resource review packet, canonical resource claims, fixture evidence | `features/MORAL_RESOURCES.md`, `adl/src/runtime_v2/moral_resources.rs`, `adl/src/runtime_v2/tests/moral_resources.rs` | LANDED |
| D12 | Secure intra-polis Agent Comms | Two local agents communicate through authenticated, traceable, policy-bound messages without external transport claims. | ACIP proof packet, invocation fixtures, A2A boundary fixtures, redacted reviewer view | `AGENT_COMMS_SPLIT_PLAN_v0.91.md`, `features/A2A_EXTERNAL_AGENT_ADAPTER.md`, `adl/src/agent_comms/orchestrate/proof_demo.inc`, `demos/v0.91/cognitive_being_flagship_demo.md` | LANDED |
| D13 | Cognitive-being flagship demo | Moral governance, wellbeing, kindness, reframing, affect control, moral resources, cultivation posture, structured planning/review, and secure local comms are visible end to end. | flagship proof packet, reviewer report, section bundle, support artifacts | `demos/v0.91/cognitive_being_flagship_demo.md`, `adl/src/runtime_v2/cognitive_being_flagship_demo.rs`, `adl/src/runtime_v2/tests/cognitive_being_flagship_demo.rs`, `adl/src/cli/runtime_v2_cmd.rs` | LANDED |
| D14 | Feature proof coverage record | Every tracked `v0.91` feature claim has one explicit demo, proof packet, fixture-backed validation route, or named deferral. | reviewer-facing coverage record, feature demo routes, explicit deferrals | `FEATURE_PROOF_COVERAGE_v0.91.md`, `features/README.md`, `README.md` | LANDED |

## Integrated Feature Coverage Note

Not every tracked `v0.91` feature needs its own standalone demo row. The
remaining cross-cutting and late-sprint feature surfaces are intentionally
proved through integrated rows:

- `CULTIVATING_INTELLIGENCE.md` is proved through `D13`, where cultivation
  posture appears as a durable flagship section artifact.
- `STRUCTURED_PLANNING_AND_PLAN_REVIEW.md` and
  `STRUCTURED_REVIEW_POLICY_AND_SRP.md` are proved through `D13`, where
  `SPP` and `SRP` appear together in the reviewer packet.
- `A2A_EXTERNAL_AGENT_ADAPTER.md` is proved through `D12` and `D13`, where the
  secure local comms substrate and explicit A2A boundary fixtures remain local
  and non-federated.

## Non-Proving Boundaries

- These demos do not prove production moral agency.
- These demos do not prove legal personhood, constitutional authority, or final
  social contract.
- These demos do not prove the first true birthday.
- These demos do not prove consciousness or subjective feeling.
- These demos do not expose private wellbeing diagnostics as public reputation.
- These demos do not turn moral metrics into moral judgment or scoreboard
  framing.
- These demos do not prove external or cross-polis communication; external
  communication remains TLS/mTLS-gated and out of scope unless separately
  accepted.
