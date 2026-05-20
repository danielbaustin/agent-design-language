# TBD Cleanup Disposition For Issue 3150

## Status

Tracked cleanup disposition for `#3150`.

## Purpose

Record the cleanup decisions from the `.adl/docs/TBD/` inventory pass without
promoting ignored workspace residue into tracked repository truth.

The canonical allocation map remains
[`TBD_PLAN_ALLOCATION_v0.91.2_TO_v0.95.md`](TBD_PLAN_ALLOCATION_v0.91.2_TO_v0.95.md).

## What Changed In Tracked Docs

- Added a tracked v0.92 feature home for ACIP binary/protobuf schema,
  public schema catalogs, deterministic JSON projection, governed message
  access, and optional WebSocket carrier proof.
- Updated the v0.92 README, design doc, WBS, and feature README so the ACIP
  transport-readiness slice is visible before v0.92 issue seeding.
- Updated the canonical feature list so ACIP binary/schema transport is not
  hidden in local TBD notes.
- Updated the tracked TBD allocation map so local ACIP schema-catalog and
  WebSocket source notes have a tracked milestone home.
- Normalized `.adl/docs/TBD/LOCAL_BACKLOG.md` locally so current product
  references use CodeFriend, while preserving legacy `codebuddy_ai/` path
  names as source provenance.

## Local-Only Cleanup Disposition

The following cleanup targets live under ignored `.adl/` workspace state. They
are real local hygiene work, but deleting or moving them is not visible in a
tracked PR.

| Local-only surface | Disposition |
| --- | --- |
| `.adl/docs/TBD/TBD_DOC_STATUS_INVENTORY.md` | Refresh from the tracked allocation map during the next local-only TBD sync. |
| `.adl/docs/TBD/LOCAL_BACKLOG.md` | Locally normalized current product naming to CodeFriend while preserving `codebuddy_ai/` source paths as historical source lineage. |
| `.adl/docs/TBD/V0911_*` review findings | Move to local retired/provenance after confirming their remediation issues are closed. |
| `.adl/docs/TBD/v0.91_gap_review.md` and `.adl/docs/TBD/v0.91.1_gap_review.md` | Move to local retired/gap-review provenance. |
| `.adl/docs/TBD/.DS_Store` | Safe local deletion. |
| `.adl/docs/TBD/test-logs.txt` | Safe local deletion once the tracked slow-test diagnostics no longer need the raw source log. |
| `.adl/docs/TBD/general-intelligence-paper/*.aux`, `*.log`, `*.out`, `*.blg` | Safe local deletion as generated LaTeX intermediates if the paper repo/source packet no longer needs them. |

## Not Moved In This PR

This PR does not move ignored `.adl/docs/TBD/` files into tracked `docs/`
archives. The source files are local workspace material, and the safer tracked
outcome is to record their disposition and keep canonical roadmap truth in
tracked planning docs.

## Follow-On Candidates

- Open a local-hygiene-only pass if the operator wants physical cleanup of
  ignored `.adl/docs/TBD/` files.
- Open a CodeFriend product-roadmap issue if
  `.adl/docs/TBD/CODEFRIEND_PRODUCT_MILESTONE_PLAN_V0912_TO_V095_2026-05-19.md`
  should become tracked roadmap truth.
- Open an ADR authoring issue for `.adl/docs/TBD/V0912_ADR_PLAN_2026-05-13.md`
  if the ADR plan should move from source planning into execution.
- Open a future architecture or publication issue for the Axiom of
  Constructability when the target surface is concrete.

## Non-Claims

This disposition does not implement ACIP protobuf, WebSocket transport, schema
catalog runtime lookup, or message-access runtime enforcement.

This disposition does not claim local ignored TBD files were physically deleted
by the tracked PR.
