# Accelerated Gate 10D operator decision

Issue: #5306

The 14-day rollback and 30-day importer windows in Gate 10C were conservative
planning defaults, not operator-specified dates. On 2026-07-14 the operator
explicitly directed: “get moving, I want it all done tonight” and required
“100% parity tonight.”

This decision authorizes an accelerated Gate 10D review path. It does not
authorize blind deletion. Before mutation, the exact approval must bind the
current Phase B evidence, Phase C evidence, generation selector, proposed
deletion manifest, and code revision. The independent v2 suite, strict Clippy,
and the 100% retained-behavior parity gate must pass. Every deletion slice must
retain useful code regardless of the reviewable LoC target, receive exact-
revision review, and pass required checks before merge.

Historical Gate 10C evidence remains immutable. The typed
`csdlc.deletion_approval.v2` record carries the explicit accelerated-window
waiver and operator instruction.
