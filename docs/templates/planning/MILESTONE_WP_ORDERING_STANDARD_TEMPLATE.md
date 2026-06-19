# ADL Milestone WP Ordering Standard Template

## Metadata

- Milestone: `<version>`
- Status: `<status>`
- Owner: `<owner>`
- Source issue: `<source_issue>`

## Purpose

Explain how this milestone applies the canonical ADL WP ordering standard.

## WP-01 Planning Gate

`WP-01` is the milestone planning and setup gate.

`WP-01` may close only when:

- milestone planning docs are tracked;
- issue wave is seeded or explicitly routed;
- sprint umbrella issues exist;
- planned work-package issues exist, are moved, or have explicit routing;
- required C-SDLC cards exist for opened issues;
- first sprint execution can begin without reconstructing intent from chat.

## Sprint Umbrellas

List each sprint umbrella issue and its child issue set.

| Sprint | Umbrella issue | Child issues | Boundary |
| --- | --- | --- | --- |
| Sprint 1 | `<issue>` | `<children>` | `<boundary>` |
| Sprint 2 | `<issue>` | `<children>` | `<boundary>` |
| Sprint 3 | `<issue>` | `<children>` | `<boundary>` |
| Sprint 4 | `<issue>` | `<children>` | `<boundary>` |

## Work Package Sequence

List the milestone WP sequence. Keep closeout-tail order stable even when WP
numbers shift.

| WP | Issue | Role | Dependencies |
| --- | --- | --- | --- |
| WP-01 | `<issue>` | Planning and issue-wave readiness | `<deps>` |
| WP-02 | `<issue>` | `<role>` | `<deps>` |

## Closeout Tail

Record the closeout-tail issue order.

| Tail role | Issue | Required proof |
| --- | --- | --- |
| Demo matrix / demo showcase refresh | `<issue>` | `<proof>` |
| Coverage / quality gate | `<issue>` | `<proof>` |
| Docs + review alignment | `<issue>` | `<proof>` |
| Internal review | `<issue>` | `<proof>` |
| External / third-party review | `<issue-or-not-required>` | `<proof>` |
| Remediation plus final preflight | `<issue>` | `<proof>` |
| Next milestone planning | `<issue>` | `<proof>` |
| Next milestone review | `<issue-or-not-required>` | `<proof>` |
| Release ceremony | `<issue>` | `<proof>` |

## New Scope Routing

For every new scope item discovered during planning, record one of:

- `later_wp`
- `sprint_child`
- `mini_sprint`
- `sidecar`
- `follow_on`
- `defer_or_reject`

## Milestone-Specific Deviations

Record any approved deviation from the standard and why it is safe.

## Validation

List focused validation commands for the planning package.

## Exit Criteria

- `WP-01` readiness is visible from tracked docs and issues.
- Sprint umbrella issues exist and have child issue lists.
- Closeout-tail issue order is explicit.
- New scope is routed rather than absorbed into `WP-01`.

## Standard closeout-tail execution policy

Use the closeout-tail section as one ordered sprint, not as a set of unrelated mini-sprints.

Required operating rules:

- every closeout-tail issue is its own tracked issue with the normal ADL lifecycle
- advance through the tail in issue order only when the required upstream truth surface is available
- assign a watcher immediately for any wait state involving checks, review, mergeability, or an upstream dependency
- while actively blocked, watcher polling should be no slower than every 30 seconds
- route quality-gate, internal-review, and external-review findings into the remediation/preflight issue unless they are clearly planning-only or explicitly out of scope
- fold recurring planning and handoff work into the next-milestone planning issue instead of preserving orphan planning steps outside the standard tail
- keep the sprint umbrella open until all ordered closeout-tail issues are merged or intentionally closed and their issue-local closeout truth is complete
- when a closeout-tail role is marked optional or `<issue-or-not-required>` in the milestone plan, preserve its ordered slot and dependency semantics if it becomes required, but do not force-create an issue that the milestone explicitly marks not required
