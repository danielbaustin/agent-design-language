# ADL Milestone Creator Output Contract

Milestone creation outputs must be reviewable, bounded, and explicit about what
was planned, moved, validated, and left gated.

## Required Markdown Sections

- `Milestone Creation Summary`
- `Setup Issue`
- `Target Milestone`
- `Planning Package`
- `Issue Routing`
- `Feature And Proof Coverage`
- `Downstream Handoff`
- `Validation Commands`
- `Review Results`
- `Non-Claims`
- `Residual Risks`

## Required JSON Shape

Schema id: `adl.milestone_creation_report.v1`

Required top-level fields:

- `schema`
- `run_id`
- `source_version`
- `target_version`
- `downstream_version`
- `setup_issue`
- `planning_package`
- `issue_routing`
- `feature_tracks`
- `validation_commands`
- `review_results`
- `non_claims`
- `residual_risks`

## Safety Flags

Every report must state:

- `release_approved: false`
- `milestone_released: false`
- `created_tags: false`
- `merged_prs: false`
- `deleted_history: false`
- `runtime_behavior_changed: false`

The skill may report that a milestone package is ready for review. It must not
claim release approval, milestone completion, or downstream readiness without
explicit review and closeout evidence.
