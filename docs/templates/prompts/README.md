# Versioned C-SDLC Prompt Templates

This directory is the canonical tracked home for C-SDLC issue-card templates.

The active template set is declared in [`current.json`](current.json). Each
template file is a direct copy-and-fill card form, not an explanatory wrapper.
Each active template entry also points at a generated structure schema artifact
used by prompt-card structure validation.

## Current Set

- Template set: `1.0.2`
- Lifecycle: `SIP -> STP -> SPP -> SRP -> SOR`
- Template root: `docs/templates/prompts/1.0.2/`
- Registry: `docs/templates/prompts/current.json`
- Structure schemas: `docs/templates/prompts/1.0.2/schemas/*.structure.json`
- Implementation owner: Rust tooling owns the canonical template registry,
  field model, schema extraction, and validation path. Python sprint helpers may
  load schema artifacts, fill templates, or call the Rust-backed validators, but
  they should not become a separate template authority.

## Staged Next Set

- Template set: `1.0.3`
- Lifecycle: `SIP -> STP -> SPP -> VPP -> SRP -> SOR`
- Status: staged for renderer/schema validation in `#4309`; not yet the active
  issue-bundle lifecycle
- Template root: `docs/templates/prompts/1.0.3/`
- Activation boundary: downstream issue-bundle/bootstrap adoption must land
  before `current.json` moves from `1.0.2` to `1.0.3`

## Values Renderer

v0.91.5 adds the next authoring model described in
[`PROMPT_TEMPLATE_VALUES_RENDERER_PLAN_v0.91.5.md`](../../tooling/PROMPT_TEMPLATE_VALUES_RENDERER_PLAN_v0.91.5.md).
The short operator checklist for using the renderer and structure schemas is
[`PROMPT_TEMPLATE_CARD_GENERATION_CHECKLIST.md`](../../tooling/PROMPT_TEMPLATE_CARD_GENERATION_CHECKLIST.md).

The intended direction is deterministic rendering:

```text
prompt template set + card kind + values object -> rendered card Markdown
```

Rendered Markdown remains the reviewable lifecycle artifact. The ordinary
editing surface can now be a values object with locked system fields, required
values, enum validation, placeholder checks, and Rust-owned static validation.
The browser editor exposes those values, but it must not become a separate
template or validation authority.

For new or fully re-rendered cards, prefer this path over direct Markdown
structure edits:

1. fill or export values YAML;
2. run `validate-values`;
3. render with `render` or `render-all`;
4. run `validate-structure` on each rendered card;
5. run `validate-schemas` and the Python schema smoke check when schema
   artifacts are touched.

Editor skills still own lifecycle-truth repairs. The renderer owns deterministic
template filling and shape preservation; it does not invent execution, review,
PR, merge, or closeout truth.

## Structure Schemas

The `*.structure.json` files under the active template set are generated from
the Markdown templates and consumed by the Rust validator. They record:

- the template set and card kind;
- the source template path;
- the parser used for Markdown extraction;
- frontmatter key inventory;
- Markdown heading order;
- fenced code block shape;
- locked template prose.

Use the Rust tool to regenerate schemas only when template structure changes
intentionally:

```sh
adl-csdlc tooling prompt-template \
  write-structure-schemas \
  --template-set 1.0.2 \
  --out-dir docs/templates/prompts/1.0.2/schemas
```

Then run both Rust and Python-readable schema checks:

```sh
adl-csdlc tooling prompt-template validate-schemas --template-set 1.0.2
python3 adl/tools/test_prompt_template_structure_schemas.py --template-set 1.0.2
```

If `adl-csdlc` is not already on `PATH`, run the same owner-binary commands
through `cargo run --manifest-path adl/Cargo.toml --bin adl-csdlc -- ...`
from a fresh checkout.

`current.json` should not move to a new active template set until every card
kind in that set has renderer fixtures, values validation, and compatibility
notes. For staged six-card sets such as `1.0.3`, that means `VPP` must be
validated alongside the existing five cards before activation.

## Local Editor

The local human editor for this template set lives at
`docs/tooling/csdlc-prompt-editor/`. Its field model and browser metadata are
generated from Rust with:

```sh
cargo run --manifest-path adl/Cargo.toml --bin adl-csdlc -- tooling csdlc-prompt-editor \
  --emit-model-js docs/tooling/csdlc-prompt-editor/editor_model.js
```

## Versioning Policy

- Template-set versions use SemVer.
- `1.0.0/` through `1.0.2/` are immutable after adoption except for obvious
  typo fixes.
- Future semantic changes create a new SemVer directory, such as `1.1.0/` or
  `2.0.0/`, then update `current.json`.
- Tools should resolve the active paths from `current.json` when practical, but
  may use the `1.0.0/` paths directly during the first adoption window.

## Template Objects

Each prompt template is treated as a first-class C-SDLC object:

- it has a semantic role (`SIP`, `STP`, `SPP`, `VPP`, `SRP`, or `SOR`)
- it belongs to one SemVer template set
- it carries a deterministic `card_status`
- it is human-readable Markdown
- it is machine-fillable through stable placeholder fields
- it remains validator-compatible after placeholders are filled

## Card Status

Every filled prompt card uses this small lifecycle enum:

- `draft`: the card exists but is incomplete or locally invalid
- `ready`: the card is filled and locally validator-clean
- `reviewed`: the card has been reviewed but is not yet an execution gate
- `approved`: the required review gate has accepted the card for use
- `completed`: the card has fulfilled its lifecycle role and is now audit truth
- `blocked`: the card cannot advance until an upstream condition changes
- `superseded`: a newer card or template version has replaced this card

The local browser editor derives `draft` or `ready` from form validation.
Lifecycle tooling should set `reviewed`, `approved`, `completed`, `blocked`,
or `superseded` only at the corresponding C-SDLC state transition.

Execution preflight is intentionally stricter than enum validation:

- `SIP`, `STP`, and `SPP` must be `ready` or `approved` before execution binds.
- `SPP` may return to `draft` when the real execution path materially diverges.
- `SRP` may be `completed` only after review results, dispositions, or an
  explicit final policy exception are recorded.
- `SOR` may be `completed` only after terminal closeout truth is present.
- `SOR` execution `Status` remains separate from card lifecycle `Card Status`.

## Host-Path Scan Wording

Lifecycle cards must not record concrete machine-local absolute paths. It is
acceptable to describe the scan patterns that were checked, such as `/Users/`,
`/home/`, `/tmp/`, and `/var/folders/`, when the card also states that no
concrete host-local paths were recorded. Do not include example usernames,
temporary directories, or full local artifact paths in durable cards.

## Compatibility

Older files under `adl/templates/cards/` and legacy structured-prompt template
docs remain compatibility surfaces. New card generation should treat the active
registry target from `current.json` as canonical, while staged future sets such
as `docs/templates/prompts/1.0.3/` may be rendered and schema-validated
explicitly through `--template-set` before activation.
