# Portable Contract Normalizer Output Contract

Portable contract normalizer artifacts must report portability findings without
hiding legitimate evidence or claiming unscanned surfaces are portable.

## Required Markdown Sections

- `Portable Contract Normalizer Summary`
- `Finding Counts`
- `Findings`
- `Safe Mechanical Normalization`
- `Design Decisions Required`
- `Applied Fixes`
- `Non-Claims`
- `Safety Flags`

## Required JSON Shape

Schema id: `adl.portable_contract_normalizer_report.v1`

Required top-level fields:

- `schema`
- `run_id`
- `status`
- `mode`
- `counts`
- `findings`
- `applied_fixes`
- `non_claims`
- `safety_flags`

Supported statuses:

- `clean`
- `findings`
- `fixed`
- `blocked`
- `not_run`

Supported finding categories:

- `absolute_host_path`
- `brittle_worktree_name`
- `machine_local_temp_path`
- `stale_contract_reference`
- `environment_specific_assertion`

## Safety Flags

Every report must state:

- `mutated_repository`: boolean
- `applied_safe_fixes_only`: boolean
- `design_decisions_resolved`: false
- `legitimate_evidence_redacted`: false
- `unbounded_scan`: false

The skill may report that scanned paths are clean. It must not claim repository
portability outside the bounded scan root.

`legitimate_evidence_redacted: false` means the report did not hide exact host
evidence without operator-approved normalization.
