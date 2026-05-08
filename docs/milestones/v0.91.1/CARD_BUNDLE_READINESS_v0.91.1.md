# Card Bundle Readiness - v0.91.1

## Status

Complete issue-wave readiness record.

`v0.91.1` is open for execution through `WP-01` / `#2823` through `WP-24` /
`#2846`. Every WP has a local ignored task bundle with prepared STP, SIP, SPP,
SRP, and SOR cards before execution binding.

## Opened Bundles

| WP | Issue | Bundle | Status |
| --- | --- | --- | --- |
| WP-01 | #2823 | `.adl/v0.91.1/tasks/issue-2823__v0-91-1-wp-01-design-pass-milestone-docs-planning/` | structurally valid before execution binding |
| WP-02 | #2824 | `.adl/v0.91.1/tasks/issue-2824__v0-91-1-wp-02-runtime-and-polis-architecture-alignment/` | structurally valid before execution binding |
| WP-03 | #2825 | `.adl/v0.91.1/tasks/issue-2825__v0-91-1-wp-03-agent-lifecycle-state-model/` | structurally valid before execution binding |
| WP-04 | #2826 | `.adl/v0.91.1/tasks/issue-2826__v0-91-1-wp-04-csm-observatory-active-surface/` | structurally valid before execution binding |
| WP-05 | #2827 | `.adl/v0.91.1/tasks/issue-2827__v0-91-1-wp-05-citizen-standing-model/` | structurally valid before execution binding |
| WP-06 | #2828 | `.adl/v0.91.1/tasks/issue-2828__v0-91-1-wp-06-citizen-state-substrate/` | structurally valid before execution binding |
| WP-07 | #2829 | `.adl/v0.91.1/tasks/issue-2829__v0-91-1-wp-07-memory-and-identity-architecture/` | structurally valid before execution binding |
| WP-08 | #2830 | `.adl/v0.91.1/tasks/issue-2830__v0-91-1-wp-08-theory-of-mind-foundation/` | structurally valid before execution binding |
| WP-09 | #2831 | `.adl/v0.91.1/tasks/issue-2831__v0-91-1-wp-09-capability-and-aptitude-testing-foundation/` | structurally valid before execution binding |
| WP-10 | #2832 | `.adl/v0.91.1/tasks/issue-2832__v0-91-1-wp-10-intelligence-metric-architecture/` | structurally valid before execution binding |
| WP-11 | #2833 | `.adl/v0.91.1/tasks/issue-2833__v0-91-1-wp-11-governed-learning-substrate/` | structurally valid before execution binding |
| WP-12 | #2834 | `.adl/v0.91.1/tasks/issue-2834__v0-91-1-wp-12-anrm-gemma-placement-and-trace-dataset/` | structurally valid before execution binding |
| WP-13 | #2835 | `.adl/v0.91.1/tasks/issue-2835__v0-91-1-wp-13-acip-conformance-and-local-encryption-hardening/` | structurally valid before execution binding |
| WP-14 | #2836 | `.adl/v0.91.1/tasks/issue-2836__v0-91-1-wp-14-a2a-adapter-boundary-and-compatibility-plan/` | structurally valid before execution binding |
| WP-15 | #2837 | `.adl/v0.91.1/tasks/issue-2837__v0-91-1-wp-15-runtime-inhabitant-integration/` | structurally valid before execution binding |
| WP-16 | #2838 | `.adl/v0.91.1/tasks/issue-2838__v0-91-1-wp-16-observatory-visible-agent-flagship-demo/` | structurally valid before execution binding |
| WP-17 | #2839 | `.adl/v0.91.1/tasks/issue-2839__v0-91-1-wp-17-demo-matrix-and-proof-coverage/` | structurally valid before execution binding |
| WP-18 | #2840 | `.adl/v0.91.1/tasks/issue-2840__v0-91-1-wp-18-coverage-quality-gate/` | structurally valid before execution binding |
| WP-19 | #2841 | `.adl/v0.91.1/tasks/issue-2841__v0-91-1-wp-19-docs-review-pass/` | structurally valid before execution binding |
| WP-20 | #2842 | `.adl/v0.91.1/tasks/issue-2842__v0-91-1-wp-20-internal-review/` | structurally valid before execution binding |
| WP-21 | #2843 | `.adl/v0.91.1/tasks/issue-2843__v0-91-1-wp-21-external-3rd-party-review/` | structurally valid before execution binding |
| WP-22 | #2844 | `.adl/v0.91.1/tasks/issue-2844__v0-91-1-wp-22-review-findings-remediation/` | structurally valid before execution binding |
| WP-23 | #2845 | `.adl/v0.91.1/tasks/issue-2845__v0-91-1-wp-23-v0-92-birthday-readiness-handoff/` | structurally valid before execution binding |
| WP-24 | #2846 | `.adl/v0.91.1/tasks/issue-2846__v0-91-1-wp-24-release-ceremony/` | structurally valid before execution binding |

## Validated Surfaces

- `stp.md`
- `sip.md`
- `spp.md`
- `srp.md`
- `sor.md`

## Validation Commands

The full card set passed these validation loops from the primary checkout:

```sh
find .adl/v0.91.1/tasks -name stp.md | sort | while IFS= read -r f; do
  bash adl/tools/validate_structured_prompt.sh --type stp --input "$f" >/dev/null
done
```

```sh
find .adl/v0.91.1/tasks -name sip.md | sort | while IFS= read -r f; do
  bash adl/tools/validate_structured_prompt.sh --type sip --input "$f" --phase bootstrap >/dev/null
done
```

```sh
find .adl/v0.91.1/tasks -name spp.md | sort | while IFS= read -r f; do
  bash adl/tools/validate_structured_prompt.sh --type spp --input "$f" >/dev/null
done
```

```sh
find .adl/v0.91.1/tasks -name srp.md | sort | while IFS= read -r f; do
  bash adl/tools/validate_structured_prompt.sh --type srp --input "$f" >/dev/null
done
```

```sh
find .adl/v0.91.1/tasks -name sor.md | sort | while IFS= read -r f; do
  bash adl/tools/validate_structured_prompt.sh --type sor --input "$f" --phase bootstrap >/dev/null
done
```

## Card Quality Notes

- STPs now carry the real sprint, queue, dependencies, source docs, canonical
  files, outcome types, and demo/proof flags.
- SIPs now name concrete repo inputs, validation expectations, execution
  boundaries, and no-main rules before `pr run` binding.
- SPPs now provide durable issue-local execution plans with dependency checks,
  stop conditions, and review handoff hooks.
- SRPs now define issue-local review scope, evidence policy, refusal policy,
  validation inputs, and follow-up routing before PR publication.
- SORs remain truthful pre-run output scaffolds and must be updated during each
  issue's execution and finish lifecycle.
