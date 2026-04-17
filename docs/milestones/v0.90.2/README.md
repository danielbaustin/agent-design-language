# v0.90.2 Runtime v2 Hardening

## Purpose

v0.90.2 hardens the Runtime v2 foundation after the first prototype exists.
The milestone should make the substrate more trustworthy, reviewable, and safe
under failure and adversarial pressure.

## Scope

In scope:

- invariant test expansion for normal, failure, and recovery paths
- stable violation artifacts
- recovery and quarantine semantics
- stronger operator review surfaces
- security-boundary proof surfaces
- governed adversarial verification hooks where they defend the polis

Out of scope:

- first true Gödel-agent birthday
- replacing v0.91 moral and emotional work
- turning red/blue/purple roles into the definition of CSM

## Acceptance Boundary

The milestone succeeds when Runtime v2 can show not only that it runs, but that
it fails legibly, recovers safely when allowed, and preserves evidence when it
must stop.

## Roadmap Link

See ../ROADMAP_RUNTIME_V2_AND_BIRTHDAY_BOUNDARY.md for the full milestone
placement and boundary model.
