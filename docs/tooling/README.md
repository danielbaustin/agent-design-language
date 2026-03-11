# Tooling Docs

This directory documents ADL tooling contracts used by card automation and reviewer flows.

## References
- [Prompt Spec](prompt-spec.md): machine-readable input-card block defining deterministic prompt generation surfaces and reviewer alignment.
- [Prompt Spec Protocol Bindings](prompt-spec.md#protocol-bindings): linkage to `card_review_checklist.v1` and `card_review_output.v1` reviewer contracts.
- [Prompt/Reviewer Surface Mapping](prompt-review-surface-mapping.md): field-by-field contract map between Prompt Spec, checklist rules, and deterministic review output fields.
- Prompt Spec execution tooling:
  - `swarm/tools/lint_prompt_spec.sh` (Prompt Spec lint/validation)
  - `swarm/tools/card_prompt.sh` (deterministic prompt generation from cards)
- [Card Reviewer GPT Instructions](card-reviewer-gpt.md): canonical reviewer behavior and deterministic YAML output contract (`card_reviewer_gpt.v1.1`).
