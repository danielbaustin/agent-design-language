# Standing, Communication, And Access

## Status

Planning contract for v0.90.3.

## Purpose

Bind citizen state to actor standing, communication rights, inspection rights,
and access-control events.

## Standing Classes

v0.90.3 should make the following classes explicit:

- citizen: durable identity-bearing member of a polis
- guest: bounded temporary or externally originated participant
- service actor: runtime substrate actor with delegated authority but no social
  rights
- external actor: outside participant mediated through a gateway or guest path
- naked actor: prohibited unclassified actor with influence

## Core Rules

- Any actor with influence must have declared standing.
- Citizen and guest standing must differ in rights, duties, duration, trace,
  and continuity.
- A guest cannot silently acquire citizen rights.
- A service actor cannot become a hidden social actor through privileges.
- Communication is a governed action.
- Communication never grants inspection rights.
- Inspection, projection, migration, wake, quarantine, challenge, appeal, and
  decryption must be explicit access events.

## Access-Control Events

Required event families:

- inspection requested
- inspection denied
- decryption requested
- decryption denied
- projection requested
- projection denied
- wake requested
- wake denied
- migration requested
- migration denied
- quarantine requested
- quarantine release requested
- challenge raised
- appeal raised

## Acceptance Tests

- naked actor cannot communicate, observe, or affect shared state
- guest cannot silently acquire citizen continuity
- service actor cannot initiate arbitrary communication
- operator cannot inspect private state without an access event
- projection access does not become raw state access
