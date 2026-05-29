# v0.91.5 Milestone Checklist

## Metadata
- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Date: `2026-05-29`
- Owner: ADL maintainers
- Status: `draft_pre_open`

## Purpose

Track the minimum planning, execution, quality, release, and post-release
checks needed for truthful v0.91.5 closeout.

## Planning

- [ ] v0.91.5 planning package exists.
- [ ] Feature docs exist for all bridge work tracks.
- [ ] Open side issues are relabeled to `version:v0.91.5`.
- [ ] v0.91.4 docs say bridge work moved, not abandoned.
- [ ] v0.92 docs depend on v0.91.5 closeout and `#3377`.
- [ ] All opened v0.91.5 issues use five prompt cards from the active registry.

## Execution Discipline

- [ ] `SIP`, `STP`, and `SPP` are design-time ready before execution starts.
- [ ] `SRP` records actual review prompts and findings.
- [ ] `SOR` records actual validation, integration, and closeout truth.
- [ ] Work executes in bound worktrees.
- [ ] Pre-PR review runs before publication.
- [ ] Scope stays within each issue or is explicitly routed.

## Quality Gates

- [ ] Multi-agent C-SDLC workcell proof completes or blocks truthfully.
- [ ] OpenRouter/provider matrix work completes or blocks truthfully.
- [ ] Public prompt packet export/redaction/index path completes or blocks
  truthfully.
- [ ] `.adl` cleanup/archive decisions are reviewed before deletion.
- [ ] Demo readiness and Unity Observatory routing are explicit.
- [ ] v0.92 activation test map is complete and consumed by `#3377`.

## Release Packaging

- [ ] Release plan complete.
- [ ] Release notes rewritten from landed evidence.
- [ ] v0.92 final preflight complete.
- [ ] Review findings fixed or routed.
- [ ] Release evidence package assembled.

## Post-Release

- [ ] v0.92 WP-01 inputs are linked.
- [ ] Deferred bridge items have owners and follow-on routing.
- [ ] Residual risks are recorded in release notes or handoff.

## Exit Criteria

- All required gates are checked or exceptions have owners.
- v0.92 can open from v0.91.5 closeout and `#3377`.
- Multi-agent completion/blocker truth is explicit.
