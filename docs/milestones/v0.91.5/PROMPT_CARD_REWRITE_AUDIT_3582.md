# v0.91.5 Prompt-Card Rewrite Audit

Issue: `#3582`
Date: 2026-06-04

## Summary

`#3582` reviewed the downstream v0.91.5 C-SDLC card surface after the
prompt-template renderer, structure schemas, and field-level values editor
landed.

The audit found that the downstream local `.adl` card bundles do not need a
mass Markdown rewrite before Sprint 1 continues. A phase-aware structured
prompt validation pass covered all 62 v0.91.5 task bundles and all five card
kinds per bundle:

- Bundles checked: 62
- Cards checked: 310
- Phase-aware structured prompt result: 310 pass, 0 fail
- Tracked Markdown card rewrites required by this issue: none

The first naive validation pass reported branch failures for pre-run and
retrospective no-PR cards. Those were validation-phase mismatches, not card
truth defects:

- pre-run `not bound yet` cards validate in bootstrap phase;
- closed `closed_no_pr` cards with `retrospective-no-branch` validate in
  completed phase.

## Tooling Used

The pass used the active C-SDLC prompt-template tooling and structured prompt
validators:

```sh
cargo run --quiet --manifest-path adl/Cargo.toml --bin adl-csdlc -- tooling prompt-template --help
```

```sh
ADL_TOOLING_RUST_BIN=adl/target/debug/adl-csdlc \
  adl/tools/validate_structured_prompt.sh --type <kind> --input <card>
```

Phase rules used by the audit:

- `sip` / `sor` with `not bound yet` branch truth: `--phase bootstrap`
- completed `sip` / `sor` terminal truth: `--phase completed`
- all other cards: default validator phase

## Disposition

The safe Sprint 1 disposition is:

- Preserve existing terminal card truth for closed/satisfied issues.
- Preserve existing pre-run card truth for open issues that validate.
- Do not regenerate rendered Markdown only for style.
- Use `edit-values` for supported field-level updates on future values-rendered
  cards.
- Use card editor skills for lifecycle-truth repairs when doctor or review
  finds an actual defect.
- Keep `#3622` gated until this audit lands; then it may split
  `csdlc_prompt_editor.rs` with the card surface stable.

## Inventory

| Issue | State | WP | Sprint | Disposition |
| --- | --- | --- | --- | --- |
| #3507 | closed | WP-01 | - | closed/satisfied; preserve terminal card truth |
| #3508 | closed | WP-01 | - | closed/satisfied; preserve terminal card truth |
| #3509 | closed | WP-01 | - | closed/satisfied; preserve terminal card truth |
| #3510 | closed | WP-01 | - | closed/satisfied; preserve terminal card truth |
| #3518 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3526 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3529 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3531 | closed | WP-14 | Sprint-4 | closed/satisfied; preserve terminal card truth |
| #3534 | closed | WP-18 | Sprint-4 | closed/satisfied; preserve terminal card truth |
| #3537 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3538 | open | unassigned | - | open; card bundle validates and remains eligible for normal conductor execution |
| #3541 | open | unassigned | - | open; card bundle validates and remains eligible for normal conductor execution |
| #3549 | open | unassigned | - | open; card bundle validates and remains eligible for normal conductor execution |
| #3553 | closed | WP-02 | Sprint-1 | closed/satisfied; preserve terminal card truth |
| #3556 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3562 | open | unassigned | - | open; card bundle validates and remains eligible for normal conductor execution |
| #3567 | closed | WP-01 | - | closed/satisfied; preserve terminal card truth |
| #3568 | closed | WP-01 | - | closed/satisfied; preserve terminal card truth |
| #3569 | open | Sprint-1 | Sprint-1 | open; card bundle validates and remains eligible for normal conductor execution |
| #3571 | open | Sprint-1 | Sprint-1 | open; card bundle validates and remains eligible for normal conductor execution |
| #3572 | open | Sprint-2 | Sprint-2 | open; card bundle validates and remains eligible for normal conductor execution |
| #3573 | open | Sprint-3 | Sprint-3 | open; card bundle validates and remains eligible for normal conductor execution |
| #3574 | open | Sprint-4 | Sprint-4 | open; card bundle validates and remains eligible for normal conductor execution |
| #3575 | open | WP-14 | Sprint-4 | open; card bundle validates and remains eligible for normal conductor execution |
| #3576 | open | WP-16 | Sprint-4 | open; card bundle validates and remains eligible for normal conductor execution |
| #3577 | open | WP-18 | Sprint-4 | open; card bundle validates and remains eligible for normal conductor execution |
| #3578 | open | WP-20 | Sprint-4 | open; card bundle validates and remains eligible for normal conductor execution |
| #3579 | open | WP-15 | Sprint-4 | open; card bundle validates and remains eligible for normal conductor execution |
| #3580 | open | WP-17 | Sprint-4 | open; card bundle validates and remains eligible for normal conductor execution |
| #3581 | open | WP-19 | Sprint-4 | open; card bundle validates and remains eligible for normal conductor execution |
| #3582 | open | WP-02 | Sprint-1 | active issue; records this audit and disposition pass |
| #3585 | closed | Sprint-1 | Sprint-1 | closed/satisfied; preserve terminal card truth |
| #3587 | closed | Sprint-1 | Sprint-1 | closed/satisfied; preserve terminal card truth |
| #3588 | closed | WP-02 | Sprint-1 | closed/satisfied; preserve terminal card truth |
| #3590 | open | Sprint-1 | Sprint-1 | open; card bundle validates and remains eligible for normal conductor execution |
| #3592 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3593 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3594 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3595 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3596 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3597 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3598 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3599 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3600 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3607 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3609 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3610 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3611 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3612 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3614 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3615 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3621 | closed | WP-02 | Sprint-1 | closed/satisfied; preserve terminal card truth |
| #3623 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3624 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3625 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3628 | closed | unassigned | - | closed/satisfied; preserve terminal card truth |
| #3636 | open | unassigned | - | open; card bundle validates and remains eligible for normal conductor execution |
| #3637 | open | unassigned | - | open; card bundle validates and remains eligible for normal conductor execution |
| #3638 | open | unassigned | - | open; card bundle validates and remains eligible for normal conductor execution |
| #3639 | open | unassigned | - | open; card bundle validates and remains eligible for normal conductor execution |
| #3640 | open | unassigned | - | open; card bundle validates and remains eligible for normal conductor execution |
| #3641 | open | unassigned | - | open; card bundle validates and remains eligible for normal conductor execution |

## Follow-On

Follow-on issue: `#3643`

This audit did not convert historical rendered cards into values YAML because
the current prompt-template tooling is intentionally one-way:

```text
values YAML -> rendered Markdown -> structure/schema validation
```

A future tool should add a bounded import/round-trip path:

```text
existing card -> values YAML candidate -> render -> structure comparison
```

`#3643` tracks that importer/round-trip normalizer. It is useful, but it is not
required to continue Sprint 1 because the current downstream cards validate and
retain truthful lifecycle state.
