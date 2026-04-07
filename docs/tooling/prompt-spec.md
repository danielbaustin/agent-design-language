# Prompt Spec

## Purpose
Prompt Spec is a machine-readable block embedded in ADL input cards. It defines how execution prompts are assembled from card fields so generation is deterministic for identical card content.

Prompt Spec reduces heuristic parsing and gives a stable contract for automation and review tooling.

## Fields
- `prompt_schema`: Versioned schema identifier (for example `adl.v1`).
- `actor`: Execution actor metadata (`role`, `name`).
- `model`: Model identity and determinism mode.
- `inputs`: Ordered card sections included in generated prompts.
- `outputs`: Expected prompt-output targets (for example output card path).
- `constraints`: Safety and determinism constraints the generator must preserve.
- `automation_hints`: Machine-readable authoring and validation expectations used by prompt-generation tooling.
- `review_surfaces`: Reviewer protocols expected to consume outputs.

### Formal artifact mapping

Prompt Spec is the contract bridge between:

- `Structured Task Prompt` authoring inputs in tracked issue-prompt artifacts
- `Structured Implementation Prompt` authoring in input cards
- `Structured Output Record` capture in output cards

Repository shorthand still uses `issue prompts`, `input cards`, and `output cards`, but Prompt Spec should define the stable machine-readable contract that connects those layers.

### Supported `inputs.sections` IDs (v1)
- `goal`
- `required_outcome`
- `acceptance_criteria`
- `inputs`
- `target_files_surfaces`
- `validation_plan`
- `demo_proof_requirements`
- `constraints_policies`
- `system_invariants`
- `reviewer_checklist`
- `non_goals_out_of_scope`
- `notes_risks`
- `instructions_to_agent`

Required section semantics:

- `goal`: the task outcome in one bounded statement
- `required_outcome`: what must concretely ship (`code`, `docs`, `tests`, `demo`, or combination)
- `acceptance_criteria`: explicit pass/fail completion rules
- `inputs`: canonical upstream docs, prompts, issues, or repo surfaces
- `target_files_surfaces`: likely implementation touch-points and validation surfaces
- `validation_plan`: concrete commands, tests, artifacts, and reviewer checks
- `demo_proof_requirements`: required demo or proof path, or an explicit statement that none is required
- `constraints_policies`: determinism, privacy, and resource bounds
- `system_invariants`: cross-cutting runtime/artifact invariants that must remain true
- `reviewer_checklist`: machine-readable review hints
- `non_goals_out_of_scope`: bounded exclusions
- `notes_risks`: drift, ambiguity, or sequencing risks
- `instructions_to_agent`: final execution guidance

### `automation_hints` keys (v1)

- `source_issue_prompt_required`
- `target_files_surfaces_recommended`
- `validation_plan_required`
- `required_outcome_type_supported`

These keys must be present and boolean so prompt-generation and lint tooling can enforce whether a card is complete enough for deterministic execution.

### `review_surfaces` Protocol ID Rules
- Use dotted version IDs (for example `card_review_output.v1`), not underscore variants.
- IDs are case-sensitive and must match referenced specs exactly.
- Unknown or misspelled IDs should be treated as contract failures by automation/review tooling.

## Agent Interpretation
Agents should interpret Prompt Spec as an execution contract:
- Preserve section order from `inputs.sections`.
- Include required invariants/checklists when flags require them.
- Honor constraints (`disallow_secrets`, `disallow_absolute_host_paths`).
- Emit outputs using declared targets and structure.
- Preserve traceability back to the source issue prompt.
- Treat `required_outcome` and `demo_proof_requirements` as first-class execution constraints, not optional prose.

If Prompt Spec is missing, tooling may fall back to legacy card parsing behavior.

## Repository Execution Surface
Current repository-controlled execution path:

1. Input card with `## Prompt Spec` block
2. `adl tooling lint-prompt-spec` validates Prompt Spec structure and section IDs
3. `adl tooling card-prompt` consumes Prompt Spec ordering/flags to generate deterministic execution prompt text
4. Output cards preserve the execution record against the same contract surfaces

Example commands:

```bash
adl tooling lint-prompt-spec --issue 761
adl tooling card-prompt --issue 761 --out /tmp/issue-761.prompt.md
```

The prompt generator remains bounded to card-template section extraction and deterministic rendering. It is not a full natural-language authoring pipeline.

Minimum enforcement expectations in v0.85:

- unsupported section IDs fail lint
- missing required top-level Prompt Spec keys fail lint
- missing boolean `constraints` or `automation_hints` keys fail lint
- prompt generation preserves declared section order rather than using heuristic extraction

## Reviewer System Integration (#649-#651)
Prompt Spec aligns generation with the reviewer/tooling stack:
- #650 checklist spec validates required prompt surfaces are present.
- #651 deterministic review output format consumes stable review fields.
- #649 Card Reviewer GPT uses these stable surfaces for consistent review behavior.

Together, Prompt Spec + reviewer specs create an end-to-end deterministic path:

Input Card -> Prompt Generation -> Agent Execution -> Output Card -> Structured Review

## Protocol Bindings
Prompt Spec should declare reviewer-facing protocol surfaces using stable IDs:
- `card_review_checklist.v1`: required checklist semantics used to evaluate card completeness and policy alignment.
- `card_review_output.v1`: deterministic review artifact envelope used for machine-readable findings output.
- `card_reviewer_gpt.v1.1`: reviewer behavior contract for deterministic card evaluation and YAML-only output.

When these IDs are present in `review_surfaces`, prompt generators and reviewers can coordinate on stable contracts without markdown heuristic coupling.

Canonical ordering for `review_surfaces`:
1. `card_review_checklist.v1`
2. `card_review_output.v1`
3. `card_reviewer_gpt.v1.1`

This ordering keeps protocol dependencies explicit and deterministic.
`adl tooling lint-prompt-spec` enforces required IDs and canonical order.

## Shared Contract Mapping

Canonical field mapping between Prompt Spec, checklist rules, and deterministic review-output fields is defined in:

- `docs/tooling/prompt-review-surface-mapping.md`

That mapping is the source of truth for cross-surface alignment and drift prevention.
