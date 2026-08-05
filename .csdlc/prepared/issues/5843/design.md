# Issue 5843 Design: Documentation And Release Truth

Status: design-time ready; execution waits for a passing WP-22 gate.

## Authority And Sources

Issue #5843 and the WP-23 rows in the WBS and issue-wave YAML own final
documentation alignment after quality acceptance. Inputs include the complete
`docs/milestones/v0.92` package, `README.md`, `CHANGELOG.md`, active feature
lists, `docs/milestones/v0.92/ADR_PLAN_v0.92.md`, release notes, release plan,
skills, and root/nested agent guidance. Existing inconsistencies are evidence
to repair, not authority: for example, the checklist currently assigns release
evidence and final notes to WP-29 while the current WBS and live issues assign
the ceremony package to WP-30.

## Outcome Contract

Produce a docs-review packet and source-grounded release-truth diff that maps
every changed claim to landed WP-22-accepted evidence. Normalize current
status, issue numbers, WP ownership, commands, links, version language, and
planned-versus-landed distinctions across canonical surfaces. Create an ADR
candidate packet only when landed architecture introduced a durable decision
that is not already represented; do not manufacture ADRs to fill a quota.

Public-facing prose may describe only landed reviewed behavior. Birthday,
identity, provider, platform, governance, citizenship, consciousness, legal,
and v0.93 claims retain the milestone non-claim boundaries.

## Execution Sequence

1. Verify WP-22 passed at an ancestral exact revision and ingest its accepted
   matrix and blockers disposition.
2. Build a canonical document/claim inventory spanning root, milestone,
   feature, ADR, release, skill, and agent-guidance surfaces.
3. Classify each statement as current, stale, planned, blocked, unsupported, or
   historical; map current claims to exact accepted evidence.
4. Apply narrowly scoped documentation corrections and generate the review and
   ADR-candidate packets.
5. Validate links, Markdown, YAML/JSON, version/WP references, commands,
   claim-boundary language, and release-note evidence mapping.
6. Run bounded exact-head docs review and leave WP-24/WP-24A publication
   artifacts and WP-25 review execution to their owners.

## Protected-Path Candidates

- `README.md` and `CHANGELOG.md` only where v0.92 canonical truth requires it
- `docs/milestones/v0.92` excluding historical immutable evidence
- active feature-list, skill, and nested `AGENTS.md` files found by inventory
- `docs/reviews/v0.92/docs-release-truth-5843`
- `.csdlc/evidence/5843`

The implementation claim must list the exact inventory result. It must not
claim all documentation or any product source root.

## Validation And Failure Policy

Required lanes are canonical inventory completeness, executable docs/YAML/JSON
parsing, relative-link resolution and command checks, version/WP ownership consistency, accepted-
evidence link checks, stale/planned/unsupported claim rejection, secret/private
path scanning, and exact-head docs review. Any unresolved contradiction or
unsupported release claim blocks completion and remains visible.

## Non-Goals

- No product implementation, historical evidence rewrite, or release approval.
- No article/podcast publication, internal review execution, or remediation.
- No claim that v0.93 governance or legal personhood is implemented.
