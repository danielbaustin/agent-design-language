# v0.91.5 Release Plan

## Metadata
- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Release date: pending release ceremony
- Release manager: ADL maintainers
- Status: `sprint_4_release_tail_active`

## How To Use

Execute this plan during the release tail. Check items only when evidence
exists. The ceremony must not hide implementation, review, or truth-maintenance
work.

Current release-tail truth:

- WP-14 quality gate is applied and recorded as release-tail input.
- WP-15 docs/review alignment is recorded as release-tail input.
- second-pass internal review closed through `#3923`.
- the first internal-review remediation wave already closed through `#3899`.
- external/third-party review closed through `#3580`.
- final review remediation and pre-v0.92 readiness routing closed through
  `#3577`.
- Rust/GitHub transport-boundary cleanup follow-on `#3929` is closed and
  consumed as release-tail tooling truth.
- The remaining pre-ceremony path is WP-19 handoff confirmation and WP-20
  ceremony.

## 0. Release-Tail Convergence

- [ ] Multi-agent proof packet or explicit blocker complete.
- [ ] Provider/model matrix and OpenRouter disposition complete.
- [ ] Public prompt packet export/redaction evidence complete.
- [ ] `.adl` archive/deletion review disposition complete.
- [ ] Demo readiness proof map complete.
- [ ] v0.92 activation test map complete.
- [ ] AEE completion tranche routed through `#3534`, with v0.92 owner/proof
      expectations explicit.
- [ ] `#3377` first-birthday readiness packet complete.
- [x] `#3929` GitHub transport-boundary cleanup merged/closed or explicitly
      rerouted as non-release-blocking.
- [ ] Closed issue/card truth refreshed for the v0.91.5 wave, including a
      replacement for the retired shell-helper audit path or an explicit
      accepted substitute.

## 1. Release Readiness

- [ ] Milestone checklist complete.
- [ ] Release notes approved.
- [ ] Go/no-go decision recorded in the decision log.
- [ ] v0.92 preflight says go, conditional go, or no-go with owners.

## 2. Branch And Tag Preparation

- [ ] Target branch confirmed: `main`.
- [ ] Working tree clean.
- [ ] Version strings validated.
- [ ] Tag created: `v0.91.5`.
- [ ] Tag pushed and verified.

## 3. GitHub Release Steps

- [ ] GitHub Release draft created from `v0.91.5`.
- [ ] Release body populated from approved release notes.
- [ ] Links to key PRs/issues included.
- [ ] Release visibility confirmed.
- [ ] Release published.

## 4. Verification

- [ ] Post-release CI status checked.
- [ ] Release links tested.
- [ ] Immediate regressions triaged and tracked.

## 5. Communication

- [ ] v0.91.6 opening inputs announced internally.
- [ ] v0.91.7 second-tranche rule announced internally.
- [ ] 15-minute operator break scheduled after ceremony before v0.91.6 starts.
- [ ] Roadmap/status docs updated.
- [ ] Deferred items routed to later backlog.

## Exit Criteria

- No hidden implementation remains in ceremony.
- Tag and release are published and accessible.
- v0.91.6 can consume the bridge evidence without reconstructing intent from
  chat, and v0.91.7/v0.92 routing is explicit.
