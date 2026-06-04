# Portable ADL Project Templates

This directory contains reusable templates for ADL-governed external
repositories.

The initial template set is:

- version: `1.0.0`
- contract issue: `#3569`
- contract doc:
  `docs/tooling/PORTABLE_ADL_PROJECT_ADAPTER_CONTRACT_v0.91.5.md`

## Templates

| Template | Purpose |
|---|---|
| `1.0.0/AGENTS.md` | Compact repo-local operating instructions for an external ADL-governed repo. |
| `1.0.0/adl_project.json` | Machine-readable project adapter contract. |

## Examples

Example `adl_project.json` files live under `1.0.0/examples/`.

They are examples only. Do not copy one into a real repository without checking
issue authority, repository name, validation profile, artifact policy, and ADL
tooling compatibility.

## Portability Rules

- Template paths must stay repo-relative.
- Templates must not embed operator-local absolute paths.
- External repositories must resolve ADL tooling through `ADL_HOME`, an explicit
  repo-relative path, or a declared sibling checkout.
- The canonical card lifecycle remains `SIP -> STP -> SPP -> SRP -> SOR`.
