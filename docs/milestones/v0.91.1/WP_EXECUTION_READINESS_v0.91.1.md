# WP Execution Readiness - v0.91.1

## Purpose

This document is the candidate card-authoring source for the v0.91.1
inhabited-runtime readiness wave. It should be reviewed and tightened before
issues are opened.

## Global Execution Rules

- Implement substantial work products; do not close WPs with planning-only
  surfaces unless the WP is explicitly a design or review WP.
- Keep v0.92 identity and birthday claims downstream.
- Keep external and cross-polis communication gated on TLS or mutual-TLS
  equivalent protection.
- Keep intelligence, capability, wellbeing, learning, and ToM signals
  evidence-bound and non-scoreboard.
- Preserve private state and private diagnostic boundaries in observatory and
  review artifacts.
- Keep A2A as an adapter over ACIP, never a parallel communication substrate.

## WP-01: Design Pass

Required outputs:

- Accepted milestone docs and issue wave.
- Opened issue cards with STP, SIP, SPP, and SOR bundles.
- Validation record showing all cards are structurally complete.

Required validation:

- Candidate issue wave matches the sprint and WBS.
- No WP has placeholder outputs, missing validation, or unclear non-goals.

## WP-02: Runtime And Polis Architecture Alignment

Required outputs:

- Source-grounded runtime/polis architecture doc.
- Current-code inventory for runtime, kernel, manifold, polis, citizen
  lifecycle, and control-plane surfaces.
- Drift report for source docs that still describe old behavior.

Required validation:

- Architecture claims cite repo evidence or are marked as planned.
- No absolute host paths or stale v0.90.x claims leak into tracked docs.

## WP-03: CSM Observatory Active Surface

Required outputs:

- Active packet shape for observatory-visible runtime work.
- Operator projection and redaction rules.
- Fixtures for visible, redacted, and invalid packets.

Required validation:

- Private fields remain hidden from public/reviewer views.
- Projection output is deterministic for identical input.

## WP-04: Citizen Standing Model

Required outputs:

- Standing classes for citizens, guests, service actors, external actors, and
  prohibited naked actors.
- Transition and rejection rules.
- Fixtures covering allowed and denied standing changes.

Required validation:

- Naked actors cannot gain authority by omission.
- Standing changes preserve traceable authority and review evidence.

## WP-05: Citizen State Substrate

Required outputs:

- Citizen-state format and security update.
- Projection rules for runtime, operator, reviewer, and public views.
- Fixtures for valid, malformed, stale, and overexposed states.

Required validation:

- State validation fails closed for malformed or unsafe records.
- Projection never exposes private state without policy permission.

## WP-06: Memory And Identity Architecture

Required outputs:

- Memory/identity architecture packet.
- Boundary language for v0.92 identity and birthday work.
- Fixtures showing evidence references rather than hidden continuity claims.

Required validation:

- Architecture does not claim full identity continuity.
- Memory references are traceable and reviewable.

## WP-07: Theory Of Mind Foundation

Required outputs:

- Agent-model schema and update-event contract.
- Evidence requirements and uncertainty handling.
- Fixtures for update, correction, unknown, and privacy-restricted states.

Required validation:

- ToM records preserve uncertainty and do not claim mind-reading.
- Updates cite observable evidence or policy-authorized state.

## WP-08: Capability And Aptitude Testing Foundation

Required outputs:

- First executable capability/aptitude test harness slice.
- Report and scorecard shape with limitations.
- Fixture set for at least three test families.

Required validation:

- Results distinguish capability evidence from ranking or reputation.
- Harness output is deterministic in fixture mode.

## WP-09: Intelligence Metric Architecture

Required outputs:

- Evidence-bound intelligence metric architecture.
- Cognitive Compression Cost or related metric boundary where appropriate.
- Fixture report explaining what the metric does and does not prove.

Required validation:

- Metrics derive from explicit traces or test artifacts.
- Output avoids punitive productivity or reputation framing.

## WP-10: Governed Learning Substrate

Required outputs:

- Learning update and feedback contract.
- Policy boundary for adaptation, review, and rollback.
- Fixtures for accepted feedback, rejected feedback, and unsafe update claims.

Required validation:

- Hidden self-modification is rejected.
- Learning updates preserve evidence and rollback references.

## WP-11: ANRM/Gemma Placement And Trace Dataset

Required outputs:

- ANRM/Gemma placement package.
- Trace extractor spec and dataset mapping.
- Fixture dataset and reviewable limitations.

Required validation:

- Dataset generation is deterministic in fixture mode.
- Docs do not claim training or benchmark success before evidence exists.

## WP-12: ACIP Conformance And Local Encryption Hardening

Required outputs:

- Secure local communication envelope and conformance fixtures.
- Encryption/authentication/redaction design for intra-polis comms.
- Rejection cases for malformed, unsigned, unauthorized, or overexposed
  messages.

Required validation:

- Messages are authenticated, traceable, redacted, and policy-bound.
- External transport remains explicitly out of scope without TLS.

## WP-13: A2A Adapter Boundary And Compatibility Plan

Required outputs:

- A2A adapter-over-ACIP compatibility slice.
- Mapping of A2A concepts to ADL identity, authority, redaction, and trace.
- Non-claims for federation and external transport.

Required validation:

- Adapter cannot bypass ACIP or ACC authority.
- Compatibility docs do not create a second communication model.

## WP-14: Runtime Inhabitant Integration

Required outputs:

- Integrated agent-shaped runtime path using standing, state, memory, comms,
  observatory, and trace evidence.
- Deterministic fixture run and operator report.

Required validation:

- Integration proves runtime execution shape, not birthday or autonomy.
- Artifacts are repo-relative and reviewable.

## WP-15: Observatory-Visible Agent Flagship Demo

Required outputs:

- Runnable demo showing an agent-shaped CSM run.
- Runtime state, communication, trace, observatory projection, and redaction
  artifacts.
- Operator-facing proof report.

Required validation:

- Demo can be rerun deterministically in fixture mode.
- Report states proof claims and non-claims.

## WP-16 Through WP-23: Release Tail

Required outputs:

- Demo matrix and proof coverage.
- Coverage and quality record.
- Docs review package.
- Internal and external review records.
- Accepted-finding remediation.
- v0.92 birthday readiness handoff.
- Release ceremony and end-of-milestone report.

Required validation:

- Release tail follows the standard sequence.
- Review findings are either remediated, deferred with issue links, or
  explicitly accepted as residual risk.
