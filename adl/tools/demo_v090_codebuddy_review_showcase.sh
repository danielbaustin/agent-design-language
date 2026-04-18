#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${1:-$ROOT_DIR/artifacts/v090/codebuddy_review_showcase}"

packet_dir="$OUT_DIR"
specialist_dir="$packet_dir/specialist_reviews"
diagram_dir="$packet_dir/diagrams"
test_dir="$packet_dir/test_recommendations"
issue_dir="$packet_dir/issue_planning"
adr_dir="$packet_dir/adr_candidates"
fitness_dir="$packet_dir/fitness_functions"

artifact_label="custom-artifact-root"
case "$OUT_DIR" in
  "$ROOT_DIR"/*)
    artifact_label="${OUT_DIR#"$ROOT_DIR"/}"
    ;;
esac

rm -rf "$OUT_DIR"
mkdir -p "$specialist_dir" "$diagram_dir" "$test_dir" "$issue_dir" "$adr_dir" "$fitness_dir"

cat >"$packet_dir/run_manifest.json" <<EOF_JSON
{
  "schema_version": "codebuddy.review_showcase.v1",
  "demo_id": "D8",
  "demo_name": "CodeBuddy multi-agent review showcase",
  "classification": "non_proving",
  "classification_reason": "Deterministic staged showcase packet; review-quality-evaluator remains pending in issue #2070.",
  "artifact_root": "$artifact_label",
  "review_target": "agent-design-language bounded self-review packet",
  "privacy_mode": "public_repo_fixture",
  "publication_allowed": false,
  "source_basis": [
    "CodeBuddy skill development plan",
    "CodeBuddy review template standard",
    "CodeBuddy review packet spec",
    "CodeBuddy skill and demo roadmap"
  ],
  "skill_lanes": [
    {"order": 1, "skill": "repo-packet-builder", "status": "represented"},
    {"order": 2, "skill": "repo-review-code", "status": "represented"},
    {"order": 3, "skill": "repo-review-security", "status": "represented"},
    {"order": 4, "skill": "repo-review-tests", "status": "represented"},
    {"order": 5, "skill": "repo-review-docs", "status": "represented"},
    {"order": 6, "skill": "repo-architecture-review", "status": "represented"},
    {"order": 7, "skill": "repo-dependency-review", "status": "represented"},
    {"order": 8, "skill": "repo-diagram-planner", "status": "represented"},
    {"order": 9, "skill": "diagram-author", "status": "represented"},
    {"order": 10, "skill": "architecture-diagram-reviewer", "status": "represented"},
    {"order": 11, "skill": "redaction-and-evidence-auditor", "status": "represented"},
    {"order": 12, "skill": "review-to-test-planner", "status": "represented"},
    {"order": 13, "skill": "finding-to-issue-planner", "status": "represented"},
    {"order": 14, "skill": "adr-curator", "status": "represented"},
    {"order": 15, "skill": "architecture-fitness-function-author", "status": "represented"},
    {"order": 16, "skill": "product-report-writer", "status": "represented"},
    {"order": 17, "skill": "review-quality-evaluator", "status": "staged_pending_2070"}
  ],
  "required_artifacts": [
    "repo_scope.md",
    "repo_inventory.json",
    "specialist_reviews/code.md",
    "specialist_reviews/security.md",
    "specialist_reviews/tests.md",
    "specialist_reviews/docs.md",
    "specialist_reviews/architecture.md",
    "specialist_reviews/dependencies.md",
    "diagrams/system_map.mmd",
    "diagrams/diagram_manifest.md",
    "diagrams/diagram_review.md",
    "redaction_report.md",
    "test_recommendations/test_gap_report.md",
    "issue_planning/issue_candidates.md",
    "adr_candidates/adr_candidates.md",
    "fitness_functions/fitness_function_plan.md",
    "final_report.md",
    "quality_evaluation.md",
    "demo_operator_result.json"
  ]
}
EOF_JSON

cat >"$packet_dir/repo_scope.md" <<'EOF_MD'
# Repo Scope

## Review Scope

- Repository: agent-design-language
- Ref / branch / diff: bounded fixture packet
- Review mode: staged multi-agent showcase
- Included paths:
  - README.md
  - demos/README.md
  - demos/v0.89/multi_agent_repo_code_review_demo.md
  - adl/tools/skills/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md
  - adl/tools/skills/repo-packet-builder/SKILL.md
  - adl/tools/skills/redaction-and-evidence-auditor/SKILL.md
  - adl/tools/skills/product-report-writer/SKILL.md
  - adl/tools/skills/workflow-conductor/SKILL.md
- Excluded paths:
  - private customer repositories
  - live provider credentials
  - generated build outputs
  - historical worktree residue
- Non-reviewed surfaces:
  - live web application code for codebuddy.ai
  - live GitHub mutation from generated issue candidates
  - external paid data or private repos
- Assumptions:
  - the demo is a fixture-backed ADL self-review rehearsal
  - staged lanes must be labeled explicitly
  - all customer-facing output must pass redaction and evidence-boundary review

## Review Questions

- Can a packet-first review flow preserve role boundaries and severity?
- Can the final report keep evidence, caveats, and residual risk visible?
- Can follow-through lanes propose tests, issues, ADRs, and fitness functions
  without mutating the repository silently?
EOF_MD

cat >"$packet_dir/repo_inventory.json" <<'EOF_JSON'
{
  "schema_version": "codebuddy.repo_inventory.fixture.v1",
  "repo_name": "agent-design-language",
  "languages": ["Rust", "Python", "Bash", "Markdown"],
  "primary_surfaces": [
    "adl/src",
    "adl/tools",
    "adl/tools/skills",
    "demos",
    "docs/milestones"
  ],
  "review_packet_policy": {
    "bounded": true,
    "private_repo": false,
    "mutation_allowed": false,
    "redaction_required_before_report": true
  },
  "specialist_assignments": {
    "code": ["adl/tools/skills", "adl/tools"],
    "security": ["credential boundaries", "artifact publication"],
    "tests": ["contract tests", "demo validators"],
    "docs": ["demos", "skills docs", "milestone docs"],
    "architecture": ["review packet flow", "skill orchestration"],
    "dependencies": ["Cargo.toml", "shell/python tooling"]
  }
}
EOF_JSON

cat >"$specialist_dir/code.md" <<'EOF_MD'
# Code Review: CodeBuddy Showcase Packet

## Review Scope

- Repository: agent-design-language
- Review mode: staged showcase packet
- Included paths: skill scripts, demo scripts, validators, review packet docs
- Excluded paths: live provider adapters and customer repos
- Assumptions: findings should be actionable without implying merge approval

## Findings

### Finding CB-CODE-001: [P2] Demo proof quality depends on validator coverage, not generated prose

- Role: code reviewer
- Confidence: High
- Affected path or artifact: demo validator and generated packet
- Trigger scenario: a future demo adds a role artifact but forgets to validate the role order, severity fields, or redaction gate
- Evidence: the packet relies on generated Markdown plus machine checks; without validator coverage, the prose can look complete while the proof contract drifts
- User/customer impact: reviewers may trust an impressive report shape that does not actually prove the expected review packet contract
- Recommended action: keep validator checks aligned with the review packet spec whenever a lane is added
- Validation or proof gap: live multi-agent execution is intentionally out of scope for this fixture-backed demo
- Related findings: CB-TEST-001

## Validation

- Commands run: validator and test wrapper in the issue validation phase
- What the commands proved: required artifact existence, schema shape, staged lane truth, report sections, and no private host-path leakage
- Commands not run and why: live provider review lanes were not run because this is a deterministic showcase packet

## Residual Risk

- Remaining uncertainty: live-model quality and false-positive burden are not measured here
- Areas needing deeper review: review-quality-evaluator once #2070 lands
- Follow-up skills recommended: review-quality-evaluator, product-report-writer

## Specialist Caveats

- Scope caveats: this is a self-review packet, not an external customer repo review
- Evidence caveats: evidence is fixture-backed and representative
- Model/tool caveats: no live model claims are made
EOF_MD

cat >"$specialist_dir/security.md" <<'EOF_MD'
# Security Review: CodeBuddy Showcase Packet

## Review Scope

- Repository: agent-design-language
- Review mode: staged showcase packet
- Included paths: publication gates, packet boundaries, generated reports
- Excluded paths: private repos, live secrets, provider request payloads
- Assumptions: publication requires redaction approval

## Findings

### Finding CB-SEC-001: [P1] Report publication must be blocked until redaction and evidence boundaries pass

- Role: security reviewer
- Confidence: High
- Affected path or artifact: final_report.md and redaction_report.md
- Trigger scenario: a customer-facing report is generated before host paths, secrets, prompt fragments, and evidence excerpts are checked
- Evidence: CodeBuddy's product value depends on packaging repo evidence for humans; that increases the impact of accidental private-data disclosure
- User/customer impact: private code metadata, internal paths, or sensitive context could leak into a deliverable
- Recommended action: require the redaction-and-evidence-auditor lane before product-report-writer output is publishable
- Validation or proof gap: this demo validates publication_allowed=false rather than testing every scanner pattern
- Related findings: CB-DOCS-001

## Validation

- Commands run: validator scans generated text for absolute host paths and secret-like markers
- What the commands proved: the fixture packet does not leak private host paths or obvious secret markers
- Commands not run and why: no live secret scanner was run because the packet is generated from controlled fixture text

## Residual Risk

- Remaining uncertainty: real customer repositories require stronger scanner coverage and manual redaction review
- Areas needing deeper review: provider exposure maps for live multi-provider reviews
- Follow-up skills recommended: redaction-and-evidence-auditor

## Specialist Caveats

- Scope caveats: this is a public-repo fixture
- Evidence caveats: scanner coverage is bounded to this demo's generated artifacts
- Model/tool caveats: no external provider received customer content
EOF_MD

cat >"$specialist_dir/tests.md" <<'EOF_MD'
# Test Review: CodeBuddy Showcase Packet

## Review Scope

- Repository: agent-design-language
- Review mode: staged showcase packet
- Included paths: demo script, validator, generated proof artifacts
- Excluded paths: full CI matrix and live provider regressions
- Assumptions: deterministic replay matters more than breadth for this demo

## Findings

### Finding CB-TEST-001: [P2] Demo claims should fail closed when staged lanes are mislabeled

- Role: test reviewer
- Confidence: High
- Affected path or artifact: run_manifest.json and quality_evaluation.md
- Trigger scenario: a staged lane such as review-quality-evaluator is presented as complete before its issue lands
- Evidence: #2070 is explicitly still underway, so the demo must classify that lane as staged_pending_2070
- User/customer impact: overclaiming makes the demo less trustworthy and could send reviewers looking for artifacts that do not exist yet
- Recommended action: validate staged lane status and non_proving classification until the lane is implemented
- Validation or proof gap: live quality scoring is deferred
- Related findings: CB-CODE-001

## Validation

- Commands run: test wrapper, validator, shell syntax checks
- What the commands proved: the generated packet is structurally complete and honest about staged status
- Commands not run and why: full batched checks are outside the narrow demo proof path

## Residual Risk

- Remaining uncertainty: future live skill runs may produce richer artifacts than this fixture packet
- Areas needing deeper review: evaluator calibration after #2070
- Follow-up skills recommended: review-quality-evaluator

## Specialist Caveats

- Scope caveats: tests prove artifact shape and truth classification, not review intelligence
- Evidence caveats: fixture-backed evidence cannot measure model accuracy
- Model/tool caveats: no live model evaluation was performed
EOF_MD

cat >"$specialist_dir/docs.md" <<'EOF_MD'
# Documentation Review: CodeBuddy Showcase Packet

## Review Scope

- Repository: agent-design-language
- Review mode: staged showcase packet
- Included paths: demo documentation, role script, final report, packet spec
- Excluded paths: product website copy and external customer docs
- Assumptions: demo readers need one coherent walkthrough

## Findings

### Finding CB-DOCS-001: [P2] The demo must read like a product workflow, not a bag of skill outputs

- Role: docs reviewer
- Confidence: High
- Affected path or artifact: demo doc and final_report.md
- Trigger scenario: reviewers see many specialist files but cannot tell how CodeBuddy turns them into a decision-ready report
- Evidence: CodeBuddy's source plans emphasize product-grade review packets, final reports, and third-party-review-like quality
- User/customer impact: the demo may look like orchestration plumbing rather than a credible review product
- Recommended action: keep the final report findings-first, scope-explicit, severity-ranked, and caveated
- Validation or proof gap: human review still determines whether the report is persuasive
- Related findings: CB-SEC-001

## Validation

- Commands run: validator checks final report sections and required role artifacts
- What the commands proved: report structure matches the expected packet spec
- Commands not run and why: no external editorial review was performed

## Residual Risk

- Remaining uncertainty: report tone and usefulness still need human review
- Areas needing deeper review: customer-facing templates after live product work starts
- Follow-up skills recommended: product-report-writer, review-quality-evaluator

## Specialist Caveats

- Scope caveats: demo docs are not product onboarding docs
- Evidence caveats: final report examples are fixture-backed
- Model/tool caveats: no external reviewer scored the packet
EOF_MD

cat >"$specialist_dir/architecture.md" <<'EOF_MD'
# Architecture Review: CodeBuddy Showcase Packet

## Review Scope

- Repository: agent-design-language
- Review mode: staged showcase packet
- Included paths: packet builder, specialist lanes, diagram gates, report pipeline
- Excluded paths: future web app runtime and billing/account systems
- Assumptions: CodeBuddy remains packet-first and human-approved

## Findings

### Finding CB-ARCH-001: [P2] Follow-through lanes must stay downstream of synthesis and redaction gates

- Role: architecture reviewer
- Confidence: High
- Affected path or artifact: review pipeline ordering
- Trigger scenario: issue creation, test planning, ADR drafting, or fitness-function authoring runs before findings are synthesized and publication boundaries are checked
- Evidence: the review packet standard requires specialist findings, residual risk, diagram truth boundaries, redaction, and report synthesis to remain visible
- User/customer impact: premature follow-through can turn review into unsafe repo mutation or hide unresolved specialist disagreement
- Recommended action: preserve packet -> specialist review -> synthesis -> diagram/redaction -> follow-through -> report ordering
- Validation or proof gap: this demo records ordering but does not enforce runtime scheduling
- Related findings: CB-SEC-001, CB-TEST-001

## Validation

- Commands run: manifest lane-order validation
- What the commands proved: the showcase packet records an explicit lane order and staged evaluator truth
- Commands not run and why: scheduler/runtime enforcement is outside this issue

## Residual Risk

- Remaining uncertainty: real multi-agent execution needs durable orchestration state
- Areas needing deeper review: long-lived CodeBuddy review sessions
- Follow-up skills recommended: architecture-fitness-function-author, adr-curator

## Specialist Caveats

- Scope caveats: this is architecture of the review workflow, not product deployment architecture
- Evidence caveats: ordering proof is artifact-level
- Model/tool caveats: no runtime scheduler was invoked
EOF_MD

cat >"$specialist_dir/dependencies.md" <<'EOF_MD'
# Dependency Review: CodeBuddy Showcase Packet

## Review Scope

- Repository: agent-design-language
- Review mode: staged showcase packet
- Included paths: shell, Python, Rust tooling surfaces named by the packet
- Excluded paths: live SaaS integrations, external package updates, product deployment dependencies
- Assumptions: demo dependencies should stay local and inexpensive

## Findings

### Finding CB-DEP-001: [P3] Demo should avoid expensive or network-only dependencies

- Role: dependency reviewer
- Confidence: High
- Affected path or artifact: demo command and validator
- Trigger scenario: the showcase requires paid model calls, customer accounts, or live data feeds just to prove packet shape
- Evidence: the issue requires execution or rehearsal without product-app code, billing, customer accounts, or external private repos
- User/customer impact: reviewers cannot rerun the demo reliably, and CI cannot protect the proof surface
- Recommended action: keep the default path fixture-backed with only Bash and Python standard-library validation
- Validation or proof gap: live-provider quality remains a separate proof path
- Related findings: CB-TEST-001

## Validation

- Commands run: shell and Python validator checks
- What the commands proved: the default demo path has no network or paid-service dependency
- Commands not run and why: dependency upgrades are out of scope

## Residual Risk

- Remaining uncertainty: future live proof paths will need provider/version recording
- Areas needing deeper review: model and renderer dependency matrix
- Follow-up skills recommended: repo-dependency-review

## Specialist Caveats

- Scope caveats: dependency review is limited to demo execution needs
- Evidence caveats: no lockfile diff was reviewed
- Model/tool caveats: live model packages are not part of this demo
EOF_MD

cat >"$diagram_dir/system_map.mmd" <<'EOF_MMD'
flowchart LR
  Packet["repo-packet-builder\nbounded review packet"] --> Specialists["specialist reviewers\ncode security tests docs architecture dependencies"]
  Specialists --> Synthesis["repo-review-synthesis\nseverity and disagreement preserved"]
  Synthesis --> DiagramPlan["repo-diagram-planner\nsource-backed diagram briefs"]
  DiagramPlan --> DiagramAuthor["diagram-author\nMermaid/D2/PlantUML sources"]
  DiagramAuthor --> DiagramReview["architecture-diagram-reviewer\ntruth boundary gate"]
  Synthesis --> Redaction["redaction-and-evidence-auditor\npublication gate"]
  DiagramReview --> Report["product-report-writer\ncustomer-grade report"]
  Redaction --> Report
  Synthesis --> FollowThrough["test, issue, ADR, and fitness planners\nhuman-approved follow-through"]
  FollowThrough --> Report
  Report --> Quality["review-quality-evaluator\nstaged pending issue 2070"]
EOF_MMD

cat >"$diagram_dir/diagram_manifest.md" <<'EOF_MD'
# Diagram Manifest

## Diagram Source

- Source path: diagrams/system_map.mmd
- Diagram family: workflow / architecture overview
- Renderer status: source generated, rendering optional

## Source-Backed Elements

- Packet builder lane
- Specialist reviewer fan-out
- Severity-preserving synthesis
- Diagram planning and diagram review gates
- Redaction gate before report publication
- Follow-through planners before final report packaging
- Staged quality evaluator lane

## Assumptions And Unknowns

- Assumption: the demo presents the intended CodeBuddy workflow order rather
  than a live runtime schedule.
- Unknown: exact review-quality-evaluator scoring contract remains pending
  until #2070 lands.

## Unsupported Claims Added

- None. The staged lane is explicitly marked as staged.
EOF_MD

cat >"$diagram_dir/diagram_review.md" <<'EOF_MD'
# Architecture Diagram Review

## Review Scope

- Diagram source path: diagrams/system_map.mmd
- Diagram family: workflow / architecture overview
- Review mode: source-grounded fixture review

## Findings

No blocking findings.

## Truth Boundary

The diagram is valid as a workflow intent map for the staged CodeBuddy showcase.
It does not claim live runtime scheduling, live model execution, or completed
quality-evaluator behavior.

## Renderer Status

The Mermaid source is generated and suitable for local rendering. Rendering is
not required for this fixture-backed proof packet.
EOF_MD

cat >"$packet_dir/redaction_report.md" <<'EOF_MD'
# Redaction And Evidence Audit

## Publication Mode

- Mode: internal showcase packet
- Publication allowed: false
- Reason: fixture-backed demo with staged review-quality-evaluator lane

## Checks

- Secret scan result: pass for generated fixture text
- Private host path scan result: pass for generated fixture text
- Source disclosure boundary: public ADL repo surfaces only
- Provider/model exposure map: no live provider calls
- Customer-data handling caveats: no customer data included

## Final Publication Status

Partial. The packet is reviewable as an internal demo artifact, but should not
be presented as a completed live CodeBuddy customer review.
EOF_MD

cat >"$test_dir/test_gap_report.md" <<'EOF_MD'
# Test Recommendation Report

## Recommendation CB-TEST-PLAN-001

- Linked finding: CB-TEST-001
- Behavior under test: staged lanes remain labeled as staged and keep the demo
  classified as non_proving
- Suggested test location: adl/tools/test_demo_v090_codebuddy_review_showcase.sh
- Fixture needs: generated demo packet in a temporary artifact root
- Expected assertion: review-quality-evaluator has status staged_pending_2070
- Validation command: bash adl/tools/test_demo_v090_codebuddy_review_showcase.sh
- Generation status: generated

## Recommendation CB-TEST-PLAN-002

- Linked finding: CB-SEC-001
- Behavior under test: generated artifacts do not leak private host paths or
  secret-like markers
- Suggested test location: adl/tools/validate_codebuddy_review_showcase_demo.py
- Fixture needs: generated demo packet
- Expected assertion: validator rejects private path and secret marker leakage
- Validation command: python3 adl/tools/validate_codebuddy_review_showcase_demo.py <artifact-root>
- Generation status: generated
EOF_MD

cat >"$issue_dir/issue_candidates.md" <<'EOF_MD'
# Issue Candidates

## Candidate 1: Keep CodeBuddy demo validators aligned with review packet spec

- Source finding: CB-CODE-001
- Severity: P2
- Evidence: generated review prose requires machine checks for role order,
  staged lane truth, redaction gates, and required final report sections
- Acceptance criteria:
  - validator checks all required packet files
  - validator rejects missing staged-lane truth
  - validator rejects private host-path leakage
- Non-goals:
  - live provider evaluation
  - customer repository mutation
- Human approval required: yes

## Candidate 2: Add live review-quality evaluation after #2070 lands

- Source finding: CB-TEST-001
- Severity: P2
- Evidence: review-quality-evaluator is staged pending #2070
- Acceptance criteria:
  - demo can record an evaluator output artifact
  - classification can move from non_proving to proving only when the live lane
    actually runs
- Non-goals:
  - claiming evaluator quality before implementation
- Human approval required: yes
EOF_MD

cat >"$adr_dir/adr_candidates.md" <<'EOF_MD'
# ADR Candidates

## ADR-CANDIDATE-001: Packet-first CodeBuddy reviews

- Status: proposed
- Context: specialist roles need shared evidence boundaries, consistent scope,
  and reusable artifacts.
- Decision: CodeBuddy reviews should start from a bounded repo packet before
  specialist review, diagrams, redaction, test planning, issue planning, ADR
  drafting, or product-report writing.
- Consequences:
  - reviewers share a common source of truth
  - exclusions and residual risk remain visible
  - live repo mutation remains opt-in and human-approved
- Supersedes: none
- Acceptance boundary: this candidate is not accepted by the demo.
EOF_MD

cat >"$fitness_dir/fitness_function_plan.md" <<'EOF_MD'
# Architecture Fitness Function Plan

## Candidate Invariant 1

- Invariant: final reports must not be publishable unless a redaction report is
  present and publication_allowed is explicit
- Machine-checkable: yes
- Suggested check: validate final_report.md and redaction_report.md existence
  plus run_manifest.json publication fields
- Expected failure mode: missing redaction artifact or ambiguous publication
  status fails the packet validator

## Candidate Invariant 2

- Invariant: staged lanes must be labeled as staged, not represented as complete
- Machine-checkable: yes
- Suggested check: require review-quality-evaluator status staged_pending_2070
  until the skill lands
- Expected failure mode: validator fails when staged truth is absent

## Human-Judgment Candidate

- Invariant: report usefulness should resemble a high-quality third-party review
- Machine-checkable: partial
- Suggested review: use review-quality-evaluator after #2070 lands
EOF_MD

cat >"$packet_dir/final_report.md" <<'EOF_MD'
# CodeBuddy Review Report: ADL Self-Review Showcase

## Executive Summary

- Overall risk: medium for demo maturity, low for repository mutation risk
- Top risks:
  - staged lanes can be overclaimed if the manifest and validator drift
  - publication must remain blocked until redaction and evidence gates pass
  - follow-through planners must not mutate repos without human approval
- Recommended remediation sequence:
  1. keep validator coverage aligned with the packet spec
  2. land review-quality-evaluator and replace the staged lane with a live artifact
  3. add live provider execution as a separate, explicitly gated proof path
- Publication/privacy status: internal showcase only, publication_allowed=false

## Review Scope

- Repository: agent-design-language
- Review mode: staged multi-agent showcase
- Included paths: demo docs, skill suite docs, review packet surfaces, generated artifacts
- Excluded paths: private repos, customer code, live provider credentials
- Non-reviewed surfaces: codebuddy.ai product app and billing/account systems
- Assumptions: artifact shape and role boundaries are the proof target

## Top Findings

### Finding CB-SEC-001: [P1] Report publication must be blocked until redaction and evidence boundaries pass

- Source role: security reviewer
- Confidence: High
- Evidence: final reports package repo evidence for human consumption and can leak sensitive data without a gate
- Impact: private metadata or source context could reach a customer-facing artifact
- Recommended action: keep redaction-and-evidence-auditor before product-report-writer publication
- Validation gap: fixture scanner coverage is not a substitute for live customer redaction review

### Finding CB-ARCH-001: [P2] Follow-through lanes must stay downstream of synthesis and redaction gates

- Source role: architecture reviewer
- Confidence: High
- Evidence: issue, test, ADR, and fitness-function lanes depend on synthesized findings and clear publication boundaries
- Impact: premature follow-through could mutate repos or hide unresolved disagreement
- Recommended action: preserve packet -> specialist review -> synthesis -> gates -> follow-through -> report ordering
- Validation gap: runtime scheduling enforcement is deferred

### Finding CB-TEST-001: [P2] Demo claims should fail closed when staged lanes are mislabeled

- Source role: test reviewer
- Confidence: High
- Evidence: review-quality-evaluator is pending in #2070
- Impact: overclaiming would make the demo less trustworthy
- Recommended action: keep non_proving classification until the lane is actually implemented and run
- Validation gap: live quality scoring is deferred

## Architecture Summary

- Major components: packet builder, specialist reviewers, synthesis, diagram
  planner/author/reviewer, redaction gate, follow-through planners, product
  report writer, staged quality evaluator
- Boundaries: no customer repo mutation, no live provider calls, no publication
  without redaction approval
- State/data/control-flow notes: one bounded packet flows forward; findings
  retain severity, confidence, evidence, and residual risk
- Architecture risks: staged lanes and follow-through planners need explicit
  truth labels
- Diagram links: diagrams/system_map.mmd and diagrams/diagram_review.md

## Security And Privacy Notes

- Trust boundaries: generated packet versus customer-facing report
- Assets: repo evidence, findings, diagrams, recommendations, issue candidates
- Abuse paths: accidental publication, host-path leakage, silent tracker mutation
- Redaction result: partial/pass for internal fixture packet, publication blocked

## Test Recommendations

- Linked finding: CB-TEST-001
- Behavior under test: staged lane truth and non_proving classification
- Suggested test location: adl/tools/test_demo_v090_codebuddy_review_showcase.sh
- Generation status: generated
- Validation command: bash adl/tools/test_demo_v090_codebuddy_review_showcase.sh

## Documentation And Onboarding Notes

- Docs drift: keep demo docs, packet spec, and validation checks synchronized
- Stale commands: none observed in the generated packet
- Missing operator guidance: live-provider proof remains a future, explicit gate

## Remediation Sequence

1. Maintain strict validator coverage for the generated packet.
2. Add the review-quality-evaluator artifact after #2070 lands.
3. Introduce a separate live multi-agent proof path only after redaction,
   provider, and customer-approval boundaries are explicit.

## Residual Risks

- Remaining uncertainty: live model quality and false-positive burden are not
  measured by this fixture
- Specialist disagreements: none unresolved in the staged packet
- Non-reviewed surfaces: product web app, billing, customer repository mutation,
  external provider execution

## Caveats

- This is a non_proving showcase packet, not a completed customer review.
- The staged quality-evaluator lane must not be treated as implemented until
  #2070 lands and produces a real artifact.
EOF_MD

cat >"$packet_dir/quality_evaluation.md" <<'EOF_MD'
# Review Quality Evaluation

## Status

Staged pending #2070.

## Evaluation Boundary

The demo records the intended evaluation lane but does not score the report yet.
This preserves truth while `review-quality-evaluator` is underway.

## Expected Future Checks

- severity calibration
- evidence sufficiency
- residual-risk honesty
- redaction gate presence
- follow-through safety
- report usefulness compared with third-party review expectations
EOF_MD

cat >"$packet_dir/demo_operator_result.json" <<'EOF_JSON'
{
  "schema_version": "adl.demo_operator_result.v1",
  "demo_name": "codebuddy_multi_agent_review_showcase",
  "command": "bash adl/tools/demo_v090_codebuddy_review_showcase.sh",
  "classification": "non_proving",
  "classification_reason": "The packet is deterministic and complete for a showcase rehearsal, but one lane remains staged pending issue #2070 and no live multi-agent execution was run.",
  "prerequisite_state": {
    "requires_live_provider": false,
    "requires_customer_repo": false,
    "requires_product_app": false,
    "review_quality_evaluator": "staged_pending_2070"
  },
  "produced_artifacts": [
    "run_manifest.json",
    "repo_scope.md",
    "repo_inventory.json",
    "specialist_reviews/",
    "diagrams/",
    "redaction_report.md",
    "test_recommendations/",
    "issue_planning/",
    "adr_candidates/",
    "fitness_functions/",
    "final_report.md",
    "quality_evaluation.md"
  ],
  "follow_up_recommendation": "After #2070 lands, rerun the showcase with a real review-quality-evaluator artifact and consider reclassifying only if the live lane executes."
}
EOF_JSON

cat >"$packet_dir/README.md" <<'EOF_MD'
# CodeBuddy Multi-Agent Review Showcase Artifacts

Start with `run_manifest.json`, then read `repo_scope.md`, specialist reviews,
`redaction_report.md`, `final_report.md`, and `demo_operator_result.json`.

This packet is intentionally classified as `non_proving` until the staged
review-quality-evaluator lane from #2070 is implemented and run.
EOF_MD

python3 "$ROOT_DIR/adl/tools/validate_codebuddy_review_showcase_demo.py" "$OUT_DIR" >/dev/null

echo "CodeBuddy review showcase demo artifacts:"
echo "  $artifact_label/run_manifest.json"
echo "  $artifact_label/final_report.md"
echo "  $artifact_label/demo_operator_result.json"
