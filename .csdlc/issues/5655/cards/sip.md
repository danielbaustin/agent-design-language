# Structured Intent Prompt

Template: 1.0.0

Issue: 5655

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make every supported GitHub lifecycle action available through typed C-SDLC v2 Rust tools without connector or wrapper fallback.

## Required Outcome

A typed Rust command creates and reconciles issues, updates issue metadata, comments, labels, assignees, closes issues, and preserves existing PR publication, ready, merge, checks, and closeout paths with exact readback and fail-closed errors.

## Scope

- typed csdlc-github Rust command and request/response schemas
- existing shared token resolver and Octocrab boundary
- issue create/update/comment/label/assignee/close operations
- focused action and reconciliation tests
- operator skill documentation

## Authority

- C-SDLC v2 Rust binaries own GitHub lifecycle mutations
- GitHub is the remote system of record; typed local records remain lifecycle authority
- No Runtime, AWS, connector, legacy wrapper, or unrelated issue scope

## Assumptions

- none

## Operator Constraints

- Rust binaries only; no GitHub connector, raw gh, legacy wrappers, shell, Python, or AWS
- Do not reopen or overload closed issues
- Use existing token resolver without printing secrets
- Keep root main clean and work only in the bound worktree
- Fail closed on permissions, identity mismatch, ambiguity, and stale requests
