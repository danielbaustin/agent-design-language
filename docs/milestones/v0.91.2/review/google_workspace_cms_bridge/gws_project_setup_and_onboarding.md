# GWS Project Setup And Onboarding

## Purpose

Provide the minimum bounded setup path for a new CodeFriend/ADL project that
wants to use the Google Workspace CMS bridge.

## Inputs To Gather

- one bounded Drive folder ID
- one bounded Google Doc ID
- one bounded Google Sheet ID
- one bounded content-card sheet range
- the project's GitHub issue/PR workflow entrypoints

## Required Environment Surface

The current bounded bridge expects these env vars:

- `ADL_GWS_LIVE_MODE`
- `ADL_GWS_DRIVE_FOLDER_ID`
- `ADL_GWS_DOC_ID`
- `ADL_GWS_SHEET_ID`
- `ADL_GWS_SHEET_RANGE`
- `ADL_GWS_WRITE_APPROVAL` when a bounded live sheet update is intentionally allowed

Recommended initial posture:

- `ADL_GWS_LIVE_MODE=dry_run`

Do not start a project in execute mode by default.
Do not set `ADL_GWS_WRITE_APPROVAL` by default; add it only immediately before
an intentionally bounded live write.

Auth is required for live bounded use, not for fixture-first or dry-run-only
adoption proof.

## Onboarding Steps

1. Confirm the project actually needs Workspace-backed draft/content-card flow.
2. Confirm GitHub remains the canonical planning and repo-change authority.
3. Decide whether the project is adopting the bridge in:
   - fixture-first / dry-run posture, or
   - live bounded posture
4. If live bounded posture is intended, install and authenticate `gws`
   explicitly as the operator.
5. Bind the narrow folder/doc/sheet scope values.
6. Run the bounded live-safety package first.
7. Run the bounded live capability execution surface second.
8. Run the content-card roundtrip surface after the first two surfaces are in
   place; dry-run or truthful skipped output is acceptable when the project is
   not yet attempting live mutation.
9. Save the resulting proof artifacts with the project packet.

## First Commands To Prefer

Use the bounded demo/report surfaces already present in ADL:

- `cargo run --manifest-path adl/Cargo.toml --bin demo_v0912_gws_live_safety_package`
- `cargo run --manifest-path adl/Cargo.toml --bin demo_v0912_gws_live_capability_execution_surface`
- `cargo run --manifest-path adl/Cargo.toml --bin demo_v0912_gws_live_content_card_roundtrip`

Use dry-run mode first unless there is a clear reason to attempt execute mode.

## What A Healthy First Run Looks Like

Healthy does not always mean proving live writes. A healthy first run may be:

- proving in dry-run mode
- skipped with a truthful auth/scope/tooling reason
- proving bounded reads while still skipping writes

That is acceptable if the recorded reason is explicit and reviewable.

## Project Handoff Expectation

The operator taking over the project should be able to answer:

- which folder/doc/sheet scope is allowed
- whether the bridge is dry-run only or execute-capable
- what content-card mutation path exists
- what still must go through GitHub issue/PR controls
