# Structured Intent Prompt

Template: 1.0.0

Issue: 4762

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Implement the #4762 auditable birth-witness register and receipt package for v0.91.8 to v0.92 handoff consumption without claiming that the v0.92 birthday occurred.

## Required Outcome

Retained witness, receipt, negative-case, validation, and handoff-consumption artifacts exist, validate deterministically, and preserve explicit non-claims for birthday occurrence, personhood, production citizenship, and v0.93 governance.

## Scope

- Retained #4762 schema, negative-case register, execution design, and validator under .csdlc/prepared/issues/4762/.
- Reviewer-facing witness register, receipt, README, and summary under docs/milestones/v0.91.8/review/v092_handoff_4762/.
- v0.91.8 and v0.92 handoff docs cite the package by exact path while preserving birth_event_status: not_claimed.
- Focused validation and exact-head review evidence retained under .csdlc/evidence/4762/.

## Authority

- Issue #4762 authorizes the birth-witnesses and receipt package only.
- This work may implement auditable handoff artifacts and validation scripts but may not start the future v0.92 birthday event.
- Publication may create a ready PR with Closes #4762; merge and post-merge closeout remain out of scope for this session.

## Assumptions

- The live reacquired claim binds this worktree and branch.
- The package is docs/artifact validation work, not runtime/product code.

## Operator Constraints

- Operate only in /Volumes/FastWork/adl-wp-4762 on codex/4762-v0918-wp14-preparation.
- Do not edit primary main and do not use /private/tmp.
- Use repaired v2 binaries from /Volumes/FastWork/adl-wp-5737/csdlc-v2/target/debug.
- Do not claim that the v0.92 birthday occurred.
