# v0.91.4 Internal Review Findings Register

## Metadata

- Milestone: `v0.91.4`
- Work package: `WP-16`
- Issue: `#3366`
- Date: `2026-05-31`
- Status: `findings_recorded`
- Source plan: `V0914_INTERNAL_REVIEW_PLAN_2026-05-30.md`

## Snapshot

- Review branch: `codex/3366-v0-91-4-wp-16-internal-review`
- Review worktree: `.worktrees/adl-wp-3366`
- Base snapshot: `origin/main` at `c675c989`
- Open v0.91.4 issues at review start: `#3362`, `#3366`, `#3367`, `#3368`, `#3369`, `#3370`, `#3371`
- Open PRs at review start: none
- WP-15 dependency: issue `#3365` closed and PR `#3540` merged on `2026-05-31`
- Repo packet: `repo_packet_2026-05-31/`

## Findings

| ID | Severity | Area | Summary | Recommended route |
| --- | --- | --- | --- | --- |
| WP16-F001 | P1 | PVF/tests | PVF policy contract test exits before checking expected release-gate reports. | Fix before external review, likely WP-18 remediation. |
| WP16-F002 | P1 | release truth | Release-facing docs still overclaim C-SDLC completion while release gate remains blocked. | Fix before external review. |
| WP16-F003 | P2 | provider/model identity | Remote Ollama provider identity uses misleading `hosted_http` runtime surface. | Fix before release or route as v0.91.5 provider work if explicitly non-blocking. |
| WP16-F004 | P2 | CI/PVF | New PVF runner/policy tests are not wired into GitHub CI. | Fix or explicitly route before release. |
| WP16-F005 | P2 | docs/routing | Public prompt-record routing is split between v0.91.4 and v0.91.5. | Clarify before external review. |
| WP16-F006 | P2 | lifecycle truth | WP-15 docs/adoption review packet still says `draft_for_pr_review` after merge. | Fix before external review. |
| WP16-F007 | P2 | redaction/evidence | Redaction audit does not cover all WP-16 evidence inputs and misses durable temp-path evidence. | Fix or caveat before external review. |
| WP16-F008 | P2 | publication safety | C-SDLC paper handoff docs expose local absolute checkout paths. | Fix or archive/localize before external review. |
| WP16-F009 | P2 | benchmark evidence | WildClawBench replayability depends on local-only state that is not fully pinned. | Route as sidecar evidence hardening, not core release blocker. |
| WP16-F010 | P2 | provider/security | Gemini adapter puts the credential into the request URL and result-path diagnostics may not redact `key=`. | Fix before hosted Gemini provider evidence is treated as publication-safe. |
| WP16-F011 | P3 | planning docs | Next milestone handoff contains scaffold-era contradiction about v0.91.5 selection. | Fix in WP-19/WP-20. |
| WP16-F012 | P3 | browser docs | Browser automation docs preserve machine-specific diagnostic paths. | Generalize or mark as local diagnostics. |
| WP16-F013 | P3 | sidecar security | CodeFriend packet exposes extra infrastructure fingerprinting details. | Decide before public/external handoff. |

## Detailed Findings

### WP16-F001 - P1 - PVF policy contract test exits before checking expected release-gate reports

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

Impact: a policy test intended to prove PVF release-gate semantics cannot reach
its assertions. This is a pre-external-review blocker because PVF truth is one of
v0.91.4's important claims.

### WP16-F002 - P1 - Release-facing docs still overclaim C-SDLC completion while release gate remains blocked

Evidence:

- `docs/milestones/v0.91.4/README.md:49`
- `docs/milestones/v0.91.4/RELEASE_NOTES_v0.91.4.md:20`
- `docs/milestones/v0.91.4/RELEASE_NOTES_v0.91.4.md:25`
- `docs/milestones/v0.91.4/QUALITY_GATE_v0.91.4.md:5`
- `docs/milestones/v0.91.4/QUALITY_GATE_v0.91.4.md:116`
- `docs/milestones/v0.91.4/MILESTONE_CHECKLIST_v0.91.4.md:58`
- `docs/milestones/v0.91.4/MILESTONE_CHECKLIST_v0.91.4.md:78`

Impact: reviewers can read release-facing docs as claiming shipped/default C-SDLC
completion before the release tail, review remediation, signed trace/release
evidence, and ceremony have completed.

### WP16-F003 - P2 - Remote Ollama provider identity uses misleading `hosted_http` runtime surface

Evidence:

- `adl/src/provider_substrate.rs:171`
- `adl/src/provider_substrate.rs:191`
- `adl/src/provider_substrate.rs:415`
- `adl/src/provider_substrate.rs:577`
- `adl/src/provider_communication.rs:379`

Impact: benchmark/provider evidence may misclassify remote Ollama as a hosted
HTTP surface instead of a local/open-model HTTP runtime surface.

### WP16-F004 - P2 - New PVF runner/policy tests are not wired into GitHub CI

Evidence:

- `.github/workflows/ci.yaml:112`
- `.github/workflows/ci.yaml:250`
- `adl/tools/test_pvf_ci_release_policy.sh:1`
- `adl/tools/run_pvf_validation_lane.sh:1`

Impact: a core v0.91.4 validation-policy surface can drift without normal PR CI
catching it.

### WP16-F005 - P2 - Public prompt-record routing is split between v0.91.4 and v0.91.5

Evidence:

- `docs/milestones/v0.91.4/README.md:208`
- `docs/milestones/v0.91.4/QUALITY_GATE_v0.91.4.md:19`
- `docs/milestones/v0.91.4/RELEASE_PLAN_v0.91.4.md:28`
- `docs/milestones/v0.91.5/features/PUBLIC_PROMPT_RECORDS_v0.91.5.md:67`

Impact: reviewers may mistake prompt-record export hardening for completed
v0.91.4 release scope rather than v0.91.5 bridge work.

### WP16-F006 - P2 - WP-15 docs/adoption review packet still says `draft_for_pr_review` after merge

Evidence:

- `docs/milestones/v0.91.4/review/docs_adoption/WP15_DOCS_ADOPTION_REVIEW_2026-05-31.md:10`
- GitHub issue `#3365`: closed at `2026-05-31T14:17:40Z`
- GitHub PR `#3540`: merged at `2026-05-31T14:17:39Z`

Impact: the immediate WP-16 input packet carries stale lifecycle truth.

### WP16-F007 - P2 - Redaction audit does not cover all WP-16 evidence inputs and misses durable temp-path evidence

Evidence:

- `docs/milestones/v0.91.4/review/quality_gate/V0914_REDACTION_AUDIT_2026-05-30.md:13`
- `docs/milestones/v0.91.4/review/quality_gate/V0914_REDACTION_AUDIT_2026-05-30.md:34`
- `docs/milestones/v0.91.4/review/internal_review/V0914_INTERNAL_REVIEW_PLAN_2026-05-30.md:218`
- `docs/milestones/v0.91.4/review/internal_review/README.md:51`
- `docs/milestones/v0.91.4/WILDCLAW_TASK_VALIDITY_AUDIT_2026-05-26.md:24`
- `docs/milestones/v0.91.4/WILDCLAW_SAFETY_ALIGNMENT_RESULTS_2026-05-27.md:52`

Impact: the redaction audit can be overread as covering all WP-16 evidence even
though sidecar docs still include local scratch/stable host path details.

### WP16-F008 - P2 - C-SDLC paper handoff docs expose local absolute checkout paths

Evidence:

- `docs/cognitive-sdlc/c_sdlc_private_paper_repo_plan.md:29`
- `docs/cognitive-sdlc/c_sdlc_private_paper_repo_plan.md:42`
- `docs/cognitive-sdlc/c_sdlc_private_paper_repo_plan.md:279`
- `docs/cognitive-sdlc/c_sdlc_paper_handoff.md:18`

Impact: public C-SDLC docs carry local machine paths despite the milestone's
public review-record hygiene policy.

### WP16-F009 - P2 - WildClawBench replayability depends on local-only state that is not fully pinned

Evidence:

- `docs/milestones/v0.91.4/WILDCLAW_SAFETY_ALIGNMENT_RESULTS_2026-05-27.md:52`
- `docs/milestones/v0.91.4/WILDCLAW_SAFETY_ALIGNMENT_RESULTS_2026-05-27.md:80`
- `docs/milestones/v0.91.4/WILDCLAW_SAFETY_ALIGNMENT_RESULTS_2026-05-27.md:91`
- `docs/milestones/v0.91.4/WILDCLAW_SAFETY_ALIGNMENT_RESULTS_2026-05-27.md:139`
- `docs/milestones/v0.91.4/WILDCLAW_SAFETY_ALIGNMENT_RESULTS_2026-05-27.md:297`

Impact: this remains useful internal sidecar evidence, but it should not be
presented as independently replayable external benchmark proof.

### WP16-F010 - P2 - Gemini adapter puts the credential into the request URL and result-path diagnostics may not redact `key=`

Evidence:

- `adl/src/provider_adapter.rs:310`
- `adl/src/provider_adapter.rs:426`
- `adl/src/provider_adapter.rs:445`
- `adl/src/provider_communication.rs:412`
- `adl/src/provider_adapter_cli.rs:34`

Impact: Gemini transport errors may persist query-string credential material in result artifacts if reqwest diagnostics include the URL. Run logs are narrower, but the CLI output JSON carries `failure.message`.

### WP16-F011 - P3 - Next milestone handoff contains scaffold-era contradiction about v0.91.5 selection

Evidence:

- `docs/milestones/v0.91.4/NEXT_MILESTONE_HANDOFF_v0.91.4.md:3`
- `docs/milestones/v0.91.4/NEXT_MILESTONE_HANDOFF_v0.91.4.md:31`
- `docs/milestones/v0.91.4/NEXT_MILESTONE_HANDOFF_v0.91.4.md:97`

Impact: this should be cleaned during WP-19/WP-20 so the next milestone handoff
is not internally contradictory.

### WP16-F012 - P3 - Browser automation docs preserve machine-specific diagnostic paths

Evidence:

- `docs/milestones/v0.91.4/review/browser_automation/BROWSER_AUTOMATION_RUNBOOK_v0.91.4.md:172`
- `docs/milestones/v0.91.4/review/browser_automation/BROWSER_AUTOMATION_RUNBOOK_v0.91.4.md:177`
- `docs/milestones/v0.91.4/review/browser_automation/BROWSER_AUTOMATION_RUNBOOK_v0.91.4.md:179`
- `docs/milestones/v0.91.4/review/browser_automation/BROWSER_AUTOMATION_RUNBOOK_v0.91.4.md:181`

Impact: low-risk but untidy for external/public review packets.

### WP16-F013 - P3 - CodeFriend packet exposes extra infrastructure fingerprinting details

Evidence:

- `docs/milestones/v0.91.4/CODEFRIEND_AWS_PUBLICATION.md:21`
- `docs/milestones/v0.91.4/CODEFRIEND_AWS_PUBLICATION.md:28`
- `docs/milestones/v0.91.4/CODEFRIEND_AWS_PUBLICATION.md:32`
- `docs/milestones/v0.91.4/CODEFRIEND_RAPID_WEBSITE_LAUNCH_DEMO.md:123`
- `docs/milestones/v0.91.4/CODEFRIEND_RAPID_WEBSITE_LAUNCH_DEMO.md:128`

Impact: not a secret leak, but public handoff should decide whether the
infrastructure details are necessary.

## What Passed

- No open PRs were hidden from the packet at review start.
- The only open v0.91.4 issues are the expected Sprint 4 release-tail issues and
  umbrella.
- WP-15 was merged before WP-16 review execution began.
- Core sidecar boundaries are mostly clear: CodeFriend, WildClawBench, Unity,
  and multi-agent completion are not treated as required C-SDLC release proof.
- The tracked C-SDLC evidence namespace exists and contains the WP-06 evidence
  bundle plus signed-trace fixture references.
- JSON parsing passed for the three PVF manifests sampled during review.

## Validation Performed

- Built repo packet with `repo-packet-builder` helper into `repo_packet_2026-05-31/`.
- Captured open issue and PR snapshot with GitHub CLI.
- Ran targeted `rg` scans for stale status, host paths, temp paths, secrets, and
  release-tail routing terms.
- Ran focused PVF test reproduction: `bash adl/tools/test_pvf_ci_release_policy.sh; echo exit:$?`.
- Parsed selected PVF JSON manifests with `python3`.
- Performed an additional code-heavy pass over provider adapter, provider communication, model identity, PVF runner, PR-fast lane selection, and multi-agent planner/validator surfaces.

## Validation Not Performed

- Full Rust test suite was not run.
- Live provider calls were not run.
- Browser demos were not rerun.
- External repositories and ignored `.adl/` local control artifacts were not
  exhaustively reviewed.
