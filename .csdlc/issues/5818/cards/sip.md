# Structured Intent Prompt

Template: 1.0.0

Issue: 5818

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Complete WP-01B: Canonical documentation and version activation.

## Required Outcome

v0.92 current-version truth across docs/planning/ADL_FEATURE_LIST.md, canonical docs, READMEs, manifests, Cargo metadata, skills, and runbooks

## Scope

- README.md and docs/README.md current entrypoints
- docs/planning/ADL_FEATURE_LIST.md and current planning indexes
- docs/milestones/v0.92 current feature, quality, demo, review, and execution entrypoints
- root and workspace Cargo.toml files, Cargo.lock, and user-visible package version metadata
- AGENTS.md, REVIEW.md, csdlc-v2/operator/skills, and current docs/tooling runbooks
- .csdlc/issues/5818, .csdlc/prepared/issues/5818, and .csdlc/evidence/5818

## Authority

- Issue 5818 owns current v0.92 documentation and authoritative version activation only
- Historical milestone, release, review, migration, and evidence records retain their original claims
- Feature owners retain implementation, proof, release, and closeout authority
- Generated files change only through their owning generator

## Assumptions

- none

## Operator Constraints

- Prepare before execution
- Never edit tracked work on main
- Use one bounded pre-PR review
- Do not substitute fixtures, receipts, or prose for required working behavior
