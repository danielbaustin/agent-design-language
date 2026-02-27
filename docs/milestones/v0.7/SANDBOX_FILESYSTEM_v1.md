# Sandbox Filesystem v1 (v0.7)

## Purpose

Define deterministic, security-preserving path validation for sandboxed reads/writes.

## Canonicalization Rules

- Sandbox validation is based on canonical (resolved) filesystem paths.
- Relative user paths are interpreted under the configured sandbox root.
- Absolute user paths are denied by policy.
- `..` traversal segments are denied by policy.
- Read and write validation follow the same canonicalization and boundary checks.

## Symlink Policy

- If symlink traversal is allowed by policy, symlinks may be followed only when the resolved target remains inside sandbox root.
- If a symlink (or canonicalized ancestor) resolves outside sandbox root, validation fails with `sandbox_escape_attempt`.
- If policy disallows symlink traversal, validation fails with `sandbox_symlink_disallowed`.

## Stable Error Codes

Sandbox path validation emits stable reason codes:

- `sandbox_path_denied`: empty path, absolute path, or `..` traversal denied by policy.
- `sandbox_path_not_found`: candidate path (or canonicalization target) does not exist.
- `sandbox_path_not_canonical`: path cannot be canonicalized deterministically.
- `sandbox_symlink_disallowed`: symlink traversal is blocked by policy.
- `sandbox_escape_attempt`: canonicalized path escapes sandbox root.
- `sandbox_io_error`: other IO errors during validation.

Errors include:

- stable `code`
- safe user-facing `message`
- optional sanitized structured fields (`requested_path`, `resolved_path`)

## Privacy and Redaction

- Absolute host filesystem paths are never emitted in sandbox error payloads.
- Paths are represented in sanitized sandbox form, for example `sandbox:/foo/bar.txt`.
- Outside-root resolutions are redacted as `sandbox:/<outside-root>`.

## How To Fix Common Errors

- `sandbox_path_denied`: use a non-empty relative path under sandbox root; remove absolute paths and `..` segments.
- `sandbox_path_not_found`: create the expected file/directory inside sandbox root before execution.
- `sandbox_path_not_canonical`: fix invalid filesystem entries and retry with a normal relative path.
- `sandbox_symlink_disallowed`: either use a non-symlink path or allow symlink traversal in policy-controlled contexts.
- `sandbox_escape_attempt`: ensure symlinks and targets remain inside sandbox root.
- `sandbox_io_error`: inspect local filesystem permissions/state and retry.

## Security Notes

This is path-hardening, not full kernel-enforced isolation. Resolution checks are best-effort and intentionally fail closed.
