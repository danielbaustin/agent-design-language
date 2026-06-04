# Local ADL State Disposition For Public Prompt Records

## Metadata

- Milestone: `v0.91.5`
- Issue: `#3473`
- Sprint lane: Sprint 1 public prompt records
- Status: `draft_for_review`
- Scope: local `.adl/` inventory and disposition planning
- Destructive actions performed: `none`

## Purpose

This document records a non-destructive inventory and disposition plan for
local `.adl/` state. It supports the public C-SDLC prompt-record transition by
separating durable public truth from local execution cache, historical
provenance, archive candidates, and sensitive or blocked material.

The rule for this issue is simple: inspect and classify, but do not delete,
move, ingest, or publish local `.adl/` content.

## Inputs Reviewed

- Local `.adl/` top-level directory inventory.
- Local `.adl/` size profile by top-level surface.
- Local `.adl/docs/TBD/` root and first-level planning corpus inventory.
- Local `.adl/reviews/` review packet and sprint-state inventory.
- Local `.adl/runs/` run artifact inventory.
- Local `.adl/v0.91.5/` current milestone card/body inventory.
- [TBD cleanup disposition](../../planning/TBD_CLEANUP_DISPOSITION_v0.91.2_3150.md)
- [TBD allocation map](../../planning/TBD_PLAN_ALLOCATION_v0.91.2_TO_v0.95.md)
- [Public prompt records feature](features/PUBLIC_PROMPT_RECORDS_v0.91.5.md)
- [v0.91.4 tracked workflow migration policy](../v0.91.4/C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md)
- [v0.91.4 active issue migration policy](../v0.91.4/features/ACTIVE_ISSUE_MIGRATION_POLICY.md)

## Inventory Summary

The current local `.adl/` tree contains execution cache, local planning
material, historical issue-card bundles, review packets, run artifacts, logs,
temporary files, and obvious cruft. The largest observed top-level surfaces are
listed below so future cleanup can focus on high-impact categories first.

| Local surface | Approx size | Observed role | Initial disposition |
| --- | ---: | --- | --- |
| `.adl/.cache` | `1.9 GB` | Build/tool/cache residue. | `safe_delete_candidate_after_operator_approval` |
| `.adl/trace-archive` | `57 MB` | Trace/archive material. | `archive_or_obsmem_candidate` |
| `.adl/reports` | `33 MB` | Local reports and manual review outputs. | `review_then_archive_or_promote_selected` |
| `.adl/runs` | `18 MB` | Runtime execution artifacts and shared ObsMem store. | `archive_or_obsmem_candidate` |
| `.adl/docs` | `10 MB` | Local planning corpus, mostly TBD source material. | `tracked_allocation_or_local_archive` |
| `.adl/cards` | `5.9 MB` | Legacy/local card surfaces. | `review_for_public_packet_or_archive` |
| `.adl/v0.91.1` through `.adl/v0.91.5` | `3.6-5.2 MB each for recent milestones` | Historical and active milestone task cards/bodies. | `selected_public_packet_export_then_local_archive` |
| `.adl/logs` | `1.5 MB` | Local logs. | `sensitive_review_before_archive_or_delete` |
| `.adl/reviews` | `1.2 MB` | Review packets, sprint states, gap-analysis packets. | `archive_or_promote_selected_review_evidence` |
| `.adl/issues` | `940 KB` | Local issue-control state. | `local_execution_cache` |
| `.adl/local_artifact_spill` | `648 KB` | Local spillover artifacts. | `blocked_review_before_action` |
| `.adl/scripts`, `.adl/skills`, `.adl/tools` | small | Local workflow helpers and skill material. | `review_before_promote_or_archive` |
| `.adl/tmp`, `.adl/.DS_Store`, `.adl/Cruft-file.txt` | small | Local temporary/cruft material. | `safe_delete_candidate_after_operator_approval` |

## Disposition Matrix

| Category | Examples | Disposition | Rationale |
| --- | --- | --- | --- |
| Public prompt packet candidates | `.adl/v0.91.5/tasks/issue-*`, selected recent milestone task bundles | Export selected packets through `adl tooling public-prompt-packet export`. | Durable C-SDLC truth should be tracked under milestone evidence, not hidden in local `.adl`. |
| Historical prompt-card archive | Older `.adl/v0.*` task bundles and legacy `.adl/cards` | Archive locally after selected public packet export and review. | Historical continuity matters, but not every local card should become public repo state. |
| Current issue-control cache | `.adl/v0.91.5/bodies`, `.adl/v0.91.5/tasks`, `.adl/issues`, `.adl/locks`, `.adl/state` | Keep local while active; archive or prune only after issue closeout review. | Active lifecycle state is operational and may be incomplete or sensitive. |
| TBD planning source corpus | `.adl/docs/TBD/*` | Follow tracked allocation map; promote only bounded docs through issues; otherwise local archive. | Planning notes are valuable source provenance but can contain stale or private assumptions. |
| Review packet provenance | `.adl/reviews/codebuddy/*`, gap-analysis packets, third-party review source packets | Promote selected final reports to tracked milestone review locations; archive source packets locally. | Final review evidence belongs in tracked docs; intermediate packets may contain local scope/context. |
| Runtime run evidence | `.adl/runs/*`, `.adl/trace-archive/*`, `.adl/reports/*` | Classify as ObsMem/archive candidates before deletion. | These can contain valuable trace, memory, and execution evidence. |
| Logs and local scratch | `.adl/logs`, `.adl/tmp`, `.adl/local_artifact_spill` | Sensitive review before archive/delete; do not publish raw. | Logs and scratch files can contain host paths, prompts, model output, or credentials. |
| Generated cache | `.adl/.cache` | Safe-delete candidate after explicit operator approval. | Cache volume is high and not durable public truth. |
| Obvious cruft | `.adl/.DS_Store`, `.adl/Cruft-file.txt`, generated paper intermediates in local TBD subtrees | Safe-delete candidate after explicit operator approval. | Disposable once no issue needs raw source evidence. |
| Blocked or sensitive | Any file with secrets, host-local paths, private logs, private keys, raw prompt transcripts, or unclear ownership | Blocked until operator review. | Public prompt transition must fail closed on privacy and provenance risk. |

## Archive And ObsMem Candidate Rules

Material should be considered for archive or ObsMem ingestion when it satisfies
at least one of these conditions:

- It records issue execution truth not yet represented in tracked `SIP`, `STP`,
  `SPP`, `SRP`, or `SOR` packets.
- It contains review findings, dispositions, or remediation evidence.
- It is a trace, run artifact, or transition memory packet that can support
  replay, retrieval, or future continuity evidence.
- It records historical planning decisions that are still referenced by current
  milestone docs.
- It has provenance value but should remain local/private until redacted.

Do not ingest or archive wholesale merely because a directory is large.
Archive decisions must be category-based and evidence-based.

## Safe-Deletion Candidates

These are candidates only. This issue performs no deletion.

| Candidate | Deletion precondition |
| --- | --- |
| `.adl/.cache` | Operator approval after confirming no active issue depends on local cache state. |
| `.adl/.DS_Store` | Operator approval. |
| `.adl/Cruft-file.txt` | Operator approval after confirming no issue references it as evidence. |
| `.adl/tmp` | Operator approval after checking no active issue run is using it. |
| generated LaTeX intermediates under local TBD paper notes | Paper-specific review confirms source and final artifacts are preserved elsewhere. |

## Blocked / Sensitive List

These categories must not be published or deleted without additional review:

- Raw `.adl/logs` content.
- Raw model/provider invocation material.
- Local artifact spill directories.
- Review packets that include private repo scan context, host-local paths, or
  raw model output.
- Any file matching secret-like token patterns, private key markers, SSH paths,
  unresolved prompt-template markers, or host-local absolute paths.
- Any active issue task/bundle for an open PR or in-progress worktree.

## Cleanup Sequencing Plan

1. Export selected current prompt packets through the public packet exporter.
2. Build the reviewer index over exported packets.
3. Add validation/redaction gates for public prompt packets.
4. Promote only curated review and planning evidence into tracked milestone
   docs.
5. Create a local-only archive plan for historical card bundles and review
   source packets.
6. Create a separate explicit operator-approved cleanup issue for safe-delete
   candidates.
7. Create a separate ObsMem-ingestion planning issue for trace/run/review
   categories with durable evidence value.

## Validation Plan

For this issue, validation should remain focused and non-destructive:

- `git diff --check`
- Markdown link check for this document and touched milestone docs.
- Redaction scan over tracked output for host-local paths, private key markers,
  common token prefixes, and unresolved prompt-template markers.
- Review that no `.adl/` files were added as tracked repository content.

## Non-Claims

- This document does not claim local `.adl/` cleanup has been performed.
- This document does not make local `.adl/` state canonical public truth.
- This document does not approve deletion of any local file.
- This document does not approve ObsMem ingestion.
- This document does not certify that every local `.adl/` file has been
  individually reviewed.

## Follow-On Routing

- `#3474`: pilot public prompt packets and reviewer index.
- `#3475`: add public prompt packet validation and redaction gates.
- Future cleanup issue: operator-approved safe-delete pass for cache and cruft.
- Future archive/ObsMem issue: reviewed ingestion plan for trace/run/review
  provenance categories.
