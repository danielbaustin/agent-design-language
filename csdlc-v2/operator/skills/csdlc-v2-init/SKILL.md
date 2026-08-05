---
name: csdlc-v2-init
description: Create and prepare claim-free C-SDLC v2 issue state from typed input, including migration of legacy preparation records.
---
Invoke `csdlc-issue create`, then `csdlc-prepare sync`, `seal`, or `run` with
typed request files. Use `csdlc-migrate preparation` or `repair` only for the
audited legacy route. These commands create no execution claim and never bind a
worktree. Do not edit Markdown/state, invoke shell/Python lifecycle logic, or
infer success from prose. The final `v1_sunset` selector makes the installed v2
generation the sole operational authority.
