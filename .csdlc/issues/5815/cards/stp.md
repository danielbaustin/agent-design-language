# Structured Task Prompt

Template: 1.0.0

Issue: 5815

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Finalize and review one repository migration runbook.

## Deliverables

- Current five-repository migration inventory
- Serial transfer and verification runbook
- Coordinated agent-logic.ai link cutover
- Gemini 3.1 Pro review and disposition

## Acceptance

1. Exactly five repositories are marked migrate
2. asksifu is retained personally and Horust is excluded
3. Existing agent-logic repositories are inventory-only
4. The website production and beta link updates are explicit
5. Each transfer has preflight, verification, stop, and rollback gates
6. Gemini findings are incorporated or dispositioned

## Dependencies

- Issue 5765 scheduling record
- Current GitHub source and destination inventories
- Current agent-logic.ai origin/main link inventory

## Inputs

- .adl/docs/TBD/AGENT_LOGIC_ACCOUNT_REPO_MIGRATION_PLAN.md
- AGENTS.md

## Non Goals

- Repository transfers
- Organization or billing changes
- Remote, website, Actions, package, DNS, secret, or permission mutation
