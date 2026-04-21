# Portable Contract Normalizer Playbook

Use this playbook when contract tests, skill docs, review packets, or validation
artifacts may contain machine-local assumptions.

## Scan Targets

- Shell, Python, Markdown, YAML, JSON, TOML, and text contract surfaces.
- Review packets and generated reports only when explicitly in scope.
- Test fixtures that are intended to be portable across machines.

## Common Findings

- Absolute user paths such as `/Users/name/...`.
- macOS temp paths such as `/private/var/folders/...`.
- Generic temp paths such as `/tmp/...`.
- Hard-coded `.worktrees/adl-wp-1234` names.
- Assertions tied to one user, hostname, shell, or platform.
- Hard-coded skill inventory expectations that should derive from scanned
  bundles instead.

## Safe Fix Examples

- Replace `/Users/name/git/repo` fixture text with `<repo-root>`.
- Replace platform temp paths with `<temp-path>`.
- Replace `.worktrees/adl-wp-1234` with `.worktrees/adl-wp-<issue>`.

If a replacement changes the meaning of the test, report a design-decision
finding instead of applying it.

