# ADL Google Drive Context Mirror Runbook

This runbook operates the bounded ADL-to-Drive context mirror. Repository files remain canonical. The mirror never imports Drive content into tracked source and never deletes remote content.

## Production command

Install the current owner binaries through the repository installer, then run from the repository root:

```bash
ADL_GWS_LIVE_MODE=execute \
ADL_GWS_WRITE_APPROVAL=approved \
ADL_GWS_RECURSIVE_SYNC=enabled \
.adl/bin/adl-gws-context-mirror \
  --repo-root "$PWD" \
  --drive-root-folder-id 1BmGQeRK-W2_CUWy2I_r59iAMrWEtSq19 \
  --drive-seed-folder-id 1T66xWxk6v2LG3MFRzRiMRfQnqOZSu04Z
```

Execute mode selects `NativeWorkspaceDriveTransport`; fixture and dry-run modes select the in-memory transport. Execute mode does not substitute demo folder IDs. The command exits unsuccessfully if its report is empty, skipped, or contains any unverified result.

Before any Drive request, the command regenerates all four seed files from the current checkout. The generated sync index records every selected Markdown path and its SHA-256 digest, and the current-state packet derives its milestone truth from the repository README and activation ledger. Seed generation failure writes the durable failure report and prevents all Drive mutation; a stale ignored staging directory is never accepted as current input.

By default, execute mode recursively mirrors regular Markdown files below `docs/` and `.adl/docs/TBD/`. It preserves repository-relative folders under the configured Drive root. Symlinks, ambiguous same-name Drive children, parent-chain escapes, API failures, and content mismatches fail closed.

## Credentials and scopes

Use one operator-approved credential source outside the repository:

1. `ADL_GWS_TOKEN` for a short-lived injected token;
2. `ADL_GWS_CREDENTIALS_FILE` for an authorized-user or service-account JSON file;
3. `GOOGLE_WORKSPACE_CLI_CREDENTIALS_FILE`;
4. `$HOME/.config/gws/credentials.json`;
5. `GOOGLE_APPLICATION_CREDENTIALS`; or
6. application-default credentials at `$HOME/.config/gcloud/application_default_credentials.json`.

Do not print, copy, or commit credential contents. Listing and metadata reads request `drive.metadata.readonly`. Create, update, and exact content readback request `drive.file`. The command does not request full-Drive write scope.

## Success contract

A successful execute report must show all of the following:

- the four staged seed files were created, updated, or already identical;
- recursive status is `recursive_live` when recursive mirroring is enabled;
- every result has `verification_ok: true`;
- each result was verified by a post-write folder listing, Drive ID, bounded parent, name, MIME type, and exact downloaded bytes;
- the report records only the redacted credential source label and requested least-privilege scopes, never token material;
- no authentication, upload, listing, ambiguity, readback, or recursive traversal warning remains.

`unchanged` is a verified success: the remote bytes were downloaded and compared before avoiding an unnecessary update. Metadata-only verification is insufficient.

The machine-readable report defaults to `.adl/tmp/google_workspace_cms/adl_gws_context_mirror_report.json`. It contains paths, IDs, dispositions, byte counts, and safe diagnostic text, but not tokens or file contents.

## Focused validation

Run the deterministic local contracts without credentials or remote mutation:

```bash
cargo test --manifest-path adl/Cargo.toml adl_gws_drive_sync
cargo test --manifest-path adl/Cargo.toml adl_gws_context_mirror
cargo test --manifest-path adl/Cargo.toml --bin adl-gws-context-mirror
```

The recursive execute test uses the in-memory transport only as deterministic proof of path construction, idempotence, and exact-byte verification. It is not live deployment evidence.

## Controlled failures

- Authentication: run execute mode without an approved credential source. The command must exit unsuccessfully and must not expose credential material.
- Write approval: omit `ADL_GWS_WRITE_APPROVAL`. The report must be skipped/unverified and the command must exit unsuccessfully.
- Write/API failure: use a bounded test transport that rejects create or update. The run must return an error; it must not emit success.
- Readback mismatch: use a bounded test transport that returns bytes different from the uploaded source. The result must set `verification_ok: false` with `verification_mismatch`.
- Listing mismatch: use a bounded test transport whose metadata GET succeeds but whose parent listing omits the file. The result must fail verification.
- MIME mismatch: use a bounded test transport that reports a MIME type different from the requested type. The result must fail verification.
- Recursive pending: use dry-run or fixture mode with recursion enabled. The report must say `recursive_pending`, never `recursive_live`.

Do not induce live failures by corrupting production Drive files or credentials.

## Scheduled Codex automation

Automation `sync-adl-google-drive-context-mirror` stays paused until one live write plus list/content readback succeeds. Its task contract is:

- archive the Codex task only after all four seed results and all selected recursive results are verified and no warning or follow-up remains;
- leave one concise actionable task unarchived for any unresolved failure;
- before creating another visible failure signal, search for the newest unresolved task with the same normalized failure class and update/supersede that signal instead of accumulating duplicates;
- never archive an authentication, upload, listing, readback, parity, or recursive-sync failure.

The 2026-07-21 connector preflight created and read back `ADL_GWS_CONTEXT_MIRROR_LIVE_PROOF_5587.md` in the configured seed folder. That proves current connector write/list/content-read access; it does not by itself prove the native command has unattended credentials or that all four production seed files are current.
