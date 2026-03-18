# Tooling Docs

This directory documents ADL tooling contracts used by structured prompt automation and reviewer flows.

Prompt Spec is the bridge between:
- structured task prompts (`issue prompts`)
- structured implementation prompts (`input cards`)
- structured output records (`output cards`)

Tracked public workflow history should live in task-centric record bundles under `docs/records/`, while `.adl/` remains the temporary draft workspace.

## References
- [Prompt Spec](prompt-spec.md): machine-readable input-card block defining deterministic prompt generation surfaces and reviewer alignment.
- [Structured Prompt Contracts](structured-prompt-contracts.md): machine-checkable contracts for Structured Task Prompts, Structured Implementation Prompts, and Structured Output Records.
- [Task Bundle Editor](editor/README.md): first bounded editor surface for tracked STP/SIP task-bundle authoring.
- [Prompt Spec Protocol Bindings](prompt-spec.md#protocol-bindings): linkage to `card_review_checklist.v1` and `card_review_output.v1` reviewer contracts.
- [Prompt/Reviewer Surface Mapping](prompt-review-surface-mapping.md): field-by-field contract map between Prompt Spec, checklist rules, and deterministic review output fields.
- [Issue Prompt Templates](issue-prompts/README.md): tracked templates and authoring guidance for structured issue prompts used to shape GitHub issues before `pr start`.
- [Public Task Records](../records/README.md): tracked task-centric record homes for canonical STP/SIP/SOR bundles.
- Prompt Spec execution tooling:
  - `swarm/tools/lint_prompt_spec.sh` (Prompt Spec lint/validation)
  - `swarm/tools/card_prompt.sh` (deterministic prompt generation from cards)
  - `swarm/tools/validate_structured_prompt.rb` (structured prompt contract validation)
- [Card Reviewer GPT Instructions](card-reviewer-gpt.md): canonical reviewer behavior and deterministic YAML output contract (`card_reviewer_gpt.v1.1`).
- [Deterministic Review Output Format](card-review-output-format.md): canonical review artifact schema including finding evidence-state semantics.
- Reviewer regression fixture (stable):
  - `docs/tooling/examples/reviewer-regression/issue-661/input_661.md`
  - `docs/tooling/examples/reviewer-regression/issue-661/output_661.md`
  - `docs/tooling/examples/reviewer-regression/issue-661/expected_review_output_661.yaml`
