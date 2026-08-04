# Findings Register

Review head: `70f4e76509de219ccff6ffb534f9199d74eaece2`

## IR5791-01 - P1 - Fixed

Current C-SDLC v2 closeout documentation and helper surfaces still referenced
deleted `csdlc-closeout` or `csdlc-merge` command paths after the closeout
authority split.

Evidence:

- `docs/tooling/C_SDLC_V2_V1_ORIGIN_PR_TAIL_PLAYBOOK.md`
- `docs/tooling/editor/command_adapter.md`
- `adl/tools/attach_post_merge_closeout.sh`
- `adl/tools/fix_git_main_sync_preserve_local_adl.sh`
- `adl/tools/generate_active_command_reference_scan.py`
- `adl/tools/editor_action.sh`
- `adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md`

Impact:

Operators could follow current docs and helper output into a deleted command,
making the closeout fix appear nonfunctional even though source authority had
changed.

Resolution:

Updated active surfaces to use `csdlc-finish` for terminal observation/finish
truth and `csdlc-clean cleanup` for post-terminal cleanup. Broadened
`gate_terminal_authority_deletion` so these active surfaces cannot regress to
the deleted command names. A bounded subagent review found one additional
active test surface in `csdlc-v2/tests/gate4.rs`; that test now models the
finish/clean split and the derived-terminal artifact path.

Validation:

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate_terminal_authority_deletion`
- `bash adl/tools/test_closeout_completed_issue_wave.sh`
- `bash adl/tools/test_editor_action.sh`
- `bash adl/tools/test_generate_active_command_reference_scan.sh`

## IR5791-02 - P2 - Routed

Several merged v0.91.8 issue records remain in tracked `published` phase while
GitHub issue/PR truth is closed/merged, and only a subset has derived-terminal
cache entries.

Evidence:

- #5356, #5766, #5778, #5779, #5780, #5788, and #5789 have tracked issue
  records still showing `phase: published` with no embedded terminal receipt.
- Derived-terminal cache entries were observed for #5778, #5779, #5780, and
  #5789, but not for #5356, #5766, or #5788.

Impact:

Release review surfaces can disagree depending on whether they read live
GitHub, derived-terminal evidence, or tracked issue records. This does not
invalidate the source fixes, but it leaves closeout visibility confusing.

Disposition:

Routed as terminal reconciliation evidence. This issue fixes the active command
surface and records the gap; mutating already-merged issue records belongs to
explicit typed finish/cleanup reconciliation for each affected issue, not a
manual edit inside the WP-18 review worktree.

## IR5791-03 - P1 - Fixed

The first committed #5791 review packet recorded `2a63c6f4e242fd...` as its
review head even though the committed fix head was
`70f4e76509de219ccff6ffb534f9199d74eaece2`.

Evidence:

- `docs/reviews/v0.91.8/internal-review-5791/README.md`
- `docs/reviews/v0.91.8/internal-review-5791/FINDINGS_REGISTER.md`
- `docs/reviews/v0.91.8/internal-review-5791/VALIDATION.md`

Impact:

The packet could not truthfully serve as exact-head review evidence for the
committed repair.

Resolution:

Updated the packet to name the committed head
`70f4e76509de219ccff6ffb534f9199d74eaece2` and retained the review finding so
the correction is explicit.
