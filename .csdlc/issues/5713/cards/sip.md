# Structured Intent Prompt

Template: 1.0.0

Issue: 5713

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make Runtime v3 local development create and reuse one durable rcgen self-signed localhost server certificate per configured absolute state root.

## Required Outcome

Runtime v3 selects managed external PEM or local self-signed TLS explicitly; local mode creates durable rustls-accepted rcgen material once, reuses it across restarts, and replaces it only through an explicit atomic operation that preserves the last valid certificate on failure.

## Scope

- repo-native Runtime v3 local TLS bootstrap configuration schema
- rcgen-backed local self-signed certificate bootstrap binary
- restrictive private key persistence and public certificate export
- restart reuse and explicit atomic replacement
- externally managed PEM preservation
- focused cross-platform Rust tests and local trust documentation

## Authority

- Issue #5713 owns only issue-local lifecycle records, the new Runtime v3 local TLS bootstrap binary/module, focused local TLS tests, and Runtime v3 TLS documentation
- No private AWS CA, local CA hierarchy, OpenSSL wrapper, shell/Python certificate generation, or TLS verification weakening is in scope
- Issue #5733, WP-21 paths, production credentials, and externally managed certificate material are out of scope

## Assumptions

- none

## Operator Constraints

- Typed C-SDLC v2 lifecycle only
- No tracked edits on primary main
- Use an issue-bound worktree and branch
- Never use /private/tmp
- Do not touch #5733 or WP-21 paths
- No tracked private keys or certificate evidence copies
- Publish only after exact GPT-5.5 pre-PR review if that reviewer is actually available
