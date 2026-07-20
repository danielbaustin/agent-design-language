# #5487 Typed terminal design repair

The repair route is a narrow Rust v2 transaction for already closed-out issues. It accepts an explicit repair authority, exact issue generation/digest, and hashes for the proposed Markdown design and diagram. The store validates the markdown.rs AST and all identities before taking the issue lock.

The transaction journals the old receipt and retained artifacts, writes synchronized staged replacements, updates the receipt and SPP/VPP references, and commits the audit/projection together. Any injected failure restores the prior receipt and artifacts. `reconcile-terminal` remains the only materialization path after a successful repair.

The route never reopens the issue, edits rendered cards directly, or performs AWS work. The focused proof repairs #5467 and verifies both success and rollback.
