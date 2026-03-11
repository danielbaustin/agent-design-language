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
- `review_surfaces`: Reviewer protocols expected to consume outputs.

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

If Prompt Spec is missing, tooling may fall back to legacy card parsing behavior.

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
`swarm/tools/lint_prompt_spec.sh` enforces required IDs and canonical order.
