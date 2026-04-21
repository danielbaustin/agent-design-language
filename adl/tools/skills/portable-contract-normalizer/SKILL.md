---
name: portable-contract-normalizer
description: Scan bounded repo contracts, tests, packets, and validation surfaces for machine-local assumptions such as absolute host paths, brittle worktree names, temp paths, stale hard-coded contract references, and environment-specific assertions, then report findings or apply only explicit safe mechanical normalizations.
---

# Portable Contract Normalizer

Find portability defects before they leak into CI, review packets, or another
operator machine. This skill scans bounded paths for host-specific assumptions
and separates safe mechanical normalization from findings that require design
decisions.

It is a portability guard and optional narrow normalizer, not a broad test
rewrite tool.

## Quick Start

1. Confirm the bounded paths to scan.
2. Confirm whether the run is report-only or explicit safe-fix mode.
3. Run the deterministic helper:
   - `scripts/normalize_portable_contracts.py --root <path> --out <artifact-root> --run-id <run_id>`
4. If the operator explicitly requests safe fixes, rerun with:
   - `scripts/normalize_portable_contracts.py --root <path> --out <artifact-root> --run-id <run_id> --apply`
5. Review the Markdown and JSON artifacts. Stop before broad contract rewrites
   or design decisions.

## Required Inputs

At minimum, gather:

- `mode`
- `target`
- `policy`

Supported modes:

- `scan_contracts`
- `scan_and_apply_safe_fixes`
- `inspect_normalizer_report`

Useful policy fields:

- `write_report`
- `apply_safe_fixes`
- `bounded_paths`
- `preserve_legitimate_host_evidence`
- `stop_before_broad_rewrite`
- `stop_before_design_decision`

If no bounded path is provided, return `not_run` or `blocked`; do not scan the
entire machine by default.

## Finding Categories

Classify findings as:

- `absolute_host_path`: local user, temp, or machine-specific absolute paths.
- `brittle_worktree_name`: hard-coded `.worktrees/adl-wp-<number>` or similar
  branch/worktree coupling.
- `machine_local_temp_path`: `/tmp`, `/private/var`, or platform temp paths
  embedded in portable artifacts.
- `stale_contract_reference`: hard-coded contract or skill inventory references
  likely to drift when the skill set changes.
- `environment_specific_assertion`: assertions tied to one user, host, shell,
  platform, or local environment.

## Fix Policy

Safe mechanical normalization may replace obvious host-specific fixture text
with portable placeholders such as `<repo-root>`, `<temp-path>`, or
`.worktrees/adl-wp-<issue>`.

Do not apply fixes when:

- the path is legitimate evidence that must remain exact
- the replacement would alter semantic test intent
- the surface is a customer or review packet without explicit permission
- the finding requires a design decision

## Output

Default artifact root:

```text
.adl/reviews/portable-contract-normalizer/<run_id>/
```

Required artifacts:

- `portable_contract_normalizer_report.md`
- `portable_contract_normalizer_report.json`

Use the detailed contract in `references/output-contract.md`.

## Stop Boundary

This skill must not:

- broadly rewrite tests or contracts without explicit operator approval
- hide legitimate host-specific evidence
- mutate customer or review packets unless explicitly requested
- replace focused remediation issues
- claim portability is proven beyond the scanned paths

