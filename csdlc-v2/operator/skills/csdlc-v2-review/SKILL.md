---
name: csdlc-v2-review
description: Assign and record exact-revision pre-publication review truth.
---
Invoke `csdlc-review`. Publication remains blocked until current review evidence exists and all actionable findings are dispositioned. `assign` requires a clean substantive commit; if a reviewed record becomes stale before publication, use typed `recover` to return it to `implemented`, preserving the audit trail before re-review.
