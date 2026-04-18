# Code / Doc / Demo Linkage Report - v0.90

## Metadata

- Milestone: v0.90
- Issue: #2031
- Work package: WP-12 Repo visibility prototype
- Slice: long-lived runtime and stock-league proof path
- Status: prototype report

## Summary

This report connects one bounded v0.90 slice from tracked milestone docs to
code, tests, demos, review surfaces, issue records, and known gaps.

The selected slice is the long-lived runtime package because it is the central
v0.90 thesis and already has enough implementation and demo evidence to make a
repo-visibility prototype useful.

## Authority Model

| Surface | Status | How to read it |
| --- | --- | --- |
| `docs/milestones/v0.90/*.md` | canonical tracked milestone truth | Public review starts here. |
| `docs/milestones/v0.90/features/*.md` | feature contracts and proof designs | Executable only when linked by WBS/readiness gates. |
| `docs/milestones/v0.90/ideas/*.md` | background and later-band context | Not shipped claims unless promoted. |
| local operator planning material | local operator planning | Not public release truth. |
| retired local planning material | historical provenance | Do not treat as active scope. |

## Canonical Docs For This Slice

| Path | Role | Linkage status |
| --- | --- | --- |
| `docs/milestones/v0.90/README.md` | milestone entrypoint | present |
| `docs/milestones/v0.90/WBS_v0.90.md` | WP map and issue graph | present |
| `docs/milestones/v0.90/SPRINT_v0.90.md` | sprint sequencing | present |
| `docs/milestones/v0.90/WP_ISSUE_WAVE_v0.90.yaml` | opened issue wave | present |
| `docs/milestones/v0.90/WP_EXECUTION_READINESS_v0.90.md` | execution gates | present |
| `docs/milestones/v0.90/DEMO_MATRIX_v0.90.md` | planned proof surfaces | present |
| `docs/milestones/v0.90/features/LONG_LIVED_AGENT_RUNTIME_FEATURE_SET.md` | package overview | present |
| `docs/milestones/v0.90/features/FEATURE_LONG_LIVED_SUPERVISOR_HEARTBEAT.md` | supervisor/heartbeat contract | present |
| `docs/milestones/v0.90/features/FEATURE_LONG_LIVED_AGENT_CYCLE_CONTRACT.md` | cycle artifact contract | present |
| `docs/milestones/v0.90/features/FEATURE_LONG_LIVED_STATE_AND_CONTINUITY.md` | state and continuity contract | present |
| `docs/milestones/v0.90/features/FEATURE_LONG_LIVED_OPERATOR_CONTROL_AND_SAFETY.md` | operator safety contract | present |
| `docs/milestones/v0.90/features/LONG_LIVED_STOCK_PICKING_AGENTS_DEMO_PLAN.md` | stock-league proof design | present |

## Implementation Linkage

| Claim area | Current implementation evidence | Status |
| --- | --- | --- |
| Long-lived agent spec and state root | `adl/src/long_lived_agent.rs` defines spec loading, state root handling, locked spec, continuity, status, stop, lease, cycle ledger, memory index, provider binding history, and cycle artifact writers. | present |
| Operator command surface | `adl/src/cli/agent_cmd.rs` exposes `agent tick`, `agent run`, `agent status`, and `agent stop`. | present |
| Bounded multi-cycle execution | `adl/src/long_lived_agent.rs` and `adl/tests/cli_smoke/agent.rs` cover bounded `run --max-cycles` behavior. | present |
| Stop/status control | `adl/src/cli/agent_cmd.rs` and `adl/src/long_lived_agent.rs` expose status and stop paths. | present |
| Cycle artifacts | `adl/src/long_lived_agent.rs` writes cycle manifests, observations, decision request/result, run refs, memory writes, guardrail reports, and summaries. | present |
| Stock-league proof | `demos/v0.90/long_lived_stock_league_demo.md`, `demos/v0.90/stock_league_demo_extensions.md`, and `adl/src/demo/stock_league/` provide the v0.90 scaffold, recurring integration, and extension proof packets. | present |
| Minimal inspection/query surface | `adl/src/long_lived_agent.rs` exposes `inspect`, latest-cycle inspection, specific-cycle inspection, and unsafe-cycle-ref rejection paths. | present |

## Test And Demo Linkage

| Surface | What it proves | Status |
| --- | --- | --- |
| `adl/tests/cli_smoke/agent.rs` | Agent run writes exactly bounded cycles, preserves ledgers, writes continuity and status, and supports status reads after completion. | present |
| `cargo test --manifest-path adl/Cargo.toml long_lived_agent::tests -- --nocapture` | Long-lived runtime tests prove heartbeat/lease, cycle artifacts, continuity, inspection, stop controls, guardrails, and paper-only rejection behavior. | present |
| `cargo run --manifest-path adl/Cargo.toml -- demo demo-j-v090-stock-league-recurring --run --trace --out out --no-open` | Fixture-backed recurring paper league produces proving demo artifacts without financial advice, broker integration, live model dependence, or host-path leakage. | present |
| `cargo run --manifest-path adl/Cargo.toml -- demo demo-k-v090-stock-league-proof-expansion --run --trace --out out --no-open` | Bounded demo extension adds reviewer evidence without replacing the primary D4 stock-league proof. | present |
| `docs/milestones/v0.90/DEMO_MATRIX_v0.90.md` D1-D5 | Landed v0.90 proof surfaces for supervisor heartbeat, cycle contract, operator controls, stock-league integration, and proof expansion. | present |
| `docs/milestones/v0.90/DEMO_MATRIX_v0.90.md` D6 | Repo visibility proof packet. | delivered by this issue |

## Issue Linkage

| Issue | WP | Role | Current linkage |
| --- | --- | --- | --- |
| #2021 | WP-02 | Long-lived supervisor and heartbeat | implementation slice |
| #2022 | WP-03 | Cycle contract and artifact root | implementation slice |
| #2023 | WP-04 | State and continuity handles | implementation slice |
| #2024 | WP-05 | Operator control and safety | implementation slice |
| #2025 | WP-06 | Minimal inspection and trace boundary | inspection slice |
| #2026 | WP-07 | Stock league demo scaffold | demo slice |
| #2027 | WP-08 | Long-lived demo integration | integration slice |
| #2028 | WP-09 | Demo extensions and proof expansion | optional demo extension lane |
| #2031 | WP-12 | Repo visibility prototype | this proof packet |

## Present / Missing / Deferred Surfaces

### Present

- Tracked milestone entrypoints and issue wave.
- Core long-lived runtime feature docs.
- Rust long-lived agent module and agent CLI command surface.
- CLI smoke test for bounded multi-cycle behavior.
- v0.90 stock-league scaffold, recurring integration, and proof expansion docs.
- This repo-visibility manifest and linkage report.

### Expected Pending Work

- WP-10 should finish the coverage ratchet evidence before release readiness.
- WP-13 should verify that this report still matches the docs and code after
  the implementation and sidecar WPs merge.
- WP-14 should finish the explicit Rust refactoring rollup after the remaining
  refactor child issues settle.

### Deferred / Out Of Scope

- Full repo semantic indexing.
- Ingesting local `.adl` planning notes as public release truth.
- Automatic release approval.
- Broad cleanup of unrelated docs, tests, or demos.
- Claiming the v0.92 identity/capability substrate is implemented by v0.90
  continuity handles.

## Reviewer Use

A reviewer can use this packet to answer:

- Which docs define the v0.90 long-lived runtime claim?
- Which code and tests currently implement or prove parts of that claim?
- Which demos are landed v0.90 proof versus still-open release-tail work?
- Which issues own the remaining implementation and integration work?
- Which materials are canonical tracked truth versus local planning or
  historical context?

## Validation Notes

Validation for this WP should confirm:

- the manifest and report paths exist;
- the referenced tracked docs, code, tests, and demos exist;
- the report does not contain unjustified absolute host paths;
- missing surfaces are marked as pending or deferred rather than inferred.
