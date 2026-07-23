---
name: csdlc-v2-publish
description: Publish only after current pre-publication review truth.
---
Invoke `csdlc-publish publish` with `draft: false` for the routine path; it creates and records one exact ready PR directly. Existing governed draft publications may still use the bounded ready reconciliation commands, but new routine work must not create a draft first. Keep the one-shot request at the Git-common path `.git/csdlc-v2/requests/<issue>.json` and overwrite it. Do not publish on missing/stale review, ambiguous remote state, or prose-only approval.
