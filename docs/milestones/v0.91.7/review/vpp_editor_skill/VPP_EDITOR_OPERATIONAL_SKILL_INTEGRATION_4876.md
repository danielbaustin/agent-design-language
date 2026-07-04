# VPP Editor Operational Skill Integration (#4876)

## Summary

After adding the local `vpp-editor` skill, the surrounding local operational skill network was updated so VPP card drift routes to the new editor instead of falling through to manual card repair.

## Updated Local Skill Surfaces

- `workflow-conductor`: includes `vpp-editor` in the card-editor family and routes card-local VPP issues to it.
- `sprint-conductor`: includes `vpp-editor` in sprint-level card-editor routing.
- `pr-ready`: names `vpp-editor` as a bounded cleanup option for readiness blockers.
- `pr-run`: composes with `vpp-editor` when validation-planning drift blocks execution binding.
- `pr-finish`: composes with `vpp-editor` when validation-planning truth drift blocks finish.
- `pr-closeout`: includes `vpp-editor` in the closeout card-editor family.

## Contract Boundary

The integration keeps the same boundary as the skill itself:

- `VPP` remains validation-planning truth.
- `SRP` remains review-result truth.
- `SOR` remains execution, validation-result, integration, and closeout truth.
- Operational skills may route to `vpp-editor`; they do not absorb VPP editing logic.

## Non-Claims

- This packet does not claim repository runtime code changed.
- This packet does not claim all future VPP defects are automatically repaired.
- This packet does not replace prompt-template/schema validation.
- The authoritative operational skill files live in the local Codex skill root; this PR records the integration evidence.
