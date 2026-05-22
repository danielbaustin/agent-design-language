# CodeFriend Setup Plan

## Status

Planning surface for issue `#3238`.

This document prepares CodeFriend setup work for a future product milestone.
Current planning assumption: CodeFriend alpha is large enough to own a whole
milestone, likely in the `v0.93.x` band. That milestone should end with a fully
working alpha version of CodeFriend ready for testing.

This document does not implement the product application, rename all historical
identifiers, or claim external delivery readiness.

## Product Name

Current product/domain name: `CodeFriend.ai`.

Use `CodeFriend` for current product-facing docs. Treat `CodeBuddy` as a legacy
working name unless the reference is deliberately historical, internal schema
lineage, or part of a compatibility path that needs a dedicated migration.

## Product Thesis

CodeFriend should not be positioned as autocomplete, generic code generation, or
an autonomous code-review authority.

The stronger positioning is:

> Continuous architectural cognition for software teams.

That means CodeFriend should help make software architecture continuously
observable, reviewable, measurable, governable, and explainable.

The v0.91.2 productization proof already established the first concrete slice:
evidence-bound review packets, specialist reviews, diagrams, remediation plans,
redaction checks, and customer-grade report templates. The next step is to turn
that proof into a clearer product setup path.

## Milestone Target

Candidate milestone: `v0.93.x`.

Alpha exit bar:

- CodeFriend has a working alpha flow ready for testing.
- The alpha can run a repository review packet through evidence collection,
  specialist review lanes, synthesis, redaction/publication checks, and a
  human-friendly report.
- The alpha has clear setup docs, runbooks, sample inputs, sample outputs, and
  review boundaries.
- The alpha demonstrates architecture-cognition value, not just generic code
  review.
- Naming, docs, product boundaries, and demo material use `CodeFriend` /
  `CodeFriend.ai` consistently.

The alpha milestone is not the whole CodeFriend roadmap. It should make
CodeFriend real enough to test. Later roadmap bands should add the remaining
architecture-intelligence, governance, memory, integration, and product-delivery
features.

## Post-Alpha Roadmap

These bands are product-roadmap planning buckets, not final ADL milestone
commitments. They should be translated into concrete milestone packages after
the `v0.93.x` alpha plan is reviewed.

### `v0.93.x`: Working Alpha

Goal: one complete, testable product path.

Expected capability:

- repository review packet creation
- specialist review lane execution
- review synthesis
- product-report generation
- redaction/publication-safety check
- architecture-aware findings and diagrams
- sample repo, sample packet, sample report, and runbook
- clear product boundary and human-review requirement

Exit bar:

- CodeFriend is ready for operator/customer-style alpha testing.

### Post-Alpha Band 1: Structural Intelligence

Goal: move beyond packetized review into architecture understanding.

Candidate features:

- dependency graph intelligence
- coupling and cohesion analysis
- connascence analysis
- architectural quantum detection
- blast-radius prediction
- PR architectural impact summaries
- architectural drift scoring
- longitudinal trend reports

Exit bar:

- CodeFriend can explain structural risk and drift over time, not just produce
  one-time review findings.

### Post-Alpha Band 2: Executable Governance

Goal: turn architectural policy into repeatable fitness functions.

Candidate features:

- architecture fitness-function authoring
- layer and boundary violation checks
- ADR-required-change detection
- evidence completeness gates
- review-packet publication gates
- policy exceptions with explicit rationale
- CI-friendly governance reports

Exit bar:

- CodeFriend can help teams define and run architecture governance checks
  without turning governance into vague advice.

### Post-Alpha Band 3: Architectural Memory

Goal: preserve rationale, tradeoffs, and engineering history.

Candidate features:

- ADR timeline generation
- stale ADR detection
- PR-to-ADR linkage
- rationale extraction from issues, PRs, and reviews
- architectural trajectory summaries
- trace-linked decision history
- queryable product/repo memory

Exit bar:

- CodeFriend can answer why the architecture changed, not only what changed.

### Post-Alpha Band 4: Productization And Delivery

Goal: make CodeFriend practical for repeated external testing and eventual
customer use.

Candidate features:

- product repo or product package decision
- polished public-facing README and examples
- sample customer-style reports
- onboarding flow
- pricing/packaging assumptions, if needed later
- redaction and legal/publication review gates
- repeatable demo and trial workflow

Exit bar:

- CodeFriend has a repeatable delivery surface suitable for broader testing.

## Near-Term Setup Goals

Before the product milestone starts, CodeFriend work should prepare:

- a single tracked planning home for product docs
- a naming migration plan from legacy `CodeBuddy` surfaces to `CodeFriend`
- an alpha product boundary grounded in review packets and product reports
- a roadmap from useful review automation toward architectural cognition
- a setup sequence for any future product repo, site, demo, or customer-facing
  package
- candidate WBS / sprint shape for the `v0.93.x` CodeFriend alpha milestone
- post-alpha roadmap bands for the remaining CodeFriend feature set
- follow-on issues for execution instead of hidden scope expansion

## Workstreams

### 1. Naming And Document Consolidation

Goal: make current product docs say `CodeFriend` and give operators one place
to find product planning.

Required follow-on work:

- update user-facing docs that still describe the current product as
  `CodeBuddy`
- preserve historical milestone/demo references when changing them would distort
  past evidence
- decide whether internal schema names such as `codebuddy.repo_packet` remain
  compatibility identifiers or receive a versioned migration
- add signposts where legacy directory names remain intentionally unchanged

Do not perform a repo-wide string replacement. The migration needs review
because some references are historical, some are product-facing, and some are
runtime/schema compatibility surfaces.

### 2. Product Boundary

Goal: keep CodeFriend credible and evidence-bound.

The current product boundary is:

- CodeFriend produces source-grounded findings, diagrams, remediation plans,
  tests recommendations, architecture summaries, redaction checks, and
  customer-grade reports.
- CodeFriend does not replace human judgment.
- CodeFriend must identify evidence, skipped surfaces, assumptions, residual
  risk, and publication boundaries.
- CodeFriend reports should be useful, but not magically certain.

This follows the v0.91.2 productization package and ADR 0025 candidate.

### 3. Architecture Cognition Roadmap

Goal: make the product direction sharper than generic AI code review.

The current strategic direction is:

- Alpha: assistive engineering automation with architecture-aware review,
  diagrams, ADR drafts, docs, reports, dependency summaries, and review packets.
- Structural intelligence: coupling, cohesion, dependency graph intelligence,
  blast-radius prediction, architecture drift detection, governance scoring,
  and PR architectural impact analysis.
- Cognitive governance: architectural memory, rationale continuity, trace-linked
  decisions, executable governance, and adaptive engineering workflows.

The important product wedge is not "AI writes code." The wedge is "teams can see
and govern architectural drift before it becomes expensive."

### 4. Architecture Fitness Functions

Goal: turn architecture governance from static advice into executable checks.

Candidate fitness-function lanes:

- dependency cycles
- forbidden imports
- layer isolation
- ADR-required changes
- security boundary checks
- traceability requirements
- evidence completeness for review packets
- product-report publication gates

This should start small. The alpha needs low-friction governance that developers
trust before more advanced architectural cognition is attempted.

### 5. Product Milestone Shape

Goal: prepare a whole CodeFriend alpha milestone instead of scattering alpha
setup across unrelated issues.

Candidate `v0.93.x` work bands:

- design pass and issue-wave readiness
- user-facing naming migration
- alpha product boundary and README/runbook
- review-packet runner polish
- specialist lane packaging
- architecture-cognition first slice
- architecture fitness-function first slice
- sample repo / sample packet / sample report
- redaction and publication-safety gate
- demo and testing handoff
- internal review
- external review
- remediation
- release/alpha readiness ceremony

The alpha milestone should prove one complete path, not every future
architecture-cognition capability.

The remaining feature set should be planned after the alpha milestone as
separate roadmap bands for structural intelligence, executable governance,
architectural memory, and product delivery.

### 6. Product Setup Sequence

The likely setup sequence is:

1. Finish this planning home and reference inventory.
2. Open a naming-migration issue for user-facing docs and obvious product copy.
3. Create a `v0.93.x` CodeFriend alpha milestone plan and issue wave.
4. Open product setup issues for the CodeFriend alpha package.
5. Decide whether CodeFriend gets a separate repo, a product subdirectory, or a
   staged external package after the alpha package is clear.
6. Prepare a polished sample review packet and product report.
7. Prepare a public-facing README or landing-page brief after product boundary
   language is reviewed.
8. Review redaction, evidence, and publication safety before any external use.
9. Close the alpha milestone only when the working alpha is ready for testing.

## Follow-On Issue Candidates

### User-Facing Naming Migration

Scope:

- update current user-facing `CodeBuddy` references to `CodeFriend`
- leave historical, schema, and compatibility references alone unless reviewed
- add signposts for intentionally preserved legacy names

Validation:

- scan for `CodeBuddy`, `codebuddy`, `CodeFriend`, and `codefriend`
- review changed references by category
- run docs link and whitespace checks

### CodeFriend Alpha Package

Scope:

- define the minimum review packet -> specialist review -> synthesis -> product
  report flow that can be shown as CodeFriend alpha
- include evidence requirements, redaction boundary, and sample output
- avoid claiming a shipped SaaS product

Validation:

- run or reference an existing fixture-backed review packet
- verify product-report template alignment
- check evidence and redaction requirements

### Architecture Cognition Roadmap

Scope:

- turn the architecture-cognition notes into a roadmap document
- separate alpha automation, structural intelligence, and cognitive governance
- identify first fitness functions

Validation:

- check against ADR 0025 and the v0.91.2 productization package
- mark speculative items as planned, not implemented

### Product Repo / Site Decision

Scope:

- decide where CodeFriend external-facing material lives
- define what must be ready before `CodeFriend.ai` is used publicly
- define what stays inside ADL as source proof and what moves to product docs

Validation:

- review redaction and publication boundaries
- verify no private local paths or unpublished customer-like claims leak

### CodeFriend Alpha Milestone Plan

Scope:

- author the `v0.93.x` CodeFriend alpha milestone package
- define WBS, sprint plan, feature docs, demo matrix, quality gate, and review
  tail
- set the alpha exit bar as "fully working alpha ready for testing"

Validation:

- compare the package against this planning home and v0.91.2 productization
  proof
- verify naming, evidence, redaction, and review boundaries
- review before issue seeding

### CodeFriend Post-Alpha Roadmap Plan

Scope:

- turn the post-alpha bands in this document into concrete future milestone
  candidates
- decide which features belong immediately after the alpha and which should wait
  until later product testing
- keep structural intelligence, executable governance, architectural memory,
  and product-delivery work separate enough to review

Validation:

- verify the roadmap does not overclaim alpha scope
- check against the CodeFriend notes, ADR 0025, and current feature list
- route any schedule conflicts back to milestone planning before issue seeding

## Non-Goals

- Do not implement the product app in this planning issue.
- Do not rename all internal `codebuddy` schemas or artifact paths in one pass.
- Do not claim CodeFriend is autonomous review authority.
- Do not depend on GWS as canonical CodeFriend infrastructure.
- Do not depend on C-SDLC completion as a prerequisite for the alpha review
  packet product lane.
- Do not use unfinished investigation notes as source truth.
- Do not treat scattered cleanup as a substitute for a dedicated CodeFriend
  alpha milestone.
- Do not treat the `v0.93.x` alpha as the final CodeFriend feature set.

## Review Checklist

- Current product naming uses `CodeFriend` / `CodeFriend.ai`.
- The planning docs live under `docs/planning/codefriend/`.
- Historical v0.91.2 proof links still resolve.
- Legacy `CodeBuddy` references are classified before migration.
- Product claims stay evidence-bound.
- Follow-on issues are bounded and do not hide implementation work in this
  planning issue.
- The plan frames CodeFriend alpha as a likely `v0.93.x` milestone with a
  working alpha ready for testing as the exit bar.
- The plan includes post-alpha roadmap bands for the rest of the CodeFriend
  feature set.
