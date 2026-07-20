# Issue 5494 design

## Goal

Complete the unfinished #5409 WP-07A acceptance boundary without rewriting its
premature terminal history.

## Runtime Contract

The production assembly owns a health observation for every declared component
and typed channel. Readiness is derived from those observations and fails closed
when any required observation is missing or unhealthy. Static topology metadata
remains descriptive and cannot make readiness green.

The production path executes component tasks through the existing Tokio
supervision primitive. A deterministic soak drives the assembled tasks and
channels over repeated cycles, injects a component failure, and proves the
resulting readiness transition and recovery.

Credential renewal retains the previous bearer for a bounded overlap window.
Current and previous generations are accepted only while active and unexpired;
explicit revocation rejects every generation immediately. Rotation and overlap
state remain private, atomic, and covered by expiry-boundary tests.

## Scope

- Runtime v2 supervision, topology assembly, readiness, and authentication.
- Focused Runtime v2 tests and retained WP-07A review evidence.
- No Runtime v3 or AWS changes.

