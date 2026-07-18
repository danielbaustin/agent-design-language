# #5463 Node 24 GitHub Action Pins

## Evidence

GitHub-hosted run `29632957768` emitted Node.js 20 deprecation annotations
for three immutable action revisions:

- `actions/checkout@34e114876b0b11c390a56381ad16ebd13914f8d5`
- `actions/upload-artifact@ea165f8d65b6e75b540449e92b4886f43607fa02`
- `Swatinem/rust-cache@779680da715d629ac1d338a641029a2f4372abb5`

Official release and action metadata resolves their reviewed replacements to:

- `actions/checkout@9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0` (`v7.0.0`, `node24`)
- `actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a` (`v7.0.1`, `node24`)
- `Swatinem/rust-cache@c19371144df3bb44fab255c43d04cbc2ab54d1c4` (`v2.9.1`, `node24`)

## Design

Replace only the three deprecated immutable revisions everywhere they occur.
Extend the existing CI runtime contract so all workflow occurrences must use
the canonical reviewed SHAs and the deprecated SHAs cannot reappear. Retain a
source-linked inventory of tag, commit, runtime, and affected workflows.

The PR's GitHub-hosted checks are the live proof surface. Their check-run
annotations must not contain the Node.js 20 deprecation message. AWS workflows
remain untouched except for static pin replacement and are not executed.

## Boundaries

- Preserve immutable full-SHA pinning.
- No floating tags.
- No AWS execution.
- No unrelated CI behavior or policy changes.
