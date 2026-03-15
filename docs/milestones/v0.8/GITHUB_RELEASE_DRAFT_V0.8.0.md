# GitHub Release Draft - v0.8.0

## Release Metadata
- Target tag: `v0.8.0` (pending final ceremony)
- Release state: Draft (do not publish yet)
- Milestone: v0.8
- Source-of-truth docs:
  - `CHANGELOG.md`
  - `docs/milestones/v0.8/RELEASE_NOTES_V0.8.md`
  - `docs/milestones/v0.8/MILESTONE_CHECKLIST_V0.8.md`
  - `docs/milestones/v0.8/QUALITY_GATE_V0.8.md`
  - `docs/milestones/v0.8/RECOVERY_AUDIT_V0.8.md`

## Release Title (GitHub UI)
`v0.8.0 - Bounded Godel Runtime and Review Artifacts`

## Suggested GitHub Release Settings
- `Set as latest release`: **No** until ceremony gates pass.
- `Set as pre-release`: **Yes** until final go/no-go flips this release to publish-ready.
- `Generate release notes`: **No** (use the curated body below).

## Draft Release Body (Paste Into GitHub Release)

```markdown
## ADL v0.8.0 (Draft)

v0.8.0 advances the unreleased development milestone with bounded, reviewable runtime surfaces and supporting demos/docs. This draft release is intentionally scoped to implemented repository truth.

### Highlights

- Bounded Godel scientific loop surfaces in runtime/demo flows:
  - `failure -> hypothesis -> mutation -> experiment -> evaluation -> record -> indexing`
- New user-visible bounded CLI commands:
  - `adl godel run`
  - `adl godel inspect`
  - `adl godel evaluate`
- Canonical review artifacts emitted/validated in bounded runtime paths:
  - `mutation.v1`
  - `canonical_evidence_view.v1`
  - `evaluation_plan.v1`
  - `experiment_record.v1`
  - `tool_result.v1` sidecar support in bounded export paths
- Refreshed demo/runbook surfaces under `demos/`, including bounded Godel and bounded AEE recovery walkthroughs.
- Bounded Rust transpiler scaffold for deterministic fixture-to-runtime verification.

### Important Scope Boundaries

This draft does **not** claim:
- autonomous policy learning,
- unconstrained self-modification,
- a fully finished Adaptive Execution Engine,
- production graduation of the Rust transpiler scaffold.

### Release Readiness Notes

- Latest tagged release remains `v0.7.0`.
- v0.8 remains an active milestone on `main` and is not yet fully released.
- Final publication depends on milestone checklist completion, quality gates, and release-ceremony validation.

### References

- `CHANGELOG.md`
- `docs/milestones/v0.8/RELEASE_NOTES_V0.8.md`
- `docs/milestones/v0.8/MILESTONE_CHECKLIST_V0.8.md`
- `docs/milestones/v0.8/QUALITY_GATE_V0.8.md`
- `docs/milestones/v0.8/RECOVERY_AUDIT_V0.8.md`
```

## Finalization Checklist (Before Clicking Publish)
- [ ] Release tag `v0.8.0` (or superseding tag) is created and pushed.
- [ ] Required quality command suite and CI are green at release baseline.
- [ ] Deferred blocker-grade findings are resolved or explicitly accepted with owner.
- [ ] Links in the release body are validated.
- [ ] Release visibility is switched from draft/prerelease to intended final state.

## Notes
This artifact is a release-preparation input and should remain aligned with repository truth; update it if release-tail findings materially change what v0.8 can claim.
