# Actor Authority Boundary Report

## Purpose

Show one allowed actor-standing shape and one blocked actor-standing shape for
the bounded `WP-05` polis contract.

## Allowed Fixture

- fixture: `fixtures/actor_standing_allowed.json`
- result: `allowed`
- reason:
  - operator retains non-delegable merge authority
  - closeout owner retains non-delegable closeout authority
  - conductor routes lifecycle state but does not gain merge or closeout power
  - implementation owner has bounded write authority and proof duties
  - reviewer and verifier roles are evidence-bound rather than inferred from
    chat participation

## Blocked Fixture

- fixture: `fixtures/actor_standing_blocked.json`
- result: `blocked`
- reason:
  - shard worker attempts to claim `merge_approval` authority without
    operator or reviewer standing
  - blocked actor also lacks required evidence references for authority
    elevation

## Review Takeaway

The bounded C-SDLC actor model is now explicit enough to distinguish legitimate
standing from hidden role inflation.
