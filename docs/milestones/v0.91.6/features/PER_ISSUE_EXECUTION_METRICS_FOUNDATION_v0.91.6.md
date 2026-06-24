# Per-Issue Execution Metrics Foundation

## Metadata

- Feature Name: Per-Issue Execution Metrics Foundation
- Milestone Target: `v0.91.6`
- Status: active template and sprint-rollup substrate aligned in `v0.91.6`; VPP expansion remains routed to `#4309`
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: policy, architecture, artifact
- Proof Modes: review, schema
- Related issues: `#4264`, `#4278`, `#4279`, `#4300`, `#4309`, `#4329`

## Purpose

Make per-issue time, token, validation, PVF-lane, and wait-state accounting
explicit enough that later VPP and nested-goal work can consume the same
truthful contract instead of inventing ad hoc issue statistics.

## Scope

In scope:

- design-time estimate fields needed on every issue-local execution plan;
- execution-time actual fields needed on every truthful outcome record;
- explicit unknown/not-collected rules for missing telemetry;
- mapping from `#4264` session-goal metrics into issue-local `SOR` truth;
- variance-analysis handoff boundaries for `#4279`;
- dormant prompt-template contract updates that `#4309` can activate later.

Out of scope:

- full nested milestone/sprint/session goal accounting;
- VPP lifecycle insertion or activation;
- automatic variance rollups and dashboards;
- claims that Codex exposes exact total token accounting for every run.

## Required Decisions

- Which estimate fields are mandatory before execution starts?
- Which actual fields are required even when the substrate cannot measure them
  directly?
- Which missing values should be recorded as `unknown` versus
  `not_collected` versus `not_applicable`?
- Which wait states belong to issue-local truth rather than later sprint
  rollups?
- Which metrics are design-time foundations for `#4309` and `#4279`, rather
  than final workflow automation?

## Dependencies

- `#4264` issue-goal token/time metrics capture.
- `#4278` SPP-estimate and SOR-actual alignment work.
- `#4279` variance analysis for estimate misses.
- `#4300` sprint umbrella and child issue goal requirements.
- `#4309` next prompt-template version and activation path.

## Per-Issue Metrics Contract

### SPP planning fields

Each issue-local `SPP` should carry the following estimate-facing fields:

| Field | Meaning |
| --- | --- |
| `initial_pvf_lane` | Fail-closed lane classification captured at issue creation time. |
| `planned_pvf_lane` | Lane expected for execution after planning review. |
| `planned_pvf_lane_source` | Why the planned lane is believed to be correct. |
| `expected_runtime_class` | Qualitative runtime posture such as `docs_only`, `tooling_control_plane`, `runtime_behavior`, `external_integration`, `mixed`, or `unknown`. |
| `estimate_elapsed_seconds` | Planned end-to-end elapsed duration for the issue run. |
| `estimate_total_tokens` | Planned total token budget or `unknown` when the substrate cannot support a truthful estimate. |
| `estimate_validation_seconds` | Planned focused-proof time, not total execution time. |
| `estimate_confidence` | Qualitative confidence such as `low`, `medium`, `high`, or `unknown`. |
| `estimate_data_source` | Where the estimate came from, such as historical issue packet, operator judgment, or issue-class heuristic. |
| `estimate_source_ref` | Stable reference for the estimate source when one exists. |

Planning rules:

- Record `unknown`, never `0`, when an estimate is unavailable.
- Keep `expected_runtime_class` qualitative and stable enough for later lane
  and variance analysis.
- Do not infer exact token counts from chat intuition alone.

### SOR actual fields

Each issue-local `SOR` should carry the following execution-facing fields:

| Field | Meaning |
| --- | --- |
| `expected_runtime_class` | The runtime posture the issue planned against. |
| `actual_elapsed_seconds` | Total measured elapsed time for the issue execution path when known. |
| `actual_active_work_seconds` | Active operator/agent implementation time when the substrate can distinguish it from wait states. |
| `actual_total_tokens` | Total actual token use when truthfully available from the goal/usage substrate. |
| `actual_validation_seconds` | Time spent on the focused proof/validation slice. |
| `actual_pr_wait_seconds` | Time spent waiting on PR checks, mergeability, or janitor follow-up when known. |
| `actual_ci_wait_seconds` | Time spent waiting on external CI/proof surfaces when separable from PR wait. |
| `actual_metrics_data_source` | Source used for actual metrics, typically the `#4264` issue-goal metrics summary or a bounded manual rollup. |
| `actual_metrics_source_ref` | Stable path or issue-local reference to that metrics source. |
| `actual_metrics_confidence` | Confidence in the actual metrics payload. |
| `estimate_error_percent` | Rolled-up estimate/actual error when computable. |
| `completion_state` | Truthful issue-local outcome such as `completed`, `completed_with_follow_on`, `blocked`, `failed`, `deferred`, `cancelled`, or `unknown`. |
| `goal_terminal_state` | Terminal-boundary truth for the selected goal record, including goal kind, declared boundary, issue/PR evidence, whether completion was allowed, and the reason. |

Actual-value rules:

- Record `unknown` when the value should exist conceptually but the substrate
  did not expose it truthfully.
- Record `not_collected` when the workflow could have captured the metric but
  did not.
- Record `not_applicable` only when the surface genuinely does not apply, such
  as CI wait on a lane with no CI proof requirement.
- Do not invent wait times from rough memory or broad timestamp subtraction
  without recording that the value is estimated.

## Session-Goal And Issue-Local Mapping

`#4264` is the current source of token/time accounting truth for tracked issue
sessions. This contract therefore treats session-goal usage as the preferred
substrate for issue-local actuals without claiming that milestone, sprint, and
nested child-goal aggregation is complete.

Current mapping rules:

- issue-local `SOR` should consume the issue-goal metrics summary when
  available;
- raw chat or CLI session logs should not be copied into `SOR` verbatim;
- if the goal substrate cannot provide a field cleanly, the issue record keeps
  `unknown` or `not_collected` instead of fabricating precision;
- later nested-goal accounting may aggregate these fields, but it must not
  rewrite historical issue truth.

## Variance And Rollup Boundary

`#4329` defines the per-issue inputs needed for later variance work, but does
not itself implement the full variance-analysis system.

Handoff rules for `#4279`:

- issue-local estimate/actual pairs must exist before variance is computed;
- variance classification should treat missing data as unknown, not as zero
  error;
- sprint rollups should count only completed variance analyses in category
  totals;
- `not_applicable` must remain outside category statistics.

## Prompt-Template Boundary

This issue does not require a brand-new template set by itself.

Current boundary:

- `docs/templates/prompts/current.json` remains the active template registry
  surface;
- active `1.0.3` `SPP`/`SOR` surfaces now carry issue-goal refs, sprint-goal
  refs, goal-rollup refs, and the issue-local timing bucket fields defined by
  this contract;
- sprint-conductor goal-metrics capture and sprint closeout rollup now record
  the same issue/sprint refs and the same timing-bucket truth instead of only
  raw elapsed/token totals;
- `#4309` still owns any future template-version activation beyond the current
  active set, plus broader VPP expansion.

## Implemented in `#4392`

This contract is no longer only a future-facing field list.

The `v0.91.6` implementation now records:

- issue-local goal refs:
  - `issue_goal_ref`
  - `sprint_goal_ref`
  - `goal_metrics_rollup_ref`
- issue-local actual timing buckets:
  - `actual_active_work_seconds`
  - `actual_validation_seconds`
  - `actual_pr_wait_seconds`
  - `actual_ci_wait_seconds`
- issue-local completion truth:
  - `completion_state`
  - metrics confidence posture
- sprint closeout rollups for:
  - known vs unknown elapsed totals
  - known vs unknown active-work totals
  - known vs unknown validation totals
  - known vs unknown PR-wait totals
  - known vs unknown CI-wait totals
  - completion-state counts
  - data-source counts
  - goal-id availability counts

Important constraint:

- the active Codex thread goal slot is still a single live-session telemetry
  surface
- ADL therefore records issue/sprint goal accounting as ADL-owned truth and
  treats missing session-goal telemetry as `unknown` / `not_collected` rather
  than pretending nested live goals are already universally available

## Validation And Review

- keep the field set concise enough that ordinary issues can fill it truthfully;
- verify schema/template alignment for the touched next-version prompt cards;
- review unknown/not-collected semantics for honesty and operator usability;
- preserve the current lifecycle boundary: `SIP -> STP -> SPP -> SRP -> SOR`
  in this issue.

## Non-Goals

- No claim that VPP is already active in the lifecycle.
- No claim that nested-goal accounting is complete.
- No claim that all token/time telemetry is available automatically today.
- No hidden upgrade of failed, skipped, blocked, or pending validation states
  into success metrics.
