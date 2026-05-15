# Demo Matrix - v0.91.2

## Status

Candidate demo matrix. These demos should be implemented by their owning WPs
and summarized again during WP-17.

| Demo | Owning WP | Purpose | Proving Surface | Non-Claims |
| --- | --- | --- | --- | --- |
| UTS + ACC JSON proposal benchmark | WP-02 | Test model proposal discipline against fixture tools. | benchmark report and fixtures | does not execute real tools |
| Provider-native tool-call comparison | WP-03 | Compare native tool-call interfaces with ADL JSON proposal mode. | comparison report by model/provider | does not claim provider-wide conformance |
| Runtime/test-cycle recovery proof | WP-04 | Show redundant proof phases are reduced safely. | `review/runtime_test_cycle_recovery_report.md` with landed `#3042`-`#3044` proof surfaces; `SLOW_TEST_TIMING_DIAGNOSTICS_v0.91.2.md` is supporting sibling evidence from `WP-05A` | does not weaken authoritative coverage |
| Coverage ergonomics demo | WP-05 | Show changed-source coverage failures point to actionable tests. | `adl/tools/check_coverage_impact.sh` diagnostic output, `adl/tools/test_check_coverage_impact.sh`, and `review/coverage_gate_ergonomics_report.md` | does not waive thresholds silently |
| CodeFriend review packet demo | WP-06 | Show repeatable packet-to-report review workflow. | `review/codefriend_productization/` workflow package, report template, evidence requirements, and skill/demo alignment | does not replace human review |
| Review heuristics demo | WP-07 | Show review heuristics and skills produce bounded review artifacts. | `review/review_heuristics_demo/review_heuristics_promotion.md`, `review/review_heuristics_demo/bounded_review_demo_packet.md`, `review/review_heuristics_demo/review_quality_acceptance_checklist.md`, and the fixture outputs under `review/review_heuristics_demo/fixture_*` | does not invent source evidence or replace human judgment |
| Google Workspace CMS bridge demo | WP-08 | Show draft content-card and promotion packet workflow. | `review/google_workspace_cms_bridge/workspace_cms_snapshot.json`, `review/google_workspace_cms_bridge/workspace_cms_bridge_demo_packet.md`, `review/google_workspace_cms_bridge/workspace_promotion_packet.md`, `review/google_workspace_cms_bridge/workspace_revision_mismatch_and_authority_rules.md`, `review/google_workspace_cms_bridge/workspace_management_report.md`, `review/google_workspace_cms_bridge/workspace_demo_manifest.json`, and `review/google_workspace_cms_bridge/workspace_tool_capability_trace.json` | does not make Workspace canonical |
| Rust-native GWS adapter boundary demo | WP-09 | Show typed native CMS capability with fixture-backed read, promotion, preview, and bounded apply flows. | `review/google_workspace_cms_bridge/rust_native_gws_adapter_boundary_report.json` | does not require live secrets or make Workspace canonical |
| Live bounded GWS capability execution | #3091 | Show one bounded `gws` folder/doc/sheet read slice with truthful skipped behavior when live auth, scope, or tooling is unavailable. | `review/google_workspace_cms_bridge/gws_live_capability_execution_report.json` and `review/google_workspace_cms_bridge/gws_live_capability_execution_snapshot.json` | does not claim live writes or canonical Workspace authority |
| Live bounded content-card mutation roundtrip | #3093 | Show one bounded content-card preview/apply contract with revision-anchor enforcement, truthful skipped live behavior, and promotion-packet handoff discipline. | `review/google_workspace_cms_bridge/gws_live_content_card_roundtrip_report.json` | does not allow silent repo edits, claim bidirectional sync, or prove live mutation without auth/scopes |
| Moderne / OpenRewrite LST modernization demo | WP-10 | Show a bounded ADL-governed modernization plan around Moderne/OpenRewrite, deterministic recipes, and LST-based transformation. | modernization packet and dry-run evidence | does not mass-rewrite by default |
| Speculative decoding prototype | WP-11 | Show bounded acceleration posture without weakening deterministic commit semantics. | prototype/evaluation packet and non-claim record | does not grant execution authority |
| Repo visibility follow-on | WP-12 | Show reviewer/planner navigation improvements from bounded manifest/linkage follow-on work. | manifest/linkage packet and source-map evidence | does not claim full repo cognition |
| Publication program package | WP-13 | Show arXiv/Medium backlog, authoring process, and review-gate readiness. | publication backlog, process docs, and review checklist | does not publish |
| General intelligence paper packet | WP-14 | Show claim/citation/review packet readiness for the general-intelligence manuscript. | claim packet, citation packet, unsupported-claim register, and reviewer handoff | does not claim proof or publish |
| Rustdoc/doc cleanup proof | WP-15 | Show rustdoc and documentation cleanup patches remain source-grounded. | rustdoc/doc cleanup report, changed docs, and validation record | does not rewrite broad docs outside issue scope |
| Workflow guardrails demo | WP-16 | Show main-write, watcher, and safe-report protections. | guardrail fixtures and failure examples | does not claim all operator error eliminated |
| Feature proof coverage record | WP-17 | Map every WP-02 through WP-16 implementation or docs/productization slice to an explicit proof route or deferral. | demo matrix, proof coverage record, source-doc linkage, and validation commands | does not replace implementation proof |

## Flagship Acceptance

The milestone should have at least two flagship proof surfaces:

- a UTS + ACC multi-model benchmark report with clear separation between
  proposal behavior and execution authority
- a workflow/test-cycle recovery proof that demonstrates faster, safer
  milestone execution without weakening review or coverage truth
