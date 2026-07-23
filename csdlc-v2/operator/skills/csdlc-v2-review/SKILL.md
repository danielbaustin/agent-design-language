---
name: csdlc-v2-review
description: Assign and record exact-revision pre-publication review truth.
---
Invoke `csdlc-review record` with evidence naming the reviewer, exact scope, and exact clean scoped revision. A passing record atomically advances to `Reviewed`; routine review does not require `assign`. Existing assignment records remain valid compatibility evidence. If reviewed work becomes stale before publication, use typed `recover` to return it to `implemented`, preserving the audit trail before re-review. Keep the one-shot request at the Git-common path `.git/csdlc-v2/requests/<issue>.json` and overwrite it.
