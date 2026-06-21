# ADL Milestone WP Ordering Standard

## Metadata

- Status: `active_standard`
- Owner: ADL maintainers
- First applied in: `v0.91.5`
- Source issue: `#3567`
- Template surface: [MILESTONE_WP_ORDERING_STANDARD_TEMPLATE.md](../templates/planning/MILESTONE_WP_ORDERING_STANDARD_TEMPLATE.md)

## Purpose

This standard makes the ADL milestone work-package script explicit so future
milestones do not reconstruct ordering from chat, memory, or a single prior
milestone.

## Canonical Shape

Every ADL milestone starts with `WP-01` as the planning and setup gate.

`WP-01` closes only after:

- the milestone planning package is tracked;
- the issue wave is seeded or explicitly routed;
- all planned sprint umbrella issues exist;
- all known work-package issues exist, are moved, or have explicit routing;
- `SIP`, `STP`, `SPP`, `SRP`, and `SOR` cards exist for opened issues;
- execution sequencing and dependencies are clear enough for the first sprint
  to begin.

Implementation work discovered during `WP-01` must be scheduled into a later
sprint, sidecar, mini-sprint, or follow-on issue. It must not silently become
`WP-01` scope merely because it was discovered during planning.

## Sprint Bands

After `WP-01`, execution work is grouped into sprint bands. A milestone may use
up to six execution sprints before the closeout tail.

Each sprint band should have a sprint umbrella issue when it coordinates more
than one child issue or when it owns closeout truth for a band. Sprint umbrella
issues are first-class lifecycle records, not optional notes.

Sprint umbrella issues should record:

- descriptive sprint objective and boundary;
- child issue list;
- dependency order;
- proof and validation expectations;
- closeout bar;
- blocked/deferred routing rules.

Sprint umbrellas coordinate work. They do not replace child issue execution,
review, validation, or closeout.

## Issue Label Taxonomy

Sprint routing should be visible in GitHub issue metadata, not only in titles or
body prose.

Use the following labels for milestone issue waves:

- `type:sprint` for a sprint umbrella issue that coordinates one sprint band.
- `type:mini-sprint` for a bounded mini-sprint umbrella or side-wave umbrella.
- `type:task` for ordinary child implementation tasks under a sprint or
  mini-sprint umbrella.
- `type:planning` for companion planning issues, setup issues, or planning-only
  sidecars.

Do not reuse `type:task` or `type:planning` to stand in for a sprint umbrella.
If a required sprint-routing label is missing from the repository, the ADL
issue tooling should fail before mutating issue metadata so operators can add
the approved label through the normal GitHub-label path.

## Closeout Tail

The closeout-tail WP numbers may change based on the number of execution WPs.
The closeout-tail order must not change without an explicit, reviewed milestone
decision.

Canonical closeout-tail order:

1. Demo matrix / demo showcase refresh.
2. Coverage / quality gate.
3. Docs + review alignment.
4. Internal review.
5. External / third-party review when required by the milestone.
6. Review findings remediation plus final preflight when applicable.
7. Next milestone planning.
8. Next milestone review when the next milestone package needs its own review
   pass before ceremony.
9. Release ceremony.

Small milestones may combine adjacent closeout roles only when the issue wave
records the combination explicitly and the combined issue preserves both proof
surfaces.

## Scheduling New Scope

New scope found during planning must be classified before execution:

- `later_wp`: belongs in an existing work package;
- `sprint_child`: belongs under an existing sprint umbrella;
- `mini_sprint`: needs a bounded side wave with its own umbrella;
- `sidecar`: useful but not part of milestone release proof;
- `follow_on`: belongs after the milestone;
- `defer_or_reject`: not accepted for current planning.

The routing decision should name the issue, sprint, dependency, and non-goals.

## v0.91.5 Application

v0.91.5 applies this standard as follows:

- `#3568` opened the milestone after v0.91.4 release.
- `#3567` records this standard and the reusable template surface.
- `#3569` is scheduled portable ADL adapter contract work, not hidden `WP-01`
  implementation.
- `#3571` through `#3574` are sprint umbrella issues.
- `#3575`, `#3579`, `#3576`, `#3580`, `#3577`, `#3581`, and `#3578`
  are concrete closeout-tail issues.

## Validation

Focused validation for milestone planning should include:

- YAML parse of the issue wave;
- planning-template validation for changed planning docs;
- Markdown link checks for milestone docs;
- redaction/path hygiene scans for host-local path leakage;
- issue-state spot checks for seeded issues and sprint umbrellas.

## Exit Criteria

- Future milestone WBS and sprint docs can cite this standard instead of
  re-explaining the WP script from memory.
- `WP-01` readiness is testable from tracked docs and issues.
- Sprint umbrella issues and closeout-tail issues are visible in the issue
  graph before execution begins.
