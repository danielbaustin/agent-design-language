# v0.91.5 Release Notes

## Metadata
- Product: `Agent Design Language`
- Version: `v0.91.5`
- Release date: 2026-06-17 pending ceremony publication
- Tag: `v0.91.5`
- Status: `release_ceremony_ready`

## How To Use
- Keep statements implementation-accurate and test-validated.
- Prefer concise bullets over marketing language.
- Explicitly separate shipped behavior from "What's Next."

# `Agent Design Language` `v0.91.5` Release Notes

## Summary

v0.91.5 is the bridge milestone between v0.91.4 C-SDLC hardening and the
planned v0.91.6 / v0.91.7 bridge tranches before v0.92. It closes the
release-tail planning, review, remediation, tooling, and handoff work needed to
start v0.91.6 from tracked evidence instead of chat memory.

## Highlights

- Multi-agent C-SDLC and provider/model readiness are captured as tracked
  release evidence, with remaining bridge work explicitly routed forward.
- Public prompt records, `.adl` authoring/public export boundaries, and related
  feature-doc evidence are visible in the v0.91.5 bridge ledger.
- Sprint 4 review, external review, remediation, and final v0.92 routing are
  closed or dispositioned through the release tail.
- v0.91.6 is the immediate next milestone, v0.91.7 is the planned second
  pre-v0.92 bridge tranche, and v0.92 remains later birthday activation work.

## What's New In Detail

### Multi-Agent And Providers
- Multi-agent C-SDLC proof surfaces, provider/model-role matrix work, OpenRouter
  disposition, and local/remote model reliability evidence are retained as
  milestone proof inputs.
- The release does not claim that every future v0.92 activation surface is
  implemented. It preserves bridge accounting and routes remaining work to
  v0.91.6 and v0.91.7.

### Public Prompt Records
- Public prompt packet export, validation, redaction, reviewer-facing indexing,
  and the `.adl` editable-authoring boundary are preserved as release evidence.
- Public export remains a governed projection from `.adl` authoring surfaces,
  not a replacement for editable local prompt-card work.

### Tooling And Lifecycle Reliability
- The C-SDLC tooling tail moved GitHub operations toward the ADL/octocrab path
  and away from direct `gh` fallback.
- Release-tail PR publication, issue closeout, and ceremony preflight now run
  through repo-native ADL commands with token handling kept outside logs.
- The retired shell closed-issue SOR gate is recorded as a tooling limitation;
  ceremony uses the script's explicit skip flag and the ADL issue/PR evidence
  gathered during WP-18/WP-19/WP-20 instead.

### v0.92 Readiness
- v0.92 activation surfaces remain explicitly routed: AEE completion,
  Memory/ObsMem handoff, ACP/cognitive profiles, provider/model matrix,
  Observatory/Unity readiness, ACIP/provider communications, public prompt
  records, Memory Palace, Guilds, security/CAV, and CodeFriend v1 / portable
  adapter v2.
- v0.91.6 opens first, v0.91.7 is planned as the second bridge tranche, and
  v0.92 opens only after those bridge surfaces are complete, blocked, deferred,
  or explicitly routed.

## Upgrade Notes

- No operator upgrade steps are required for this documentation and planning
  bridge release.
- Existing ADL workflow users should continue to use `adl/tools/pr.sh` lifecycle
  commands and pass GitHub tokens only through approved secret sources.

## Known Limitations

- v0.91.5 is a bridge and release-tail truth milestone, not v0.92 activation.
- v0.91.6 and v0.91.7 are required before v0.92 opens.
- The release ceremony does not implement new runtime, provider, security, or
  product surfaces; it packages and publishes landed evidence and routing truth.
- Aptitude Atlas remains post-v0.95 productization; v0.95 may consume capability
  evidence but does not require Aptitude Atlas as MVP scope.

## Validation Notes

- Release ceremony preflight passed against the issue-branch docs with
  `adl/tools/release_ceremony.sh --version v0.91.5 --skip-sor-gate
  --allow-dirty --target-branch <bound-issue-branch>`.
- The clean-`main` mutation preflight must be re-run after this docs PR merges
  and before tag/release publication.
- WP-19 PR `#3962` was merged with successful `adl-ci` and `adl-coverage`
  checks, and `adl-slow-proof` skipped by lane policy.
- WP-20 preflight passed with `adl/tools/pr.sh doctor 3578 ... --mode
  preflight --json`.
- Runtime/tool validation belongs to the issues that changed runtime/tool
  behavior; this ceremony issue is documentation and release-publication
  focused.

## What's Next

- v0.91.6 opens after a 15-minute operator break and consumes the bridge feature
  docs, tooling proof-loop reliability, security/CAV, resilience, curiosity,
  Memory Palace, CodeFriend route, and AWS account setup planning.
- v0.91.7 follows as the second pre-v0.92 bridge tranche.
- v0.92 remains the later first-birthday activation milestone.
- v0.93.x carries CodeFriend alpha and governance-oriented work.

## Exit Criteria

- Notes reflect only shipped behavior.
- Known limitations and future work are explicitly separated.
- Final text is ready to paste into GitHub Release UI without further editing.
