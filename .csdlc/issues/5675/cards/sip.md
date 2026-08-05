# Structured Intent Prompt

Template: 1.0.0

Issue: 5675

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Add reliable Rust-native hosted adapter routes for Kimi/Moonshot and MiniMax.

## Required Outcome

Typed Kimi and MiniMax dispatch, bounded request contracts, provider error classification, focused tests, and live credentialed probe evidence.

## Scope

- adl/src/provider_adapter.rs
- adl/src/provider/profiles.rs

## Authority

- The adapter owns request routing, bounded payloads, response extraction, and typed provider failures; it does not provision credentials or alter lifecycle policy.

## Assumptions

- none

## Operator Constraints

- never write main
- use typed v2 binaries
- no raw gh
- no AWS
- no shell/Python lifecycle wrappers
