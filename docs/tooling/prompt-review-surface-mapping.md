# Prompt/Reviewer Surface Mapping

Status: canonical v0.8 alignment surface for issue #668

## Purpose

Define the shared field contract between:

1. Prompt Spec block in input cards (`adl.v1`)
2. Card Review Checklist (`card_review_checklist.v1`)
3. Deterministic review output artifact (`card_review_output.v1`)

This mapping is the anti-drift reference for reviewer tooling behavior.

## Deterministic Contract

For identical input card + output card content:

- checklist rule evaluation order is fixed by checklist spec
- findings ordering is fixed by review output spec
- protocol IDs and versions are validated from Prompt Spec

No hidden inference should be required to bind these surfaces.

## Field Mapping

| Prompt Spec surface | Checklist surface | Review output surface | Notes |
|---|---|---|---|
| `prompt_schema` (`adl.v1`) | `CRS-STR-005` (Prompt Spec contract compatibility) | `review_target.prompt_spec_bindings.prompt_schema` | Reviewer confirms recognized Prompt Spec schema version. |
| `review_surfaces` includes `card_review_checklist.v1` | `CRS-STR-005` | `review_metadata.checklist_version` + `review_target.prompt_spec_bindings.review_surfaces` | Checklist version in output must match Prompt Spec declared protocol. |
| `review_surfaces` includes `card_review_output.v1` | `CRS-STR-005` | `review_format_version` + `review_target.prompt_spec_bindings.review_surfaces` | Review artifact schema version must match Prompt Spec declared protocol. |
| `review_surfaces` includes `card_reviewer_gpt.v1.1` | `CRS-STR-005` | `review_metadata.reviewer` + `review_target.prompt_spec_bindings.review_surfaces` | Reviewer identity/version should be compatible with Prompt Spec declaration. |
| `inputs.sections` ordering | `CRS-DET-002` | `determinism_checks.ordering_contract_verified` | Ordering contract for prompt generation must be explicit and stable. |
| `constraints.disallow_secrets` | `CRS-SEC-001` / `CRS-SEC-002` | `security_privacy_checks.secrets_present` and `security_privacy_checks.raw_prompt_or_tool_args_present` | Reviewer confirms no leakage in output artifacts. |
| `constraints.disallow_absolute_host_paths` | `CRS-SEC-003` | `security_privacy_checks.absolute_host_paths_present` | Reviewer confirms repo-relative path hygiene. |
| `constraints.include_system_invariants` | `CRS-DET-001` | `determinism_checks.notes` | Reviewer checks determinism assertions are present when required by card contract. |
| `constraints.include_reviewer_checklist` | `CRS-VAL-001` and `CRS-ACC-001` | `validation_checks.required_commands_run` and `acceptance_criteria` | Reviewer checks checklist-driven validation claims are evidenced. |

## Canonical Version Set (v0.8)

- Prompt Spec schema: `adl.v1`
- Checklist schema: `card_review_checklist.v1`
- Review output schema: `card_review_output.v1`
- Reviewer protocol id: `card_reviewer_gpt.v1.1`

If any one of these changes, a versioned update to this mapping is required.

