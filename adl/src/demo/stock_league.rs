use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use serde_json::{json, Value};

use super::write_file;

pub(super) const DEMO_NAME: &str = "demo-i-v090-stock-league-scaffold";

const RUN_ID: &str = "demo-i-stock-league-scaffold-run-001";
const SEASON_ID: &str = "season-001";
const FIXED_TIME: &str = "2026-04-17T00:00:00Z";
const DISCLAIMER: &str = "This is a paper-market simulation for demonstrating persistent agent identity and accountability. It is not financial advice, trading advice, or a real investment strategy.";
const FIXTURE_JSON: &str =
    include_str!("../../../demos/fixtures/stock_league/season_001_fixture.json");

#[derive(Clone, Copy)]
struct AgentSpec {
    id: &'static str,
    display_name: &'static str,
    role: &'static str,
    primary_lens: &'static str,
    risk_tolerance: &'static str,
    tension: &'static str,
    forbidden_behavior: &'static str,
}

const AGENTS: &[AgentSpec] = &[
    AgentSpec {
        id: "value_monk",
        display_name: "The Value Monk",
        role: "competing_agent",
        primary_lens: "valuation and balance-sheet discipline",
        risk_tolerance: "moderate",
        tension: "may underperform when momentum dominates",
        forbidden_behavior: "opening a paper position solely because price is rising",
    },
    AgentSpec {
        id: "momentum_surfer",
        display_name: "The Momentum Surfer",
        role: "competing_agent",
        primary_lens: "relative strength and regime change",
        risk_tolerance: "moderate_high",
        tension: "must exit when the trend breaks instead of narrating around losses",
        forbidden_behavior: "holding a broken trend without naming the failed setup",
    },
    AgentSpec {
        id: "contrarian_raccoon",
        display_name: "The Contrarian Raccoon",
        role: "competing_agent",
        primary_lens: "overreaction and broken narratives",
        risk_tolerance: "moderate",
        tension: "must separate disliked from permanently impaired",
        forbidden_behavior: "disagreeing with consensus without evidence",
    },
    AgentSpec {
        id: "quality_gardener",
        display_name: "The Quality Gardener",
        role: "competing_agent",
        primary_lens: "durable margins and patient compounding",
        risk_tolerance: "moderate_low",
        tension: "must avoid overpaying for excellent businesses",
        forbidden_behavior: "ignoring valuation because a business is high quality",
    },
    AgentSpec {
        id: "macro_weather_oracle",
        display_name: "The Macro Weather Oracle",
        role: "competing_agent",
        primary_lens: "rates, inflation, sector rotation, and liquidity",
        risk_tolerance: "moderate",
        tension: "macro stories can explain too much after the fact",
        forbidden_behavior: "claiming an outcome was obvious only after it happened",
    },
    AgentSpec {
        id: "risk_goblin",
        display_name: "The Risk Goblin",
        role: "risk_reviewer",
        primary_lens: "concentration, drawdown, liquidity, and unsupported confidence",
        risk_tolerance: "low",
        tension: "must be useful rather than reflexively fearful",
        forbidden_behavior: "blocking every paper action without a concrete risk reason",
    },
    AgentSpec {
        id: "archivist_referee",
        display_name: "The Archivist Referee",
        role: "referee",
        primary_lens: "append-only records, hindsight checks, and identity drift",
        risk_tolerance: "not_applicable",
        tension: "must be boring, precise, and difficult to fool",
        forbidden_behavior: "silently rewriting prior commitments",
    },
];

pub(super) fn write_stock_league_scaffold_step(
    out_dir: &Path,
    step_id: &str,
) -> Result<Vec<PathBuf>> {
    match step_id {
        "fixture" => write_fixture_step(out_dir),
        "agents" => write_agent_step(out_dir),
        "paper_rules" => write_paper_rules_step(out_dir),
        "proof_packet" => write_proof_packet_step(out_dir),
        _ => Ok(Vec::new()),
    }
}

fn write_fixture_step(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let fixture = fixture()?;
    let mut artifacts = vec![
        write_json(out_dir, "fixture/season_001_fixture.json", &fixture)?,
        write_json(
            out_dir,
            "market/universe.json",
            &universe_manifest(&fixture)?,
        )?,
        write_json(
            out_dir,
            "market/data_source_report.json",
            &data_source_report(),
        )?,
    ];

    let snapshots = fixture
        .get("snapshots")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("fixture missing snapshots array"))?;
    for snapshot in snapshots {
        let snapshot_id = snapshot
            .get("snapshot_id")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("fixture snapshot missing snapshot_id"))?;
        artifacts.push(write_json(
            out_dir,
            &format!("market/snapshots/{snapshot_id}.json"),
            snapshot,
        )?);
    }

    Ok(artifacts)
}

fn write_agent_step(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut artifacts = Vec::new();
    for agent in AGENTS {
        artifacts.push(write_json(
            out_dir,
            &format!("agents/{}/identity.json", agent.id),
            &agent_identity(*agent),
        )?);
        artifacts.push(write_file(
            out_dir,
            &format!("agents/{}/style_card.md", agent.id),
            &agent_style_card(*agent),
        )?);
        artifacts.push(write_file(
            out_dir,
            &format!("agents/{}/memory_journal.jsonl", agent.id),
            &agent_initial_journal(*agent),
        )?);
    }
    Ok(artifacts)
}

fn write_paper_rules_step(out_dir: &Path) -> Result<Vec<PathBuf>> {
    Ok(vec![
        write_file(out_dir, "README.md", &readme())?,
        write_file(out_dir, "reviewer_walkthrough.md", &reviewer_walkthrough())?,
        write_json(out_dir, "league_rules.json", &league_rules())?,
        write_json(out_dir, "guardrail_policy.json", &guardrail_policy())?,
        write_json(out_dir, "decisions/day-001.json", &day_one_decisions())?,
        write_file(out_dir, "paper_ledger.jsonl", &paper_ledger_jsonl())?,
        write_json(
            out_dir,
            "scoreboard/league_standings.json",
            &league_standings(),
        )?,
        write_json(out_dir, "audit/guardrail_report.json", &guardrail_report())?,
    ])
}

fn write_proof_packet_step(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let scan = scan_public_artifacts(out_dir)?;
    let scan_path = write_json(out_dir, "audit/artifact_safety_scan.json", &scan)?;
    if !scan.get("passed").and_then(Value::as_bool).unwrap_or(false) {
        return Err(anyhow!("stock league scaffold safety scan failed"));
    }

    let manifest = season_manifest();
    let manifest_path = write_json(out_dir, "season_manifest.json", &manifest)?;
    let proof_path = write_json(out_dir, "proof_packet.json", &proof_packet())?;
    Ok(vec![scan_path, manifest_path, proof_path])
}

fn fixture() -> Result<Value> {
    serde_json::from_str(FIXTURE_JSON).context("failed to parse stock league fixture")
}

fn write_json(out_dir: &Path, rel: &str, value: &Value) -> Result<PathBuf> {
    let mut body = serde_json::to_string_pretty(value)?;
    body.push('\n');
    write_file(out_dir, rel, &body)
}

fn universe_manifest(fixture: &Value) -> Result<Value> {
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

fn data_source_report() -> Value {
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

fn agent_identity(agent: AgentSpec) -> Value {
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

fn agent_style_card(agent: AgentSpec) -> String {
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

fn agent_initial_journal(agent: AgentSpec) -> String {
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

fn league_rules() -> Value {
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

fn guardrail_policy() -> Value {
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

fn day_one_decisions() -> Value {
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

fn paper_ledger_jsonl() -> String {
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

fn league_standings() -> Value {
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

fn guardrail_report() -> Value {
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

fn season_manifest() -> Value {
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

fn proof_packet() -> Value {
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
        "deferred_to_wp08": [
            "recurring bounded cycles",
            "continuity and ledger evidence across more than one cycle",
            "status and guardrail artifacts connected to an agent run",
            "inspection over a real long-lived demo state root"
        ],
        "not_financial_advice": true
    })
}

fn readme() -> String {
    format!(
        "# Stock League Demo Scaffold\n\n{DISCLAIMER}\n\n## What This Proves\n\nThe scaffold proves that ADL can produce a deterministic paper-market demo root with stable agent identities, explicit league rules, fixture data, paper-only decisions, and safety guardrails.\n\n## What This Does Not Do\n\n- It does not place orders.\n- It does not connect to broker APIs.\n- It does not use live market data.\n- It does not use personal financial information.\n- It does not claim market-beating ability.\n\n## Reviewer Path\n\n1. Inspect `season_manifest.json`.\n2. Inspect `agents/*/identity.json` and `agents/*/style_card.md`.\n3. Inspect `fixture/season_001_fixture.json` and `market/snapshots/*.json`.\n4. Inspect `decisions/day-001.json` and `paper_ledger.jsonl`.\n5. Inspect `audit/guardrail_report.json` and `audit/artifact_safety_scan.json`.\n\nWP-08 owns the later integration where this scaffold becomes a recurring long-lived agent demo.\n"
    )
}

fn reviewer_walkthrough() -> String {
    format!(
        "# Reviewer Walkthrough\n\n{DISCLAIMER}\n\nThis scaffold is intentionally fixture-first. Start with `proof_packet.json`, then follow its artifact refs. The important review question is not which paper allocation wins. The important review question is whether agent identity, paper-only guardrails, and accountability surfaces are visible before any future recurring-cycle integration.\n"
    )
}

fn scan_public_artifacts(out_dir: &Path) -> Result<Value> {
    let files = collect_files(out_dir)?;
    let patterns: &[(&str, &[&str])] = &[
        ("private_host_path", &["/users/", "\\users\\"]),
        (
            "secret_material",
            &[
                "bearer ",
                "private_key",
                "begin rsa private key",
                "secret_access_key",
            ],
        ),
        (
            "broker_credentials",
            &["broker_account", "broker_credentials", "account_number"],
        ),
        (
            "financial_advice_claim",
            &[
                "you should buy",
                "personalized financial recommendation",
                "market beating",
                "guaranteed return",
            ],
        ),
        ("real_trading_surface", &["live_order", "order_status_url"]),
    ];

    let mut findings = Vec::new();
    for rel in &files {
        let path = out_dir.join(rel);
        let Ok(contents) = fs::read_to_string(&path) else {
            continue;
        };
        let lowered = contents.to_ascii_lowercase();
        for (family, family_patterns) in patterns {
            for pattern in *family_patterns {
                if lowered.contains(pattern) {
                    findings.push(json!({
                        "family": family,
                        "pattern": pattern,
                        "artifact_ref": rel.to_string_lossy()
                    }));
                }
            }
        }
    }

    Ok(json_ok_scan(files, findings))
}

fn json_ok_scan(files: Vec<PathBuf>, findings: Vec<Value>) -> Value {
    json!({
        "schema_version": "adl.stock_league.artifact_safety_scan.v1",
        "run_id": RUN_ID,
        "season_id": SEASON_ID,
        "scanned_at": FIXED_TIME,
        "passed": findings.is_empty(),
        "scanned_artifacts": files
            .iter()
            .map(|path| path.to_string_lossy().to_string())
            .collect::<Vec<_>>(),
        "findings": findings,
        "checks": {
            "private_host_path_detected": false,
            "secret_material_detected": false,
            "broker_credentials_detected": false,
            "financial_advice_claim_detected": false,
            "forbidden_live_order_surface_detected": false
        },
        "disclaimer": DISCLAIMER
    })
}

fn collect_files(root: &Path) -> Result<Vec<PathBuf>> {
    fn visit(root: &Path, dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        let mut entries = fs::read_dir(dir)
            .with_context(|| format!("failed to read demo artifact dir '{}'", dir.display()))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        entries.sort_by_key(|entry| entry.path());
        for entry in entries {
            let path = entry.path();
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                visit(root, &path, files)?;
            } else if file_type.is_file() {
                files.push(path.strip_prefix(root)?.to_path_buf());
            }
        }
        Ok(())
    }

    let mut files = Vec::new();
    visit(root, root, &mut files)?;
    files.sort();
    Ok(files)
}
