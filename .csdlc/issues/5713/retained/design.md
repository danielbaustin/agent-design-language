# Issue #5713 Design: Stable Local Runtime TLS Bootstrap

## Goal

Runtime v3 local development can create and reuse one durable self-signed
localhost server certificate per configured absolute state root. The operator
can trust the public certificate once, while production and managed
deployments continue to use externally supplied PEM material.

## Scope

- Add a configuration-selected TLS certificate source with explicit
  `managed_external` and `local_self_signed` modes.
- Implement rcgen-backed local self-signed certificate bootstrap in Rust under
  the configured Runtime v3 state root.
- Persist the private key and public certificate separately, with restrictive
  permissions for the key on Unix and best-effort cross-platform behavior.
- Reuse an existing valid local certificate on ordinary restarts.
- Replace a local certificate only through an explicit atomic replacement
  operation that keeps the last valid certificate on failure.
- Keep externally managed PEM paths supported without mutation.
- Document the one-time trust action and replacement tradeoff.

## Non-Goals

- No AWS Private CA or local CA hierarchy.
- No TLS verification weakening or trust-store mutation by Runtime v3.
- No wrapper, OpenSSL, shell, Python, or simulated server certificate path.
- No public Runtime or Observatory API contract change beyond deployment docs.
- No #5733, WP-21, or unrelated lifecycle record changes.

## Implementation Shape

The Runtime v3 init schema remains the authority for TLS deployment. A new
deployment mode on `api.tls` selects either externally managed certificate/key
paths or local self-signed material. Local mode requires an absolute
`state_root`, relative path children beneath `paths.tls_dir`, configured DNS
and IP SANs, and explicit replacement intent. The bootstrap function validates
existing PEM material with rustls before reuse, generates with rcgen only when
the local files are absent or replacement is requested, writes to temporary
siblings, fsyncs where practical, and atomically renames into place.

## Validation Plan

- Focused Rust tests in `adl-runtime-kernel` prove first bootstrap, restart
  reuse, SAN validation, restrictive key permissions, concurrent bootstrap
  exclusion, explicit atomic replacement, failed replacement preservation, and
  externally managed preservation.
- Runtime TLS loading continues through rustls with the generated certificate.
- `git diff --check`, crate fmt, and focused tests run from the issue worktree
  with state under the worktree or `/Volumes/FastWork`.

## Review Prompts

- Can production config ever implicitly create a local development certificate?
- Can externally managed certificate/key paths be mutated by local bootstrap?
- Are private key bytes ever logged, copied to evidence, or written with broad
  permissions?
- Does restart reuse preserve certificate identity and make replacement
  explicit and atomic?
- Are DNS/IP SANs and server-auth usage represented in the generated
  certificate and accepted by rustls?
