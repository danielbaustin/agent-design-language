# v0.91.4 Internal Review Specialist Findings

## Metadata

- Milestone: `v0.91.4`
- Work package: `WP-16`
- Issue: `#3366`
- Date: `2026-05-31`
- Review type: internal specialist review notes
- Status: `captured_for_synthesis`

## Scope

This document preserves the bounded specialist-lane findings used by the WP-16
internal review synthesis. It is not a remediation plan and does not approve
release readiness.

## Code / PVF / Provider Lane

### P1 - PVF policy contract test exits before checking expected release-gate reports

`adl/tools/test_pvf_ci_release_policy.sh` enables `set -e`, then invokes the
PVF runner in PR mode. The runner exits non-zero when aggregate status is
`release_gate_required`, but the test expects to inspect that report later. The
script therefore stops before its own assertions.

Evidence:

- `adl/tools/test_pvf_ci_release_policy.sh:2`
- `adl/tools/test_pvf_ci_release_policy.sh:18`
- `adl/tools/test_pvf_ci_release_policy.sh:31`
- `adl/tools/run_pvf_validation_lane.sh:415`

Focused reproduction:

```bash
bash adl/tools/test_pvf_ci_release_policy.sh; echo exit:$?
```

Observed result:

```text
exit:1
```

### P2 - Remote Ollama provider identity uses misleading `hosted_http` runtime surface

Remote Ollama with `base_url` is correctly classified as HTTP transport, but the
runtime surface label for every HTTP transport is `hosted_http`. That makes a
remote Ollama runtime look like a hosted frontier-provider surface in model
identity, while the provider communication helper uses the clearer
`ollama_http` surface.

Evidence:

- `adl/src/provider_substrate.rs:171`
- `adl/src/provider_substrate.rs:191`
- `adl/src/provider_substrate.rs:415`
- `adl/src/provider_substrate.rs:577`
- `adl/src/provider_communication.rs:379`

### P2 - New PVF runner/policy tests are not wired into GitHub CI

The v0.91.4 PVF runner and policy test exist, but the workflow still appears to
invoke older proof-validation paths rather than the new policy test. This leaves
the new PVF release-policy surface outside normal PR enforcement.

Evidence:

- `.github/workflows/ci.yaml:112`
- `.github/workflows/ci.yaml:250`
- `adl/tools/test_pvf_ci_release_policy.sh:1`
- `adl/tools/run_pvf_validation_lane.sh:1`

## Documentation / Release-Truth Lane

### P1 - Release-facing docs still overclaim C-SDLC completion while release gate remains blocked

The milestone README and release notes use completion/default-path wording while
the quality gate and checklist still show the release tail is blocked or
unchecked. The release notes say they are draft, but the summary/highlight
language can still be read as shipped truth.

Evidence:

- `docs/milestones/v0.91.4/README.md:49`
- `docs/milestones/v0.91.4/RELEASE_NOTES_v0.91.4.md:20`
- `docs/milestones/v0.91.4/RELEASE_NOTES_v0.91.4.md:25`
- `docs/milestones/v0.91.4/QUALITY_GATE_v0.91.4.md:5`
- `docs/milestones/v0.91.4/QUALITY_GATE_v0.91.4.md:116`
- `docs/milestones/v0.91.4/MILESTONE_CHECKLIST_v0.91.4.md:58`
- `docs/milestones/v0.91.4/MILESTONE_CHECKLIST_v0.91.4.md:78`

### P2 - Public prompt-record routing is split between v0.91.4 and v0.91.5

Some v0.91.4 docs say public prompt-record packets are exported into tracked
evidence, while release-truth docs and v0.91.5 planning route public prompt
record implementation/export hardening to v0.91.5. The safest current truth is
that v0.91.4 defines the transition/contract and v0.91.5 owns export hardening.

Evidence:

- `docs/milestones/v0.91.4/README.md:208`
- `docs/milestones/v0.91.4/QUALITY_GATE_v0.91.4.md:19`
- `docs/milestones/v0.91.4/RELEASE_PLAN_v0.91.4.md:28`
- `docs/milestones/v0.91.5/features/PUBLIC_PROMPT_RECORDS_v0.91.5.md:67`

### P2 - WP-15 docs/adoption review packet still says `draft_for_pr_review` after merge

The WP-15 review packet remains in draft status even though issue `#3365` is
closed and PR `#3540` is merged. This is a small but concrete lifecycle-truth
problem in the immediate input to WP-16.

Evidence:

- `docs/milestones/v0.91.4/review/docs_adoption/WP15_DOCS_ADOPTION_REVIEW_2026-05-31.md:10`
- GitHub issue `#3365`: closed at `2026-05-31T14:17:40Z`
- GitHub PR `#3540`: merged at `2026-05-31T14:17:39Z`

### P3 - Next milestone handoff contains scaffold-era contradiction about v0.91.5 selection

The handoff says it is a WP-19 scaffold and does not claim the next milestone
has already been selected, but it also says the next milestone is now planned as
v0.91.5. This is likely acceptable before WP-19, but it should be cleaned before
WP-20 review.

Evidence:

- `docs/milestones/v0.91.4/NEXT_MILESTONE_HANDOFF_v0.91.4.md:3`
- `docs/milestones/v0.91.4/NEXT_MILESTONE_HANDOFF_v0.91.4.md:31`
- `docs/milestones/v0.91.4/NEXT_MILESTONE_HANDOFF_v0.91.4.md:97`

## Security / Evidence / Publication-Safety Lane

### P2 - Redaction audit does not cover all WP-16 evidence inputs and misses durable temp-path evidence

The redaction audit limits its reviewed surfaces, then reports no local host-path
leak in those surfaces. WP-16 explicitly includes CodeFriend and WildClawBench
sidecar evidence, and several WildClawBench docs preserve `/private/tmp` or
`$HOME/temp` execution paths as durable evidence.

Evidence:

- `docs/milestones/v0.91.4/review/quality_gate/V0914_REDACTION_AUDIT_2026-05-30.md:13`
- `docs/milestones/v0.91.4/review/quality_gate/V0914_REDACTION_AUDIT_2026-05-30.md:34`
- `docs/milestones/v0.91.4/review/internal_review/V0914_INTERNAL_REVIEW_PLAN_2026-05-30.md:218`
- `docs/milestones/v0.91.4/review/internal_review/README.md:51`
- `docs/milestones/v0.91.4/WILDCLAW_TASK_VALIDITY_AUDIT_2026-05-26.md:24`
- `docs/milestones/v0.91.4/WILDCLAW_SAFETY_ALIGNMENT_RESULTS_2026-05-27.md:52`

### P2 - C-SDLC paper handoff docs expose local absolute checkout paths

The public C-SDLC docs include `/Users/daniel/...` local checkout paths for the
private paper repository and related local workflow references. These are useful
operator notes, but they conflict with the public-review record policy for
tracked reviewer-facing records.

Evidence:

- `docs/cognitive-sdlc/c_sdlc_private_paper_repo_plan.md:29`
- `docs/cognitive-sdlc/c_sdlc_private_paper_repo_plan.md:42`
- `docs/cognitive-sdlc/c_sdlc_private_paper_repo_plan.md:279`
- `docs/cognitive-sdlc/c_sdlc_paper_handoff.md:18`

### P2 - WildClawBench replayability depends on local-only state that is not fully pinned

The WildClawBench sidecar evidence is bounded and honest, but replay still
depends on local benchmark placement, downloaded workspace payloads, a loaded
Docker image, helper tools, and local credentials. It is acceptable as internal
sidecar evidence, but not independently replayable benchmark proof.

Evidence:

- `docs/milestones/v0.91.4/WILDCLAW_SAFETY_ALIGNMENT_RESULTS_2026-05-27.md:52`
- `docs/milestones/v0.91.4/WILDCLAW_SAFETY_ALIGNMENT_RESULTS_2026-05-27.md:80`
- `docs/milestones/v0.91.4/WILDCLAW_SAFETY_ALIGNMENT_RESULTS_2026-05-27.md:91`
- `docs/milestones/v0.91.4/WILDCLAW_SAFETY_ALIGNMENT_RESULTS_2026-05-27.md:119`
- `docs/milestones/v0.91.4/WILDCLAW_SAFETY_ALIGNMENT_RESULTS_2026-05-27.md:139`
- `docs/milestones/v0.91.4/WILDCLAW_SAFETY_ALIGNMENT_RESULTS_2026-05-27.md:297`

### P3 - Browser automation docs preserve machine-specific diagnostic paths

The browser automation runbook records machine-specific browser paths. These are
useful diagnostics but should be generalized or clearly marked local diagnostic
examples before public/external packet use.

Evidence:

- `docs/milestones/v0.91.4/review/browser_automation/BROWSER_AUTOMATION_RUNBOOK_v0.91.4.md:172`
- `docs/milestones/v0.91.4/review/browser_automation/BROWSER_AUTOMATION_RUNBOOK_v0.91.4.md:177`
- `docs/milestones/v0.91.4/review/browser_automation/BROWSER_AUTOMATION_RUNBOOK_v0.91.4.md:179`
- `docs/milestones/v0.91.4/review/browser_automation/BROWSER_AUTOMATION_RUNBOOK_v0.91.4.md:181`

### P3 - CodeFriend packet exposes extra infrastructure fingerprinting details

The CodeFriend sidecar docs list CloudFront/AWS implementation details. No
secret was found, but external/public handoff should decide whether the
distribution hostname and infrastructure detail are necessary.

Evidence:

- `docs/milestones/v0.91.4/CODEFRIEND_AWS_PUBLICATION.md:21`
- `docs/milestones/v0.91.4/CODEFRIEND_AWS_PUBLICATION.md:28`
- `docs/milestones/v0.91.4/CODEFRIEND_AWS_PUBLICATION.md:32`
- `docs/milestones/v0.91.4/CODEFRIEND_RAPID_WEBSITE_LAUNCH_DEMO.md:123`
- `docs/milestones/v0.91.4/CODEFRIEND_RAPID_WEBSITE_LAUNCH_DEMO.md:128`

## Coverage Notes

Specialist review lanes were read-only. They covered:

- code/PVF/provider/runtime slices under `adl/` and `.github/`
- v0.91.4 and v0.91.5 planning/release-tail docs
- `docs/cognitive-sdlc/`
- `docs/tooling/card-lifecycle.md`
- `docs/tooling/structured-prompt-contracts.md`
- v0.91.4 review/proof/demo/sidecar packets

The review did not exhaustively audit every source file, ignored `.adl/` local
control artifacts, external repos, untracked provider logs, or binary artifacts.

## Additional Code Review Pass

### P2 - Gemini adapter puts the credential into the request URL and result-path diagnostics may not redact `key=`

The Gemini adapter builds the API key into the query string, then maps reqwest
transport errors through `error.to_string()`. The provider message sanitizer
redacts several secret markers, but not `key=` or query-string credential shapes.
Run logs only include failure kind/status, but the CLI writes the full
`ProviderInvocationResultV1` to disk, including `failure.message`. A Gemini
transport error that includes the request URL could therefore persist a query key
in the result artifact.

Evidence:

- `adl/src/provider_adapter.rs:310`
- `adl/src/provider_adapter.rs:426`
- `adl/src/provider_adapter.rs:445`
- `adl/src/provider_communication.rs:412`
- `adl/src/provider_adapter_cli.rs:34`

Recommended route: change Gemini auth handling or extend provider diagnostic
redaction to cover URL query credentials such as `key=` / `?key=` before external
provider evidence is treated as publication-safe.
