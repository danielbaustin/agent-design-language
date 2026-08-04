# Deletion and Cutover

Deletion and default switch were executed through WP-12 and WP-13 for the
bounded v0.91.8 bridge.

Required inputs:

- reviewed shadow parity;
- opt-in soak;
- rollback proof;
- selector transaction proof;
- C-SDLC v2 acceptance from `#5358`;
- Runtime v3 acceptance from `#5361`;
- deletion eligibility manifest;
- post-deletion validation.

The retained WP-13 validation is
`docs/milestones/v0.91.8/evidence/wp13/5346-post-deletion-validation.v1.json`.
It records `status: pass` and 46,358 deleted lines against the 46,358-line
pinned denominator. This document does not approve additional deletion beyond
that retained evidence.
