# CodeFriend v1 Build Plan

## Status

Planning artifact for `#4756`.

This document defines the complete CodeFriend v1 build plan. It does not claim
that CodeFriend v1 is implemented, externally usable, or publication-ready.
Implementation must be executed through future tracked issues, with retained
proof for each release gate.

## Product Goal

CodeFriend v1 is the first complete product version of CodeFriend: a practical,
evidence-bound architecture cognition system for software teams.

The product should run a real repository through a governed review flow, produce
source-grounded architecture and engineering findings, explain tradeoffs, route
remediation and tests, preserve human judgment, and generate a report a team can
actually use.

## Source Baseline

CodeFriend v1 builds on already completed baseline work:

- `v0.90` CodeFriend/CodeBuddy review showcase and architecture-document
  generation established the first review/demo lane.
- `v0.91.2` CodeFriend productization packaged the review-packet workflow,
  evidence requirements, product-report template, and skill/demo alignment.
- ADR 0025 accepted CodeFriend as an evidence-bound review-packet and
  product-report workflow, not autonomous review authority.
- `v0.91.4` created the private `agent-logic/codefriend.ai` product/site
  repository and published the coming-soon site at `https://codefriend.ai` and
  `https://www.codefriend.ai` over CloudFront HTTPS.

These prove product substrate and review-workflow baseline. They do not prove
CodeFriend v1 product completion, adapter v2, external-repo execution,
architecture cognition, governance, memory, or customer/publication readiness.

## Positioning

CodeFriend is not autocomplete, generic code generation, or autonomous review
authority.

The v1 positioning is:

> Continuous architectural cognition for software teams.

That means CodeFriend helps teams observe, review, govern, remember, and improve
software architecture over time.

## v1 Exit Bar

CodeFriend v1 is complete only when all of these are true:

- A bounded external repository can be reviewed through adapter v2.
- The run produces a retained input manifest, evidence inventory, skipped-surface
  register, review packet, specialist lane outputs, synthesis, redaction result,
  architecture artifacts, remediation plan, test plan, and product report.
- Findings are source-grounded, severity-classified, deduplicated, and tied to
  evidence.
- Architecture cognition features produce useful dependency, boundary, drift,
  ADR, and governance insight beyond generic code review.
- Human review and publication approval remain explicit gates.
- Sample inputs and sample outputs are committed as non-secret fixtures.
- Operator setup, user workflow, validation, and release notes are complete.
- Public/customer-facing claims match retained proof.

## Non-Goals

- Do not claim autonomous code-review authority.
- Do not mutate customer repositories without explicit user approval.
- Do not publish customer-like reports without redaction and operator approval.
- Do not replace human engineering judgment.
- Do not hide C-SDLC or ADL workflow defects inside product polish.
- Do not make CodeFriend v1 a blocker for v0.92 birthday readiness.

## Core Architecture

CodeFriend v1 has six product layers:

| Layer | Responsibility |
| --- | --- |
| Product shell | User workflow, project setup, run history, report access, and operator controls. |
| Adapter v2 | Portable external-repo ingestion, manifest generation, evidence capture, and ADL lifecycle mapping. |
| Evidence core | Source inventory, skipped-surface tracking, redaction boundaries, provenance, and retained artifacts. |
| Architecture cognition | Dependency, boundary, ADR, drift, governance, and structural-risk analysis. |
| Review engine | Specialist lanes, synthesis, severity normalization, remediation, tests, diagrams, and report generation. |
| Publication gate | Human review, redaction, non-claims, public/customer-ready packaging, and release evidence. |

## Feature Set

### 1. Product Shell

Required features:

- connect to the existing private `agent-logic/codefriend.ai` product/site
  repository or a successor product repo selected by a tracked decision
- create a CodeFriend project for one repository
- configure repository source, branch/ref, scope, exclusions, and evidence budget
- select review profile: architecture review, security review, test review,
  documentation review, dependency review, or full product report
- show run status, retained artifacts, findings, diagrams, report drafts, and
  publication state
- preserve operator override and stop controls
- export a report packet with manifest and non-claims

Acceptance:

- A user can start a bounded review without knowing ADL internals.
- The UI/CLI names every skipped or unavailable surface.
- Product copy never implies autonomous authority.
- The live coming-soon site can link to or hand off to the product shell only
  after publication review approves the claim boundary.

### 2. Adapter v2

Required features:

- external-repo input manifest
- source checkout or archive ingestion policy
- path-safe evidence root
- file inventory with include/exclude rules
- language/framework detection
- dependency manifest discovery
- test/doc/build surface discovery
- skipped-surface register with reasons
- secret/path hygiene checks
- ADL lifecycle mapping from product run to issue/card/proof records
- stable artifact IDs for replay and report provenance
- compatibility mapping for legacy `codebuddy` schema names and filenames where
  current skills still require them

Acceptance:

- A fixture external repo can run end-to-end through adapter v2.
- The adapter can produce the same manifest and evidence inventory for identical
  inputs.
- The adapter fails closed on missing scope, unsafe paths, secret-like material,
  or ambiguous publication state.

### 3. Evidence Core

Required features:

- repository packet builder
- evidence inventory
- source excerpts within allowed quotation limits
- skipped-surface ledger
- unknowns and assumptions ledger
- residual-risk ledger
- redaction policy
- artifact retention manifest
- portable path normalization
- evidence-to-finding traceability
- explicit compatibility record when legacy artifact names such as
  `codebuddy_product_report.md`, `codebuddy_product_report.json`, or
  `codebuddy.repo_packet` are retained

Acceptance:

- Every finding can point to evidence or explicitly declare uncertainty.
- Every omitted surface is visible to the reviewer.
- No absolute host paths, secrets, or private operator prompts appear in
  exported artifacts.

### 4. Architecture Cognition

Required features:

- dependency graph summary
- module/package boundary map
- coupling and cohesion signals
- connascence and change-amplification candidates
- architecture drift notes
- architectural quantum candidates
- blast-radius estimates for important changes
- PR or diff impact summary
- ADR discovery, stale ADR detection, and ADR candidate generation
- rationale and decision-history summary
- architecture risk scoring with evidence and uncertainty

Acceptance:

- The product produces architecture-specific insight, not only code-review
  lint.
- Each architecture claim is grounded in source evidence, repo metadata, docs,
  or explicitly marked as an inference.
- Structural-risk scores are explainable and non-authoritative.

### 5. Executable Governance

Required features:

- architecture fitness-function authoring
- layer and forbidden-import checks
- dependency-cycle checks
- ADR-required-change detection
- security-boundary checks
- evidence-completeness checks
- publication-readiness checks
- policy exceptions with rationale
- CI-friendly governance output

Acceptance:

- At least one fixture repo has runnable governance checks.
- Policy failures include evidence, affected paths, severity, and remediation
  guidance.
- Governance output is advisory unless explicitly configured as a release gate.

### 6. Review Engine

Required specialist lanes:

- code/correctness review
- architecture review
- security review
- test-quality review
- dependency/supply-chain review
- documentation review
- diagram planning and diagram review when requested
- redaction and evidence audit before publication

Required synthesis features:

- finding deduplication
- severity normalization
- disagreement and uncertainty preservation
- remediation sequence
- test-generation plan
- architecture diagram packet
- ADR candidates
- product report generation
- compatibility-preserving output names until a tracked schema migration creates
  versioned `codefriend` successors

Acceptance:

- Specialist lanes can run independently and feed one synthesis packet.
- Synthesis does not hide uncertainty or downgrade unsupported severe findings.
- Report output separates findings, assumptions, non-claims, and residual risk.

### 7. Human Review And Publication Gate

Required features:

- human approval before customer-facing export
- redaction status visible on every report packet
- public-claim and non-claim checklist
- legal/privacy-sensitive cue review
- publication manifest
- release note and changelog packet for CodeFriend product releases

Acceptance:

- No report is publication-ready without explicit approval state.
- Reports preserve the human-review requirement.
- Public copy matches retained proof and non-claims.

### 8. Memory And Longitudinal Intelligence

Required features:

- project memory index
- prior finding carryforward
- ADR timeline
- trend summary across repeated runs
- recurring architectural risk register
- remediation outcome tracking
- queryable rationale history

Acceptance:

- A second run can compare against a previous run.
- Persisted memory is scoped to the project and can be exported or deleted.
- Longitudinal claims distinguish observed history from inferred trend.

### 9. Integrations

Required integrations:

- `agent-logic/codefriend.ai` product/site repository handoff
- GitHub repository input
- local filesystem repository input
- CI artifact input where available
- optional issue/PR comment export
- optional report export to Markdown, PDF, or HTML
- future hooks for Jira, Linear, Google Drive, or Slack without making them v1
  blockers

Acceptance:

- GitHub and local input paths are proven.
- Optional integrations are feature-gated and cannot block core review.
- Exported artifacts remain portable.

### 10. Evaluation And Quality

Required features:

- sample repositories
- golden review packets
- golden reports
- fixture-based adapter tests
- redaction regression tests
- evidence traceability tests
- architecture-cognition quality review
- benchmark run-time and cost envelope
- false-positive and false-negative review notes

Acceptance:

- v1 has repeatable local proof for adapter, evidence, review, redaction, and
  report generation.
- Reviewer quality is evaluated with retained packets, not only happy-path demos.
- Known weaknesses are recorded as residual risks or follow-on issues.

## Implementation Milestones

### Milestone A: v0.93.x Working Alpha

Goal: one complete, testable CodeFriend flow.

Deliverables:

- product/site repository handoff from the proven `v0.91.4` coming-soon surface
- product shell skeleton
- adapter v2 first slice
- external-repo fixture
- evidence inventory
- review packet builder
- specialist lanes and synthesis
- redaction check
- product report template
- sample packet and report
- operator runbook

Exit bar:

- One fixture repo runs through packet -> review -> synthesis -> redaction ->
  report.

### Milestone B: v0.95 MVP Proof Packaging

Goal: prove CodeFriend v1 and adapter v2 are ready for MVP consumption.

Deliverables:

- consumption of the complete CodeFriend v1 build plan from this document
- adapter v2 manifest and replay proof
- D4b CodeFriend external-repo demo packet
- architecture cognition first slice
- governance first slice
- publication gate
- retained proof artifacts

Exit bar:

- One bounded external-repo review runs through ADL with redaction and manifest
  evidence, consuming the `#4756` pre-v0.92 obligation boundary.

### Milestone C: CodeFriend v1 Product Completion

Goal: complete the full feature set listed above.

Deliverables:

- product shell completion
- architecture cognition suite
- executable governance suite
- project memory
- repeated-run trend evidence
- GitHub/local input support
- report export support
- quality/evaluation packet
- launch readiness review
- versioned naming/schema migration decision for legacy `CodeBuddy` /
  `codebuddy` compatibility surfaces

Exit bar:

- CodeFriend v1 is ready for real operator/customer-style testing with truthful
  product claims, retained proof, and publication controls.

## Work Breakdown

| ID | Workstream | Outcome |
| --- | --- | --- |
| CFV1-01 | Product boundary and UX | Product shell, user journeys, non-claim copy, and operator controls. |
| CFV1-02 | Adapter v2 | External-repo manifest, ingestion, inventory, skipped surfaces, and lifecycle mapping. |
| CFV1-03 | Evidence core | Evidence packet, traceability, redaction, retention, and portability. |
| CFV1-04 | Review engine | Specialist lanes, synthesis, remediation, test planning, diagrams, ADRs, and reports. |
| CFV1-05 | Architecture cognition | Dependency graph, coupling, drift, ADR, blast radius, and risk scoring. |
| CFV1-06 | Executable governance | Fitness functions, policy checks, exceptions, and CI-friendly output. |
| CFV1-07 | Memory | Prior findings, ADR timeline, trend reports, and project memory controls. |
| CFV1-08 | Integrations | GitHub/local input, optional exports, and feature-gated external integrations. |
| CFV1-09 | Quality and evaluation | Fixtures, golden packets, regression tests, reviewer-quality checks, and cost envelope. |
| CFV1-10 | Publication and release | Human approval, redaction, public-claim review, launch packet, and release notes. |
| CFV1-11 | Naming and compatibility | Current CodeFriend naming, legacy CodeBuddy compatibility, schema/file migration decisions, and historical signposts. |

## Release Gates

CodeFriend v1 cannot be declared complete until these gates pass:

- adapter v2 fixture proof
- external-repo end-to-end proof
- evidence traceability proof
- redaction and publication-safety proof
- architecture-cognition quality review
- governance check proof
- repeated-run memory proof
- product-report quality review
- human approval workflow proof
- residual-risk and non-claim review
- naming and compatibility migration decision

## Validation Strategy

Use a layered validation strategy:

1. Unit tests for manifest parsing, evidence inventory, redaction, and report
   generation.
2. Fixture integration tests for adapter v2 and review-packet generation.
3. Golden packet tests for synthesis and report output.
4. Architecture cognition review against known fixture repositories.
5. End-to-end run through a bounded external repository.
6. Publication-gate review with redaction and non-claim checks.

## Open Decisions

- Whether CodeFriend v1 ships from a dedicated product repo, an ADL package, or
  a staged external package.
- Whether the existing private `agent-logic/codefriend.ai` repository becomes
  the v1 product repo or remains the static site / publication repo.
- Which fixture repository becomes the canonical v1 proof target.
- Which UI surface is first: CLI, local web app, hosted app, or report-first
  workflow.
- Which optional integrations enter v1 versus post-v1.
- Which governance checks are advisory by default and which may become blocking.
- Which legacy `codebuddy` schemas, artifact roots, and generated filenames
  remain compatibility identifiers versus receive versioned `codefriend`
  successors.

## Relationship To `#4756`

Issue `#4756` owns the pre-v0.92 obligation boundary. It may prove and link this
plan, but it does not implement CodeFriend v1.

The executable `runtime_v2.codefriend_adapter_obligations.v1` packet records
that:

- CodeFriend v1 and adapter v2 are real tracked obligations.
- v0.92 may consume only bounded handoff truth.
- v0.95 owns the external-repo proof packaging path.
- full CodeFriend v1 completion requires future tracked implementation issues.
