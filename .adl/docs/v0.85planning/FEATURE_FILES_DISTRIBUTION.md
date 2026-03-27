# Feature Files Distribution (Planning Directories)

This table defines the correct target planning directory for each roadmap milestone. Feature files must be moved into these directories during reconciliation. No files should be placed in `docs/milestones/` until the milestone is active.

| Roadmap band | Theme | Planning source |
| --- | --- | --- |
| `v0.85` | Operational maturity | `.adl/docs/v0.85planning/` |
| `v0.86` | Cognitive control band | `.adl/docs/v0.86planning/` |
| `v0.88` | Persistence, instinct, and bounded agency band | `.adl/docs/v0.88planning/` |
| `v0.89` | AEE convergence and security/threat modeling band | `.adl/docs/v0.89planning/` |
| `v0.90` | Reasoning graph, signed trace, and trace query band | `.adl/docs/v0.90planning/` |
| `v0.91` | Affect and moral cognition band | `.adl/docs/v0.91planning/` |
| `v0.92` | Identity, continuity, and provider capability band | `.adl/docs/v0.92planning/` |
| `v0.93` | Governance and delegation band | `.adl/docs/v0.93planning/` |
| `v0.95` | MVP convergence, tooling migration, and optional Zed band | `.adl/docs/v0.95planning/` |

## Rules

- Each feature file must exist in exactly one planning directory.
- The planning directory must match the roadmap milestone band.
- Do not duplicate files across planning directories.
- Do not promote files into `docs/milestones/` until that milestone is active.
