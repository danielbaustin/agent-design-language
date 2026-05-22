# CodeFriend Planning Home

## Status

Tracked planning home for current CodeFriend work.

Current milestone planning assumption: CodeFriend alpha should become a
dedicated product milestone, likely in the `v0.93.x` band. The exit bar for
that milestone is a fully working alpha version of CodeFriend ready for
operator/customer-style testing.

Near-term setup assumption: the CodeFriend pre-alpha repo and S3-backed
CloudFront landing-page mini-sprint should execute in `v0.91.4`, not
`v0.91.3`. The `v0.91.3` work records the plan only so the current milestone
does not expand at close.

Planning must also extend beyond `v0.93.x`: the alpha milestone should prove a
usable first product, while later CodeFriend roadmap bands add the remaining
structural-intelligence, architecture-governance, fitness-function,
organizational-memory, and product-delivery features.

CodeFriend is the current product name. `CodeFriend.ai` is the current domain
name. Older `CodeBuddy` / `codebuddy` references are historical working-name
lineage unless a later issue explicitly promotes them to current product copy.

## Purpose

Keep CodeFriend planning in one tracked location instead of spreading product
strategy, naming migration, setup notes, and source maps across milestone docs
and local-only notes.

## Documents In This Home

- [CodeFriend setup plan](CODEFRIEND_SETUP_PLAN.md)
- [CodeFriend reference inventory](CODEFRIEND_REFERENCE_INVENTORY.md)
- [CodeFriend pre-alpha repo and S3 welcome mini-sprint](CODEFRIEND_PRE_ALPHA_REPO_AND_S3_WELCOME_MINI_SPRINT.md)

## Source Baseline

Current tracked baseline:

- [CodeFriend productization feature](../../milestones/v0.91.2/features/CODEFRIEND_PRODUCTIZATION.md)
- [CodeFriend review-packet workflow package](../../milestones/v0.91.2/review/codefriend_productization/review_packet_workflow_package.md)
- [CodeFriend product report template](../../milestones/v0.91.2/review/codefriend_productization/product_report_template.md)
- [CodeFriend evidence requirements](../../milestones/v0.91.2/review/codefriend_productization/evidence_requirements.md)
- [CodeFriend skill and demo alignment](../../milestones/v0.91.2/review/codefriend_productization/skill_demo_alignment.md)
- [ADR 0025 candidate](../../architecture/adr/0025-codefriend-review-packet-product-boundary.md)
- [ADL feature list](../ADL_FEATURE_LIST.md)

Operator-provided local input used for this planning pass:

- `.adl/docs/TBD/codebuddy_ai/CODEFRIEND_NOTES.md`

That local note is not required to read this package; the relevant planning
content is summarized in the tracked setup plan.

Explicitly ignored for this pass:

- `.adl/docs/TBD/codebuddy_ai/ADL_MULTIAGENT_INVESTIGATE_ENHANCE.md`

## Placement Rule

Future CodeFriend planning docs should go under `docs/planning/codefriend/`
unless they are milestone-specific proof artifacts. Milestone proof artifacts may
stay inside their milestone package, but this directory should link to them so
operators have one durable CodeFriend entry point.

## Boundary

This home does not claim CodeFriend is a shipped product. It records planning
for product setup, naming cleanup, architecture-cognition positioning, and a
future `v0.93.x` alpha milestone whose goal is a working testable product. It
also records that later CodeFriend planning must continue past the alpha to
complete the full feature set.
