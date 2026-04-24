# Issue Splitter Output Contract

Issue-splitter artifacts must plan one bounded split decision without creating
follow-on issues, mutating cards, or hiding uncertainty.

## Required Markdown Sections

- `Issue Splitter Summary`
- `Classification`
- `Concern Buckets`
- `Current Scope Recommendation`
- `Proposed Follow-Ons`
- `Issue Graph Notes`
- `Recommended Handoff`
- `Non-Claims`
- `Safety Flags`

## Required JSON Shape

Schema id: `adl.issue_splitter_report.v1`

Required top-level fields:

- `schema`
- `run_id`
- `status`
- `classification`
- `summary`
- `concern_buckets`
- `current_scope_recommendation`
- `proposed_follow_ons`
- `issue_graph_notes`
- `recommended_handoff`
- `non_claims`
- `safety_flags`

Supported status values:

- `keep_as_is`
- `split_now`
- `defer`
- `blocked`

## Safety Flags

Every report must state:

- `issues_created: false`
- `cards_mutated: false`
- `tracker_mutated: false`
- `scope_silently_rewritten: false`
- `implementation_claimed: false`

This skill may recommend split actions. It must not claim that the split or
follow-on issue creation already happened.
