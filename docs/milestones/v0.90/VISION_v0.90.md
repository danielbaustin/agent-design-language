# Vision - v0.90

## Metadata

- Project: ADL
- Milestone: v0.90
- Version: v0.90
- Date: 2026-04-16
- Owner: Daniel Austin
- Status: tracked planning package

## Purpose

Define the intended shape of v0.90 as the tracked planning package promoted by
`v0.89.1` WP-19.

## Overview

v0.90 is the milestone where ADL should move from bounded single-run proof
surfaces into bounded long-lived agency.

The key change is continuity under supervision:

- agents can run across cycles
- each cycle is bounded and reviewable
- state and continuity are explicit artifacts
- operators can inspect, stop, and constrain the system
- demos show persistence without pretending the system has become unbounded

## Core Goals

v0.90 advances ADL in five areas:

1. long-lived supervisor and heartbeat
2. cycle contracts and durable artifacts
3. pre-identity continuity handles
4. operator control and safety
5. reviewable long-lived demos

## 1. Long-Lived Supervisor And Heartbeat

The runtime should be able to supervise one or more bounded agents over repeated
cycles.

The milestone should define:

- agent specs
- supervisor state
- lease and heartbeat behavior
- cycle scheduling
- timeout and stale-agent handling

## 2. Cycle Contracts And Durable Artifacts

Every long-lived step should produce artifacts that reviewers can inspect.

The milestone should define:

- cycle manifest
- observations
- decision request and result
- run references
- memory-write candidates
- cycle ledger entries

## 3. Pre-Identity Continuity Handles

v0.90 should create continuity handles that are useful now and migrate cleanly
later.

The milestone must not claim the full v0.92 identity model. Instead it should
create explicit handles, ledgers, and provider-binding history that v0.92 can
adopt or replace.

## 4. Operator Control And Safety

Long-lived agents need stronger operator authority than ordinary one-shot demos.

The milestone should provide:

- status inspection
- stop controls
- guardrail reports
- safe artifact sanitization
- clear no-financial-advice boundaries for the stock demo

## 5. Reviewable Long-Lived Demos

The stock league demo should prove the idea without becoming theatrical or
unsafe.

It should show:

- recurring supervised cycles
- evidence gathering
- bounded decisions
- memory and continuity across cycles
- operator-visible status and safety controls

## Scope Boundary

v0.90 should not absorb every future reasoning, signed-trace, query, temporal,
identity, or society concept.

Reasoning graph, signed trace, and trace query may be included only if the
`v0.89.1` WP-19 promotion gate chooses a narrow inspection slice that
directly supports the long-lived runtime story.

## Success Definition

v0.90 succeeds when reviewers can see a bounded long-lived agent run, inspect
its cycles and continuity artifacts, and verify that operator controls remain in
force across time.
