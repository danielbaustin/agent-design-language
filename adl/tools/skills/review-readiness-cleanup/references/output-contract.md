# Review Readiness Cleanup Output Contract

Review readiness cleanup artifacts must classify structural review-cycle issues
without changing findings or approving readiness.

## Required Markdown Sections

- `Review Readiness Cleanup Summary`
- `Classification Counts`
- `Items`
- `Safe Mechanical Cleanup`
- `Blockers`
- `Skipped Surfaces`
- `Follow-On Needed`
- `Non-Claims`
- `Safety Flags`

## Required JSON Shape

Schema id: `adl.review_readiness_cleanup_report.v1`

Required top-level fields:

- `schema`
- `run_id`
- `status`
- `summary`
- `counts`
- `items`
- `recommended_handoffs`
- `non_claims`
- `safety_flags`

Supported item categories:

- `safe_mechanical_cleanup`
- `blocker`
- `skipped`
- `follow_on_needed`

Supported status values:

- `ready`
- `cleanup_needed`
- `blocked`
- `skipped`

## Safety Flags

Every report must state:

- `review_approved: false`
- `findings_rewritten: false`
- `published_report: false`
- `created_issues: false`
- `created_prs: false`
- `mutated_repository: false`

This skill may say a packet has cleanup findings. It must not claim the review
cycle is approved, complete, published, or remediated.

