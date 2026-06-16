# TBD Documentation Backlog Closeout (#3635)

Issue: #3635  
Captured: 2026-06-16  
Status: ready_for_final_issue_closeout

## Summary

#3635 organized `.adl/docs/TBD/` as ADL's local editable authoring and
incubation surface. The work separated active planning, scheduled feature
material, historical provenance, generated/local junk, externally owned paper
material, and misplaced root documents.

This closeout packet exists because the work was intentionally done in `.adl/`
authoring space, which is not a tracked public repo surface. The tracked repo
needs a durable summary so the issue can close without losing what happened.

## Current Observed Inventory

Observed from the primary local `.adl/docs/TBD/` tree on 2026-06-16:

- total files: 582
- root files: 13
- directories: 65

Current root files:

- `.adl/docs/TBD/ADL_AND_GUILDS.md`
- `.adl/docs/TBD/ADL_DOC_CLEANUP_LEDGER.md`
- `.adl/docs/TBD/ADL_MEMORY_PALACE_CONTEXT_PROBLEM.md`
- `.adl/docs/TBD/ADL_PROFILES_PROVIDERS_V2.MD`
- `.adl/docs/TBD/ADL_PROVIDER_V2.md`
- `.adl/docs/TBD/ADL_STRATEGIC_COGNITIVE_RESERVE.md`
- `.adl/docs/TBD/LOCAL_BACKLOG.md`
- `.adl/docs/TBD/MILESTONE_CLOSEOUT_CHECKLIST.md`
- `.adl/docs/TBD/NEW_FEATURE_MILESTONE_ASSIGNMENT_PLAN.md`
- `.adl/docs/TBD/RUSTDOC_GAP_ANALYSIS.md`
- `.adl/docs/TBD/TBD_CLEANUP_PLAN_2026-06-15.md`
- `.adl/docs/TBD/TBD_DOC_STATUS_INVENTORY.md`
- `.adl/docs/TBD/v0.91.5-closed-issues-review.md`

The inventory is intentionally not zero. The remaining root files are active
control, scratch, backlog, or planning surfaces rather than unreviewed clutter.

## Local Records Created Or Updated

The durable local authoring records for this cleanup are:

- `.adl/docs/TBD/TBD_CLEANUP_PLAN_2026-06-15.md`
- `.adl/docs/TBD/TBD_DOC_STATUS_INVENTORY.md`
- `.adl/docs/TBD/ADL_DOC_CLEANUP_LEDGER.md`
- `.adl/docs/TBD/planning/TBD_CLEANUP_3635_CLOSEOUT_PREP_2026-06-15.md`
- `.adl/docs/TBD/planning/MVP_SCOPE_LOCK_CROSSCHECK_2026-06-15.md`
- `.adl/docs/TBD/planning/MVP_FEATURE_DOC_PRODUCTION_PLAN_2026-06-15.md`
- `.adl/docs/TBD/planning/FEATURE_DOC_ISSUE_SPLIT_PLAN_2026-06-15.md`

These are local `.adl/` authoring surfaces. They were not exported as public
docs by #3635.

The tracked appendix
[TBD_DOCUMENTATION_BACKLOG_FILE_AND_DECISION_INVENTORY_3635.md](TBD_DOCUMENTATION_BACKLOG_FILE_AND_DECISION_INVENTORY_3635.md)
lists the observed `.adl/docs/TBD/` file set and the cleanup decisions so the
issue can close with a durable public summary even though the source tree is
local authoring state.

## Cleanup Actions Completed

The #3635 cleanup pass:

- retired obvious historical review, gap, roadmap, and provenance files under
  `.adl/docs/TBD/retired/`
- routed obvious active or completed planning docs from the root into thematic
  subdirectories
- removed generated/local junk only after explicit operator approval
- removed local paper `.tex` and `.bib` copies only after confirming paper
  repositories are authoritative
- removed the duplicate `STARTUP_GRANTS_PLAN_0.1.md` only after explicit
  operator approval
- moved CodeFriend planning into `.adl/docs/TBD/codefriend_ai/` and changed the
  local bucket from `codebuddy_ai/` to `codefriend_ai`
- preserved speculative decoding material as retired provenance after
  confirming the v0.91.2 bounded evaluation had already run
- routed UTS/tooling material into `.adl/docs/TBD/tools/` while leaving
  standalone repository migration as a later decision

## Buckets Created Or Strengthened

The cleanup created or clarified first-class local buckets for:

- `security/`
- `upstream_delegation/`
- `reasoning_graphs/`
- `resilience/`
- `curiosity/`
- `constructability/`
- `mvp_cleanup/`
- `guilds/`
- `codefriend_ai/`
- `tools/`

These buckets prevent active feature ideas from disappearing into the TBD root.

## Planning And Backlog Captured

The cleanup captured and routed the following planning surfaces so they would
not be lost in the TBD root:

- resilience, citizen persistence, sleep/wake, and transient fault handling
- Curiosity Engine / Discovery Substrate
- Constructability Gate
- reasoning graphs, loop runtime, and future `adl.skill.v1` bridge planning
- ACIP schema/access/transport and A2A decisions
- security bridge readiness and Continuous Adversarial Verification
- provider/model and multi-agent reliability
- Observatory/Unity readiness
- public prompt records export, redaction, and indexing
- Memory Palace / long-running context-problem planning
- Guilds governance/product planning
- CodeFriend v1 plus portable adapter v2 planning
- Rust refactoring planning with test-burden reduction as an explicit concern

The cleanup also preserved local issue drafts and backlog entries for feature
doc production, bridge-tranche planning, CodeFriend v1 proof, Memory Palace
bridge work, feature-list sync, and ADR sprint follow-up.

## Boundaries

#3635 did not:

- claim that every TBD idea is implemented
- export the `.adl/docs/TBD/` tree to the public repo
- close or execute the feature work discovered during cleanup
- migrate external repo material such as UTS, papers, demos, or CodeFriend
  product work without separate approval
- delete historical evidence without review

The cleanup made the backlog governable; it did not complete the backlog.

## Validation Performed

Final closeout review used:

- `pr doctor 3635 --mode full --json`
  - confirmed the issue is lifecycle-ready and has a bound worktree.
- read-only inventory of the current `.adl/docs/TBD/` tree
  - confirmed the current file, root-file, and directory counts.
- review of the local closeout-prep record
  - confirmed cleanup actions, planning outputs, and residuals are recorded.

The stale bound worktree `.worktrees/adl-wp-3635` is not used as the source of
truth for the cleanup contents because `.adl/docs/TBD/` is local authoring
state and is absent from that worktree checkout.

## Residual Work

Residual work is intentionally routed outside #3635:

- public feature-list roadmap sync
- feature-doc production waves
- v0.91.6 and v0.91.7 bridge tranche execution
- Memory Palace feature doc and implementation planning
- CodeFriend v1 and portable adapter v2 proof
- Rust refactoring sprint
- any future public export of `.adl/` authoring records

## Closeout Decision

#3635 is ready for final closeout. The local TBD surface is organized enough to
stop broad cleanup under this issue, and remaining work has been routed into
specific planning buckets, backlog entries, issue drafts, or later milestone
work.
