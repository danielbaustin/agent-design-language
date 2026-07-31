# Structured Task Prompt

Template: 1.0.0

Issue: 5692

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement and prove only the closing-keyword policy and publication verifier guard for #5692.

## Deliverables

- AGENTS.md states implementation PR bodies must include the correct GitHub closing keyword
- csdlc-publish rejects request and remote PR bodies that merely mention #<issue> without a closing keyword
- Focused tests cover accepted and rejected linkage forms

## Acceptance

1. AC-1 AGENTS.md requires every implementation PR body to include the correct GitHub closing keyword, normally Closes #<issue>
2. AC-2 csdlc-publish publication request validation rejects body-only issue mentions such as Related #5692
3. AC-3 csdlc-publish remote validation rejects existing PRs or remote observations without a valid closing keyword for the issue
4. AC-4 Merge readiness can rely on GitHub auto-closing because the governed PR body contains the closing keyword; typed closeout remains nonblocking after issue closure
5. AC-5 The PR opened for this issue includes Closes #5692 in its body

## Dependencies

- Existing csdlc-publish publication verifier
- Existing csdlc-github issue close/read support

## Inputs

- AGENTS.md
- csdlc-v2/src/publication.rs
- csdlc-v2/tests/gate6.rs
- .github/workflows/ci.yaml

## Non Goals

- Workflow rewrite
- Receipt or closeout redesign
- New GitHub connector dependency
- Changing legacy v1 wrappers
