# v0.91.6 Late C-SDLC Lifecycle / Goal / Readiness Tranche Review

Issue: `#4504`
Status: `retained_integrated_review`
Date: 2026-06-24
Scope: `#4412`, `#4413`, `#4425`, `#4442`, `#4443`, `#4457`, `#4459`, and `#4470`

## Findings

### P1: Closed issues `#4442` and `#4457` do not have merged implementation evidence on `main`

The integrated tranche was supposed to cover late lifecycle accounting and
pre-start readiness enforcement as implemented behavior, not policy-only
documentation. But the concrete issue-linked commits currently associated with
`#4442` and `#4457` are not ancestors of `main`, even though both issues are
closed in GitHub.

Evidence:

- `git merge-base --is-ancestor 0735780e main` failed for the visible `#4442`
  branch commit `[v0.91.6] Add transcript-backed goal snapshot capture`.
- `git merge-base --is-ancestor ebb900e4 main` failed for the visible `#4457`
  branch commit `[v0.91.6][csdlc][readiness] Repair lifecycle fixture design-time budgets`.
- The visible branch-tip payloads are materially narrower than the original
  issue outcomes:
  - `#4442` branch tip touches only
    `adl/tools/skills/sprint-conductor/scripts/issue_goal_metrics.py`,
    `adl/tools/skills/sprint-conductor/scripts/record_issue_goal_stage_from_codex_session.py`,
    `adl/tools/test_sprint_conductor_helpers.sh`, and `docs/default_workflow.md`.
  - `#4457` branch tip touches only
    `adl/src/cli/tests/pr_cmd_inline/support.rs`.
- The issue bodies for `#4442` and `#4457` promised broader lifecycle-hook and
  hard pre-start gate outcomes than those non-mainline deltas establish.

Required remediation:

- follow-on issue `#4511` now owns the implementation/closure-truth repair that
  must either land the missing behavior on `main` or reopen/reclassify the
  closed issues so the tranche no longer overclaims completion.
- As of 2026-06-24, `#4511` has already made the closure caveat visible in the
  live GitHub surfaces for `#4442` and `#4457` by adding issue-local correction
  comments and prepending `Status Correction (2026-06-24)` sections to both
  closed issue bodies. That resolves the hidden-closure-truth defect, and the
  bounded repair is now published for review as PR `#4514`, but it does not yet
  prove the missing bounded implementation landed on `main`.

## Scope

This packet reviews the integrated late control-plane tranche that `#4502`
identified as too important to leave scattered across singleton closures.

## Review Result

The tranche is only partially consumable as implemented behavior.

The following issues do have visible mainline evidence consistent with their
review role:

| Issue | Mainline evidence |
| --- | --- |
| `#4412` | `d3978c38` session ledger + coordination commands |
| `#4413` | `3bacef26` delegate liveness + observability |
| `#4425` | `e1650ba1` generated/validated VPP convergence |
| `#4443` | `a190ae2e` lifecycle shepherd contract |
| `#4459` | `283afc34` template budget/readiness fields |
| `#4470` | `d5936994` issue-bound goal terminal-state policy/hooks |

But `#4442` and `#4457` remain closure-truth defects in the integrated story.

## Evidence Table

| Issue | Intended surface | Visible evidence reviewed | Result |
| --- | --- | --- | --- |
| `#4412` | session ledger and collision truth | `adl/src/session_ledger.rs`, `adl/src/cli/session_cmd.rs`, `docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md` via `d3978c38` | consumed |
| `#4413` | lifecycle liveness and progress diagnostics | `adl/tools/pr.sh`, `adl/tools/observability.sh`, focused delegate-liveness tests via `3bacef26` | consumed |
| `#4425` | VPP/profile derivation and drift checks | `adl/src/cli/pr_cmd_cards/cards.rs`, `adl/src/cli/tests/pr_cmd_inline/*` via `e1650ba1` | consumed |
| `#4442` | host goal snapshot capture at lifecycle checkpoints | non-mainline branch tip `0735780e`; no merged `main` evidence found | not consumed |
| `#4443` | full issue-lifecycle shepherd contract | `docs/tooling/ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md` and related skill contracts via `a190ae2e` | consumed |
| `#4457` | enforced hard pre-start readiness gate | non-mainline branch tip `ebb900e4`; no merged `main` evidence found | not consumed |
| `#4459` | required metrics/readiness template fields | prompt/sprint template updates via `283afc34` | consumed |
| `#4470` | issue-bound goal terminal-state truth | goal policy/scripts/docs via `d5936994` | consumed |

## Relationship To `#4433`

Open issue `#4433` remains an adoption/operationalization sprint, not a valid
substitute for missing merged implementation on `#4442` or `#4457`.

This review therefore does not treat the open adoption sprint as proof that the
late tranche is fully delivered.

## Validation And Review Coverage

Review lanes exercised:

- code: reviewed merged command, script, template, and policy surfaces tied to the tranche
- docs: checked workflow/policy/template docs against visible merged code
- evidence_and_closeout: compared closed issue state to mainline reachability

Focused local checks for this packet:

```text
git merge-base --is-ancestor <issue_commit> main
git diff --check
```

## Non-Claims

- This packet does not claim `#4433` is complete.
- This packet does not claim `#4442` or `#4457` are safely consumable as merged implementation.
- This packet does not reopen issues by itself; it records the retained review truth.
- This packet does not rerun the full validation suites for every merged issue in the tranche.

## Closeout Position

`#4504` is satisfied as a retained integrated review once this packet and the
issue-local review records land, because the review truth is now durable and the
closure caveat is no longer hidden on the live issue surfaces.

What remains open is not retained-review authoring but implementation
settlement: follow-on PR `#4514` still needs to either land the missing bounded
behavior on `main` or record a stronger folded/superseding proof path.
