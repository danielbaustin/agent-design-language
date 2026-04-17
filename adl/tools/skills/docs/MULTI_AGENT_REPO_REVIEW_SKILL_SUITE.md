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
- `repo-diagram-planner`
- `repo-review-synthesis`

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

The first four roles may run independently when the operator wants parallel
review. The synthesis role should run after at least one specialist artifact is
available, and ideally after all required roles have reported.
The diagram planner should normally run after packet building and after the
specialist artifacts that may propose diagram work. It emits bounded task briefs
for `diagram-author`; it is not itself a review or diagram-authoring lane.

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
- `repo-review-docs` must include `commands_or_claims_checked` when docs make runnable claims.
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

The suite must not:
- edit code, tests, docs, configs, or issue state
- claim merge approval
- claim remediation
- run unbounded repository-wide analysis without a declared target
- use network or paid data feeds
- hide severity, disagreement, skipped roles, or residual risk
- invent diagrams, render diagram assets, or publish visual artifacts from the
  planning lane

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
- the synthesis artifact should show specialist coverage and disagreement
