# v0.91.4 Internal Review Synthesis

## Metadata

- Milestone: `v0.91.4`
- Work package: `WP-16`
- Issue: `#3366`
- Date: `2026-05-31`
- Status: `ready_for_remediation_routing`

## Overall Assessment

`v0.91.4` is structurally close, but not ready for external review until the P1
findings are fixed or explicitly routed. The review did not find a catastrophic
release failure, secret leak, or hidden open PR wave. It did find two material
pre-external-review blockers:

1. PVF release-policy proof is weakened because its dedicated test exits before
   assertions.
2. Release-facing docs still use completion/default-path language while the
   quality gate and release tail are visibly blocked.

The rest of the findings are mostly evidence hygiene, routing clarity, provider
identity precision, and publication-safety cleanup.

## Specialist Coverage Matrix

| Lane | Status | Notes |
| --- | --- | --- |
| Workflow / issue truth | covered | Snapshot confirmed expected v0.91.4 release-tail issues remain open and no open PRs are hidden. |
| Code / PVF / provider | covered | Found PVF test blocker and provider identity/runtime-surface drift. |
| Test / CI | covered | Found PVF policy test not wired into CI. |
| Docs / release truth | covered | Found release-overclaim and prompt-record routing drift. |
| Evidence / demos | covered | Found sidecar replayability/publication caveats, but no core sidecar-boundary collapse. |
| Security / redaction | covered | No raw keys found in sampled tracked surfaces; found host/temp-path and infrastructure-detail hygiene issues. |
| Dependency / CI | partially covered | CI workflow routing was sampled for PVF surfaces; no broad supply-chain review was run. |

## Findings By Release Route

### Must fix or explicitly route before WP-17 external review

- `WP16-F001` P1: PVF policy contract test exits before assertions.
- `WP16-F002` P1: release-facing docs overclaim completion while gate is blocked.
- `WP16-F005` P2: public prompt-record routing ambiguity can mislead review scope.
- `WP16-F006` P2: WP-15 packet stale `draft_for_pr_review` state.
- `WP16-F007` P2: redaction audit scope does not cover all WP-16 evidence inputs.
- `WP16-F008` P2: public C-SDLC paper handoff docs expose local absolute paths.

### Should fix before release or route with explicit owner

- `WP16-F003` P2: remote Ollama model identity runtime surface is misleading.
- `WP16-F004` P2: PVF runner/policy tests are not wired into CI.
- `WP16-F009` P2: WildClawBench replayability needs stronger environment pinning
  before any external benchmark-proof claim.
- `WP16-F010` P2: Gemini adapter credential-in-URL diagnostics need redaction/auth hardening before publication-safe hosted evidence claims.

### Can route to WP-19/WP-20 or low-risk cleanup

- `WP16-F011` P3: next milestone handoff scaffold contradiction.
- `WP16-F012` P3: browser automation diagnostics carry machine-specific paths.
- `WP16-F013` P3: CodeFriend sidecar exposes extra infrastructure fingerprinting
  details.

## Non-Claims Preserved

This review does not claim:

- v0.91.4 is release-ready
- external review is complete
- remediation is complete
- WildClawBench proves ADL runtime superiority
- CodeFriend product publication proves C-SDLC core completion
- live multi-agent workcell completion is required for v0.91.4
- Unity/demo-completion work is required for v0.91.4

## Recommended Remediation Shape

Use `#3368` as the umbrella for review findings remediation. Suggested issue
clusters:

1. `[v0.91.4][quality] Fix PVF release-policy test and CI wiring`
   - covers `WP16-F001` and `WP16-F004`
2. `[v0.91.4][docs] Normalize release-tail completion and prompt-record routing truth`
   - covers `WP16-F002`, `WP16-F005`, `WP16-F006`, and `WP16-F010`
3. `[v0.91.4][review] Harden redaction and publication-safety evidence before external review`
   - covers `WP16-F007`, `WP16-F008`, `WP16-F011`, and `WP16-F012`
4. `[v0.91.4 or v0.91.5][provider] Correct provider identity and hosted diagnostic redaction`
   - covers `WP16-F003` and `WP16-F010`; fix before any external hosted-provider evidence packet relies on Gemini diagnostics
5. `[v0.91.5][benchmark] Pin WildClawBench replay environment before public benchmark claims`
   - covers `WP16-F009`

## Residual Risk

The review was broad but still bounded. The largest residual risks are:

- hidden drift in surfaces not sampled by specialist lanes
- ignored `.adl/` local control artifacts not represented in tracked review docs
- sidecar/external repo truth not fully reproducible from ADL alone
- full release-gate validation still pending later Sprint 4 stages

## Exit Recommendation For WP-16

WP-16 can move toward publication after this packet is reviewed and the P1/P2
routing is accepted. WP-17 should not begin as an external review until the two
P1 findings and the WP-15 stale status finding are fixed or explicitly routed in
tracked remediation state.
