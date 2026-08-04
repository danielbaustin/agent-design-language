# Refactor Intent

This is an explicit supported-behavior change: remove every supported command and library export that creates, repairs, transports, reconciles, or treats tracked post-merge terminal projections and receipts as authority. Delete the `csdlc-closeout` binary and skill instead of retaining a compatibility wrapper. Preserve read-only deserialization and legacy terminal indexing, while making `csdlc-finish`, `csdlc-pr-state`, and `csdlc-clean` the only supported finish, status, and cleanup surfaces.

Sequence the work as characterization, operator-surface deletion, writer/API deletion, test contraction with new negative guards, then documentation and reduction evidence.
