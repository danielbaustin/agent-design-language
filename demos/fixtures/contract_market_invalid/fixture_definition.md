# Fixture Definition: Contract Market Invalid Packet

## Purpose

This negative packet exists for later runner denial cases.

It intentionally violates the v0.90.4 bounded contract-market rules so WP-12
and WP-14 can prove fail-closed behavior without inventing new invalid fixture
formats later.

## Included Invalid Cases

- tool requirement that incorrectly grants execution authority
- completion event that omits required artifact refs

These fixtures are intentionally invalid and must not be normalized into valid
execution packets by the WP-11 validation protocol.
