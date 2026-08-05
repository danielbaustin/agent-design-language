# Vector Provisioning

Runtime v3 #5691 uses the pinned Vector binary at `.adl/bin/vector`; it does
not rely on `PATH`, a mock process, Python, or a custom logger.

For this issue worktree the binary was provisioned from the repo-approved
installed Vector binary at
`/Users/daniel/git/agent-design-language/.adl/bin/vector` into the ignored
worktree-local operational path `.adl/bin/vector`.

Observed version:

```text
vector 0.56.0 (aarch64-apple-darwin 6817c02 2026-06-03 14:25:37.451398530)
```

Worktree-local binary SHA-256:

```text
efe1b8ca5ec22587e62d1772413dbd54d9131a30a0b6a5791c3e8977b9108f89
```
