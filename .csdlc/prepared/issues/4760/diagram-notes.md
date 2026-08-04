# #4760 Diagram Notes

## Metadata

- Skill: diagram-author
- Subject: issue #4760 Memory Palace implementation/proof dependency
- Date: 2026-07-31
- Output location: `.csdlc/prepared/issues/4760/diagram.mmd`

## Target

- Mode: draft_from_issue
- Source: GitHub #4760 plus the source references in `design.md`
- Audience: execution owner and ADR reviewer
- Diagram goal: document the smallest producer/consumer path and the hard
  evidence gate before #5007

## Diagram Decision

- Diagram family: flowchart
- Backend: Mermaid
- Rationale: the packet needs a small Markdown-native dependency and evidence
  flow, not a full architecture model.

## Truth Boundary

Source-backed elements:

- ObsMem records/citations and temporal anchors exist in
  `adl/src/obsmem_contract/models.rs`.
- `AgentSpec.memory` exists in `adl/src/long_lived_agent/types.rs`.
- `decision_request.memory_refs` is currently emitted by
  `adl/src/long_lived_agent.rs`.
- #4760 is the implementation/proof owner; #5007 / ADR 0051 remains deferred.

Prepared implementation elements:

- `memory_palace.rs`, topology/working-set/stale packets, and the runtime edge
  are intended #4760 work and are not drawn as current implementation truth.

Unknowns:

- Exact accepted Chronosense compatibility fields must be refreshed from
  #4765, #4768, and #4771 at execution time.
- #5007's eventual review disposition is unknown and must remain evidence-led.

Unsupported claims added: false.

## Render Validation

- Render attempted: recorded in `validation/preparation-validation.md`.
- Validation command: `mmdc -i .csdlc/prepared/issues/4760/diagram.mmd -o .csdlc/prepared/issues/4760/validation/diagram.svg`
- Publication attempted: false.
- External upload attempted: false.
- Human review required: yes; included in the bounded preparation review.
