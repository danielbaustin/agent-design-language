# Task Bundles

This directory is the tracked home for canonical task bundles.

Each task bundle should live at:

- `docs/records/v0.85/tasks/<task-id>/`

With the canonical bundle contents:

- `stp.md`
- `sip.md`
- `sor.md`

Optional integration metadata may later reference:

- GitHub issue numbers
- PR numbers
- external tracker ids
- signatures
- replay/preservation artifacts

The important architectural rule is that the task bundle is the public record. GitHub issues and other trackers may reflect the task, but they do not define the bundle shape.

The matching canonical local draft shape is:

- `.adl/<scope>/tasks/<task-id>__<slug>/`

With the local bundle contents:

- `stp.stub.md`
- `stp.md`
- `sip.md`
- `sor.md`

If a GitHub issue number exists, it should be represented consistently in the local task id and filename segment, but it remains integration metadata rather than ontology.
