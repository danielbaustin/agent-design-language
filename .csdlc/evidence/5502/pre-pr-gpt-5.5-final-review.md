The convergence logic mostly validates assignment and artifact paths, but evidence references can still escape the repository-local normalized contract. This is a functional hygiene gap in the newly introduced crate.

Review comment:

- [P2] Normalize evidence refs as repository paths — /Volumes/FastWork/adl-wp-5502/adl-v2/crates/adl-workcell-convergence/src/lib.rs:441-451
  When a task output supplies `validation_refs` or `review_refs` as an absolute path such as `/tmp/proof.json` or a traversal path like `../outside`, this check still accepts it because it only rejects empty/control/URL strings. Those refs are then copied into the integration step and decision identity, so convergence records can contain non-repository-local evidence despite the error text and surrounding path hygiene requiring local normalized references.