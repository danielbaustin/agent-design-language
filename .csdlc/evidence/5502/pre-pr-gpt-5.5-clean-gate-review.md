The new convergence path has a hygiene gap that can leak unvalidated changed-assumption strings into returned decision records. This should be fixed before considering the patch correct.

Review comment:

- [P2] Validate changed-assumption fields before echoing them — /Volumes/FastWork/adl-wp-5502/adl-v2/crates/adl-workcell-convergence/src/lib.rs:64-64
  When a task output reports `changed_assumptions`, these strings are cloned directly into the Replan decision without the secret/control-character hygiene applied to other user-controlled fields. For outputs whose observed/expected assumption text includes a token-like value or other sensitive string, `converge` will return and hash an envelope containing that value instead of rejecting it, which bypasses the crate's stated secret-bearing input guardrails.