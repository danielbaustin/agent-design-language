The new convergence logic can produce successful decisions for outputs from a different revision than the envelope's declared source revision, which breaks the exact-head safety property this change is meant to enforce.

Review comment:

- [P1] Reject assignment revisions that differ from the envelope head — /Volumes/FastWork/adl-wp-5502/adl-v2/crates/adl-workcell-convergence/src/lib.rs:93-93
  When an input envelope names one `source_revision` but its assignments and matching outputs are bound to another valid 40-char revision, convergence can still return an integration plan whose top-level decision claims the envelope revision while each integrated step points at the stale assignment revision. For exact-head review, this lets stale worker outputs be integrated under a current-head decision instead of blocking as stale, because `normalize_assignments` only validates SHA shape and `validate_output_binding` only compares output-to-assignment.