---
name: csdlc-v2-validate
description: Execute declared PVF validation DAGs with typed evidence.
---
Use `csdlc-schedule` for read-only classification. For routine issue execution, use `csdlc-validate --root <worktree> finalize --request <shared-git-request>` so execution, passing validation, and `Implemented` are one atomic state transition. Keep the one-shot request at the Git-common path `.git/csdlc-v2/requests/<issue>.json` and overwrite it. Do not embed shell command strings or treat skipped/pending proof as passed.
