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
