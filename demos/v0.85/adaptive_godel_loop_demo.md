# v0.85 Demo: Adaptive Godel Loop Policy Update

This bounded demo shows the first real WP-11 policy-learning surface.

It takes the deterministic hypothesis artifact produced by the Godel loop and
derives a structured policy artifact plus a before/after policy comparison
record. The demo is intentionally bounded: it does not claim open-ended online
learning or unconstrained policy mutation.

## One-command review path

From repository root:

```bash
adl/tools/demo_adaptive_godel_loop.sh
```

The script runs:

1. `adl godel run`
2. `adl godel inspect`
3. prints the persisted policy artifact
4. prints the persisted policy comparison artifact

## Deterministic proof surface

The demo emits:

- `runs/review-godel-policy-001/godel/godel_hypothesis.v1.json`
- `runs/review-godel-policy-001/godel/godel_policy.v1.json`
- `runs/review-godel-policy-001/godel/godel_policy_comparison.v1.json`
- deterministic `godel inspect` JSON summary

For identical inputs, the persisted policy decision and comparison artifact
should be byte-for-byte stable.

## What this proves

- WP-10 hypothesis output is consumed by WP-11
- policy selection is derived from a prior artifact rather than hard-coded
  config mutation alone
- reviewers can inspect both the resulting policy decision and the before/after
  comparison surface

## Out of scope

- open-ended learning
- hidden runtime state mutation
- scheduler or trust-policy mutation
