# v0.91.3 Design-Time Card Completion Catch-Up

Issue: `#3268`
Date: 2026-05-22
Milestone: `v0.91.3`

## Summary

This pass applied the design-time card-completion rule from `#3264` and `#3267`
to the remaining `v0.91.3` issue wave before execution continues.

The purpose was not to execute child work packages. The purpose was to remove
generic bootstrap drift so future `v0.91.3` work starts from complete,
issue-specific `SIP`, `STP`, `SPP`, and pre-review `SRP` surfaces.

## Scope

Open `v0.91.3` issues checked:

- `#3208` / `WP-13` internal review
- `#3209` / `WP-15` review findings remediation
- `#3210` / `WP-16` next milestone planning
- `#3211` / `WP-18` release ceremony
- `#3228` / `WP-12` docs + review pass
- `#3229` / `WP-14` external / third-party review
- `#3230` / `WP-17` next milestone review pass
- `#3231` / Sprint 4 umbrella
- `#3268` / design-time card catch-up
- `#3270` / duplicate validation optimization

Additional Sprint 4 child bundles checked:

- `#3226` / `WP-10` demo matrix and proof coverage
- `#3227` / `WP-11` quality coverage gate

`#3226` and `#3227` are already closed out, but their SIPs were normalized
because the Sprint 4 structured-prompt readiness helper evaluates the full
ordered child list before sprint advancement.

## Repairs Applied

- Replaced generic bootstrap `SIP` intent with issue-specific design-time
  prompts for the open issue set.
- Replaced generic or truncated `SPP` surfaces with approved, issue-local
  design-time plans for the open issue set.
- Replaced generic pre-review `SRP` surfaces with issue-specific Structured
  Review Prompts for the open issue set.
- Preserved `SOR` lifecycle truth instead of forcing completion claims.
- Mirrored active bound-card repairs for `#3208` and `#3268` into their
  respective issue worktrees.
- Normalized closed Sprint 4 child SIPs for `#3226` and `#3227` without
  reopening, re-executing, or changing their SOR closeout truth.

## Validation

`pr doctor` design-time check:

- All open `v0.91.3` issue bundles listed above reported `pr_run=ready`.
- No open issue reported `SIP`, `STP`, `SPP`, or `SRP` design-time defects.

Structured prompt validation:

- `sip` validation passed for every normalized open issue bundle using
  `bash adl/tools/validate_structured_prompt.sh --type sip --phase bootstrap
  --input <card>`.
- `spp` validation passed for every normalized open issue bundle using
  `bash adl/tools/validate_structured_prompt.sh --type spp --input <card>`.
- `srp` validation passed for every normalized open issue bundle using
  `bash adl/tools/validate_structured_prompt.sh --type srp --input <card>`.
- `sip`, `spp`, and `srp` validation passed for the active worktree copies of
  `#3208` and `#3268`; the `sip` checks used `--phase bootstrap`.
- `sip` validation passed for the closed Sprint 4 child bundles `#3226` and
  `#3227` using `--phase bootstrap`.

Sprint 4 readiness check:

```text
python3 adl/tools/skills/sprint-conductor/scripts/check_sprint_structured_prompt_readiness.py \
  --repo-root "$REPO_ROOT" \
  --ordered-issues 3226,3227,3228,3208,3229,3209,3210,3230,3211 \
  --require-spp \
  --require-srp \
  --print-json
```

Result:

```text
status: ready
notes: All ordered child issue structured cards are ready for sprint start.
```

## Non-Claims

- This pass did not execute any child WP.
- This pass did not close or merge unrelated issues.
- This pass did not claim final review results for issues whose implementation
  has not yet been reviewed.
- This pass did not rewrite closed issue output truth.

## Residual Watch Items

- The card files live in local workflow state under `.adl/`, so this tracked
  review record is the repo-visible proof surface for the catch-up.
- Future issue creation should rely on the `#3267` enforcement path rather than
  repeating this manual catch-up sweep.
- `#3270` remains the next tooling optimization and should address duplicate
  validation cost without weakening proof.
