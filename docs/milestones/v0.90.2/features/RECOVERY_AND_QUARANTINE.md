# Recovery And Quarantine

## Purpose

Define how Runtime v2 decides whether a failed state can safely resume or must
be quarantined.

## Recovery Allowed When

- the failed action did not mutate committed state
- trace and snapshot evidence are sufficient
- identity and temporal invariants remain valid
- operator policy allows resume

## Quarantine Required When

- committed state may be inconsistent
- identity may have forked or duplicated
- replay evidence is missing
- security-boundary violation risk remains unresolved

## Required Output

- recovery decision record
- quarantine artifact
- negative tests for unsafe resume
