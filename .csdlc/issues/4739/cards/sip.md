# Structured Intent Prompt

Template: 1.0.0

Issue: 4739

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make Unity-MCP project and endpoint alignment deterministic and reviewable before any live Observatory proof is claimed.

## Required Outcome

A repository-safe probe proves that the intended Observatory Unity project, its active MCP endpoint, and a read-only MCP operation agree, or emits one precise fail-closed blocker without exposing secrets.

## Scope

- Unity project path and MCP endpoint identity resolution
- Repository-safe redacted alignment probe
- Deterministic parser and failure-classification tests
- Permission-safe endpoint liveness evidence
- Bounded operator runbook and WP-15 routing note

## Authority

- #4739 owns project/endpoint alignment and read-only MCP proof only
- #4741 owns editor liveness, batch-mode selection, and watchdog behavior
- #5332 owns Unity ILPP GetDomainName retry-loop diagnosis
- No scene staging, runtime contract generation, asset publication, investor rendering, or walkthrough capture
- No fixed MCP port assumption, cloud fallback, raw gh, broad process scan, or secret-bearing Unity user settings

## Assumptions

- none

## Operator Constraints

- Typed C-SDLC v2 lifecycle only
- Work only in the issue-bound v0.91.8 worktree
- Preserve the older dirty #4739 worktree unchanged
- Use repository binaries and permission-safe process checks
- No Unity mutation, raw gh fallback, cloud fallback, broad process scan, or secret exposure during preparation
