# v0.91.6 Native Google Workspace Drive Sync And Context Mirror Review

Issue: `#4505`
Status: `retained_singleton_review`
Date: 2026-06-24
Scope: `#4406`

## Findings

### P1: Native Drive method defaults overgrant full-Drive scope instead of preserving the bounded least-privilege contract

The native Google Workspace integration advertises bounded scope control and
explicit parity with the `v0.91.2` bridge, but the built-in Drive method
catalog currently defaults list, get, create, and update operations to the full
`https://www.googleapis.com/auth/drive` scope rather than the narrower
`drive.file`-style contract expected for bounded file sync.

Evidence:

- `adl/src/adl_gws_native.rs` defines:
  - `ADL_GWS_DEFAULT_SCOPE = "https://www.googleapis.com/auth/drive"`
  - `ADL_GWS_SCOPE_DRIVE_FILE = "https://www.googleapis.com/auth/drive.file"`
- The `default_drive_method_catalog()` implementation assigns
  `ADL_GWS_DEFAULT_SCOPE` to every Drive list/get/create/update method instead
  of using a narrower per-operation scope mix.
- The implementation plan and `v0.91.2` bridge docs both explicitly frame the
  integration as bounded and least-privilege rather than ambient broad Drive
  authority.

Required remediation:

- follow-on issue `#4512` now owns the least-privilege fix that must narrow the
  default Drive scopes to the bounded mirror/file-sync contract, or explicitly
  revise the docs/non-claims if broad Drive scope is intentionally required.
- As of 2026-06-24, `#4512` has now published PR `#4515`, which carries the
  concrete code patch that narrows both the default Drive method catalog and
  the discovery-derived Drive method descriptor path to bounded metadata-read
  and `drive.file` scopes. The repair is therefore in review, while this
  retained packet still correctly reports the merged `v0.91.6` surface as
  broader than claimed.

## Scope

This packet reviews the `#4406` native Google Workspace integration as a
sensitive connector-adjacent singleton touching external document state,
credentials, and context mirror reporting.

## Review Result

The implementation is directionally sound on architecture and safety posture,
but not yet fully consumable as a least-privilege Workspace bridge because the
default scope contract is broader than claimed.

What looks good:

- native integration is split into bounded subsystems:
  `adl_gws_native`, `adl_gws_drive_sync`, and `adl_gws_context_mirror`
- fixture-backed ordinary validation remains the default posture
- execute mode requires explicit write approval before mutation
- context mirror reports preserve non-claims that Workspace is not canonical
  repo truth and that recursive mirroring is not claimed unless explicitly live
- reports avoid recording raw token values

## Surfaces Reviewed

Code:

- `adl/src/adl_gws_native.rs`
- `adl/src/adl_gws_drive_sync.rs`
- `adl/src/adl_gws_context_mirror.rs`
- `adl/src/bin/demo_adl_gws_native_drive_sync.rs`
- `adl/src/bin/demo_adl_gws_context_mirror.rs`

Planning / baseline docs:

- `.adl/docs/TBD/google_workspace_cms/ADL_NATIVE_GOOGLE_WORKSPACE_INTEGRATION_PLAN.md`
- `docs/milestones/v0.91.2/features/GOOGLE_WORKSPACE_CMS_BRIDGE.md`

Merged entrypoint:

- `86b99d4c` `[v0.91.6]native-google-workspace-drive-sync-and-context-mirror (#4430)`

## Security / Operational Assessment

- Secret handling:
  auth source reporting is present without writing token values into reports.
- Write control:
  execute mode is gated by `ADL_GWS_WRITE_APPROVAL`.
- Canonical truth:
  non-claims correctly reject Workspace as canonical repo truth.
- Path hygiene:
  reports and artifacts write under repo-owned `.adl/tmp/google_workspace_cms/`
  rather than ambient home-directory locations.
- Residual risk:
  default Drive scope selection is too broad for the bounded contract currently
  claimed by the feature and implementation-plan docs.

## Validation And Review Coverage

Review lanes exercised:

- code
- docs
- security

Focused local checks for this packet:

```text
git diff --check
```

This review did not perform live Google Workspace writes.

## Non-Claims

- This packet does not claim live auth/execute acceptance was reproved locally.
- This packet does not claim recursive docs mirroring is implemented.
- This packet does not claim Workspace is canonical repo truth.
- This packet does not approve broad ambient Drive authority.

## Closeout Position

`#4505` is satisfied as a retained singleton review once this packet and the
issue-local review records land, because the least-privilege defect is now
durably identified and routed.

What remains open is implementation publication: follow-on issue `#4512` still
needs to merge PR `#4515` before the native Workspace surface can be treated as
fully review-clean.
