# Demo Matrix - v0.91.2

## Status

Release-tail convergence matrix. The owning WPs have now landed their bounded
demo or proof surfaces, and WP-17 is responsible for making the strongest
truthful showcase path explicit rather than leaving the milestone in candidate
demo language.

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
| Project-ready GWS CMS operational package | #3094 | Show the bridge can be reused on future CodeFriend/ADL projects with bounded setup, safe defaults, workflow templates, and GitHub boundary rules. | `review/google_workspace_cms_bridge/codefriend_gws_operational_package.md`, `review/google_workspace_cms_bridge/gws_project_setup_and_onboarding.md`, `review/google_workspace_cms_bridge/gws_safe_defaults_and_scope_checklist.md`, `review/google_workspace_cms_bridge/gws_project_workflow_template.md`, `review/google_workspace_cms_bridge/codefriend_gws_git_workspace_boundary_runbook.md`, and `review/google_workspace_cms_bridge/gws_reusable_proof_packet_template.md` | does not claim broad enterprise Workspace administration or canonical Git replacement |
| `.adl` to GWS migration plan | #3112 | Record how future projects should use GWS as bounded collaboration infrastructure without replacing local `.adl` workflow truth or GitHub-tracked canonical truth. | `features/GOOGLE_WORKSPACE_CMS_BRIDGE.md` migration-plan section plus the GWS operational package docs | does not retire `.adl` cards in `v0.91.2` or make Workspace canonical |
| Moderne / OpenRewrite LST modernization demo | WP-10 | Show a bounded ADL-governed modernization plan around Moderne/OpenRewrite, deterministic recipes, and LST-based transformation. | `review/code_modernization/modernization_demo_packet.md`, `review/code_modernization/modernization_interaction_plan.md`, `review/code_modernization/modernization_dry_run_evidence.md`, `review/code_modernization/modernization_reversibility_and_review_policy.md`, `review/code_modernization/modernization_execution_command.md`, `review/code_modernization/modernization_execution_log.txt`, and `review/code_modernization/modernization_rewrite.patch` | does not mass-rewrite by default or claim broad production modernization readiness from one bounded recipe run |
| Speculative decoding prototype | WP-11 | Show whether speculative decoding is worth continuing for ADL workloads without weakening deterministic commit semantics. | `review/speculative_decoding/speculative_decoding_prototype_report.json` and `review/speculative_decoding/speculative_decoding_prototype_packet.md` | does not grant execution authority or claim production provider speedups |
| Repo visibility follow-on | WP-12 | Show reviewer/planner navigation improvements from bounded manifest/linkage follow-on work. | `review/repo_visibility/CANONICAL_SOURCE_MANIFEST_v0.91.2.yaml`, `review/repo_visibility/CODE_DOC_LINKAGE_REPORT_v0.91.2.md`, and `review/repo_visibility/REVIEWER_NAVIGATION_PACKET_v0.91.2.md` | does not claim full repo cognition or hidden indexing |
| Publication program package | WP-13 | Show arXiv/Medium backlog, authoring process, and review-gate readiness. | `review/publication_program/ARXIV_AND_MEDIUM_PUBLICATION_BACKLOG_v0.91.2.md`, `review/publication_program/PUBLICATION_REVIEW_GATES_v0.91.2.md`, and `review/publication_program/GODEL_AGENTS_GHB_BACKLOG_NOTE_v0.91.2.md` | does not publish or imply submission approval |
| General intelligence paper packet | WP-14 | Show claim/citation/review packet readiness for the general-intelligence manuscript while keeping the separate paper repo as canonical manuscript home. | `review/general_intelligence_paper/GENERAL_INTELLIGENCE_PAPER_CLAIM_AND_CITATION_PACKET_v0.91.2.md`, `review/general_intelligence_paper/GENERAL_INTELLIGENCE_PAPER_REVIEW_HANDOFF_v0.91.2.md`, `review/general_intelligence_paper/GENERAL_INTELLIGENCE_PAPER_RESIDUAL_RISK_AND_UNSUPPORTED_CLAIMS_v0.91.2.md`, and `review/general_intelligence_paper/GENERAL_INTELLIGENCE_PAPER_NEXT_AUTHORING_STEPS_v0.91.2.md` | does not claim proof, publication, or canonical manuscript ownership inside this repo |
| Rustdoc/doc cleanup proof | WP-15 | Show rustdoc and documentation cleanup patches remain source-grounded. | `features/RUSTDOC_DOC_CLEANUP.md`, the tracked docs updates landed by WP-15, and the WP-17 convergence packet that records the cleanup proof route and boundary | does not rewrite broad docs outside issue scope |
| Workflow guardrails demo | WP-16 | Show main-write, closeout-watch, safe-report-command, and card-drift protections. | `adl/tools/workflow_guardrails.sh`, `adl/tools/test_workflow_guardrails.sh`, and `review/workflow_guardrails/` | does not claim all operator error eliminated |
| Feature proof coverage record | WP-17 | Map every WP-02 through WP-16 implementation or docs/productization slice to an explicit proof route or deferral. | demo matrix, proof coverage record, and `review/demo_proof_convergence/DEMO_PROOF_CONVERGENCE_PACKET_v0.91.2.md` | does not replace implementation proof or fabricate missing live demos |

## Flagship Acceptance

The milestone should have at least two flagship proof surfaces:

- a UTS + ACC multi-model benchmark report with clear separation between
  proposal behavior and execution authority
- a workflow/test-cycle recovery proof that demonstrates faster, safer
  milestone execution without weakening review or coverage truth

The strongest additional showcase paths now are:

- the Google Workspace CMS bridge stack (`WP-08`, `WP-09`, `#3091`, `#3093`,
  `#3094`) as the milestone's richest bounded collaboration and live-safety
  demo lane
- the Moderne / OpenRewrite bounded dry-run with retained patch/log evidence
  as the milestone's most concrete external modernization proof
- the workflow guardrails tool/test/runbook packet as the milestone's clearest
  operator-safety demo
