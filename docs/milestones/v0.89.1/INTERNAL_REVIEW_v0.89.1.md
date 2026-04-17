# Internal Review - v0.89.1

## Metadata
- Milestone: `v0.89.1`
- Version: `v0.89.1`
- Canonical issue / WP: `#1937` / `WP-16`
- Date: 2026-04-16
- Scope: internal review of milestone truth, proof surfaces, implementation risk, workflow closeout, and reviewer readiness

## Executive Summary

Recommendation: proceed to `WP-17` third-party review after this `WP-16`
record lands, but do not treat `v0.89.1` as release-ready until accepted
findings are remediated or explicitly deferred by `WP-18`.

The implementation tranche is substantial. The adversarial runtime,
exploit/replay, continuous-verification, operational-skills, skill-composition,
provider-packaging, proof-entry, Hey Jude, manuscript, quality-gate, and
docs-review surfaces all map to concrete code, docs, scripts, or tests. The
review did not find evidence that the milestone is hollow or only process work.

The important risks are sharper than that: signing trust, provider credential
binding, UTF-8-safe provider error handling, retry classification, and closeout
automation truth. Those should be handled as real remediation inputs rather
than vague release-tail concerns.

## Review Method

This pass used three review lanes:
- multi-agent specialist review for code, security, and docs
- `repo-code-review` skill pass for a structured repo-wide risk review
- manual Codex review against milestone docs, proof scripts, closeout tooling,
  and selected Rust trust-boundary surfaces

The attempted tests-specialist subagent did not complete in time and was closed
rather than allowed to hold the review indefinitely. Test coverage was therefore
handled by manual targeted validation and skill-contract checks in this pass.

Local supporting artifacts were generated during the review session. They are
useful audit support, but this tracked document is the repo-visible `WP-16`
milestone review record.

## Findings

### F1. P1 - Runtime signature enforcement accepts self-signed embedded keys

Location: `adl/src/cli/run.rs:583`, `adl/src/signing.rs:56`,
`adl/src/signing.rs:248`

The `run` signature gate calls verification without an explicit trusted public
key. The default verification profile allows embedded keys, so a workflow can
embed an attacker-owned public key, sign itself, and pass integrity
verification. That proves internal consistency, not provenance or
authorization.

Recommended action: require an explicit trusted key source for non-dev
execution, or require a clearly named operator opt-in when accepting embedded
self-signed keys.

### F2. P1 - Provider error-body truncation can panic on UTF-8

Location: `adl/src/provider/http_family.rs:43`

Provider error rendering truncates response text with a byte slice. A multibyte
UTF-8 character at the 200-byte boundary can panic while formatting an upstream
HTTP error, converting a normal provider failure into a local crash.

Recommended action: make provider body truncation character-aware and add a
regression test with non-ASCII response text across the truncation boundary.

### F3. P2 - Provider credentials are not bound to trusted provider hosts

Location: `adl/src/provider/http_family/config.rs:39`,
`adl/src/provider/http_family.rs:270`, `adl/src/provider/http_family.rs:329`,
`adl/src/provider/http_family.rs:517`

Endpoint validation requires HTTPS for remote endpoints, but OpenAI,
Anthropic, and generic bearer-auth providers can still send tokens and prompt
payloads to arbitrary HTTPS hosts selected by repo-controlled configuration.

Recommended action: add a host allowlist or credential-to-host trust policy,
with a separate explicit mode for custom endpoints.

### F4. P2 - Deterministic setup failures are retried as if transient

Location: `adl/src/provider/mod.rs:176`, `adl/src/execute/runner.rs:311`,
`adl/src/execute/runner.rs:336`, `adl/src/execute/runner.rs:447`

The retry classifier treats untyped errors as retryable. Missing prompt
bindings, failed input materialization, unknown provider IDs, and provider
construction failures can therefore consume retry budget and produce traces
that imply a transient failure.

Recommended action: add non-retryable execution/setup error classes or classify
pre-provider setup failures before retry.

### F5. P2 - Closeout catch-up defaults to the newest local `.adl` directory

Location: `adl/tools/fix_git_main_sync_preserve_local_adl.sh:68`,
`adl/tools/fix_git_main_sync_preserve_local_adl.sh:97`

When no closeout version is explicitly configured, the main-sync helper chooses
the highest local `.adl` directory. That is brittle during overlapping
milestone planning: a newer planning lane such as `v0.90` can cause automatic
catch-up to skip the release lane that actually needs post-merge closeout.

Recommended action: handle through existing follow-up `#1992` by making
closeout version selection explicit, milestone-aware, or derived from the
merged issue context.

### F6. P2 - Closeout truth validation does not prove final STP/SIP truth

Location: `adl/src/cli/pr_cmd/lifecycle.rs:172`,
`adl/src/cli/pr_cmd/lifecycle.rs:267`,
`adl/tools/closeout_completed_issue_wave.sh:169`

The closeout path normalizes and validates SOR fields, then prunes. It does
not prove that STP and SIP have also been normalized from active execution
truth into final closed-issue truth. This matches the closeout gap observed
before `#1991` and is still the permanent-fix shape for `#1992`.

Recommended action: extend closeout to compose `stp-editor` and `sip-editor`,
or add first-class STP/SIP closure-state validation before pruning.

### F7. P2 - Release-tail status docs were one step behind WP-16

Location: `docs/milestones/v0.89.1/WP_ISSUE_WAVE_v0.89.1.yaml`,
`docs/milestones/v0.89.1/MILESTONE_CHECKLIST_v0.89.1.md`

The docs specialist found that the wave note and checklist still described the
state from the `WP-15` docs-review moment rather than the `WP-16` internal
review moment.

Disposition: remediated by this `WP-16` PR through this internal-review record
and aligned status language in the milestone package.

### F8. P3 - Large module hotspots remain residual maintainability debt

Location: `adl/src/cli/identity_cmd/tests.rs`,
`adl/src/cli/run_artifacts/runtime/trace_validation.rs`,
`adl/src/cli/tooling_cmd/tests.rs`, `adl/src/instrumentation.rs`

The refactor tranche improved several parent files, but large extracted child
modules remain. This is not a release blocker for `v0.89.1`, but it should stay
visible in the quality-gate watch list and next-milestone planning.

Recommended action: convert the hottest modules into targeted `v0.90`
maintainability work rather than expanding `v0.89.1`.

## Milestone Achievement

The milestone has delivered the core implementation and proof bands it claims:
- adversarial runtime and red/blue/purple role architecture
- exploit artifact and replay model
- continuous verification and self-attack contract surfaces
- operational skills, skill composition, and delegation/refusal coordination
- provider-extension packaging and demo proof entry points
- D7 reviewer-facing proof package
- D8 five-agent Hey Jude proof packet
- D9 arXiv manuscript workflow packet
- D10 quality-gate walkthrough surface
- WP-15 docs-review convergence surface

The remaining release-tail work is now narrower and clearer:
- `WP-17`: third-party review
- `WP-18`: accepted finding remediation or explicit deferral
- `WP-19`: next-milestone planning handoff
- `WP-20`: release ceremony

## Validation Performed

Passed during this review:
- `bash adl/tools/test_demo_v0891_wp13_demo_integration.sh`
- `bash adl/tools/test_demo_v0891_five_agent_hey_jude.sh`
- `bash adl/tools/test_demo_v0891_arxiv_manuscript_workflow.sh`
- `bash adl/tools/test_arxiv_paper_writer_skill_contracts.sh`
- `bash adl/tools/test_repo_code_review_skill_contracts.sh`
- `bash adl/tools/test_multi_agent_repo_review_skill_suite_contracts.sh`
- `bash adl/tools/test_workflow_conductor_skill_contracts.sh`
- `bash adl/tools/test_pr_closeout_skill_contracts.sh`
- `bash adl/tools/test_closeout_completed_issue_wave.sh`
- `bash adl/tools/check_release_notes_commands.sh`
- `bash -n adl/tools/demo_v0891_quality_gate.sh`
- `cargo fmt --manifest-path adl/Cargo.toml --all --check`
- `cargo test --manifest-path adl/Cargo.toml adversarial --quiet`
- `cargo test --manifest-path adl/Cargo.toml exploit --quiet`
- `cargo test --manifest-path adl/Cargo.toml operational_skills --quiet`

Not run in this pass:
- full `cargo test --manifest-path adl/Cargo.toml`
- full `cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings`
- full `bash adl/tools/demo_v0891_quality_gate.sh`
- live provider calls

Rationale: `WP-14` owns the full quality-gate walkthrough; this issue owns
internal review signal and targeted proof of the review surfaces.

## Residual Risk

No P0 issues were identified. The P1/P2 findings above should feed `WP-18`
unless the release manager explicitly defers some of them with rationale.

The tests-specialist lane did not complete, so this review should not be read
as a full independent test-audit replacement. It does include targeted
validation of the proof/demo scripts, skill contracts, and selected Rust
runtime areas.

## Final Assessment

`v0.89.1` is ready for external review, not release. The next correct move is
to run `WP-17`, then use `WP-18` to remediate or explicitly defer accepted
findings before next-milestone planning and release ceremony.

## Post-Internal-Review Remediation Status

This tracked review is no longer an open-ended finding list. The release tail
has now closed the internal-review remediation issues needed before
third-party review:

- F1 trusted run signature enforcement: remediated by `#1994`
- F2 UTF-8-safe provider error truncation: remediated by `#1995`
- F3 provider credentials bound to trusted endpoints: remediated by `#1996`
- F4 deterministic setup failures retried as transient: remediated by `#1997`
- F5/F6 closeout truth and version-selection gaps: remediated by `#1992`
- F7 release-tail status lag: remediated by tracked review and release-tail docs
- F8 large-module hotspots: intentionally deferred to v0.90 maintainability
  planning unless third-party review finds a concrete release-blocking defect

The third-party reviewer should verify these dispositions rather than treating
the internal review as unresolved release-tail ambiguity.
