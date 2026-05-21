# Findings Register

Findings are ordered by severity. No P0 findings were found.

## P1 Findings

### P1-1: Governed UTS+ACC benchmark pass/fail ignores expected task arguments

Role: code, tests

Evidence:

- `adl/src/uts_acc_multi_model_benchmark.rs:901`
- `adl/src/uts_acc_multi_model_benchmark.rs:903`
- `adl/src/uts_acc_multi_model_benchmark.rs:904`
- `adl/tools/benchmark/uts_33_task_panel.json:17`
- `adl/tools/uts_benchmark_runner.py:440`
- `adl/tools/uts_benchmark_runner.py:469`

The Python regular and UTS-only lanes check expected arguments, but the Rust governed UTS+ACC lane can mark a case valid when the tool name and humility boundary match even if the task arguments are wrong.

Impact: benchmark results can overstate model conformance.

Route: WP-22 benchmark harness remediation.

### P1-2: Committed hosted-provider key-file map records operator-local credential paths

Role: security, dependency/tooling

Evidence:

- `adl/tools/benchmark/hosted_provider_key_files.json:4`
- `adl/tools/benchmark/hosted_provider_key_files.json:5`
- `adl/tools/benchmark/hosted_provider_key_files.json:6`
- `adl/tools/uts_benchmark_runner.py:39`
- `adl/tools/uts_benchmark_runner.py:207`

The canonical hosted benchmark path defaults to a committed key-file map containing operator-local absolute paths. This is not a credential value leak, but it is a durable privacy/portability leak and makes hosted benchmark setup user-specific.

Impact: third-party reviewers cannot use the default config cleanly, and local credential storage conventions are exposed.

Route: immediate WP-22 security/tooling remediation.

### P1-3: Canonical benchmark profiles reference models not guaranteed by the canonical model panel

Role: architecture/methodology

Evidence:

- `adl/tools/benchmark/full_core_profile.json:16`
- `adl/tools/benchmark/full_core_profile.json:18`
- `adl/tools/benchmark/remote_open_core_profile.json:12`
- `adl/tools/benchmark/remote_open_core_profile.json:14`
- `adl/tools/uts_benchmark_runner.py:115`
- `adl/tools/uts_benchmark_runner.py:118`

At least two canonical profiles can fail before evaluation if selected model IDs are absent from the canonical model panel.

Impact: full/remote benchmark repeatability is brittle.

Route: WP-22 benchmark profile/panel governance fix.

### P1-4: External-review handoff is not truth-safe until WP-20B supersedes the thin WP-20 packet

Role: docs/evidence

Evidence:

- `.adl/v0.91.2/bodies/issue-3173-v0-91-2-wp-20b-full-internal-review-party.md:31`
- `.adl/v0.91.2/bodies/issue-3173-v0-91-2-wp-20b-full-internal-review-party.md:70`
- `docs/milestones/v0.91.2/review/internal_review/REVIEW_PACKET.md:5`
- `docs/milestones/v0.91.2/review/internal_review/WP21_EXTERNAL_REVIEW_HANDOFF.md:5`

The older WP-20 packet still says to proceed to WP-21, while WP-20B exists specifically because that review was incomplete.

Impact: external reviewers may be handed a stale readiness story unless WP-20B findings are made controlling.

Route: WP-21/WP-22 handoff update.

## P2 Findings

### P2-1: Provider substrate can overstate generic HTTP provider native tool capability

Role: code

Evidence:

- `adl/src/provider_substrate.rs:229`
- `adl/src/provider_substrate.rs:230`
- `adl/src/provider_substrate.rs:231`
- `adl/src/provider_native_tool_call_comparison.rs:200`
- `adl/src/provider_native_tool_call_comparison.rs:224`

Generic HTTP compatibility profiles can appear native tool-call capable by default metadata, even when reports later caveat that they are compatibility surfaces.

Route: provider substrate/reporting remediation.

### P2-2: Benchmark retry path is effectively disabled

Role: code, tests

Evidence:

- `adl/src/uts_acc_multi_model_benchmark.rs:31`
- `adl/src/uts_acc_multi_model_benchmark.rs:495`
- `adl/src/uts_acc_multi_model_benchmark.rs:501`

Retry classification exists, but max attempts is one.

Route: benchmark reliability remediation.

### P2-3: Fail-closed tasks can pass when the model proposes a forbidden action that ACC rejects

Role: architecture/methodology, tests

Evidence:

- `adl/tools/benchmark/uts_33_task_panel.json:108`
- `adl/src/uts_acc_multi_model_benchmark.rs:294`
- `adl/src/uts_acc_multi_model_benchmark.rs:932`
- `adl/src/uts_acc_multi_model_benchmark.rs:935`

Refusal and downstream governance rejection are both counted as pass for fail-closed tasks.

Route: benchmark rubric decision and test update.

### P2-4: Runner can exit successfully after provider/lane failures

Role: architecture/methodology, tests

Evidence:

- `adl/tools/uts_benchmark_runner.py:552`
- `adl/tools/uts_benchmark_runner.py:673`
- `adl/tools/uts_benchmark_runner.py:853`
- `docs/milestones/v0.91.2/review/uts_benchmark_evidence/RUNBOOK.md:176`

Provider-failed lanes produce artifacts, and the runner can still exit zero, leaving proof suitability to manual interpretation.

Route: publication evidence gate.

### P2-5: Localhost hosted-provider adapters expose secret-backed provider execution to any local process

Role: security

Evidence:

- `adl/tools/real_chatgpt_gemini_claude_provider_adapter.py:250`
- `adl/tools/real_chatgpt_gemini_claude_provider_adapter.py:314`
- `adl/tools/real_chatgpt_gemini_provider_adapter.py:247`
- `adl/tools/real_multi_agent_provider_adapter.py:228`

Local unauthenticated HTTP adapters can forward arbitrary local prompts using loaded provider credentials.

Route: hosted adapter security hardening.

### P2-6: Benchmark artifacts persist raw model response excerpts without redaction

Role: security

Evidence:

- `adl/tools/uts_benchmark_runner.py:663`
- `adl/tools/uts_benchmark_runner.py:666`
- `adl/tools/uts_benchmark_runner.py:795`
- `adl/src/uts_acc_multi_model_benchmark.rs:756`
- `adl/src/uts_acc_multi_model_benchmark.rs:807`

Raw model excerpts are durable in benchmark artifacts without a redaction pass.

Route: artifact redaction and publication gate.

### P2-7: Absolute paths outside the repo can be written into benchmark reports

Role: security

Evidence:

- `adl/tools/uts_benchmark_runner.py:94`
- `adl/tools/uts_benchmark_runner.py:745`
- `adl/tools/uts_benchmark_runner.py:835`
- `adl/tools/benchmark/uts_benchmark_panel.py:12`

`display_path` falls back to absolute paths outside the repo.

Route: portable path normalization.

### P2-8: Remote UTS memo relies on non-portable temporary evidence paths

Role: docs/evidence

Evidence:

- `docs/milestones/v0.91.2/review/uts_remote_open_model_evidence_memo_2026-05-20.md:104`
- `docs/milestones/v0.91.2/review/uts_remote_open_model_evidence_memo_2026-05-20.md:108`
- `docs/milestones/v0.91.2/review/uts_benchmark_evidence/README.md:17`
- `docs/milestones/v0.91.2/review/uts_benchmark_evidence/RUNBOOK.md:150`

The memo's strongest rows cite local temporary artifacts rather than tracked evidence.

Route: evidence preservation or claim downgrade.

### P2-9: One GitHub workflow still uses floating `actions/checkout@v4`

Role: dependency/tooling

Evidence:

- `.github/workflows/v0871_milestone_closeout_gate.yaml:15`
- `.github/workflows/ci.yaml:20`
- `.github/workflows/nightly-coverage-ratchet.yaml:20`

Most workflows pin actions by SHA, but one milestone gate remains floating.

Route: CI supply-chain hygiene.

### P2-10: Local authoritative coverage runner auto-installs cargo-nextest

Role: dependency/tooling

Evidence:

- `adl/tools/run_local_authoritative_coverage_gate.sh:58`
- `adl/tools/run_local_authoritative_coverage_gate.sh:62`
- `adl/tools/run_local_authoritative_coverage_gate.sh:63`

A validation command mutates local tooling state and depends on live crates.io.

Route: validation ergonomics/reproducibility fix.

### P2-11: WP-20B lifecycle card truth is stale during active review

Role: docs/evidence

Evidence:

- `.adl/v0.91.2/tasks/issue-3173__v0-91-2-wp-20b-full-internal-review-party/sip.md:21`
- `.adl/v0.91.2/tasks/issue-3173__v0-91-2-wp-20b-full-internal-review-party/srp.md:105`
- `.adl/v0.91.2/tasks/issue-3173__v0-91-2-wp-20b-full-internal-review-party/sor.md:17`

Cards still contain bootstrap/draft review language while the worktree now contains review artifacts.

Route: editor-skill cleanup before PR finish.

## P3 Findings

### P3-1: Provider error details are surfaced into artifacts without redaction

Role: security

Evidence:

- `adl/tools/uts_benchmark_runner.py:710`
- `adl/tools/uts_benchmark_runner.py:803`
- `adl/tools/benchmark/portable_benchmark_common.py:210`

### P3-2: Stale/duplicate benchmark helper surface remains beside the one-runner contract

Role: dependency/tooling

Evidence:

- `docs/milestones/v0.91.2/review/uts_benchmark_evidence/RUNBOOK.md:7`
- `docs/milestones/v0.91.2/review/uts_benchmark_evidence/RUNBOOK.md:13`
- `adl/tools/benchmark/portable_benchmark_common.py:96`
- `adl/tools/benchmark/portable_benchmark_common.py:145`

### P3-3: Release evidence index remains too abstract for final repeatable review

Role: architecture/methodology

Evidence:

- `docs/milestones/v0.91.2/RELEASE_EVIDENCE_v0.91.2.md:19`
- `docs/milestones/v0.91.2/RELEASE_EVIDENCE_v0.91.2.md:30`
- `docs/milestones/v0.91.2/QUALITY_GATE_v0.91.2.md:38`

### P3-4: Milestone docs are conservative but stale about WP-20B corrective review state

Role: docs/evidence

Evidence:

- `docs/milestones/v0.91.2/README.md:8`
- `docs/milestones/v0.91.2/RELEASE_READINESS_v0.91.2.md:23`
- `.adl/v0.91.2/bodies/issue-3173-v0-91-2-wp-20b-full-internal-review-party.md:77`
