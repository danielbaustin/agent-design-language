# Toolkit Simplification Sprint Closeout (#3740)

Sprint issue: #3732  
Closeout issue: #3740  
Captured: 2026-06-16  
Status: ready_for_final_issue_closeout

## Summary

The v0.91.5 toolkit-simplification sprint completed the guidance and scan work
needed to keep the simplified tool surface truthful in live issue workflow.
The sprint did not complete a full small-binary rewrite of every remaining
tooling path. That deeper decomposition was explicitly carried forward into
`#3838` and should be treated as deferred continuation work, not hidden scope
inside this closeout.

This packet records the final sprint truth for the repo-tracked guidance wave:

- the active workflow spine still teaches `adl/tools/pr.sh` for tracked issue
  execution;
- prompt-template authoring and validation guidance now teaches the direct
  `adl-csdlc tooling prompt-template ...` surface in repo-tracked skill/docs
  guidance;
- docs-only and workflow-doc issues keep the focused `docs-bounded` validation
  fast path instead of pretending broad runtime proof is required;
- compatibility shims remain retained until separate scan-backed cut issues
  close them explicitly.

## What This Sprint Removed

- Repo-tracked skill and template guidance no longer needs to teach
  `cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template ...`
  as the normal operator-facing prompt-template workflow path.
- The simplification sprint no longer depends on an implicit "all work finished
  in the original 8-card wave" story; the closeout truth now records the later
  `8/9` expansion and the deferred continuation issue explicitly.

## What This Sprint Retained

- `adl/tools/pr.sh` remains the canonical tracked-issue workflow wrapper.
- `workflow-conductor` remains the required lifecycle router.
- Compatibility shims such as direct `adl pr ...`, `adl tooling ...`, and
  `adl/tools/codex_pr.sh` remain governed compatibility surfaces rather than
  silent removals.
- Historical records are preserved as readability evidence instead of being
  rewritten just to remove old command strings.

## What This Sprint Deferred

- `#3838`: remaining direct small-binary decomposition work beyond the first
  simplification wave.
- Active and unknown command-reference findings recorded in
  `docs/milestones/v0.91.5/ACTIVE_COMMAND_REFERENCE_SCAN_3735.md`.
- Any future compatibility-cut or fail-closed removal work that depends on the
  scan gate and bounded follow-on issues.

## Key Evidence

- `AGENTS.md`
- `docs/templates/prompts/current.json`
- `docs/milestones/v0.91.5/CLI_SHIM_DEPRECATION_POLICY_3615.md`
- `docs/milestones/v0.91.5/CLI_WRAPPER_MIGRATION_CONTRACT_3597.md`
- `docs/milestones/v0.91.5/CONTROL_PLANE_OBSERVABILITY_CONTRACT_3609.md`
- `docs/milestones/v0.91.5/TOOL_SURFACE_REGISTRY_3734.md`
- `docs/milestones/v0.91.5/ACTIVE_COMMAND_REFERENCE_SCAN_3735.md`

## Residual Risks

- The active-command scan still reports retained `active` and `unknown`
  references for legacy command families, so shim deletion remains unsafe.
- Some milestone and inventory docs still describe future owner-command shapes
  as planning truth rather than live workflow guidance; those remain follow-on
  cleanup rather than evidence that this sprint failed.
- The broader all-tools simplification goal now spans beyond the original
  sprint wave and should not be silently collapsed back into `#3740`.

## Validation Performed

- Focused repo-guidance update review against the active workflow contract in
  `AGENTS.md`, the prompt-template registry in `docs/templates/prompts/current.json`,
  and the scan/registry packets `#3734` and `#3735`.
- Command-guidance cleanup for repo-tracked skill/docs surfaces that still
  taught older prompt-template invocation forms.

## Closeout Decision

`#3740` is ready to close once its paired guidance edits, review card truth,
and SOR outcome record are complete. The broader simplification story remains
open only through the explicit deferred follow-ons, especially `#3838`.
