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

### Supported `inputs.sections` IDs (v1)
- `goal`
- `acceptance_criteria`
- `inputs`
- `constraints_policies`
- `system_invariants`
- `reviewer_checklist`
- `non_goals_out_of_scope`
- `notes_risks`
- `instructions_to_agent`

## Agent Interpretation
Agents should interpret Prompt Spec as an execution contract:
- Preserve section order from `inputs.sections`.
- Include required invariants/checklists when flags require them.
- Honor constraints (`disallow_secrets`, `disallow_absolute_host_paths`).
- Emit outputs using declared targets and structure.

If Prompt Spec is missing, tooling may fall back to legacy card parsing behavior.

## Repository Execution Surface
Current repository-controlled execution path:

1. Input card with `## Prompt Spec` block
2. `swarm/tools/lint_prompt_spec.sh` validates Prompt Spec structure and section IDs
3. `swarm/tools/card_prompt.sh` consumes Prompt Spec ordering/flags to generate deterministic execution prompt text

Example commands:

```bash
swarm/tools/lint_prompt_spec.sh --issue 761
swarm/tools/card_prompt.sh --issue 761 --out /tmp/issue-761.prompt.md
```

The prompt generator remains bounded to card-template section extraction and deterministic rendering. It is not a full natural-language authoring pipeline.

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
- `card_reviewer_gpt.v1`: reviewer behavior protocol binding for deterministic review interpretation.

When these IDs are present in `review_surfaces`, prompt generators and reviewers can coordinate on stable contracts without markdown heuristic coupling.
