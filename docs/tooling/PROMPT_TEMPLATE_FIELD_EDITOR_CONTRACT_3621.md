# Prompt-Template Field Editor Contract

Issue: `#3621`
Milestone: `v0.91.5`

## Purpose

The field editor gives Sprint 1 a deterministic way to update prompt-card
values without editing locked rendered Markdown. It is intentionally narrower
than a full Markdown editor: the editable surface is the values YAML consumed by
the active prompt-template renderer.

## Command

```sh
cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template \
  edit-values \
  --kind <sip|stp|spp|srp|sor> \
  --values <values.yaml> \
  --set <field=value> \
  [--set <field=value> ...] \
  [--out <values.yaml>] \
  [--repo-root <path>]
```

The command is also available through the owner binary path:

```sh
cargo run --manifest-path adl/Cargo.toml --bin adl-csdlc -- tooling prompt-template edit-values ...
```

## First-Slice Editable Fields

The supported fields are the declared editable fields in the Rust prompt editor
model for each card kind. The first slice proves at least one high-value update
path for every active card:

| Card | Example editable field | Typical use |
|---|---|---|
| `sip` | `goal` | Tighten issue intent without touching branch, issue, or routing fields. |
| `stp` | `summary` | Tighten task summary or acceptance-facing prose. |
| `spp` | `plan_summary` | Tighten the operative plan before execution binds. |
| `srp` | `notes_risks` | Record review scope notes before or after review. |
| `sor` | `status` | Move execution status through the declared enum when evidence supports it. |

## Validation Contract

`edit-values` fails closed before writing output when:

- `--set` is omitted or malformed;
- the card kind is unsupported;
- the values file schema, template set, or card kind does not match the active
  registry;
- the target field is unknown;
- the target field is locked/system-owned;
- enum or required-field validation fails;
- rendering leaves unresolved placeholders; or
- rendered Markdown structure fails the tracked schema validation.

After applying updates in memory, the tool runs the same validation path as
`validate-values` plus an in-memory `render` and `validate-structure` pass
against `docs/templates/prompts/<version>/schemas/*.structure.json`.

## Boundary

Use this tool for supported field edits. Use the card editor skills for
lifecycle-truth judgment, review dispositions, execution evidence, or repairs
that require semantic reasoning. Do not patch rendered locked prose by hand
when a declared values-field update is sufficient.
