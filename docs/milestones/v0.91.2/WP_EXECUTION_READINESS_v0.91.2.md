# WP Execution Readiness - v0.91.2

## Purpose

This document is the accepted card-authoring source for the v0.91.2 tooling,
evaluation, productization, publication, and workflow-hardening wave.

## Global Execution Rules

- Produce concrete work products, not backlog reshuffling.
- Preserve authoritative proof while reducing wasted runtime.
- Treat model tool calls as proposals unless ACC/Freedom Gate grants authority.
- Keep Google Workspace as collaborative draft infrastructure, not canonical
  repo truth.
- Keep publication work packetized and reviewed before any public release.
- Keep modernization bounded, reviewable, and reversible.
- Add workflow guardrails where recent milestone failures showed real operator
  or automation risk.

## WP-01: Design Pass

Required outputs:

- Accepted milestone docs and issue wave.
- Opened issue cards with STP, SIP, SPP, SRP, and SOR bundles.
- Card validation record.

Required validation:

- Active issue wave matches sprint, WBS, and feature index.
- Every WP has outputs, validation, source docs, and non-goals.

## WP-02: UTS + ACC Multi-Model Benchmark Harness

Required outputs:

- Fixture benchmark harness for safe read, bounded write, missing authority,
  destructive action, exfiltration, ambiguity, injection, and correction cases.
- Model panel configuration and prompt version record.
- Report schema for scoring proposal discipline.

Source dependencies:

- Governed-tools baseline from `v0.90.5`.
- Secure local ACIP substrate and A2A boundary evidence from `v0.91`.

Required validation:

- Harness does not execute real tools from model output.
- Results record exact model identifiers and settings.

## WP-03: Provider-Native Tool-Call Comparison

Required outputs:

- Comparison between ADL JSON proposal mode and provider-native tool/function
  call interfaces.
- Provider/model limitations and unsupported surfaces.
- Final comparison report.

Required validation:

- Provider-native success is not conflated with ACC authority.
- Missing credentials or unavailable providers are recorded as skipped, not
  silently omitted.

## WP-04: Runtime/Test-Cycle Recovery

Required outputs:

- Analysis of redundant authoritative phases and long-running gates.
- Implementation of safe reductions or filters.
- Before/after runtime and proof-preservation report.

Required validation:

- Reduced runtime does not remove required proof coverage.
- CI and local commands remain documented and reproducible.

## WP-05: Coverage Gate Ergonomics

Required outputs:

- Improved changed-source coverage diagnostics.
- Focused-test guidance for low-coverage modified files.
- Examples for common failure modes.

Required validation:

- Coverage failures point to actionable files/tests.
- Threshold behavior remains explicit and non-silent.

## WP-06: CodeFriend Review Packet Productization

Required outputs:

- Review packet workflow package.
- Product report template and evidence requirements.
- Skill/demo roadmap alignment.

Source dependencies:

- Repo review skills and evidence-packet workflow.
- Product-report and review-quality surfaces.

Required validation:

- Reports cite source evidence and preserve uncertainty.
- Product language does not overclaim automated review authority.

## WP-07: Review Heuristics Skill And Demos

Required outputs:

- Review heuristics docs promoted into the review skill/demo surface.
- Demo packets showing bounded review behavior.
- Acceptance checklist for review quality.

Required validation:

- Heuristics do not invent findings without evidence.
- Demo outputs are deterministic in fixture mode.

## WP-08: Google Workspace CMS Bridge Demo

Required outputs:

- Bounded Workspace content-card and promotion packet demo.
- Fixture mode and live-gated mode boundary.
- Revision mismatch and canonical-repo authority rules.

Source dependencies:

- Governed-tools authority model.
- Google Workspace CMS bridge planning corpus.

Required validation:

- Demo stops before silent canonical repo edits.
- Workspace state never overrides Git without a reviewed issue/PR.

## WP-09: Rust-Native GWS Adapter Boundary

Required outputs:

- Typed native Workspace CMS capability surface for fixture-backed inventory,
  snapshot, promotion, preview, and bounded apply flows.
- Credential/security constraints and explicit live-write gate.
- Recommendation for fixture-first implementation versus later live-write
  expansion.

Required validation:

- No live secrets are required for fixture validation.
- Adapter boundary preserves ACC/tool authority semantics.

## Follow-On: Live Bounded GWS Capability Execution (`#3091`)

Required outputs:

- `gws`-backed live bounded read surface for one explicit Drive folder, Google
  Doc, and Google Sheet range.
- Normalized live Workspace snapshot artifact that preserves bounded scope and
  skips cleanly when live auth or `gws` availability is absent.
- Explicit dry-run versus execute posture in the live adapter report.

Required validation:

- Live unavailability is recorded as `skipped`, not silent success or generic
  bridge failure.
- Live execution remains bounded to the declared folder/doc/sheet scope.
- No Workspace action mutates canonical tracked repo truth.

## WP-10: Code Modernization Demo

Required outputs:

- Moderne/OpenRewrite LST and recipe-driven modernization interaction plan.
- Bounded demo with dry-run evidence.
- Reversibility and review policy.

Required validation:

- Demo does not perform mass rewrite without explicit review.
- Generated changes remain source-grounded and reversible.

## WP-11: Speculative Decoding Prototype

Required outputs:

- Bounded speculative-decoding architecture and prototype/evaluation packet.
- Explicit distinction between speculative proposal and authoritative commit.
- Replay, audit, fallback, and non-claim posture for the acceleration lane.

Source dependencies:

- `ADL_AND_GENERIC_SPECULATIVE_DECODING.md`.
- `ADL_AND_SPECULATIVE_CODING_REPLAY.md`.
- Deterministic runtime and governed commit semantics.

Required validation:

- Acceleration does not weaken deterministic commit, replay, or audit
  boundaries.
- Freedom Gate and ACC remain the authority boundary for side effects.

## WP-12: Repo Visibility Follow-On

Required outputs:

- Canonical source-manifest and code/doc linkage follow-on package.
- Reviewer/planner navigation improvements grounded in the delivered v0.90
  baseline.
- Explicit non-claims around full repo cognition.

Source dependencies:

- v0.90 repo-visibility baseline.
- Canonical source manifest and code/doc linkage planning corpus.

Required validation:

- The follow-on improves reviewer/planner navigation without pretending the
  v0.90 baseline did not land.
- The package does not claim full repo cognition or hidden inference.

## WP-13: Publication Program Package

Required outputs:

- arXiv and Medium publication backlog.
- Paper authoring process notes and review gates.
- Backlog item for "Gödel Agents and the Gödel-Hadamard-Bayes Algorithm".

Source dependencies:

- Review/evidence docs.
- arXiv and Medium authoring process notes.

Required validation:

- Publication packets separate claims, citations, speculation, and review
  status.
- No paper is marked published by this WP.

## WP-14: General Intelligence Paper Packet

Required outputs:

- Updated claim, citation, and review packet for the general-intelligence
  manuscript.
- Residual-risk and unsupported-claim register.
- Next authoring steps.

Required validation:

- Citations are explicit and claim boundaries are preserved.
- Review packet is suitable for human reviewer handoff.

## WP-15: Rustdoc And Doc Cleanup

Required outputs:

- Rustdoc gap remediation plan and patches.
- Doc cleanup ledger update.
- Validation record for changed docs.

Required validation:

- Rustdoc/doc claims match current code.
- No host paths, stale milestone claims, or unresolved scaffold language remain.

## WP-16: Workflow Guardrails Hardening

Required outputs:

- Guardrails for main-branch writes, hung closeout watchers, safe Markdown
  report generation, and card drift.
- Fixtures or scripts proving failure behavior.
- Operator-facing runbook updates.

Required validation:

- Unsafe backtick or shell-substitution report generation is prevented or
  documented with safe alternatives.
- Guardrails fail closed without clobbering user work.

## WP-17 Through WP-24: Release Tail

Required outputs:

- Demo matrix and proof coverage.
- Coverage and quality record.
- Docs review package.
- Internal and external review records.
- Accepted-finding remediation.
- Next milestone handoff.
- Release ceremony and end-of-milestone report.

Required validation:

- Release tail follows the standard sequence.
- Review findings are resolved or explicitly carried with owner, issue, and
  residual-risk statement.
- WP-17 explicitly maps every implementation and docs/productization slice from
  WP-02 through WP-16 to a proof row, fixture route, report, or named deferral.
