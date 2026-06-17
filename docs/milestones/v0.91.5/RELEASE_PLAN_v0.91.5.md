# v0.91.5 Release Plan

## Metadata
- Milestone: `v0.91.5`
- Version: `v0.91.5`
- Release date: 2026-06-17 pending ceremony publication
- Release manager: ADL maintainers
- Status: `release_ceremony_active`

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
- WP-19 handoff confirmation closed through `#3581` / PR `#3962`.
- The remaining path is WP-20 release ceremony publication and closeout.

## 0. Release-Tail Convergence

- [x] Multi-agent proof packet or explicit blocker complete.
- [x] Provider/model matrix and OpenRouter disposition complete.
- [x] Public prompt packet export/redaction evidence complete.
- [x] `.adl` archive/deletion review disposition complete.
- [x] Demo readiness proof map complete.
- [x] v0.92 activation test map complete.
- [x] AEE completion tranche routed through `#3534`, with v0.92 owner/proof
      expectations explicit.
- [x] `#3377` first-birthday readiness packet complete.
- [x] `#3929` GitHub transport-boundary cleanup merged/closed or explicitly
      rerouted as non-release-blocking.
- [x] Closed issue/card truth refreshed for the v0.91.5 wave through release-tail
      issue/PR evidence and ADL preflight checks; the retired shell-helper audit
      path is skipped with the ceremony script's explicit `--skip-sor-gate`
      flag.

## 1. Release Readiness

- [x] Milestone checklist complete for release-tail purposes, with remaining
      future work routed rather than hidden.
- [x] Release notes approved for ceremony publication.
- [x] Go/no-go decision recorded in the release-tail handoff and local WP-20 SOR
      for issue `#3578`; the generated local card path retains an older slug,
      so publication docs use the issue number and WP-20 truth instead.
- [x] v0.92 preflight says conditional go through v0.91.6/v0.91.7 bridge
      tranches, with activation surfaces routed.

## 2. Branch And Tag Preparation

- [x] Target branch confirmed: `main`.
- [x] Working tree clean.
- [x] Version strings validated.
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

- [x] v0.91.6 opening inputs announced internally.
- [x] v0.91.7 second-tranche rule announced internally.
- [x] 15-minute operator break scheduled after ceremony before v0.91.6 starts.
- [x] Roadmap/status docs updated.
- [x] Deferred items routed to later backlog.

## Exit Criteria

- No hidden implementation remains in ceremony.
- Tag and release are published and accessible.
- v0.91.6 can consume the bridge evidence without reconstructing intent from
  chat, and v0.91.7/v0.92 routing is explicit.
