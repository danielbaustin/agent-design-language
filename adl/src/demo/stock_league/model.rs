use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use serde_json::{json, Value};

use super::super::write_file;
use super::shared::{
    AgentSpec, AGENTS, DEMO_NAME, DISCLAIMER, EXTENSION_DEMO_NAME, EXTENSION_RUN_ID, FIXED_TIME,
    FIXTURE_JSON, INTEGRATION_DEMO_NAME, INTEGRATION_RUN_ID, RUN_ID, SEASON_ID,
};

pub(super) fn fixture() -> Result<Value> {
    serde_json::from_str(FIXTURE_JSON).context("failed to parse stock league fixture")
}

pub(super) fn write_json(out_dir: &Path, rel: &str, value: &Value) -> Result<PathBuf> {
    let mut body = serde_json::to_string_pretty(value)?;
    body.push('\n');
    write_file(out_dir, rel, &body)
}

pub(super) fn universe_manifest(fixture: &Value) -> Result<Value> {
    let symbols = fixture
        .get("symbols")
        .cloned()
        .ok_or_else(|| anyhow!("fixture missing symbols"))?;
    Ok(json!({
        "schema_version": "adl.stock_league.universe.v1",
        "season_id": SEASON_ID,
        "mode": "fixture_replay",
        "disclaimer": DISCLAIMER,
        "symbols": symbols,
        "benchmarks": ["SPY", "QQQ"],
        "network_required": false,
        "broker_required": false,
        "selection_note": "Tiny deterministic fixture universe for review; not a security selection service."
    }))
}

pub(super) fn data_source_report() -> Value {
    json!({
        "schema_version": "adl.stock_league.data_source_report.v1",
        "season_id": SEASON_ID,
        "mode": "fixture_replay",
        "sources": [{
            "source_id": "repo-fixture-season-001",
            "source_type": "committed_fixture",
            "path": "fixture/season_001_fixture.json",
            "network_required": false,
            "api_key_required": false,
            "broker_required": false,
            "freshness": "historical_fixture"
        }],
        "forbidden_sources": [
            "live quote feeds",
            "broker APIs",
            "private portfolio data",
            "paid terminals",
            "unlicensed scraping"
        ],
        "disclaimer": DISCLAIMER
    })
}

pub(super) fn agent_identity(agent: AgentSpec) -> Value {
    json!({
        "schema_version": "adl.stock_league.agent_identity.v1",
        "agent_id": agent.id,
        "display_name": agent.display_name,
        "role": agent.role,
        "season_id": SEASON_ID,
        "disclaimer": DISCLAIMER,
        "style_contract": {
            "primary_lens": agent.primary_lens,
            "forbidden_behaviors": [
                agent.forbidden_behavior,
                "rewriting old claims after outcomes are known",
                "presenting paper decisions as personalized advice"
            ],
            "risk_tolerance": agent.risk_tolerance,
            "rebalance_cadence": "weekly_fixture"
        },
        "memory_policy": {
            "append_only_journal": true,
            "can_summarize_old_memory": true,
            "cannot_delete_prior_commitments": true,
            "must_name_prior_belief_when_revising": true
        },
        "score_policy": {
            "raw_return_weight": 0.25,
            "risk_adjusted_return_weight": 0.20,
            "calibration_weight": 0.20,
            "identity_consistency_weight": 0.20,
            "explanation_quality_weight": 0.15
        },
        "persistent_tension": agent.tension,
        "not_financial_advice": true
    })
}

pub(super) fn agent_style_card(agent: AgentSpec) -> String {
    format!(
        "# {display_name}\n\n{disclaimer}\n\n- Agent id: `{agent_id}`\n- Role: `{role}`\n- Primary lens: {primary_lens}\n- Risk tolerance: {risk_tolerance}\n- Persistent tension: {tension}\n- Forbidden behavior: {forbidden_behavior}\n\nThis card defines a paper-market demo identity, not an investment advisory profile.\n",
        display_name = agent.display_name,
        disclaimer = DISCLAIMER,
        agent_id = agent.id,
        role = agent.role,
        primary_lens = agent.primary_lens,
        risk_tolerance = agent.risk_tolerance,
        tension = agent.tension,
        forbidden_behavior = agent.forbidden_behavior,
    )
}

pub(super) fn agent_initial_journal(agent: AgentSpec) -> String {
    let entry = json!({
        "schema_version": "adl.stock_league.memory_journal_entry.v1",
        "season_id": SEASON_ID,
        "agent_id": agent.id,
        "entry_id": format!("{}-journal-000001", agent.id),
        "created_at": FIXED_TIME,
        "kind": "identity_seed",
        "summary": format!("{} entered the paper-market league with a stable style contract.", agent.display_name),
        "not_financial_advice": true
    });
    format!(
        "{}\n",
        serde_json::to_string(&entry).expect("serialize journal")
    )
}

pub(super) fn league_rules() -> Value {
    json!({
        "schema_version": "adl.stock_league.rules.v1",
        "season_id": SEASON_ID,
        "disclaimer": DISCLAIMER,
        "starting_paper_capital_usd": 100000,
        "max_open_positions": 5,
        "max_allocation_per_ticker_pct": 25,
        "minimum_cash_pct": 0,
        "leverage_allowed": false,
        "options_allowed": false,
        "short_selling_allowed": false,
        "intraday_loop_allowed": false,
        "paper_fill_rule": "next_available_daily_close_in_fixture_mode",
        "allowed_actions": [
            "open_position",
            "increase_position",
            "trim_position",
            "close_position",
            "hold",
            "stay_in_cash",
            "challenge_peer",
            "revise_thesis"
        ],
        "forbidden_actions": [
            "execute_order",
            "place_order",
            "connect_broker",
            "request_personal_financial_profile",
            "claim_market_beating_strategy"
        ],
        "benchmarks": ["SPY", "QQQ"],
        "not_financial_advice": true
    })
}

pub(super) fn guardrail_policy() -> Value {
    json!({
        "schema_version": "adl.stock_league.guardrail_policy.v1",
        "season_id": SEASON_ID,
        "required_checks": [
            "fixture_mode",
            "network_disabled",
            "no_real_trading",
            "no_broker_integration",
            "not_financial_advice",
            "paper_only_ledger",
            "no_personalized_inputs",
            "artifact_sanitization"
        ],
        "operator_role": "league_commissioner",
        "operator_disallowed_actions": [
            "ask_what_should_i_buy",
            "provide_personal_assets_or_risk_profile",
            "route_paper_decisions_to_real_world_system",
            "rewrite_prior_ledger_entries"
        ],
        "not_financial_advice": true
    })
}

pub(super) fn day_one_decisions() -> Value {
    json!({
        "schema_version": "adl.stock_league.decision_batch.v1",
        "season_id": SEASON_ID,
        "decision_date": "2024-10-01",
        "market_snapshot_id": "market-2024-10-01",
        "mode": "fixture_replay",
        "disclaimer": DISCLAIMER,
        "decisions": [
            paper_decision("value_monk", "open_position", "MSFT", 20, "Durable cash generation and balance-sheet quality fit the valuation lens in this paper fixture.", "Multiple compression or weaker cloud growth would invalidate the paper thesis."),
            paper_decision("momentum_surfer", "open_position", "NVDA", 15, "Relative strength is visible in the fixture and the allocation is capped by the risk policy.", "A break below the fixture risk level would invalidate the paper setup."),
            paper_decision("contrarian_raccoon", "stay_in_cash", Option::<&str>::None, 0, "The hated-asset bar is not met in the fixture, so refusing action is more faithful than forcing disagreement.", "If energy weakness creates a clear overreaction later, revisit with a timestamped thesis."),
            paper_decision("quality_gardener", "open_position", "KO", 10, "Steady consumer-staples behavior fits the quality lens with a small paper allocation.", "If valuation becomes the only reason to hold, reduce confidence."),
            paper_decision("macro_weather_oracle", "hold", "SPY", 0, "The macro agent records benchmark context without asserting a directional paper allocation.", "A fixture drawdown should weaken overconfident regime claims."),
            paper_decision("risk_goblin", "challenge_peer", "NVDA", 0, "The momentum paper allocation is allowed but concentration and reversal risk must be recorded.", "The warning is wrong if the trend persists without drawdown."),
            paper_decision("archivist_referee", "hold", Option::<&str>::None, 0, "The referee preserves decisions and checks that no agent edits prior claims.", "Any missing timestamp or rewritten thesis triggers audit failure.")
        ],
        "not_financial_advice": true
    })
}

fn paper_decision(
    agent_id: &str,
    action: &str,
    ticker: impl serde::Serialize,
    paper_allocation_pct: u64,
    thesis: &str,
    risk_thesis: &str,
) -> Value {
    json!({
        "decision_id": format!("{agent_id}-2024-10-01-001"),
        "agent_id": agent_id,
        "season_id": SEASON_ID,
        "decision_date": "2024-10-01",
        "market_snapshot_id": "market-2024-10-01",
        "action": action,
        "ticker": ticker,
        "paper_allocation_pct": paper_allocation_pct,
        "time_horizon": "fixture_review_window",
        "thesis": thesis,
        "risk_thesis": risk_thesis,
        "disconfirming_evidence": [
            "fixture drawdown violates the stated risk threshold",
            "agent cannot explain the decision without changing its identity contract"
        ],
        "identity_self_check": {
            "consistent_with_style": true,
            "style_note": "The decision is scored against the agent identity card."
        },
        "not_financial_advice": true,
        "paper_only": true
    })
}

pub(super) fn paper_ledger_jsonl() -> String {
    let rows = [
        json!({
            "schema_version": "adl.stock_league.paper_ledger_entry.v1",
            "ledger_entry_id": "ledger-value_monk-000001",
            "season_id": SEASON_ID,
            "agent_id": "value_monk",
            "decision_id": "value_monk-2024-10-01-001",
            "ticker": "MSFT",
            "paper_allocation_pct": 20,
            "paper_fill_basis": "fixture_next_daily_close",
            "real_world_side_effect": false,
            "not_financial_advice": true
        }),
        json!({
            "schema_version": "adl.stock_league.paper_ledger_entry.v1",
            "ledger_entry_id": "ledger-momentum_surfer-000001",
            "season_id": SEASON_ID,
            "agent_id": "momentum_surfer",
            "decision_id": "momentum_surfer-2024-10-01-001",
            "ticker": "NVDA",
            "paper_allocation_pct": 15,
            "paper_fill_basis": "fixture_next_daily_close",
            "real_world_side_effect": false,
            "not_financial_advice": true
        }),
        json!({
            "schema_version": "adl.stock_league.paper_ledger_entry.v1",
            "ledger_entry_id": "ledger-quality_gardener-000001",
            "season_id": SEASON_ID,
            "agent_id": "quality_gardener",
            "decision_id": "quality_gardener-2024-10-01-001",
            "ticker": "KO",
            "paper_allocation_pct": 10,
            "paper_fill_basis": "fixture_next_daily_close",
            "real_world_side_effect": false,
            "not_financial_advice": true
        }),
    ];
    let mut body = rows
        .iter()
        .map(|row| serde_json::to_string(row).expect("serialize ledger row"))
        .collect::<Vec<_>>()
        .join("\n");
    body.push('\n');
    body
}

pub(super) fn league_standings() -> Value {
    json!({
        "schema_version": "adl.stock_league.scoreboard.v1",
        "season_id": SEASON_ID,
        "as_of_date": "2024-10-02",
        "disclaimer": DISCLAIMER,
        "ranking_basis": "identity_accountability_first",
        "score_components": [
            "paper_return",
            "risk_adjusted_return",
            "calibration",
            "identity_consistency",
            "explanation_quality"
        ],
        "standings": [
            {
                "agent_id": "quality_gardener",
                "paper_return_pct": 0.89,
                "identity_consistency_score": 0.96,
                "calibration_score": 0.88,
                "rank_note": "Small paper action stayed aligned with the style card."
            },
            {
                "agent_id": "value_monk",
                "paper_return_pct": -1.19,
                "identity_consistency_score": 0.93,
                "calibration_score": 0.82,
                "rank_note": "Loss is acceptable in the scaffold because accountability is visible."
            },
            {
                "agent_id": "momentum_surfer",
                "paper_return_pct": -2.02,
                "identity_consistency_score": 0.85,
                "calibration_score": 0.76,
                "rank_note": "Risk Goblin warning is preserved for later cycle integration."
            }
        ],
        "not_financial_advice": true
    })
}

pub(super) fn guardrail_report() -> Value {
    json!({
        "schema_version": "adl.stock_league.guardrail_report.v1",
        "season_id": SEASON_ID,
        "run_id": RUN_ID,
        "status": "pass",
        "checks": [
            pass_check("fixture_mode", "The canonical proof path uses the committed fixture only."),
            pass_check("network_disabled", "No network or live data source is required."),
            pass_check("no_real_trading", "All ledger entries are paper-only and have no real-world side effect."),
            pass_check("no_broker_integration", "No broker URL, credential, or account identifier is accepted."),
            pass_check("not_financial_advice", "Every public surface carries the paper-market disclaimer."),
            pass_check("paper_only_ledger", "The ledger records hypothetical fixture fills only."),
            pass_check("no_personalized_inputs", "The operator is a league commissioner, not a customer profile."),
            pass_check("artifact_sanitization", "The proof step scans generated public artifacts.")
        ],
        "rejected_actions": [{
            "action": "execute_order",
            "status": "rejected_by_policy",
            "reason": "WP-07 scaffold forbids real order placement."
        }],
        "not_financial_advice": true
    })
}

fn pass_check(check_id: &str, summary: &str) -> Value {
    json!({
        "check_id": check_id,
        "status": "pass",
        "summary": summary
    })
}

pub(super) fn season_manifest() -> Value {
    json!({
        "schema_version": "adl.stock_league.season_manifest.v1",
        "demo_id": DEMO_NAME,
        "run_id": RUN_ID,
        "season_id": SEASON_ID,
        "mode": "fixture_replay",
        "generated_at": FIXED_TIME,
        "disclaimer": DISCLAIMER,
        "primary_claim": "bounded stock league scaffold with persistent paper-market identities",
        "artifact_root": ".",
        "canonical_fixture": "fixture/season_001_fixture.json",
        "agents": AGENTS.iter().map(|agent| json!({
            "agent_id": agent.id,
            "identity_ref": format!("agents/{}/identity.json", agent.id),
            "style_card_ref": format!("agents/{}/style_card.md", agent.id),
            "journal_ref": format!("agents/{}/memory_journal.jsonl", agent.id)
        })).collect::<Vec<_>>(),
        "proof_refs": {
            "league_rules": "league_rules.json",
            "guardrail_policy": "guardrail_policy.json",
            "decisions": "decisions/day-001.json",
            "paper_ledger": "paper_ledger.jsonl",
            "guardrail_report": "audit/guardrail_report.json",
            "safety_scan": "audit/artifact_safety_scan.json",
            "proof_packet": "proof_packet.json"
        },
        "non_goals": [
            "no live trading",
            "no personalized advice",
            "no broker integration",
            "no market-beating claim",
            "no paid data dependency"
        ],
        "not_financial_advice": true
    })
}

pub(super) fn proof_packet() -> Value {
    json!({
        "schema_version": "adl.stock_league.proof_packet.v1",
        "demo_id": DEMO_NAME,
        "run_id": RUN_ID,
        "season_id": SEASON_ID,
        "status": "scaffold_ready",
        "primary_claim": "WP-07 provides a fixture-backed paper-market scaffold for long-lived agent identity review.",
        "disclaimer": DISCLAIMER,
        "proof_command": "cargo run --manifest-path adl/Cargo.toml -- demo demo-i-v090-stock-league-scaffold --run --trace --out out --no-open",
        "validation_command": "cargo test --manifest-path adl/Cargo.toml stock_league_scaffold -- --nocapture",
        "review_walkthrough": [
            "Read README.md for the no-advice and no-real-trading boundary.",
            "Inspect season_manifest.json for the artifact map.",
            "Inspect agents/*/identity.json for persistent identity cards.",
            "Inspect fixture/season_001_fixture.json for deterministic market data.",
            "Inspect decisions/day-001.json and paper_ledger.jsonl for paper-only decisions.",
            "Inspect audit/guardrail_report.json and audit/artifact_safety_scan.json for safety proof."
        ],
        "required_outputs": {
            "fixture_backed_scaffold": true,
            "paper_only_league_rules": true,
            "agent_identity_style_cards": true,
            "demo_artifact_root": true,
            "public_guardrails": true,
            "cheap_deterministic_fixture_path": true
        },
        "wp08_integration": {
            "status": "available_as_follow_on_demo",
            "demo_id": INTEGRATION_DEMO_NAME,
            "proof_command": "cargo run --manifest-path adl/Cargo.toml -- demo demo-j-v090-stock-league-recurring --run --trace --out out --no-open",
            "adds": [
                "recurring bounded cycles",
                "continuity and ledger evidence across more than one cycle",
                "status and guardrail artifacts connected to an agent run",
                "inspection over a real long-lived demo state root"
            ]
        },
        "not_financial_advice": true
    })
}

pub(super) fn stock_league_agent_spec() -> String {
    format!(
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: stock-league-archivist
display_name: Stock League Archivist
state_root: state
workflow:
  kind: demo_adapter
  name: stock_league_fixture_recorder
  run_args:
    season_id: {season_id}
    fixture_ref: ../../fixture/season_001_fixture.json
    scaffold_manifest_ref: ../../integration_manifest.json
    requested_action: record_cycle
    paper_only: true
    public_claim: fixture-backed recurring stock league accountability demo
    competing_agents:
      - value_monk
      - momentum_surfer
      - contrarian_raccoon
      - quality_gardener
      - macro_weather_oracle
    reviewers:
      - risk_goblin
      - archivist_referee
heartbeat:
  interval_secs: 0
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: stock-league/season-001/archivist
  write_policy: append_only
"#,
        season_id = SEASON_ID
    )
}

pub(super) fn integration_manifest() -> Value {
    json!({
        "schema_version": "adl.stock_league.integration_manifest.v1",
        "demo_id": INTEGRATION_DEMO_NAME,
        "run_id": INTEGRATION_RUN_ID,
        "season_id": SEASON_ID,
        "mode": "fixture_replay_long_lived_agent",
        "generated_at": FIXED_TIME,
        "disclaimer": DISCLAIMER,
        "primary_claim": "bounded recurring stock league cycles preserve visible history and guardrails",
        "artifact_root": ".",
        "state_root_ref": "long_lived_agent/state",
        "agent_spec_ref": "long_lived_agent/stock_league_agent.yaml",
        "scaffold_refs": {
            "fixture": "fixture/season_001_fixture.json",
            "agents": "agents/*/identity.json",
            "league_rules": "league_rules.json",
            "paper_ledger": "paper_ledger.jsonl"
        },
        "runtime_refs": {
            "status": "long_lived_agent/state/status.json",
            "cycle_ledger": "long_lived_agent/state/cycle_ledger.jsonl",
            "continuity": "long_lived_agent/state/continuity.json",
            "memory_index": "long_lived_agent/state/memory_index.json",
            "provider_binding_history": "long_lived_agent/state/provider_binding_history.jsonl",
            "cycles": "long_lived_agent/state/cycles"
        },
        "proof_refs": {
            "latest_inspection": "inspection/latest.json",
            "first_cycle_inspection": "inspection/cycle-000001.json",
            "latest_cycle_inspection": "inspection/cycle-000003.json",
            "continuity_proof": "continuity/continuity_proof.json",
            "guardrail_summary": "audit/recurring_guardrail_summary.json",
            "safety_scan": "audit/artifact_safety_scan.json",
            "proof_packet": "integration_proof_packet.json"
        },
        "reviewer_steps": [
            "Inspect long_lived_agent/state/status.json for completed multi-cycle state.",
            "Inspect long_lived_agent/state/cycle_ledger.jsonl for append-only cycle order.",
            "Inspect continuity/continuity_proof.json for previous-cycle links and preserved first-cycle artifacts.",
            "Inspect inspection/latest.json and inspection/cycle-000001.json to compare latest and prior commitments.",
            "Inspect audit/recurring_guardrail_summary.json for per-cycle safety results."
        ],
        "not_financial_advice": true
    })
}

pub(super) fn integration_proof_packet(continuity: &Value, guardrails: &Value) -> Value {
    json!({
        "schema_version": "adl.stock_league.recurring_integration_proof_packet.v1",
        "demo_id": INTEGRATION_DEMO_NAME,
        "run_id": INTEGRATION_RUN_ID,
        "season_id": SEASON_ID,
        "status": "pass",
        "primary_claim": "WP-08 integrates the stock league scaffold with the long-lived agent runtime for recurring bounded fixture cycles.",
        "disclaimer": DISCLAIMER,
        "proof_command": "cargo run --manifest-path adl/Cargo.toml -- demo demo-j-v090-stock-league-recurring --run --trace --out out --no-open",
        "validation_command": "cargo test --manifest-path adl/Cargo.toml stock_league -- --nocapture",
        "state_root_ref": "long_lived_agent/state",
        "cycle_ledger_ref": "long_lived_agent/state/cycle_ledger.jsonl",
        "status_ref": "long_lived_agent/state/status.json",
        "inspection_refs": [
            "inspection/latest.json",
            "inspection/cycle-000001.json",
            "inspection/cycle-000003.json"
        ],
        "continuity_proof": continuity,
        "guardrail_summary": guardrails,
        "required_outputs": {
            "recurring_bounded_cycles": true,
            "more_than_one_cycle": true,
            "continuity_and_ledger_evidence": true,
            "status_and_guardrail_artifacts": true,
            "history_preservation_proof": true,
            "deterministic_fixture_mode": true
        },
        "non_goals": [
            "no live market loop",
            "no hidden long-running process",
            "no investment recommendation framing",
            "no broker integration",
            "no real order placement"
        ],
        "not_financial_advice": true
    })
}

pub(super) fn extension_selection() -> Value {
    json!({
        "schema_version": "adl.stock_league.demo_extension_selection.v1",
        "demo_id": EXTENSION_DEMO_NAME,
        "run_id": EXTENSION_RUN_ID,
        "matrix_row": "D5",
        "status": "selected",
        "selection_time": FIXED_TIME,
        "selection_rule": "extend the D4 stock-league proof path without adding live data, broker surfaces, or unrelated product claims",
        "selected_demo_choices": [
            {
                "choice_id": "D5-A",
                "name": "stock_league_reviewer_evidence_index",
                "entrypoint": EXTENSION_DEMO_NAME,
                "extends_demo": INTEGRATION_DEMO_NAME,
                "classification": "proving",
                "proof_claim": "A reviewer can follow one stable evidence index from the selected D5 extension claim to the D4 recurring state root, continuity proof, guardrail summary, and replay command.",
                "proof_packet_ref": "extension_proof_packet.json",
                "expected_artifact_root": "out/demo-k-v090-stock-league-proof-expansion"
            }
        ],
        "non_proving_surfaces": [
            {
                "name": "scaffold_only_walkthrough",
                "classification": "supporting",
                "reason": "The WP-07 scaffold is useful context, but D5 proof rests on the recurring D4 integration artifacts."
            }
        ],
        "explicit_deferrals": [
            {
                "name": "live_market_data_extension",
                "deferred_to": "post-v0.90",
                "reason": "Live market data would add external dependency and review burden beyond this milestone."
            },
            {
                "name": "broker_or_order_execution_extension",
                "deferred_to": "out_of_scope",
                "reason": "The v0.90 demo lane remains paper-only and must not expose order placement surfaces."
            },
            {
                "name": "multi-provider_competitive_league",
                "deferred_to": "post-v0.90",
                "reason": "Provider competition would distract from the long-lived state and accountability proof."
            }
        ],
        "not_financial_advice": true
    })
}

pub(super) fn extension_proof_claims(
    integration_proof: &Value,
    continuity: &Value,
    guardrails: &Value,
) -> Value {
    json!({
        "schema_version": "adl.stock_league.demo_extension_claims.v1",
        "demo_id": EXTENSION_DEMO_NAME,
        "run_id": EXTENSION_RUN_ID,
        "status": "pass",
        "claims": [
            {
                "claim_id": "D5-A-1",
                "claim": "The selected extension is named, bounded, and classified before proof review.",
                "evidence_refs": ["demo_extension_selection.json"],
                "result": "pass"
            },
            {
                "claim_id": "D5-A-2",
                "claim": "The extension preserves D4 as the primary long-lived stock-league proof instead of competing with it.",
                "evidence_refs": ["integration_proof_packet.json", "extensions/evidence_index.json"],
                "result": if integration_proof.get("status").and_then(Value::as_str) == Some("pass") {
                    "pass"
                } else {
                    "fail"
                }
            },
            {
                "claim_id": "D5-A-3",
                "claim": "The recurring proof still shows three bounded cycles with preserved prior commitments.",
                "evidence_refs": ["continuity/continuity_proof.json", "long_lived_agent/state/cycle_ledger.jsonl"],
                "result": if continuity.get("status").and_then(Value::as_str) == Some("pass")
                    && continuity.get("cycle_count").and_then(Value::as_u64) == Some(3)
                    && continuity
                        .pointer("/history_preservation/prior_commitments_preserved")
                        .and_then(Value::as_bool)
                        == Some(true)
                {
                    "pass"
                } else {
                    "fail"
                }
            },
            {
                "claim_id": "D5-A-4",
                "claim": "The extension keeps the no-broker, paper-only, no-real-world-side-effect guardrail boundary visible.",
                "evidence_refs": ["audit/recurring_guardrail_summary.json", "extensions/extension_artifact_safety_scan.json"],
                "result": if guardrails.get("status").and_then(Value::as_str) == Some("pass") {
                    "pass"
                } else {
                    "fail"
                }
            }
        ],
        "not_financial_advice": true
    })
}

pub(super) fn extension_evidence_index(integration_scan: &Value) -> Value {
    json!({
        "schema_version": "adl.stock_league.demo_extension_evidence_index.v1",
        "demo_id": EXTENSION_DEMO_NAME,
        "run_id": EXTENSION_RUN_ID,
        "extends_demo": INTEGRATION_DEMO_NAME,
        "status": if integration_scan.get("passed").and_then(Value::as_bool) == Some(true) {
            "pass"
        } else {
            "fail"
        },
        "proof_command": "cargo run --manifest-path adl/Cargo.toml -- demo demo-k-v090-stock-league-proof-expansion --run --trace --out out --no-open",
        "source_proof_command": "cargo run --manifest-path adl/Cargo.toml -- demo demo-j-v090-stock-league-recurring --run --trace --out out --no-open",
        "evidence_refs": [
            {
                "artifact_ref": "demo_extension_selection.json",
                "purpose": "selected D5 demo choices, explicit non-goals, and deferrals"
            },
            {
                "artifact_ref": "integration_proof_packet.json",
                "purpose": "source D4 recurring proof packet reused as the baseline"
            },
            {
                "artifact_ref": "continuity/continuity_proof.json",
                "purpose": "prior-cycle links and preserved first-cycle artifacts"
            },
            {
                "artifact_ref": "audit/recurring_guardrail_summary.json",
                "purpose": "per-cycle paper-only guardrail status"
            },
            {
                "artifact_ref": "inspection/latest.json",
                "purpose": "latest reviewer inspection packet"
            },
            {
                "artifact_ref": "extensions/replay_manifest.json",
                "purpose": "deterministic replay command and expected outputs"
            },
            {
                "artifact_ref": "extensions/proof_claims.json",
                "purpose": "claim-by-claim proof registry for the selected extension"
            }
        ],
        "non_goals_ref": "extensions/non_goals_and_deferrals.json",
        "not_financial_advice": true
    })
}

pub(super) fn extension_replay_manifest() -> Value {
    json!({
        "schema_version": "adl.stock_league.demo_extension_replay_manifest.v1",
        "demo_id": EXTENSION_DEMO_NAME,
        "run_id": EXTENSION_RUN_ID,
        "replay_mode": "deterministic_fixture",
        "command": "cargo run --manifest-path adl/Cargo.toml -- demo demo-k-v090-stock-league-proof-expansion --run --trace --out out --no-open",
        "expected_artifacts": [
            "demo_extension_selection.json",
            "integration_proof_packet.json",
            "continuity/continuity_proof.json",
            "audit/recurring_guardrail_summary.json",
            "extensions/evidence_index.json",
            "extensions/proof_claims.json",
            "extensions/non_goals_and_deferrals.json",
            "extensions/extension_artifact_safety_scan.json",
            "extension_proof_packet.json"
        ],
        "expected_cycle_count": 3,
        "expected_latest_cycle_id": "cycle-000003",
        "network_required": false,
        "broker_required": false,
        "real_world_side_effects": false,
        "not_financial_advice": true
    })
}

pub(super) fn extension_deferral_register() -> Value {
    json!({
        "schema_version": "adl.stock_league.demo_extension_deferrals.v1",
        "demo_id": EXTENSION_DEMO_NAME,
        "run_id": EXTENSION_RUN_ID,
        "non_goals": [
            "no new live-market loop",
            "no broker integration",
            "no order placement",
            "no provider competition claim",
            "no product-readiness claim beyond the fixture-backed proof packet"
        ],
        "deferred": [
            {
                "name": "live_market_data_extension",
                "home": "post-v0.90 planning",
                "reason": "External data freshness and licensing are outside the bounded fixture proof."
            },
            {
                "name": "broker_or_order_execution_extension",
                "home": "out of scope for ADL v0.90",
                "reason": "The release must remain paper-only."
            },
            {
                "name": "multi-provider_competitive_league",
                "home": "post-v0.90 planning",
                "reason": "The v0.90 proof is about long-lived state, not provider ranking."
            }
        ],
        "not_financial_advice": true
    })
}

pub(super) fn extension_proof_packet(
    selection: &Value,
    proof_claims: &Value,
    evidence: &Value,
    replay: &Value,
    deferrals: &Value,
    extension_scan: &Value,
) -> Value {
    let claims_pass = proof_claims
        .get("claims")
        .and_then(Value::as_array)
        .is_some_and(|claims| {
            claims
                .iter()
                .all(|claim| claim.get("result").and_then(Value::as_str) == Some("pass"))
        });
    let scan_pass = extension_scan.get("passed").and_then(Value::as_bool) == Some(true);

    json!({
        "schema_version": "adl.stock_league.demo_extension_proof_packet.v1",
        "demo_id": EXTENSION_DEMO_NAME,
        "run_id": EXTENSION_RUN_ID,
        "matrix_row": "D5",
        "season_id": SEASON_ID,
        "status": if claims_pass && scan_pass { "pass" } else { "fail" },
        "primary_claim": "WP-09 lands one bounded D5 demo extension that expands reviewer proof around the D4 recurring stock-league path without creating unrelated demo scope.",
        "proof_command": "cargo run --manifest-path adl/Cargo.toml -- demo demo-k-v090-stock-league-proof-expansion --run --trace --out out --no-open",
        "validation_command": "cargo test --manifest-path adl/Cargo.toml stock_league -- --nocapture",
        "selected_demo_choices": selection["selected_demo_choices"].clone(),
        "source_demo": {
            "demo_id": INTEGRATION_DEMO_NAME,
            "proof_packet_ref": "integration_proof_packet.json",
            "state_root_ref": "long_lived_agent/state"
        },
        "proof_claims": proof_claims,
        "evidence_index": evidence,
        "replay_manifest": replay,
        "non_goals_and_deferrals": deferrals,
        "extension_safety_scan": extension_scan,
        "required_outputs": {
            "named_demo_choices": true,
            "proof_claims": true,
            "proof_command": true,
            "proof_packet": true,
            "non_goals": true,
            "demo_matrix_disposition": true,
            "deterministic_fixture_mode": true
        },
        "disclaimer": DISCLAIMER,
        "not_financial_advice": true
    })
}

pub(super) fn recurring_continuity_proof(out_dir: &Path) -> Result<Value> {
    let state = "long_lived_agent/state";
    let status = read_json_rel(out_dir, &format!("{state}/status.json"))?;
    let continuity = read_json_rel(out_dir, &format!("{state}/continuity.json"))?;
    let memory_index = read_json_rel(out_dir, &format!("{state}/memory_index.json"))?;
    let ledger_lines = read_jsonl_rel(out_dir, &format!("{state}/cycle_ledger.jsonl"))?;
    let mut cycles = Vec::new();
    for cycle_id in ["cycle-000001", "cycle-000002", "cycle-000003"] {
        let manifest_ref = format!("{state}/cycles/{cycle_id}/cycle_manifest.json");
        let manifest = read_json_rel(out_dir, &manifest_ref)?;
        let guardrail_ref = format!("{state}/cycles/{cycle_id}/guardrail_report.json");
        let guardrail = read_json_rel(out_dir, &guardrail_ref)?;
        cycles.push(json!({
            "cycle_id": cycle_id,
            "status": manifest["status"].clone(),
            "previous_cycle_id": manifest["previous_cycle_id"].clone(),
            "manifest_ref": manifest_ref,
            "guardrail_report_ref": guardrail_ref,
            "memory_writes_ref": format!("{state}/cycles/{cycle_id}/memory_writes.jsonl"),
            "guardrail_status": guardrail["status"].clone()
        }));
    }

    let first_manifest_still_present = out_dir
        .join("long_lived_agent/state/cycles/cycle-000001/cycle_manifest.json")
        .is_file();
    let third_links_second = cycles
        .get(2)
        .and_then(|cycle| cycle.get("previous_cycle_id"))
        .and_then(Value::as_str)
        == Some("cycle-000002");
    let second_links_first = cycles
        .get(1)
        .and_then(|cycle| cycle.get("previous_cycle_id"))
        .and_then(Value::as_str)
        == Some("cycle-000001");
    let first_has_no_previous = cycles
        .first()
        .and_then(|cycle| cycle.get("previous_cycle_id"))
        .is_some_and(Value::is_null);
    let cycle_count = ledger_lines.len();

    Ok(json!({
        "schema_version": "adl.stock_league.continuity_proof.v1",
        "run_id": INTEGRATION_RUN_ID,
        "season_id": SEASON_ID,
        "status": if cycle_count == 3 && first_has_no_previous && second_links_first && third_links_second && first_manifest_still_present {
            "pass"
        } else {
            "fail"
        },
        "state_root_ref": "long_lived_agent/state",
        "cycle_count": cycle_count,
        "completed_cycle_count": status["completed_cycle_count"].clone(),
        "latest_cycle_id": continuity["latest_cycle_id"].clone(),
        "latest_cycle_status": continuity["latest_cycle_status"].clone(),
        "append_only_ledger_ref": "long_lived_agent/state/cycle_ledger.jsonl",
        "ledger_entry_count": ledger_lines.len(),
        "cycles": cycles,
        "history_preservation": {
            "prior_commitments_preserved": first_manifest_still_present && second_links_first && third_links_second,
            "cycle_000001_manifest_still_present_after_cycle_000003": first_manifest_still_present,
            "cycle_chain_links_prior_cycle_ids": first_has_no_previous && second_links_first && third_links_second,
            "memory_index_ref": "long_lived_agent/state/memory_index.json",
            "memory_index_schema": memory_index["schema"].clone()
        },
        "fixture_commitment_refs": [
            "decisions/day-001.json",
            "paper_ledger.jsonl",
            "agents/value_monk/memory_journal.jsonl"
        ],
        "not_financial_advice": true
    }))
}

pub(super) fn recurring_guardrail_summary(out_dir: &Path) -> Result<Value> {
    let mut cycle_reports = Vec::new();
    for cycle_id in ["cycle-000001", "cycle-000002", "cycle-000003"] {
        let rel = format!("long_lived_agent/state/cycles/{cycle_id}/guardrail_report.json");
        let report = read_json_rel(out_dir, &rel)?;
        let failed_checks = report
            .get("checks")
            .and_then(Value::as_array)
            .map(|checks| {
                checks
                    .iter()
                    .filter(|check| check.get("result").and_then(Value::as_str) == Some("fail"))
                    .map(|check| check.get("check_id").cloned().unwrap_or(Value::Null))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        cycle_reports.push(json!({
            "cycle_id": cycle_id,
            "guardrail_report_ref": rel,
            "status": report["status"].clone(),
            "failed_checks": failed_checks,
            "rejected_actions": report["rejected_actions"].clone()
        }));
    }
    let all_passed = cycle_reports
        .iter()
        .all(|report| report.get("status").and_then(Value::as_str) == Some("pass"));

    Ok(json!({
        "schema_version": "adl.stock_league.recurring_guardrail_summary.v1",
        "run_id": INTEGRATION_RUN_ID,
        "season_id": SEASON_ID,
        "status": if all_passed { "pass" } else { "fail" },
        "cycle_reports": cycle_reports,
        "paper_only": true,
        "network_required": false,
        "broker_required": false,
        "real_world_side_effects": false,
        "not_financial_advice": true,
        "disclaimer": DISCLAIMER
    }))
}

pub(super) fn read_json_rel(out_dir: &Path, rel: &str) -> Result<Value> {
    let path = out_dir.join(rel);
    let raw = fs::read_to_string(&path)
        .with_context(|| format!("failed reading stock league artifact {rel}"))?;
    serde_json::from_str(&raw)
        .with_context(|| format!("failed parsing stock league artifact {rel}"))
}

fn read_jsonl_rel(out_dir: &Path, rel: &str) -> Result<Vec<Value>> {
    let raw = fs::read_to_string(out_dir.join(rel))
        .with_context(|| format!("failed reading stock league jsonl artifact {rel}"))?;
    raw.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str(line)
                .with_context(|| format!("failed parsing stock league jsonl artifact {rel}"))
        })
        .collect()
}
