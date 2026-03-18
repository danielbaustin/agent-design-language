# v0.85 Structured Prompt Records

This directory holds the tracked, public prompt-record surfaces for the v0.85 milestone.

The intended split is:

- `.adl/` = temporary draft workspace, generated intermediate files, local editor state
- `docs/prompts/v0.85/` = canonical public record for prompt artifacts that should be reviewable and auditable

The canonical tracked homes are:

- `stp/`
  - Structured Task Prompt records
- `sip/`
  - Structured Implementation Prompt records
- `sor/`
  - Structured Output Record records

Near-term workflow rule:

1. Draft locally in `.adl/`.
2. Promote the artifact into the tracked record tree before the lifecycle transition that makes it authoritative.
3. Keep GitHub issues and workflow commands aligned to the tracked artifact rather than treating `.adl/` as the durable source of truth.

This layout is intended to support both the current shell workflow and the planned editor/tooling surfaces.
