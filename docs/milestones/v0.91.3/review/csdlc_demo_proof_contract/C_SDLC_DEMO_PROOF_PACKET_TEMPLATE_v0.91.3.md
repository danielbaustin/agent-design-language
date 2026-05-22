# C-SDLC Demo Proof Packet Template v0.91.3

Use this template for later C-SDLC demo packets.

Replace placeholders with demo-specific truth. Delete sections only when the
packet explicitly records `not run` or `skipped` and explains why.

## Demo Identity

- demo name:
- issue / WP:
- milestone version: `v0.91.3`
- primary artifact:

## Bounded Purpose

State the one bounded purpose of the demo.

## Claims

- claim 1:
- claim 2:

## Non-Claims

- non-claim 1:
- non-claim 2:

## Run Path

- primary command:
- operator prerequisites:
- run status: `passed` | `partial` | `skipped` | `failed` | `not run`

## Timebox Truth

- timebox claim:
- evidence type: `measured` | `estimated` | `not run`
- start evidence:
- end evidence:
- elapsed result:

## Validation Evidence

```bash
# focused commands actually run
```

Validation not run:

- none | explain omitted validation

## Review Evidence

- review surface:
- findings fixed before publication:
- residual risks:

## Result Classification

| Claim | Classification | Reason |
| --- | --- | --- |
| main bounded claim | `passed` | |
| timebox claim | `partial` | |

## Skipped Work

- skipped scope:
- why it was skipped:

## Repo-Relative Artifacts

- `path/to/demo/output`
- `path/to/review/packet`
