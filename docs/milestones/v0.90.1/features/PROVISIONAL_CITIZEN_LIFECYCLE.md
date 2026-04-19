# Provisional Citizen Lifecycle

## Purpose

Define the minimal citizen record needed for Runtime v2 foundation proof without
claiming true Gödel-agent birth.

## Lifecycle States

- proposed
- admitted
- active
- paused
- sleeping
- waking
- terminated
- rejected

## Record Minimum

Each provisional citizen should include:

- citizen id
- display name
- provisional status
- current lifecycle state
- manifold id
- created timestamp
- last wake timestamp if applicable
- memory/identity refs as placeholders or bounded handles
- policy boundary refs

## Required Invariants

- no duplicate active citizen id
- inactive citizens cannot execute episodes
- wake must pass rehydration validation
- termination must be recorded before resources are released

## Boundary

These are provisional engineering records. They are not yet true identity-bearing
Gödel agents with birthday semantics.
