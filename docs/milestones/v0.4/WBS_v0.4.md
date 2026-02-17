# Work Breakdown Structure (WBS) Template

## Metadata
- Milestone: `{{milestone}}`
- Version: `{{version}}`
- Date: `{{date}}`
- Owner: `{{owner}}`

## How To Use
- Break work into independently-mergeable issues.
- Keep each item measurable and testable.
- Include deliverables + dependencies + issue links.

## WBS Summary
{{wbs_summary}}

## Work Packages
| ID | Work Package | Description | Deliverable | Dependencies | Issue |
|---|---|---|---|---|---|
| WP-01 | {{package_1}} | {{description_1}} | {{deliverable_1}} | {{deps_1}} | {{issue_1}} |
| WP-02 | {{package_2}} | {{description_2}} | {{deliverable_2}} | {{deps_2}} | {{issue_2}} |
| WP-03 | {{package_3}} | {{description_3}} | {{deliverable_3}} | {{deps_3}} | {{issue_3}} |

## Sequencing
- Phase 1: {{phase_1}}
- Phase 2: {{phase_2}}
- Phase 3: {{phase_3}}

## Acceptance Mapping
- {{package_1}} -> {{acceptance_criteria_1}}
- {{package_2}} -> {{acceptance_criteria_2}}
- {{package_3}} -> {{acceptance_criteria_3}}

## Exit Criteria
- Every in-scope requirement maps to at least one WBS item.
- Every WBS item has an owner, issue reference, and concrete deliverable.
- Dependency order is explicit enough to execute deterministically.

# ADL v0.4 – Work Breakdown Structure

## Metadata
- Milestone: `v0.4`
- Version: `0.4`
- Date: {{date}}
- Owner: Daniel Austin

---

## WBS Summary
v0.4 delivers a deterministic fork/join runtime execution scaffold with bounded concurrency, join semantics, and full traceability. Work is divided into independently mergeable packages aligned to issue #291 (and sub-issues as needed).

Primary objective: ship a minimal but real concurrency engine without regressing sequential execution or CI stability.

---

## Work Packages

| ID | Work Package | Description | Deliverable | Dependencies | Issue |
|---|---|---|---|---|---|
| WP-01 | Execution Plan & DAG Validation | Define `ExecutionPlan` type and validate fork/join DAG structure before execution. | Validated execution plan + unit tests for invalid graphs. | Existing sequential executor | #291 |
| WP-02 | Bounded Fork Executor | Implement bounded executor (threadpool) for branch execution. | Executor module + concurrency tests. | WP-01 | #291 |
| WP-03 | Deterministic Join Barrier | Implement wait-for-all join with ordered merge by branch ID. | JoinBarrier implementation + deterministic merge tests. | WP-02 | #291 |
| WP-04 | Sequential Mode Preservation | Ensure existing sequential execution path remains intact and testable. | Passing regression suite under sequential mode. | WP-01 | #291 |
| WP-05 | Failure & Retry Integration | Integrate existing retry logic with concurrent execution and enforce fail-fast semantics. | Branch failure tests + deterministic retry validation. | WP-02, WP-03 | #291 |
| WP-06 | Determinism Test Harness | Multi-run execution test to verify identical artifacts across runs. | Determinism test suite + artifact comparison checks. | WP-02, WP-03 | #291 |
| WP-07 | Demo Workflow Update | Update v0.4 demo to showcase fork/join concurrency. | Working demo workflow + README example. | WP-03 | #291 |

---

## Sequencing

- Phase 1: Core planning and validation (WP-01)
- Phase 2: Concurrency primitives (WP-02, WP-03)
- Phase 3: Safety & determinism guarantees (WP-04, WP-05, WP-06)
- Phase 4: Demo integration and polish (WP-07)

Execution order must follow dependency chain; no join implementation before executor exists.

---

## Acceptance Mapping

- Fork execution requirement -> WP-02
- Deterministic join requirement -> WP-03
- Sequential fallback requirement -> WP-04
- Failure semantics requirement -> WP-05
- Determinism guarantee requirement -> WP-06
- Demo validation requirement -> WP-07

---

## Exit Criteria

- Every functional requirement in DESIGN_v0.4.md maps to at least one WBS item.
- All WBS items have corresponding PR(s) and passing CI.
- Concurrency execution works with deterministic artifacts.
- Sequential execution mode passes full regression suite.
- Demo workflow reflects actual shipped runtime behavior.