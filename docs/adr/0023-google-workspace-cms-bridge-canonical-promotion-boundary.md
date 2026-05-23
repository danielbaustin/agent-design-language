# ADR 0023: Google Workspace CMS Bridge And Canonical Repo Promotion Boundary

- Status: Accepted
- Date: 2026-05-23
- Accepted in: v0.91.3
- Candidate source: docs/architecture/adr/0023-google-workspace-cms-bridge-canonical-promotion-boundary.md
- Target milestone: v0.91.2
- Related issues: #3091, #3092, #3093, #3094, #3111, #3112, #3122, #3124
- Related ADRs: ADR 0013, ADR 0017, ADR 0020 and ADR 0021

## Context

v0.91.2 makes the Google Workspace CMS bridge real enough to need a durable
architecture boundary. The bridge now covers fixture-backed demo evidence,
typed adapter boundaries, bounded live read surfaces, live safety packaging,
bounded content-card roundtrip contracts, and a reusable project operational
package.

That power is useful because Workspace is a humane collaborative surface for
drafts, comments, content cards, review packets, and promotion packets. It is
risky because Workspace can drift from Git history, hide authority boundaries,
or become an accidental parallel source of truth.

## Decision

ADL may use Google Workspace as bounded draft, collaboration, content-card,
review, and promotion-packet infrastructure.

Google Workspace is not canonical repository truth.

Tracked repository files and GitHub issue/PR history remain the canonical
promotion boundary for ADL product, milestone, feature, release, and lifecycle
truth. Workspace state may not silently edit, replace, or override canonical
repo files.

## Requirements

- Workspace use must distinguish fixture mode, dry-run mode, and live-gated
  execution.
- Live execution must require explicit auth, explicit scope, explicit mode, and
  explicit write approval.
- Revision anchors and document/content-card bindings must be checked before
  bounded mutation.
- Promotion packets must identify source Workspace state, target repo paths,
  issue/PR routing, and non-claims.
- Workspace credentials and live outputs must avoid secrets and inappropriate
  visibility leakage.
- ACC/tool authority applies to live or write-capable Workspace actions.

## Consequences

### Positive

- Enables a more collaborative working surface without replacing Git.
- Makes Workspace proof packets useful to CodeFriend and future projects.
- Gives future agents a clear answer to what GWS can and cannot mean.

### Negative

- GWS work requires strict promotion and revision discipline.
- Live Workspace actions need safety, auth, scope, and redaction proof.
- GWS cannot be used as an excuse to skip tracked repo updates.

## Alternatives Considered

### Make Workspace canonical

This would improve collaborative editing ergonomics but undermine Git-native
review, release evidence, and deterministic promotion.

### Keep Workspace as demo-only

This is safer but wastes the now-proven operational bridge.

## Validation Notes

This ADR was reviewed against the v0.91.2 GWS feature doc, review
packet artifacts, live safety report, content-card roundtrip report, and the
follow-on hardening that required confirmed live write persistence before
claiming proof.

## Non-Claims

- This ADR does not require GWS for normal ADL issue execution.
- This ADR does not authorize silent sync from Workspace into tracked repo
  files.
