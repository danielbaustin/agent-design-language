The convergence logic can emit an Integrate decision while tracking remaining incomplete issues, and it also bypasses input hygiene for task-supplied blocker evidence. These are behavioral issues in the new crate that should be fixed before treating the patch as correct.

Full review comments:

- [P2] Do not integrate while issues remain incomplete — /Volumes/FastWork/adl-wp-5502/adl-v2/crates/adl-workcell-convergence/src/lib.rs:96-105
  When an output is `Partial` without blockers/changed assumptions (or `Blocked`/`Failed` with no generated artifact blocker, e.g. no expected outputs), the loop adds the issue to `remaining_issues` but leaves `blockers` and `changed_assumptions` empty. This branch then still returns `ConvergenceDecision::Integrate`, producing an integration decision even though the projection says work remains; include `remaining_issues` in the decision gate or synthesize a residual blocker/replan for non-succeeded statuses.

- [P2] Validate task-supplied blocker evidence before echoing it — /Volumes/FastWork/adl-wp-5502/adl-v2/crates/adl-workcell-convergence/src/lib.rs:50-50
  If a task output includes `blockers`, those records are cloned directly into the convergence envelope without normalizing `evidence_refs` or applying the same secret/control-character checks used for validation and review refs. A blocked task can therefore cause absolute paths, URLs, traversal refs, or secret-bearing strings to be persisted in `residual_blockers`/`BlockedRecord`, bypassing the hygiene contract this crate otherwise enforces.