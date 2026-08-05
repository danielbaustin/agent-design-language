# #5678 Opus review runbook repair

## Goal

Make the operator runbook describe the current Rust `adl-provider-adapter`
JSON request interface and provide a bounded, source-grounded Opus review
procedure.

## Scope

- the Opus runbook;
- a focused repository contract check for its command/schema claims;
- no provider implementation or lifecycle behavior changes.

## Acceptance

1. The runbook uses `--request`, `--out`, and `--log`.
2. The JSON example matches the current adapter request shape and records how
   provider/model identity is verified.
3. Credential handling remains one-command, `$HOME/keys`-based, and secret-free.
4. A focused check fails when the documented interface drifts.

## Non-goals

No provider calls, no credential inspection, no AWS, and no broad documentation
rewrite.
