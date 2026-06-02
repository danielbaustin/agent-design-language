# Prompt Template Values Renderer Plan

## Metadata

- Milestone: `v0.91.5`
- Issue: `#3553`
- Status: `implemented_for_v1_0_0_templates_with_downstream_rewrite_follow_on`
- Owner: ADL maintainers
- Scope: C-SDLC prompt cards `SIP -> STP -> SPP -> SRP -> SOR`

## Summary

ADL should move prompt-card authoring from direct Markdown structure editing to
a deterministic `template + values -> rendered Markdown` model.

The rendered Markdown remains the human-readable review artifact and the
surface consumed by existing validators. The editable surface becomes a values
object with static validation for required fields, locked system fields, enum
checks, and placeholder resolution. This reduces the card churn seen during
v0.91.4/v0.91.5 setup without changing C-SDLC lifecycle semantics.

## Target Model

```text
prompt template set + card kind + values object -> rendered card Markdown
```

The renderer must be deterministic:

- same template set, card kind, and values object produce byte-stable Markdown;
- missing required values fail before rendering;
- unresolved placeholders fail validation;
- invalid enum values fail validation;
- locked/system fields cannot be modified through ordinary editor values;
- rendered Markdown remains compatible with `validate_structured_prompt.sh`.

## Template Set

The existing active prompt-template registry remains the compatibility baseline:

- Registry: `docs/templates/prompts/current.json`
- Current set: `docs/templates/prompts/1.0.0/`

The v0.91.5 implementation renders the current `1.0.0` templates from values
metadata without moving `current.json`. A future `1.1.0` template set should be
created only if the renderer contract requires semantic template changes.

Do not mutate `current.json` to a new active set until:

- all five card kinds render from values;
- focused renderer/validator fixtures pass;
- compatibility behavior for existing Markdown cards is recorded;
- the C-SDLC prompt editor consumes the Rust-owned values model.

## Values Object Shape

Values should be represented as JSON or YAML with one object per rendered card.
The canonical schema should include:

- `schema`: values object schema id;
- `template_set`: SemVer template set;
- `card_kind`: one of `sip`, `stp`, `spp`, `srp`, `sor`;
- `issue`: GitHub issue number;
- `version`: milestone version;
- `slug`: normalized issue slug;
- `title`: issue title;
- `branch`: execution branch or `not bound yet`;
- `card_status`: lifecycle card status;
- `generated_at`: timestamp supplied by caller when needed;
- `system`: locked system fields;
- `values`: human/operator editable fields;
- `validation`: optional expected validation posture for fixtures.

The implementation should treat `system` fields as authoritative control-plane
inputs and `values` fields as the ordinary editing surface.

## Locked/System Fields

The renderer must not let ordinary values edits rewrite:

- issue number;
- template set;
- card kind;
- normalized slug;
- output-card path;
- source issue prompt path;
- required outcome type;
- demo-required flag;
- branch/worktree truth;
- lifecycle rules and system invariants;
- validator or review-surface identifiers.

If a locked field is wrong, the fix belongs in the conductor/control-plane
source, not in the values editor.

## Editable Fields

Editable fields should be explicit and card-specific.

Common editable groups:

- summary;
- goal;
- required outcome;
- deliverables;
- acceptance criteria;
- repo inputs;
- dependencies;
- target files/surfaces;
- validation plan;
- demo/proof requirements;
- non-goals;
- issue-graph notes;
- notes/risks.

`SRP` and `SOR` need additional guarded fields for review-result truth and
execution/integration truth. Those fields should not be marked complete unless
review or execution evidence exists.

## Implemented Validation Failure Modes

The v0.91.5 values validator implemented by `#3553` rejects:

- unknown template set;
- unknown card kind;
- missing required value;
- unresolved `{{var}}` placeholder in rendered output;
- unresolved legacy angle placeholder in rendered output when validating a
  versioned prompt-template card;
- invalid enum value;
- invalid issue, version, slug, and card-status values;
- locked/system field supplied through ordinary `values`.

The following validation surfaces are intentionally routed to follow-on guard
work instead of being claimed by the `#3553` renderer:

- rendered Markdown structure drift;
- host-local absolute paths or secret markers in public card output;
- richer typed list/item schema validation beyond the current values model.

Markdown structure immutability is tracked by `#3585`, which must land before
the downstream card rewrite in `#3582`. Public artifact hygiene and redaction
checks remain separate review/validation surfaces unless a later issue binds
them directly to the renderer.

Validation should fail fast with short, deterministic messages. These docs are
small; failures should be effectively immediate.

## Renderer CLI

The Rust-owned CLI shape is:

```sh
cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template render \
  --kind stp \
  --values path/to/stp.values.yaml \
  --out path/to/stp.md
```

Recommended validation shape:

```sh
cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template validate-values \
  --kind stp \
  --values path/to/stp.values.yaml
```

The implementation also supports:

```sh
cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template write-sample-values \
  --out-dir /tmp/csdlc-prompt-values

cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template render-all \
  --values-dir /tmp/csdlc-prompt-values \
  --out-dir /tmp/csdlc-prompt-cards
```

The existing Markdown validator remains in use:

```sh
bash adl/tools/validate_structured_prompt.sh --type stp --input path/to/stp.md
```

## Editor Integration

The C-SDLC prompt editor should edit values, not Markdown structure.

Implemented editor behavior:

- consume the Rust-generated prompt editor model;
- show locked/system fields as read-only;
- allow only editable values fields to change;
- render preview Markdown deterministically;
- export both the values object and rendered Markdown where useful;
- keep browser validation advisory;
- rely on Rust validation as authoritative.

The editor must not become a separate schema authority.

## Compatibility Path

Existing rendered Markdown cards remain valid lifecycle records.

Compatibility rules:

- existing `1.0.0` Markdown cards can be validated as Markdown;
- new values-rendered cards should carry enough metadata to identify the
  template set and card kind;
- downstream card rewrite after the renderer and AST structure guard is tracked
  through `#3582`;
- historical cards are not migrated in `#3553`;
- cards with truthful lifecycle state should not be rewritten merely for style.

## Follow-On Routing

This issue implements the renderer and values validator. The remaining bounded
follow-ons are Markdown AST template immutability validation (`#3585`) and then
downstream card rewrite/normalization after both guards land (`#3582`).

## Acceptance Bar For Follow-Ons

Renderer implementation is complete when:

- all five card kinds can generate sample values;
- renderer output is deterministic for identical values;
- values validation rejects the implemented failure modes above;
- rendered Markdown passes existing structured prompt validation;
- editor model is Rust-owned and can render/preview the values contract;
- `#3585` can add structure immutability proof without redesigning the renderer;
- `#3582` can rewrite downstream v0.91.5 cards without hand-editing structure
  after the renderer and AST guard are available.

## Non-Claims

- This plan does not claim a new `1.1.0` template set is active.
- This plan does not migrate historical cards.
- This plan does not change C-SDLC lifecycle semantics.
- This plan does not make browser/editor validation authoritative.
- This plan does not claim rendered Markdown structure immutability, redaction,
  or host-path scanning is implemented by `#3553`.
- This plan does not close `#3585`.
- This plan does not close `#3582`.
