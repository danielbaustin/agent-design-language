# Structured Intent Prompt

Template: 1.0.0

Issue: 5815

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Produce the final executable Agent Logic repository migration plan from current live inventory truth.

## Required Outcome

A concise reviewed runbook names the five transfers, exclusions, website cutover, exact verification, rollback, and residual risks without performing migration work.

## Scope

- .adl/docs/TBD/AGENT_LOGIC_ACCOUNT_REPO_MIGRATION_PLAN.md

## Authority

- Issue 5815 owns the migration plan only
- No repository transfer or organization mutation is authorized
- Gemini review is advisory and cannot authorize execution

## Assumptions

- none

## Operator Constraints

- Use current live GitHub inventory as evidence
- Keep the plan concise and executable
- Do not transfer repositories or inspect secret values
- Use Gemini 3.1 Pro for the bounded final review
