# Code-Feature Demo Follow-Ons Packet - v0.91.2

## Status

Tracked `WP-17A` packet for strengthening code-feature demos after `WP-17`
convergence.

## Purpose

`WP-17` made the milestone showcase legible, but it did not itself create new
demo surfaces for the code features whose proof still felt more packetized than
demo-first.

This follow-on closes that gap for the minimum named code lanes:

- `WP-11` speculative decoding prototype
- `WP-16` workflow guardrails hardening

## Delivered Demo Surfaces

### WP-11 Speculative Decoding Showcase

Command:

```bash
bash adl/tools/demo_v0912_speculative_decoding_showcase.sh
```

What it now does:

- runs the bounded `demo_v0912_speculative_decoding_prototype` binary
- regenerates or writes the JSON report
- prints a concise scenario-by-scenario operator summary with worthiness,
  acceptance rate, and speedup ratio

Why this is stronger:

- the prototype already existed, but this creates a clean operator-facing
  showcase entrypoint instead of forcing reviewers to inspect only JSON or a
  long packet narrative

### WP-16 Workflow Guardrails Showcase

Command:

```bash
bash adl/tools/demo_v0912_workflow_guardrails_showcase.sh
```

What it now does:

- creates bounded temporary fixture repos
- demonstrates clean-main pass and dirty-main block behavior
- demonstrates unsafe-report-command blocking and safe-report-command passing
- demonstrates that `card-drift` delegates to `pr doctor`

Why this is stronger:

- the underlying tool and tests already existed, but this creates one coherent
  operator-facing showcase pass instead of asking reviewers to mentally extract
  the demo from the test script

## Validation

Focused proof command:

```bash
bash adl/tools/test_wp17a_demo_follow_ons.sh
```

The existing feature-local validation should still pass as well:

```bash
cargo test --manifest-path adl/Cargo.toml demo_v0912_speculative_decoding_prototype -- --nocapture
bash adl/tools/test_workflow_guardrails.sh
```

## Non-Claims

- This packet does not turn speculative decoding into a production acceleration
  claim.
- This packet does not expand workflow guardrails into full lifecycle
  automation.
- This packet only upgrades the demo surfaces for the named code features; it
  does not reopen non-code milestone slices just to force them into fake demos.
