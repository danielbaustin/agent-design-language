# v0.91.7 C-SDLC Closure Truth Repair

Issue: `#4511`
Status: `closure_truth_repair`
Date: 2026-06-24
Scope: `#4442`, `#4457`, retained review `#4504`

## Summary

Retained review `#4504` found that closed issues `#4442` and `#4457` did not
have visible merged `main` implementation evidence for the full acceptance
surface claimed by their original issue bodies.

This repair does not pretend the missing implementation landed. Instead it makes
the closure caveat visible in the live GitHub issue surfaces and records the
bounded repair route explicitly.

## Actions Taken

- Added issue-local correction comments to `#4442` and `#4457`.
- Updated the live GitHub issue bodies for `#4442` and `#4457` to prepend a
  `Status Correction (2026-06-24)` section.
- Recorded that follow-on issue `#4511` owns the remaining bounded repair path.

## Corrected Truth

### `#4442`

- The visible issue-linked branch-tip commit `0735780e` is not an ancestor of
  `origin/main`.
- The branch-tip payload proves transcript-backed goal snapshot helper work, but
  it does not prove the full native lifecycle-checkpoint capture outcome claimed
  by the original `#4442` issue body.

### `#4457`

- The visible issue-linked branch-tip commit `ebb900e4` is not an ancestor of
  `origin/main`.
- The branch-tip payload proves fixture-side design-time budget repair, but it
  does not prove the full pre-start readiness and issue token-budget gate
  outcome claimed by the original `#4457` issue body.

## Result

The GitHub issue surfaces no longer silently overclaim that the full original
`#4442` and `#4457` outcomes landed on `main`.

What remains unresolved is implementation settlement, not closure visibility:

- if stronger folded/superseding mainline evidence is found later, `#4511`
  should record it explicitly
- if the missing behavior still matters as executable capability, it should land
  through a bounded implementation issue rather than being implied by the now
  corrected closed issues

## Validation

Focused checks used for this repair:

```text
git -C .worktrees/adl-wp-4511 merge-base --is-ancestor 0735780e origin/main
git -C .worktrees/adl-wp-4511 merge-base --is-ancestor ebb900e4 origin/main
ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token bash adl/tools/pr.sh issue view 4442 --json | jq -r '.body' | sed -n '1,24p'
ADL_GITHUB_TOKEN_FILE=$HOME/keys/github.token bash adl/tools/pr.sh issue view 4457 --json | jq -r '.body' | sed -n '1,24p'
git -C .worktrees/adl-wp-4511 diff --check
```

## Non-Claims

- This repair does not claim the missing `#4442` / `#4457` implementation
  landed on `main`.
- This repair does not reopen the issues through a native ADL reopen command,
  because that route is not currently exposed by the repo-native issue surface
  used here.
- This repair does not treat open issue `#4433` as a substitute for missing
  merged implementation evidence.
