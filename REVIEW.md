# ADL Review Guide

**Purpose:** This document gives a reviewer, human or AI, a consistent way to review ADL without freezing the project into one exact repo shape. It captures stable review principles, the invariants ADL cares about most, recurring historical failure modes, and a practical structure for the final review output.

This guide is intentionally:

- stable in principles
- milestone-relative in judgment
- cautious about repo-shape assumptions
- strict about truthfulness, determinism, and reviewability

This guide is **not** a substitute for the milestone's own claimed scope. A correct ADL review always asks:

1. What did this milestone claim it would deliver?
2. What invariants were already established by prior milestones?
3. Which changes are acceptable evolution, and which are regressions?

The reviewer should not audit ADL against a frozen abstract standard alone. The reviewer should determine whether the repository **truthfully satisfies the milestone's own claims while preserving core ADL invariants**.

---

## Current Review Entry Point

For the active v0.90.1 WP-13 documentation and review-readiness pass, start
with:

- `docs/milestones/v0.90.1/README.md`
- `docs/milestones/v0.90.1/WBS_v0.90.1.md`
- `docs/milestones/v0.90.1/SPRINT_v0.90.1.md`
- `docs/milestones/v0.90.1/DEMO_MATRIX_v0.90.1.md`
- `docs/milestones/v0.90.1/FEATURE_DOCS_v0.90.1.md`
- `docs/milestones/v0.90.1/MILESTONE_CHECKLIST_v0.90.1.md`
- `docs/milestones/v0.90.1/RELEASE_PLAN_v0.90.1.md`
- `docs/milestones/v0.90.1/RELEASE_NOTES_v0.90.1.md`
- `docs/milestones/v0.90.1/WP_ISSUE_WAVE_v0.90.1.yaml`
- `docs/planning/ADL_FEATURE_LIST.md`
- `CHANGELOG.md`
- `README.md`
- `adl/Cargo.toml`
- `adl/Cargo.lock`

The current v0.90.1 review posture is pre-release review copy. WP-01 through
WP-12 are closed, WP-13 is aligning the documentation package, WP-14 quality
gate, WP-15 internal review, WP-15A third-party review, WP-16 remediation, and
the release tail remain ahead.

The crate version remains `0.90.0` until the v0.90.1 release bump. Reviewers
should treat that as a pre-release version boundary, not as evidence that the
active v0.90.1 work is absent.

For the most recently completed v0.90 post-release review, start with:

- `docs/milestones/v0.90/README.md`
- `docs/milestones/v0.90/V090_PRE_THIRD_PARTY_READINESS_REPORT.md`
- `docs/milestones/v0.90/DEMO_MATRIX_v0.90.md`
- `docs/milestones/v0.90/MILESTONE_CHECKLIST_v0.90.md`
- `docs/milestones/v0.90/RELEASE_NOTES_v0.90.md`
- `CHANGELOG.md`
- `README.md`
- `adl/Cargo.toml`
- `adl/Cargo.lock`

The current v0.90 review posture is final release copy. Runtime, demo, sidecar,
coverage, Rust refactor, docs, internal-review work, third-party review,
accepted findings remediation, next planning, and release ceremony preparation
have landed; the release ceremony script created the tag and GitHub release.

Current tracker values to preserve in review:

- coverage: `92.40%` workspace line coverage, rounded to the intended `93%`
  tranche; WP-10 also recorded `92.46%`
- Rust watch list: one `RATIONALE`, nineteen `REVIEW`, and fourteen `WATCH`
  items after the WP-14 child split wave
- closeout: the release ceremony closed-issue truth gate passes for v0.90

---

## 1. What ADL Is

ADL is a deterministic, contract-driven orchestration system for AI workflows, implemented primarily in Rust.

At a high level, ADL aims to provide:

- deterministic or bounded-repeatable execution semantics where claimed
- explicit workflow contracts and reviewable artifacts
- bounded execution and policy surfaces rather than implicit agent behavior
- durable proof surfaces for inspection, replay, and review
- a repository and runtime that can be evaluated as engineering systems rather than as prompt theater

Reviewers should treat ADL as an engineering substrate, not as a generic "AI agent framework."

### Stable Architectural Themes

Across milestones, ADL commonly centers on:

- execution semantics
- provider abstraction
- policy and safety boundaries
- signing and trust surfaces
- trace and artifact emission
- review surfaces
- operational tooling and workflow control planes
- milestone-bounded demos and proof packages

The exact implementation boundaries may change by milestone. The review should evaluate whether the current milestone preserves clarity and truth across those themes.

---

## 2. Review Model

The correct ADL review model is:

- review the actual repository as it exists now
- anchor judgment in the active milestone package
- compare implementation, docs, demos, and review artifacts against each other
- distinguish shipped work from planned work
- distinguish bounded proof from broader aspiration

The reviewer should not assume:

- every architecture note is a shipped commitment
- every future-facing doc is part of the active milestone
- every review surface is intended to be canonical for all future milestones

The reviewer should verify:

- what is current tracked truth
- what is local planning state
- what the milestone explicitly claims
- whether those claims are honestly satisfied

---

## 3. Core Standards

These are the review standards that matter across ADL milestones.

### 3.1 Version Consistency

The runtime/package version source of truth must be internally coherent.

Review for:

- manifest version and lockfile alignment
- root README release/milestone statements
- milestone-doc version references
- changelog/release-note consistency

Do not assume one exact set of files forever. Check the current repo's actual release surfaces and see whether they agree.

### 3.2 Determinism and Boundedness

ADL makes strong claims about determinism, bounded behavior, and reviewable execution. Reviewers should check whether those claims are:

- explicitly scoped
- supported by code and artifacts
- not overstated in docs or review outputs

Any new behavior that introduces uncontrolled non-determinism, hidden mutable state, unstable ordering, or unbounded side effects is high risk unless explicitly isolated and documented.

### 3.3 Security and Trust Boundaries

The following surfaces are especially sensitive and must not regress:

- sandboxing and path safety
- signing and verification
- provider and remote-execution trust boundaries
- delegation boundaries
- policy or learning guardrails

The review should focus on whether these boundaries remain explicit, enforced, and reviewable.

### 3.4 Coverage and Validation Discipline

ADL uses strong quality-gate expectations. Reviewers should verify:

- the currently documented coverage thresholds
- whether CI and local validation surfaces agree
- whether exclusions are justified
- whether risky code paths are actually exercised

Do not score from memory. Use the current quality-gate surface for the milestone being reviewed.

### 3.5 Documentation and Review-Surface Truthfulness

ADL depends heavily on docs, milestone records, demos, and review surfaces. Reviewers should check whether:

- milestone docs match implementation truth
- demo matrices match runnable proof surfaces
- checklist/release surfaces are honest about remaining work
- output artifacts do not overclaim validation
- future planning docs are not presented as shipped work

### 3.6 File Growth and Modularity

Large files and mixed-responsibility modules are recurring ADL risks.

The review should check:

- unusually large source files
- test files that have become dumping grounds
- modules that mix many responsibilities
- refactors that only partially completed modularization

There is no magic number that applies forever, but rapid file growth and concentrated responsibility are always review-relevant.

### 3.7 Release Discipline

Before a milestone is called release-ready, reviewers should confirm:

- quality gates are passing or explicitly dispositioned
- milestone docs are coherent
- demos and proof surfaces are navigable
- review findings are either fixed or explicitly deferred
- release notes / changelog / release plan are aligned

---

## 4. Stable Review Questions

These questions apply to nearly every ADL review.

### 4.1 Milestone Truth

- What does the active milestone say it delivers?
- Which items are complete, in review, deferred, or still planned?
- Do docs and demos tell the same story?

### 4.2 Behavioral Correctness

- Does the code preserve the claimed execution semantics?
- Are error paths, retries, cancellation, and partial-failure behavior handled cleanly?
- Are state transitions explicit and testable?

### 4.3 Determinism and Replay

- Are ordering guarantees stable where claimed?
- Are traces/artifacts replayable or bounded-repeatable where claimed?
- Are claims about determinism properly scoped rather than universalized?

### 4.4 Trust and Safety

- Do trust-boundary checks still happen before sensitive execution?
- Is untrusted input normalized and validated?
- Is security-sensitive behavior observable and test-covered?

### 4.5 Reviewability

- Could an uninvolved reviewer find the relevant proof surfaces?
- Do demo/review packages point to real artifacts?
- Are the repository's entry surfaces coherent and navigable?

### 4.6 Engineering Discipline

- Is CI green or truthfully dispositioned?
- Are temporary or prototype surfaces leaking into canonical docs?
- Do manifests, lockfiles, and release surfaces agree with the code?

---

## 5. Recommended Review Process

This is the preferred review sequence.

### Step 1: Establish Milestone Context

Identify:

- the active version or milestone under review
- the canonical milestone README / design / WBS / sprint / checklist / release surfaces
- the relevant demo matrix or equivalent proof index
- any prior review findings or remediation docs

The milestone package defines the review target. Start there before deep code inspection.

### Step 2: Establish Current Repo Truth

Identify the current:

- release/version surfaces
- quality-gate surfaces
- review-entry surfaces
- demo/proof entry surfaces
- current active issue/PR state if reviewing a pre-release milestone

Do not assume file names from older milestones. Use the repo's actual tracked surfaces.

### Step 3: Inspect High-Risk Technical Surfaces

Prioritize:

- top-level manifests and lockfiles
- core runtime modules
- trust-boundary and security-sensitive code
- major new modules introduced by the milestone
- largest source files and largest test files
- validation and CI surfaces

### Step 4: Inspect Milestone Proof Surfaces

Review:

- milestone docs
- demo matrix / demo package
- quality-gate artifacts
- internal/external review surfaces
- release-tail docs

Check for contradictions between code, docs, demos, and reviewer-facing claims.

### Step 5: Compare Against Prior Review History

Identify:

- prior P0/P1 findings
- recurring risk areas
- whether known problems were actually fixed
- whether new work silently reintroduced previously solved problems

### Step 6: Produce Findings First

The review output should prioritize:

- correctness problems
- release blockers
- security regressions
- broken or misleading review surfaces
- milestone-truth drift

Style-only comments should not dominate the review.

---

## 6. Typical Discovery Commands

These are examples, not mandatory fixed commands.

Use the repo's current layout and tooling first.

Typical categories:

- version/manifests:
  - inspect `Cargo.toml`, `Cargo.lock`, root `README.md`, release notes, changelog
- large-file audit:
  - list large Rust source files and large test files
- milestone package audit:
  - inspect the active milestone doc directory and its canonical entry docs
- validation audit:
  - inspect the repo's current quality-gate and CI surfaces
- demo/proof audit:
  - inspect the milestone demo matrix and the current proof/review entrypoints
- ADR audit:
  - inspect `docs/architecture/adr/` and compare major new architectural commitments against ADR coverage

If command examples are included in a future canonical version of this guide, they should be clearly marked as:

- current repo examples
- not universal truths

---

## 7. Recurring Review Risks

These are historically important in ADL and deserve extra scrutiny.

### High-Signal Historical Risks

- version drift between manifests, lockfiles, README, and release surfaces
- milestone docs claiming more than the implementation/proof actually supports
- stale reviewer-entry docs after milestone structure changes
- large files regrowing after earlier refactors
- quality-gate claims not matching actual validation surfaces
- output/review records overclaiming determinism or validation
- local planning or prototype surfaces being mistaken for canonical tracked docs

### Ongoing Watch Areas

- runtime and instrumentation modules that accrete too much responsibility
- large test files that become difficult to review safely
- provider/runtime/demo surfaces drifting apart
- release-tail docs lagging behind milestone completion state
- path leakage, artifact-path drift, or review packages pointing to the wrong roots

These are watch areas, not automatic findings. The reviewer should confirm current truth rather than rely on historical memory alone.

---

## 8. Review Rubric

This rubric is intended to help structure judgment, not replace it.

### Categories

The review should explicitly evaluate all of the following dimensions:

1. Response to prior review
2. Milestone achievement
3. Security and trust-boundary integrity
4. Feature/runtime correctness
5. Code quality
6. Test quality
7. Code organization and modularity
8. Refactoring quality
9. Architecture quality
10. Documentation quality
11. Document consistency and review-surface truthfulness
12. Demo/proof quality
13. Release readiness
14. Engineering discipline and professionalism
15. Attention to detail

These categories are intentionally not perfectly independent. They are a review scaffold.

### Calibration

Use the rubric to answer:

- Is the milestone truthful?
- Is it technically sound?
- Is it reviewer-usable?
- Is it release-ready?

Do not inflate scores because the project is ambitious.
Do not deflate scores because the project is solo.

Judge the work honestly against its claims and invariants.

---

## 9. Priority Definitions

Use these priority bands consistently.

- `P0`
  - release blocker
  - severe correctness or security failure
  - broken version/release state
  - CI/quality-gate failure that invalidates release readiness
- `P1`
  - should fix before release
  - major milestone-truth drift
  - missing or misleading proof surface for a claimed capability
  - missing trust-boundary enforcement or major validation gap
- `P2`
  - important but not release-blocking
  - maintainability risk
  - modularity drift
  - reviewer friction that does not invalidate milestone truth
- `P3`
  - lower-severity improvement
  - useful cleanup or clarification with concrete value

---

## 10. Standard Review Output

The standard ADL external-style review output should be produced as a structured long-form review document and, when the review is being finalized for delivery, rendered to PDF.

The review should be findings-first, but it should also include the full structured package below.

### Standard 15-Part Review Structure

Use this section order unless there is a strong reason to deviate:

1. Executive Summary
2. Response to Prior Review
3. Milestone Achievement
4. Security Review
5. Feature / Runtime Correctness
6. Code Quality
7. Test Quality
8. Code Organization and Modularity
9. Refactoring Quality
10. Architecture Quality
11. Documentation Quality
12. Document Consistency and Review-Surface Truthfulness
13. Demo / Proof / Validation Surface Quality
14. Release Readiness, Engineering Discipline, and Professionalism
15. Conclusion and Recommendation

This 15-part structure is the default expected review shape.

### Required Content Inside the Review

The final review should include:

- prioritized findings with severity
- concrete evidence and affected files/surfaces
- explicit discussion of security as a top-level section, not a footnote
- explicit discussion of test quality, not just coverage numbers
- explicit discussion of code quality and refactoring quality
- explicit discussion of document consistency and review-surface truthfulness
- explicit discussion of architecture quality
- explicit discussion of professionalism and attention to detail
- milestone-readiness judgment
- open questions or assumptions
- validation performed

### Output Expectations

When preparing the final reviewer-facing deliverable:

- produce the review in the standard structured document form first
- render to PDF when a polished external/shareable review artifact is needed
- ensure the PDF matches the written source and preserves the 15-part structure

If a scorecard is used, it should support the written judgment rather than replace it.

---

## 11. Tone and Calibration

Reviews should be:

- direct
- specific
- evidence-based
- historically aware
- fair about what was actually claimed

Good ADL reviews do all of the following:

- call out real problems clearly
- acknowledge strong engineering where earned
- avoid style-nit inflation
- distinguish regressions from planned future work
- preserve the difference between "not yet implemented" and "incorrectly claimed as implemented"

This project often aims at unusually high rigor for a small team. Respect that rigor, but do not soften real findings.

---

## 12. Living-Document Rule

This guide should evolve after real review cycles.

Update it when:

- a new recurring failure mode appears
- a formerly stable assumption is no longer true
- the milestone/review model changes
- a better review pattern becomes clear

When updating it, prefer:

- stable principles over brittle repo trivia
- milestone-relative judgment over frozen checklists
- current examples clearly labeled as examples
