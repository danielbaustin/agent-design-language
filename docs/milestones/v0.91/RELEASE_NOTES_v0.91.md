# v0.91 Release Notes

## Status

Closeout-candidate release notes for the landed `v0.91` implementation through
`WP-20`.

These notes describe behavior and evidence that exists in the repository after
the core implementation and proof lanes. They do not imply that the release
ceremony is complete. Internal review, third-party review, accepted-finding
remediation, next-milestone handoff, and final ceremony remain assigned to the
standard release-tail WPs.

## Theme

`v0.91` is the moral governance, wellbeing, and cognitive-being foundation
milestone for ADL.

It turns moral and affect-adjacent language into reviewable engineering
surfaces: events, validation, trace, attribution, metrics, trajectory review,
anti-harm constraints, wellbeing diagnostics, kindness, bounded reframing,
affect-like reasoning control, moral resources, cultivation posture, structured
planning/review policy, secure local agent communication, and integrated
proof/demo evidence.

## Landed Highlights

- Freedom Gate moral event records with selected and rejected alternatives.
- Moral event validation with negative cases for incomplete, evasive,
  contradictory, or unreviewable evidence.
- Moral trace schema examples for ordinary, refusal, delegation, and deferred
  decision paths.
- Outcome linkage and attribution with uncertainty and delegation lineage.
- Moral metrics as trace-derived evidence, not moral verdicts or scoreboards.
- Moral trajectory review packets over event, segment, and longitudinal
  evidence.
- Anti-harm trajectory constraints for harmful patterns assembled from
  individually benign-looking steps.
- Wellbeing metrics as private, decomposed diagnostics with citizen self-access
  and redacted operator/reviewer/public views.
- Kindness under conflict as inspectable dignity, autonomy, non-harm,
  constructive benefit, and long-horizon support.
- Humor and absurdity as bounded wrong-frame, contradiction, and reframing
  surfaces rather than theatrical personality claims.
- Affect-like reasoning-control signals with explicit policy and trace hooks.
- Moral resources as care, refusal, anti-dehumanization, and moral attention
  resources.
- Cultivating intelligence as formation, restraint, reasonableness, and moral
  participation.
- Structured planning (`SPP`) and Structured Review Policy (`SRP`) as durable
  workflow artifacts and review-policy surfaces.
- Secure intra-polis Agent Communication and Invocation Protocol evidence with
  local-only, policy-bound, traceable communication and explicit A2A/external
  boundary language.
- Cognitive-being flagship demo proof packet that composes the landed moral,
  wellbeing, kindness, affect/reframing, moral-resource, cultivation,
  structured-planning/review, and secure-comms surfaces.
- Demo matrix and feature-proof coverage map tying every tracked `v0.91`
  feature to a demo, proof route, fixture-backed validation, or explicit
  deferral.
- Coverage and quality gate record after `WP-18`, with current main CI and
  coverage green and closed-issue SOR truth repaired.
- Accepted `v0.91.0` ADRs for moral evidence and cognitive-being substrate,
  secure local Agent Comms and A2A boundary, and structured planning / review
  policy artifacts.

## Evidence

Primary milestone evidence:

- `docs/milestones/v0.91/DEMO_MATRIX_v0.91.md`
- `docs/milestones/v0.91/FEATURE_PROOF_COVERAGE_v0.91.md`
- `docs/milestones/v0.91/QUALITY_GATE_v0.91.md`
- `docs/milestones/v0.91/features/README.md`
- `demos/v0.91/cognitive_being_flagship_demo.md`
- `docs/adr/0016-moral-evidence-and-cognitive-being-substrate.md`
- `docs/adr/0017-secure-local-agent-comms-and-a2a-boundary.md`
- `docs/adr/0018-structured-planning-and-review-policy-artifacts.md`

Current quality evidence:

- Main CI run `25514295183` after `WP-18`: `adl-ci` success and
  `adl-coverage` success.
- Coverage run evidence: 1813 tests run, 1813 passed, 2 skipped, 90.37%
  workspace line coverage, and per-file coverage gate passing at the 80%
  threshold.
- Closed-issue SOR truth validator: PASS for 27 closed `v0.91` issues after
  local record repair.

## Explicit Deferrals

The following work is intentionally not claimed as complete in `v0.91`:

- `v0.91.1`: inhabited-runtime readiness, agent lifecycle states, ACIP state
  eligibility, capability/aptitude testing, intelligence metrics, ANRM/Gemma,
  Theory of Mind, memory/identity alignment, runtime-v2/polis docs alignment,
  and ACIP/A2A hardening.
- `v0.91.2`: UTS + ACC multi-model benchmarking, provider-native tool-call
  comparison, runtime/test-cycle recovery, coverage ergonomics, CodeBuddy
  productization, Google Workspace CMS bridge, modernization demo, publication
  packets, rustdoc/doc cleanup, and workflow guardrails.
- `v0.92`: identity, continuity, and first true Gödel-agent birthday work.
- `v0.93`: constitutional citizenship, polis governance, IAM, social contract,
  and broader enterprise-security work.

## Not Claimed

`v0.91` does not claim:

- production moral agency
- legal personhood
- consciousness or subjective feeling
- complete constitutional authority
- the first true birthday
- durable identity architecture
- scalar karma, scalar happiness, or final moral judgment
- public wellbeing diagnostics or public reputation derived from private
  wellbeing state
- external or cross-polis agent communication without TLS/mTLS-equivalent
  protection

## Remaining Release Tail

The remaining release-tail WPs still need to complete before ceremony:

- `WP-21`: internal review
- `WP-22`: external / third-party review
- `WP-23`: accepted-finding remediation
- `WP-24`: next milestone planning
- `WP-25`: release ceremony
