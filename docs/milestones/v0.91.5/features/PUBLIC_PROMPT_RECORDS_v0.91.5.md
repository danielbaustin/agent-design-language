# Public Prompt Records v0.91.5

## Metadata

- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-05-29`
- Owner: ADL maintainers
- Status: `active_wp_01_opening`
- Related issues: `#3472`, `#3473`, `#3474`, `#3475`, `#3476`
- Local ADL state disposition: [../LOCAL_ADL_STATE_DISPOSITION_3473.md](../LOCAL_ADL_STATE_DISPOSITION_3473.md)

## Template Rules

This is a planning feature doc, not a publication approval.

## Purpose

Define the transition from local C-SDLC prompt records to public, reviewable,
redaction-safe prompt packets.

## Context

Prompt cards are durable C-SDLC state. Local `.adl` state also contains
execution cache and historical working files. v0.91.5 must separate public
records from local cruft before cleanup.

## Coverage / Ownership

This feature owns prompt packet export, redaction, validation, reviewer index,
and `.adl` archive/deletion review expectations.

## Overview

The public prompt-record lane should export selected prompt packets, validate
machine-readable shape, redact local/private data, and index them for review.

The companion local-state disposition for `#3473` classifies local `.adl/`
content before cleanup or archive work proceeds. It explicitly performs no
deletion and does not promote local `.adl` state as canonical public truth.

## Design

- Export prompt packets from tracked or approved local sources.
- Validate structure and unresolved placeholders.
- Run redaction checks before publication.
- Inventory `.adl` state before archive or deletion.
- Require review before destructive cleanup.

## Export Command

Initial packet export is handled by the repository tooling command:

```bash
adl tooling public-prompt-packet export \
  --issue <number> \
  --slug <normalized-slug> \
  --version <milestone-version> \
  [--source <card-bundle-dir>] \
  [--out-root <public-packet-root>] \
  [--tracker-url <github-issue-url>] \
  [--repo-root <repo-root>]
```

By default, the exporter reads from
`.adl/<version>/tasks/issue-<number>__<slug>/` and writes to
`docs/milestones/<version>/review/evidence/csdlc/issues/issue-<number>-<slug>/`.

The command copies `sip.md`, `stp.md`, `spp.md`, `srp.md`, and `sor.md` into a
public `cards/` directory, writes a machine-readable `manifest.json`, and writes
a packet `README.md`. The manifest separates tracker identity from
tracker-agnostic work-item identity so future Jira or other adapters do not have
to reinterpret GitHub issue fields as the only source of work identity.

The exporter refuses, rather than rewrites, source cards containing obvious
host-local absolute paths, secret-like tokens, private key markers, local scratch
paths, or unresolved template markers. Later redaction gates may add richer
review, but this first exporter must not silently sanitize away lifecycle truth.

## Validation Command

Public prompt packets are validated with the paired read-only command:

```bash
adl tooling public-prompt-packet validate \
  --packet <packet-dir-or-packet-root> \
  [--repo-root <repo-root>]
```

The command accepts either one packet directory containing `manifest.json` or a
packet root whose direct children contain public prompt packet manifests. It
fails closed when a packet is missing required manifest fields, has non-GitHub
tracker metadata for the current v1 packet shape, points public paths into
`.adl/`, references missing card files, contains unresolved template markers,
or contains obvious host-local paths, local scratch/worktree paths, private-key
markers, or secret-like tokens.

The validator treats `.adl/<version>/tasks/...` as allowable source provenance
only when it is recorded as a repo-relative task-bundle path. `.worktrees/`,
`.codex/`, absolute checkout paths, and temp/scratch paths are not valid public
packet provenance.

The gate also runs the structured prompt validators over all five exported
cards. `SIP`, `STP`, `SPP`, and `SRP` must satisfy their current structured
prompt contracts. `SOR` may satisfy either the completed-phase or bootstrap
structured contract so the gate can validate both completed public records and
freshly rendered sample packets without rewriting card truth.

## Execution Flow

1. Define/export prompt packets.
2. Inventory `.adl` local state.
3. Pilot packets and reviewer index.
4. Add validation/redaction gates.
5. Close umbrella with disposition truth.

## Determinism and Constraints

Public packet generation must not depend on host paths or hidden local state.

## Integration Points

- [../WBS_v0.91.5.md](../WBS_v0.91.5.md)
- [../MILESTONE_CHECKLIST_v0.91.5.md](../MILESTONE_CHECKLIST_v0.91.5.md)

## Validation

Validation should include prompt packet structure, manifest shape, tracker
metadata, redaction/path safety scanning, structured prompt-card checks, link
checks, and archive/deletion review checklist. The public packet validator is a
focused docs/tooling gate, not a runtime proof lane.

## Acceptance Criteria

- Public prompt packets are exportable and reviewable.
- Redaction and validation gates exist.
- `.adl` cleanup has review-before-delete disposition.

## Risks

- Local records may contain private or machine-specific content.
- Over-cleanup could lose useful historical evidence.

## Future Work

Future milestones can ingest archived records into ObsMem or publish curated
prompt corpora.

## Notes

This feature does not require all `.adl` history to become tracked repo state.
