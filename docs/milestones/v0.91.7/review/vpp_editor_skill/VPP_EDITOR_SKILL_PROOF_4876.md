# VPP Editor Skill Proof (#4876)

## Summary

Issue `#4876` adds a local Codex `vpp-editor` skill so ADL agents have a dedicated card-editor surface for Validation Planning Prompt cards.

The skill lives in the operator skill root, not in this repository, so this proof packet records the reviewable contract and validation performed for the issue PR.

## Local Skill Surface

- Skill name: `vpp-editor`
- Local path: operator Codex skill root, `skills/vpp-editor/SKILL.md`
- UI metadata: `skills/vpp-editor/agents/openai.yaml`

## Contract Covered

The skill documents:

- VPP as validation-planning truth, not validation-result truth.
- Lifecycle placement: `SIP -> STP -> SPP -> VPP -> SRP -> SOR`.
- PVF lane selection and validation profile repair.
- Planned validation commands, selected lanes, parallel groups, and failure policy.
- Explicit run/defer/block semantics so skipped or pending lanes cannot be mistaken for proof.
- Required estimate and goal-budget fields for execution binding.
- Prompt-template renderer guidance for `kind vpp` where supported.
- Handoffs to `spp-editor`, `srp-editor`, and `sor-editor` when the defect is not actually VPP planning truth.

## Validation

Validated frontmatter with Ruby's built-in YAML parser:

```sh
ruby -e 'require "yaml"; ...'
```

Result: passed.

The skill-creator `quick_validate.py` script was also attempted, but the local Python runtimes available in this session do not have `PyYAML` installed, so that specific helper could not run without modifying the tool environment. That is an environment/tooling limitation, not a skill frontmatter failure.

## Non-Claims

- This issue does not change ADL repository code.
- This issue does not create or alter VPP prompt templates.
- This issue does not claim PVF runtime behavior changes.
- This issue does not claim any validation lane was executed beyond local skill-frontmatter validation.

## Tracked Snapshot

For reviewer convenience, this PR includes a snapshot copy of the local skill contract at:

- `docs/milestones/v0.91.7/review/vpp_editor_skill/vpp-editor.SKILL.md`

The authoritative runtime skill remains the local Codex skill root copy.

## Operational Skill Routing Update

The surrounding local operational skills were updated after the first PR publication so VPP card defects route to `vpp-editor` consistently. See:

- `docs/milestones/v0.91.7/review/vpp_editor_skill/VPP_EDITOR_OPERATIONAL_SKILL_INTEGRATION_4876.md`
