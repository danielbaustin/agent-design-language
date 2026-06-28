# v0.91.6 WP-18 Next Milestone Review

## Metadata

- Issue: `#3983`
- Work package: `WP-18`
- Review date: `2026-06-28`
- Review target: v0.91.7 handoff and v0.92 activation-blocker planning truth
- Review status: `pass_with_release_ceremony_followup`

## Verdict

WP-18 finds the v0.91.7 handoff and next-milestone planning package ready for
the v0.91.6 release ceremony to proceed, provided WP-19 refreshes the final
v0.91.6 release notes, checklist, release plan, and closeout truth before any
release publication claim.

The v0.91.7 planning package is not claiming v0.92 activation readiness. It
keeps v0.92 blocked until bridge surfaces are complete, blocked, deferred, or
routed, and it points ceremony readers to the source-capture ledger for the
full carry-forward route set.

## Findings

### P2: v0.91.6 release-publication docs still need ceremony-time refresh

The next-milestone handoff is current, but some v0.91.6 release-publication
surfaces still intentionally lag the final release-tail state and must be
refreshed in WP-19 before ceremony closeout is published.

Evidence:

- `docs/milestones/v0.91.6/RELEASE_NOTES_v0.91.6.md` still says WP-16
  remediation/final preflight remains in progress.
- `docs/milestones/v0.91.6/RELEASE_PLAN_v0.91.6.md` and
  `docs/milestones/v0.91.6/MILESTONE_CHECKLIST_v0.91.6.md` still retain
  stale WP-15/WP-16 release-tail checklist text.
- `docs/milestones/v0.91.6/WBS_v0.91.6.md` still describes `#3980` as open
  even though WP-15 is closed.

Disposition: `route_to_WP19_release_ceremony`. This is not a WP-18 blocker
because WP-18 reviews the next-milestone handoff. It is a WP-19 release
ceremony requirement.

## Checks Passed

- `docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml` parses as YAML.
- `docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml` parses as YAML.
- `docs/milestones/v0.91.7/V0916_TO_V0917_HANDOFF_ADDENDUM_3982.md` records:
  - failed-but-closed WP-15 external-review truth;
  - closed WP-16 remediation/final-preflight truth;
  - closed `#4620` and `#4621`;
  - open v0.91.7 route `#4622`;
  - explicit non-claims for v0.91.6 completion, v0.91.7 execution approval,
    EC2 Spot proof, runtime Soak #2, and v0.92 activation readiness.
- `docs/milestones/v0.91.7/WBS_v0.91.7.md` is a v0.91.7 WBS rather than a
  duplicated v0.91.6 issue-state ledger.
- `docs/milestones/v0.91.7/PLANNING_SOURCE_CAPTURE_v0.91.7.md` remains the
  authority for the required carry-forward route set and keeps `#4622` visible
  as v0.91.7 tooling work.
- `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md` keeps every
  named activation surface routed rather than complete and explicitly says
  v0.92 activation remains blocked.
- `docs/planning/ADL_FEATURE_LIST.md` was refreshed in this issue so the
  roadmap no longer says v0.91.6 is merely after WP-12 with internal review
  next.

## Carry-Forward And Live Issue Truth

Repo-tracked carry-forward truth lives in
`docs/milestones/v0.91.7/PLANNING_SOURCE_CAPTURE_v0.91.7.md`, not in this
review packet. That ledger names the broader v0.91.7 route set, including
ADR/tooling remediation, session-ledger/lifecycle, goal snapshot, lifecycle
shepherd, C-SDLC control-plane, build-throughput, runtime, scheduler, security,
protocol, demo, and conceptual bridge inputs.

This WP-18 review relies on that ledger rather than duplicating every carried
row here.

The concrete live release-tail / handoff issues observed during WP-18 were:

- `#4622`: v0.91.7 repo-native PR inventory route.
- `#4604`: v0.91.6 closeout-tail umbrella, to remain open until WP-19 closes.
- `#3984`: v0.91.6 WP-19 release ceremony, next required release-tail issue.

No other open `version:v0.91.6` release-tail child issue was observed besides
`#3983`, `#3984`, and umbrella `#4604` at the start of WP-18.

## Go / No-Go Recommendation

Recommendation: `go_to_WP19_release_ceremony`.

Conditions for WP-19:

- Refresh release-publication docs from current issue truth.
- Preserve the external-review failure as release truth rather than rewriting
  it into a clean pass.
- Keep v0.92 activation blocked.
- Close `#4604` only after WP-19 records child issue status and residual
  v0.91.7 routes.

## Validation

Commands run:

```bash
ruby -e 'require "yaml"; %w[docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml].each{|p| YAML.load_file(p); puts "yaml ok #{p}"}'
rg -n "WP-16.*open|WP-16.*remains|WP-15.*open|#3980.*open|#3981.*open|internal review `#4582` is next|after WP-12|release-tail execution after WP-12|WP-14A.*next|#3982.*open|#3983.*open|#3984.*open|v0\\.91\\.6 is complete|v0\\.91\\.6.*complete|v0\\.92 activation readiness|activation readiness" docs/milestones/v0.91.7 docs/milestones/v0.92 docs/planning/ADL_FEATURE_LIST.md docs/milestones/v0.91.6/RELEASE_NOTES_v0.91.6.md docs/milestones/v0.91.6/RELEASE_PLAN_v0.91.6.md docs/milestones/v0.91.6/MILESTONE_CHECKLIST_v0.91.6.md docs/milestones/v0.91.6/WBS_v0.91.6.md docs/milestones/v0.91.6/README.md
```

Result:

- YAML parse checks passed.
- Stale-state scan found no WP-18-blocking v0.91.7/v0.92 activation overclaim.
- Stale v0.91.6 release-publication text remains and is routed to WP-19.

## Non-Claims

- This review does not perform the release ceremony.
- This review does not approve v0.92 execution.
- This review does not implement v0.91.7 work.
- This review does not claim EC2 Spot builds, runtime Soak #2, or v0.92
  activation proof has landed.
