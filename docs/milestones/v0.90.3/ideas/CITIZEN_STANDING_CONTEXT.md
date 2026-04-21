# Citizen Standing Context

## Purpose

v0.90.3 depends on a clear standing model.

The polis cannot safely host actors that communicate, observe, or affect shared
state without declared standing, bounded authority, and traceable identity.

## Standing Summary

- citizens are durable identity-bearing members of a polis
- guests are bounded temporary or externally originated participants
- service actors are runtime substrate actors, not social actors
- external actors must enter through governed border mechanisms
- naked actors are prohibited

## Communication Boundary

Communication is a governed action across identity boundaries. It is not raw
state access and does not imply a right of inspection.

Synchronous communication is especially sensitive because it can reveal timing,
attention, affect, and adaptation signals. Those are projections of interior
state, so communication artifacts need policy, trace, and redaction.

## Why This Belongs In v0.90.3

Citizen-state protection is incomplete if an actor can bypass standing or use
communication as an inspection channel. v0.90.3 should make those boundaries
executable before later milestones make stronger claims about citizens.
