# Multi-Agent Repo Review Skill Suite

## Purpose

Provide a modular review suite for repository-wide or large-slice review. The
suite decomposes review into specialist roles and a synthesis role instead of
asking one monolithic prompt to absorb every concern.

Use the existing `repo-code-review` skill when one bounded monolithic review is
enough. Use this suite when the operator wants distinct role artifacts, deeper
coverage, or an explicit multi-agent review demo/proof surface.

## Skills

- `repo-review-code`
- `repo-review-security`
- `repo-review-tests`
- `repo-review-docs`
- `repo-architecture-review`
- `repo-dependency-review`
- `repo-review-synthesis`
- `repo-diagram-planner`
- `architecture-diagram-reviewer`
- `review-to-test-planner`
- `gap-analysis`
- `refactoring-helper`
- `adr-curator`
- `architecture-fitness-function-author`
- `finding-to-issue-planner`
- `product-report-writer`
- `review-quality-evaluator`

## Invocation Order

Recommended order:

1. `repo-review-code`
2. `repo-review-security`
3. `repo-review-tests`
4. `repo-review-docs`
5. `repo-architecture-review`
6. `repo-dependency-review`
7. `repo-review-synthesis`
8. `repo-diagram-planner`
9. `architecture-diagram-reviewer`
10. `review-to-test-planner`
11. `gap-analysis`
12. `refactoring-helper`
13. `adr-curator`
14. `architecture-fitness-function-author`
15. `finding-to-issue-planner`
16. `product-report-writer`
17. `review-quality-evaluator`

The first four roles may run independently when the operator wants parallel
review. The suite treats documentation review as required for all internal and
external review packets unless a lane is explicitly skipped.

For this suite, the synthesis role must run after at least one specialist
artifact is available, and ideally after all required roles have reported.
When a required role is intentionally skipped, record an explicit skip reason,
the owner of that skip decision, and a follow-up path in the synthesis record.
The diagram planner should normally run after packet building and after the
specialist artifacts that may propose diagram work. It emits bounded task briefs
for `diagram-author`; it is not itself a review or diagram-authoring lane.
The architecture diagram reviewer should run after `diagram-author` has produced
a diagram packet or source set. It is a source-grounded quality gate that checks
diagram truth, renderability evidence, assumptions, unknowns, and correction
handoffs without authoring or rendering diagrams.
The review-to-test planner should run after specialist findings or synthesis
exist. It maps findings to safe, bounded `test-generator` handoffs without
writing tests or turning review follow-up into broad implementation work.
The gap-analysis skill should run when a concrete expected baseline must be
reconciled against observed implementation, docs, tests, review, report, PR, or
closeout evidence. It emits source-grounded gap findings and stops before fixes,
approval, publication, issue creation, PR creation, or repository mutation.
The refactoring-helper skill should run when review findings, architecture
findings, gap reports, or code surfaces need to be turned into bounded,
behavior-preserving refactor slices. It identifies current behavior, invariants,
risks, validation commands, rollback notes, residual risk, and follow-on slices;
it stops before broad rewrites, silent behavior changes, issue creation, PR
creation, or unapproved repository mutation.
The ADR curator should run after architecture findings, synthesis, migration
notes, or repo evidence identify durable decisions that need Architecture
Decision Record candidates. It drafts proposed ADR packets without accepting
decisions, editing ADR files, or mutating repositories.
The architecture fitness-function author should run after architecture findings
or synthesis identify durable architecture rules worth preserving. It separates
machine-checkable invariants from human-judgment candidates and deferred
automation before any tests, CI gates, policy files, or repo checks are edited.
The finding-to-issue planner should run only after review findings exist. It
emits grouped, human-approved issue candidates and stops before tracker
creation, PR creation, remediation, or test generation.
The product report writer should run after the review packet has enough
specialist, synthesis, diagram, test, issue-planning, redaction, and quality
evidence to produce a customer-grade report. It writes report artifacts only and
stops before publication, approval claims, remediation, or repository mutation.
The review quality evaluator should run after specialist evidence, synthesis,
redaction, diagram/test follow-through planning, and product report writing have
produced the candidate packet/report. It emits pass, partial, fail, or not-run
with concrete blockers and warnings, and it stops before publication, approval
claims, remediation, issue creation, PR creation, tests, diagrams, or repository
mutation.

## Shared Specialist Input Shape

```yaml
skill_input_schema: repo_review_code.v1 | repo_review_security.v1 | repo_review_tests.v1 | repo_review_docs.v1 | repo_architecture_review.v1 | repo_dependency_review.v1
mode: review_repository | review_path | review_branch | review_diff | review_packet
repo_root: /absolute/path
target:
  target_path: <path or null>
  branch: <string or null>
  diff_base: <string or null>
  review_packet_path: <path or null>
  changed_paths:
    - <path>
  artifact_root: <path or null>
policy:
  review_depth: quick | standard | deep
  validation_mode: targeted | inspect_only | none
  write_review_artifact: true | false
  stop_after_review: true
```

## Synthesis Input Shape

```yaml
skill_input_schema: repo_review_synthesis.v1
mode: synthesize_specialist_artifacts | synthesize_review_packet
repo_root: /absolute/path
target:
  target_path: <path or null>
  branch: <string or null>
  diff_base: <string or null>
  specialist_artifacts:
    code: <path or null>
    security: <path or null>
    tests: <path or null>
    docs: <path or null>
    architecture: <path or null>
    dependency: <path or null>
  artifact_root: <path or null>
policy:
  required_roles:
    - code
    - security
    - tests
    - docs
    - architecture
    - dependency
  severity_policy: preserve_highest | preserve_role_severity
  write_review_artifact: true | false
  stop_after_synthesis: true
```

## Specialist Output Contract

Each specialist artifact should use this shape:

```md
## Metadata
- Skill: repo-review-code | repo-review-security | repo-review-tests | repo-review-docs | repo-architecture-review | repo-dependency-review
- Target: <repo/path/branch/diff>
- Date: <UTC timestamp or calendar date>
- Artifact: <path or none>

## Findings
- <priority>: <title>
  File: <repo-relative path or none>
  Role: <code | security | tests | docs | architecture | dependency>
  Scenario: <trigger or review condition>
  Impact: <behavioral consequence>
  Evidence: <specific code/doc/test/config observation>

## Reviewed Surfaces
- <bounded list>

## Validation Performed
- <command and what it proved, or explicit not run rationale>

## Residual Risk
- <what this role did not inspect or could not prove>
```

## Role-Specific Required Fields

- `repo-review-code` must include `reviewed_surfaces` and code-risk residuals.
- `repo-review-security` must include `trust_boundaries` and asset/attacker notes when relevant.
- `repo-review-tests` must include a `missing_proof_map` when coverage gaps are found.
- `repo-review-docs` must include `documentation_objects` for the bounded scope,
  and `commands_or_claims_checked` when docs make runnable claims.
- `repo-architecture-review` must include an `architecture_map`, candidate diagram tasks, candidate ADRs, and candidate fitness functions.
- `repo-dependency-review` must include a `dependency_surface_map`, candidate supply-chain findings, candidate dependency test gaps, and candidate license review notes.

## Synthesis Output Contract

```md
## Metadata
- Skill: repo-review-synthesis
- Target: <repo/path/branch/diff>
- Date: <UTC timestamp or calendar date>
- Specialist Artifacts:
  - code: <path or missing>
  - security: <path or missing>
  - tests: <path or missing>
  - docs: <path or missing>
  - architecture: <path or missing>
  - dependency: <path or missing>

## Findings
- <priority>: <title>
  Source Roles: <role list>
  File: <repo-relative path or none>
  Scenario: <trigger or review condition>
  Impact: <behavioral consequence>
  Evidence: <merged evidence without hiding disagreement>

## Coverage Matrix
- Code: present | missing | skipped
- Security: present | missing | skipped
- Tests: present | missing | skipped
- Docs: present | missing | skipped
- Architecture: present | missing | skipped
- Dependency: present | missing | skipped

Use `docs: skipped` only when the lane was intentionally skipped, with a
named rationale and a follow-up owner captured in `Residual Risk` or
`Recommended Follow-up Issues`.

## Dedupe Notes
- <what was merged and why>

## Disagreements
- <role disagreements or explicit none>

## Validation Performed
- <commands from specialist artifacts; synthesis should not invent new validation>

## Residual Risk
- <missing roles, skipped paths, unexecuted tests, generated/vendor exclusions>

## Recommended Follow-up Issues
- <bounded issue candidate or explicit none>
```

## Diagram Planning Input Shape

```yaml
skill_input_schema: repo_diagram_planner.v1
mode: plan_from_review_packet | plan_from_specialist_artifacts | plan_from_path | plan_from_issue | refresh_diagram_plan
repo_root: /absolute/path
target:
  review_packet_path: <path or null>
  specialist_artifacts:
    architecture: <path or null>
    security: <path or null>
    dependency: <path or null>
    docs: <path or null>
  target_path: <path or null>
  issue_number: <number or null>
  doc_path: <path or null>
  artifact_root: <path or null>
policy:
  audience: reviewers | maintainers | operators | users | mixed
  diagram_goals:
    - orientation
    - architecture_boundaries
    - workflow
    - state
    - data_flow
    - dependencies
    - responsibility_map
  max_tasks: <number>
  write_plan_artifact: true | false
  stop_after_plan: true
```

## Architecture Diagram Review Input Shape

```yaml
skill_input_schema: architecture_diagram_reviewer.v1
mode: review_diagram_packet | review_diagram_sources | review_rendered_artifacts | review_revision
repo_root: /absolute/path
target:
  review_packet_path: <path or null>
  diagram_packet_path: <path or null>
  diagram_sources:
    - <path>
  rendered_artifacts:
    - <path>
  artifact_root: <path or null>
policy:
  evidence_required: true
  render_status_required: true
  correction_handoff: diagram_planner | diagram_author | both | none
  write_review_artifact: true | false
  stop_after_review: true
```

## Review To Test Planning Input Shape

```yaml
skill_input_schema: review_to_test_planner.v1
mode: plan_from_review_packet | plan_from_specialist_artifacts | plan_from_synthesis | plan_from_findings_file
repo_root: /absolute/path
target:
  review_packet_path: <path or null>
  specialist_artifacts:
    code: <path or null>
    security: <path or null>
    tests: <path or null>
    docs: <path or null>
    architecture: <path or null>
    dependency: <path or null>
    synthesis: <path or null>
  synthesis_artifact: <path or null>
  findings_file: <path or null>
  artifact_root: <path or null>
policy:
  test_depth: focused | moderate
  validation_mode: targeted | inspect_only | none
  allow_handoff_generation: true | false
  unsafe_task_policy: mark_unsafe | skip
  write_plan_artifact: true | false
  stop_after_plan: true
```

## Gap Analysis Input Shape

```yaml
skill_input_schema: gap_analysis.v1
mode: compare_issue_to_implementation | compare_milestone_to_evidence | compare_spec_to_docs | compare_review_to_closeout | compare_packet_to_report
expected_baseline:
  issue_ref: <issue or null>
  milestone_plan: <path or null>
  spec_path: <path or null>
  review_packet_path: <path or null>
  closeout_record: <path or null>
observed_evidence:
  changed_paths:
    - <path>
  validation_artifacts:
    - <path>
  docs_paths:
    - <path>
  report_path: <path or null>
  closeout_record: <path or null>
policy:
  severity_floor: P0 | P1 | P2 | P3
  required_gap_types:
    - missing_evidence
    - implementation_gap
    - docs_drift
    - test_gap
    - closeout_drift
    - scope_ambiguity
  uncertainty_policy: record_explicitly
  issue_creation_allowed: false
  write_gap_artifact: true | false
  stop_before_fix: true
  stop_before_mutation: true
```

## ADR Curator Input Shape

```yaml
skill_input_schema: adr_curator.v1
mode: curate_from_review_packet | curate_from_architecture_review | curate_from_findings_file | curate_from_migration_notes | curate_from_path | refresh_adr_packet
repo_root: /absolute/path
target:
  review_packet_path: <path or null>
  architecture_review_artifact: <path or null>
  findings_file: <path or null>
  migration_notes_path: <path or null>
  target_path: <path or null>
  existing_adr_dir: <path or null>
  existing_adr_packet: <path or null>
  artifact_root: <path or null>
policy:
  adr_status_policy: conservative | preserve_source_status
  approval_required: true
  mutation_allowed: false
  supersession_policy: preserve_explicit | infer_candidates
  write_candidate_packet: true | false
  stop_before_acceptance: true
```

## Architecture Fitness Function Author Input Shape

```yaml
skill_input_schema: architecture_fitness_function_author.v1
mode: author_from_review_packet | author_from_architecture_review | author_from_findings_file | author_from_path
repo_root: /absolute/path
target:
  review_packet_path: <path or null>
  architecture_review_artifact: <path or null>
  findings_file: <path or null>
  target_path: <path or null>
  artifact_root: <path or null>
policy:
  allowed_check_types:
    - dependency_rule
    - forbidden_import
    - contract_test
    - docs_check
    - ci_gate
    - repo_policy_check
  validation_mode: targeted | inspect_only | none
  implementation_allowed: false
  ci_gate_allowed: true | false
  write_plan_artifact: true | false
  stop_after_plan: true
```

## Finding To Issue Planner Input Shape

```yaml
skill_input_schema: finding_to_issue_planner.v1
mode: plan_from_review | plan_from_synthesis | plan_from_packet | refresh_issue_plan
finding_source: <review artifact or packet root>
policy:
  approval_required: true
  tracker_creation_allowed: false
  grouping_policy: exact | conservative | none
  severity_floor: P0 | P1 | P2 | P3
  preserve_specialist_disagreement: true
  stop_before_mutation: true
```

## Product Report Writer Input Shape

```yaml
skill_input_schema: product_report_writer.v1
mode: write_from_packet | write_from_synthesis | write_from_specialist_artifacts | refresh_report
artifact_root: <review packet root>
audience: internal_review | customer_private | public_candidate
policy:
  privacy_mode: local_only | customer_private | public_candidate
  publication_intent: none | internal_review | customer_private | public_candidate
  write_report_artifact: true | false
  require_redaction_status: true | false
  preserve_specialist_disagreement: true
  stop_before_publication: true
  stop_before_mutation: true
```

## Review Quality Evaluator Input Shape

```yaml
skill_input_schema: review_quality_evaluator.v1
mode: evaluate_packet | evaluate_report | evaluate_synthesis | pre_publication_gate
artifact_root: <review packet or report root>
publication_intent: none | internal_review | customer_private | public_candidate
policy:
  required_roles:
    - code
    - security
    - tests
    - docs
    - architecture
  severity_floor: P0 | P1 | P2 | P3
  require_redaction_status: true | false
  require_template_sections: true | false
  reject_unsupported_claims: true
  write_evaluation_artifact: true | false
  stop_before_publication: true
  stop_before_mutation: true
```

## Severity Rules

- Preserve the highest severity attached to a merged finding unless the source
  role explicitly withdraws it.
- Do not downgrade security findings merely because the code reviewer did not
  mention them.
- Do not hide missing tests behind a "no code defect found" result.
- Do not convert docs truth drift into style feedback when it affects reviewer
  reproducibility or operator safety.
- Do not collapse architecture drift into code style or docs polish when it
  affects boundaries, layering, lifecycle, or state ownership.
- Do not collapse dependency and supply-chain drift into generic security or
  test feedback when it affects installability, package manager policy,
  lockfile truth, license-sensitive evidence, or release reproducibility.
- Mark disagreement explicitly instead of silently choosing one role's view.

## Boundaries

The suite may:
- inspect repo files and local diffs
- run bounded local validation commands when safe
- write review artifacts under `.adl/reviews`
- recommend follow-up issues
- plan bounded diagram tasks for `diagram-author`
- review diagram packets and rendered artifact metadata for source-grounded
  truth, missing major components, unsupported relationships, stale labels,
  unrenderable sources, and correction handoffs
- plan bounded test-generation handoffs from review findings, including
  behavior under test, fixture needs, expected assertions, validation commands,
  and `generated` / `recommended` / `deferred` / `unsafe` status
- compare explicit baselines to observed evidence and emit source-grounded gap
  findings, missing-evidence records, uncertainty, and follow-up recommendations
- draft source-grounded ADR candidate packets with status, context, decision,
  consequences, validation notes, supersession links, and approval boundaries
- author bounded architecture fitness-function plans that separate
  machine-checkable invariants, human-judgment candidates, deferred automation,
  validation commands, expected failure modes, and implementation handoffs
- draft grouped issue candidates from findings after explicit review evidence
  exists, while preserving highest severity and specialist disagreement
- write customer-grade product report artifacts from existing review packet
  evidence while preserving severity, disagreement, publication boundaries,
  caveats, and residual risk
- evaluate packet and report quality against evidence, severity, actionability,
  duplication, unsupported-claim, specialist-coverage, template-compliance,
  residual-risk, and publication-safety gates before customer-facing use

The suite must not:
- edit code, tests, docs, configs, or issue state
- claim merge approval
- author, edit, render, publish, or replace diagrams from the diagram review
  lane
- write tests or fixtures from the review-to-test planning lane
- fix gaps, create issues or PRs, approve closeout, approve release, or mutate
  repositories from the gap-analysis lane
- accept, reject, supersede, publish, or commit ADRs from the ADR curation lane
- edit ADR files, docs, tests, CI, code, policy files, issues, or PRs from the
  ADR curation lane
- install or modify tests, CI gates, docs checks, dependency rules, policy files,
  issues, or PRs from the architecture fitness-function authoring lane
- claim remediation
- run unbounded repository-wide analysis without a declared target
- use network or paid data feeds
- hide severity, disagreement, skipped roles, or residual risk
- invent diagrams, render diagram assets, or publish visual artifacts from the
  planning lane
- create tracker items from the issue-planning lane without explicit operator
  approval
- publish reports, claim approval, claim compliance, claim merge-readiness, or
  claim remediation completion from the product report writing lane
- publish reports, approve reports, rewrite reports, claim customer readiness,
  create issues or PRs, run specialist lanes, generate tests or diagrams, or
  mutate repositories from the review-quality evaluation lane

## Relationship To `repo-code-review`

Use `repo-code-review` when:
- one reviewer is enough
- the review is quick or standard depth
- the operator does not need separate role artifacts
- the result should be a single findings-first pass

Use this suite when:
- the review is deep or release-adjacent
- role separation is useful for traceability
- security, tests, or docs need explicit ownership
- architecture boundaries, state models, layering, or drift need explicit ownership
- dependency manifests, lockfiles, CI install paths, containers, or license cues
  need explicit ownership
- diagram planning needs to be source-grounded before asking `diagram-author` to
  create a specific visual artifact
- expected issue, milestone, review, report, or closeout truth must be compared
  against observed evidence before release or closeout
- durable architecture decisions need proposed ADR packets before acceptance,
  implementation, or follow-up automation work
- durable architecture rules need executable-check planning before separate
  implementation work installs tests, policy checks, or CI gates
- a customer-grade report is needed from completed review artifacts without
  turning report writing into publication, approval, or remediation
- a customer-facing packet or report needs a third-party-review-style quality
  gate before publication or delivery
- the synthesis artifact should show specialist coverage and disagreement
