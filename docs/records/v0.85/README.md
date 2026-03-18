# v0.85 Public Task Records

This directory holds tracked public task records for the v0.85 milestone.

The intended split is:

- `.adl/`
  - temporary draft workspace
  - generated intermediate files
  - editor-local scratch state
- `docs/records/v0.85/tasks/`
  - canonical public task bundles for artifacts that should be reviewable, auditable, and preservable

Each task bundle is intended to hold:

- `stp.md`
  - Structured Task Prompt
- `sip.md`
  - Structured Implementation Prompt
- `sor.md`
  - Structured Output Record
- later, optional preservation material such as:
  - metadata
  - signatures
  - artifact references

This shape is deliberately task-centric so ADL can support software engineering and non-software task domains without making GitHub issues the primary ontology.
