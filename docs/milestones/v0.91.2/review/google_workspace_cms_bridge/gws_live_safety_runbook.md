# GWS Live Safety Runbook

## Purpose

Make the Google Workspace CMS bridge safe enough for routine project use by
fixing one explicit contract for auth mode, scope, redaction, trace posture,
and skipped-state behavior.

Use this runbook whenever a project wants to move beyond fixture proof and
touch a real bounded Drive, Docs, or Sheets surface.

## Core Rules

- Git remains canonical repo truth.
- Workspace remains collaborative draft and management infrastructure.
- Fixture mode remains the default proving path for CI and PR validation.
- Live Workspace use is bounded, operator-visible, and optional.
- Missing auth, missing scopes, or unavailable `gws` classify as `skipped`, not
  silent failure and not proof of correctness.

## Allowed Auth Modes

- `fixture_only`
  - use for normal CI, tracked report regeneration, and proof maintenance
  - no live secrets or network access required
- `operator_oauth_user`
  - use for one human-owned bounded project workflow
  - record the auth mode only, never the token
- `service_account_scoped`
  - use only when the account is narrowed to the exact Drive/Docs/Sheets scope
  - treat write scopes as higher risk
- `external_cli_credential_store`
  - likely first live `gws` posture
  - record only that the CLI credential flow was used, not any secret-bearing
    path or material

## Minimum Scope Rules

- Inventory reads:
  - one explicit Drive folder only
- Docs snapshot reads:
  - one explicit document only
- Content-card reads:
  - one explicit sheet range only
- Content-card writes:
  - one explicit sheet range only
  - preview first
  - explicit operator approval required
- Revision-anchor recording:
  - one explicit reviewed source doc only

Do not broaden into ambient account-wide Drive, Docs, or Sheets authority.

## Redaction Rules

- Folder inventory:
  - metadata only
- Docs snapshot:
  - redact by default
  - never publish full private document bodies in public artifacts
- Content-card state:
  - publish only lifecycle metadata intended for repo-facing management
- Comments and suggestions:
  - record existence and linkage, not private discussion bodies
- Promotion packets:
  - record title, revision anchor, target repo path, and issue/PR linkage
  - do not leak document bodies

## Live Read Path

1. Confirm the project actually needs live Workspace access.
2. Declare the exact folder/doc/sheet scope.
3. Record the auth mode without recording any secrets.
4. Run bounded inventory/read operations first.
5. Emit metadata-only or redacted traces.
6. If auth, scope, network, or `gws` availability is missing, classify the
   live path as `skipped`.

## Live Write Path

1. Start from a bounded preview operation.
2. Require explicit operator approval before apply.
3. Limit the write to one declared sheet range or equivalent bounded surface.
4. Record revision anchors and issue/PR linkage before any promotion-oriented
   action.
5. Stop if revision mismatch or authority drift appears.

## Promotion Boundary

- Workspace may prepare a promotion packet.
- Workspace may not directly edit tracked repo files.
- Promotion remains issue-backed and PR-reviewed.
- Revision mismatch is a stop condition, not background noise.

## Skipped-State Taxonomy

- `live_mode_disabled`
- `gws_unavailable`
- `missing_auth`
- `missing_scopes`
- `network_unavailable`
- `operator_declined`

All of these are truthful live-path skips. None of them should be rewritten as
fixture success or hidden behind generic pass/fail language.

## Non-Claims

- This runbook does not authorize broad Workspace administration.
- This runbook does not make Workspace canonical repo truth.
- This runbook does not allow private document body export by default.
