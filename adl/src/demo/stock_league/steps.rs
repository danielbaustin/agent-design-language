use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use serde_json::Value;

use crate::long_lived_agent::{self, InspectOptions, RunOptions};

use super::super::write_file;
use super::files::collect_existing_files;
use super::model::{
    agent_identity, agent_initial_journal, agent_style_card, data_source_report, day_one_decisions,
    extension_deferral_register, extension_evidence_index, extension_proof_claims,
    extension_proof_packet, extension_replay_manifest, extension_selection, fixture,
    guardrail_policy, guardrail_report, integration_manifest, integration_proof_packet,
    league_rules, league_standings, paper_ledger_jsonl, proof_packet, read_json_rel,
    recurring_continuity_proof, recurring_guardrail_summary, season_manifest,
    stock_league_agent_spec, universe_manifest, write_json,
};
use super::reports::{
    extension_readme, extension_reviewer_walkthrough, integration_readme,
    integration_reviewer_walkthrough, readme, reviewer_walkthrough, scan_public_artifacts,
    scan_public_artifacts_for_run,
};
use super::shared::{AGENTS, EXTENSION_RUN_ID, INTEGRATION_RUN_ID};

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

pub(super) fn write_stock_league_integration_step(
    out_dir: &Path,
    step_id: &str,
) -> Result<Vec<PathBuf>> {
    match step_id {
        "scaffold" => write_integration_scaffold_step(out_dir),
        "recurring_cycles" => write_recurring_cycles_step(out_dir),
        "inspection" => write_inspection_step(out_dir),
        "proof_packet" => write_integration_proof_step(out_dir),
        _ => Ok(Vec::new()),
    }
}

pub(super) fn write_stock_league_extension_step(
    out_dir: &Path,
    step_id: &str,
) -> Result<Vec<PathBuf>> {
    match step_id {
        "selected_demos" => write_extension_selection_step(out_dir),
        "recurring_proof" => write_extension_recurring_proof_step(out_dir),
        "evidence_index" => write_extension_evidence_step(out_dir),
        "review_packet" => write_extension_review_packet_step(out_dir),
        _ => Ok(Vec::new()),
    }
}

fn write_integration_scaffold_step(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut artifacts = Vec::new();
    artifacts.extend(write_fixture_step(out_dir)?);
    artifacts.extend(write_agent_step(out_dir)?);
    let mut paper_rule_artifacts = write_paper_rules_step(out_dir)?;
    paper_rule_artifacts.retain(|path| {
        path.strip_prefix(out_dir)
            .ok()
            .and_then(Path::to_str)
            .is_none_or(|rel| !matches!(rel, "README.md" | "reviewer_walkthrough.md"))
    });
    artifacts.extend(paper_rule_artifacts);
    artifacts.push(write_file(out_dir, "README.md", &integration_readme())?);
    artifacts.push(write_file(
        out_dir,
        "reviewer_walkthrough.md",
        &integration_reviewer_walkthrough(),
    )?);
    Ok(artifacts)
}

fn write_recurring_cycles_step(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let agent_root = out_dir.join("long_lived_agent");
    if agent_root.exists() {
        fs::remove_dir_all(&agent_root).with_context(|| {
            format!(
                "failed to reset stock league recurring state root '{}'",
                agent_root.display()
            )
        })?;
    }

    let spec_rel = "long_lived_agent/stock_league_agent.yaml";
    let spec_path = write_file(out_dir, spec_rel, &stock_league_agent_spec())?;
    let status = long_lived_agent::run(
        &spec_path,
        RunOptions {
            max_cycles: 3,
            interval_secs: Some(0),
            no_sleep: true,
            recover_stale_lease: false,
        },
    )?;

    let mut artifacts = vec![spec_path];
    artifacts.push(write_json(
        out_dir,
        "long_lived_agent/run_status.json",
        &serde_json::to_value(status)?,
    )?);
    artifacts.extend(collect_existing_files(
        &out_dir.join("long_lived_agent/state"),
    )?);
    Ok(artifacts)
}

fn write_inspection_step(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let spec_path = out_dir.join("long_lived_agent/stock_league_agent.yaml");
    if !spec_path.exists() {
        return Err(anyhow!(
            "stock league recurring integration requires recurring_cycles before inspection"
        ));
    }
    Ok(vec![
        write_json(
            out_dir,
            "inspection/latest.json",
            &long_lived_agent::inspect(&spec_path, InspectOptions { cycle_id: None })?,
        )?,
        write_json(
            out_dir,
            "inspection/cycle-000001.json",
            &long_lived_agent::inspect(
                &spec_path,
                InspectOptions {
                    cycle_id: Some("cycle-000001".to_string()),
                },
            )?,
        )?,
        write_json(
            out_dir,
            "inspection/cycle-000003.json",
            &long_lived_agent::inspect(
                &spec_path,
                InspectOptions {
                    cycle_id: Some("cycle-000003".to_string()),
                },
            )?,
        )?,
    ])
}

fn write_integration_proof_step(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let continuity = recurring_continuity_proof(out_dir)?;
    let continuity_path = write_json(out_dir, "continuity/continuity_proof.json", &continuity)?;
    let guardrails = recurring_guardrail_summary(out_dir)?;
    let guardrail_summary_path = write_json(
        out_dir,
        "audit/recurring_guardrail_summary.json",
        &guardrails,
    )?;
    let manifest = integration_manifest();
    let manifest_path = write_json(out_dir, "integration_manifest.json", &manifest)?;
    let scan = scan_public_artifacts_for_run(out_dir, INTEGRATION_RUN_ID)?;
    let scan_path = write_json(out_dir, "audit/artifact_safety_scan.json", &scan)?;
    if !scan.get("passed").and_then(Value::as_bool).unwrap_or(false) {
        return Err(anyhow!(
            "stock league recurring integration safety scan failed"
        ));
    }
    let proof = integration_proof_packet(&continuity, &guardrails);
    let proof_path = write_json(out_dir, "integration_proof_packet.json", &proof)?;
    Ok(vec![
        continuity_path,
        guardrail_summary_path,
        manifest_path,
        scan_path,
        proof_path,
    ])
}

fn write_extension_selection_step(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let extension_root = out_dir.join("extensions");
    if extension_root.exists() {
        fs::remove_dir_all(&extension_root).with_context(|| {
            format!(
                "failed to reset stock league extension root '{}'",
                extension_root.display()
            )
        })?;
    }

    let old_proof = out_dir.join("extension_proof_packet.json");
    if old_proof.exists() {
        fs::remove_file(&old_proof).with_context(|| {
            format!(
                "failed to remove stale stock league extension proof '{}'",
                old_proof.display()
            )
        })?;
    }

    Ok(vec![write_json(
        out_dir,
        "demo_extension_selection.json",
        &extension_selection(),
    )?])
}

fn write_extension_recurring_proof_step(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut artifacts = Vec::new();
    let mut scaffold_artifacts = write_integration_scaffold_step(out_dir)?;
    scaffold_artifacts.retain(|path| {
        path.strip_prefix(out_dir)
            .ok()
            .and_then(Path::to_str)
            .is_none_or(|rel| !matches!(rel, "README.md" | "reviewer_walkthrough.md"))
    });
    artifacts.extend(scaffold_artifacts);
    artifacts.extend(write_recurring_cycles_step(out_dir)?);
    artifacts.extend(write_inspection_step(out_dir)?);
    artifacts.extend(write_integration_proof_step(out_dir)?);
    Ok(artifacts)
}

fn write_extension_evidence_step(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let integration_proof = read_json_rel(out_dir, "integration_proof_packet.json")?;
    let continuity = read_json_rel(out_dir, "continuity/continuity_proof.json")?;
    let guardrails = read_json_rel(out_dir, "audit/recurring_guardrail_summary.json")?;
    let integration_scan = read_json_rel(out_dir, "audit/artifact_safety_scan.json")?;

    let proof_claims = extension_proof_claims(&integration_proof, &continuity, &guardrails);
    let evidence = extension_evidence_index(&integration_scan);
    let replay = extension_replay_manifest();
    let deferrals = extension_deferral_register();

    Ok(vec![
        write_json(out_dir, "extensions/proof_claims.json", &proof_claims)?,
        write_json(out_dir, "extensions/evidence_index.json", &evidence)?,
        write_json(out_dir, "extensions/replay_manifest.json", &replay)?,
        write_json(
            out_dir,
            "extensions/non_goals_and_deferrals.json",
            &deferrals,
        )?,
    ])
}

fn write_extension_review_packet_step(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let selection = read_json_rel(out_dir, "demo_extension_selection.json")?;
    let proof_claims = read_json_rel(out_dir, "extensions/proof_claims.json")?;
    let evidence = read_json_rel(out_dir, "extensions/evidence_index.json")?;
    let replay = read_json_rel(out_dir, "extensions/replay_manifest.json")?;
    let deferrals = read_json_rel(out_dir, "extensions/non_goals_and_deferrals.json")?;

    let readme_path = write_file(out_dir, "README.md", &extension_readme())?;
    let walkthrough_path = write_file(
        out_dir,
        "reviewer_walkthrough.md",
        &extension_reviewer_walkthrough(),
    )?;

    let extension_scan = scan_public_artifacts_for_run(out_dir, EXTENSION_RUN_ID)?;
    let scan_path = write_json(
        out_dir,
        "extensions/extension_artifact_safety_scan.json",
        &extension_scan,
    )?;
    if !extension_scan
        .get("passed")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        return Err(anyhow!("stock league extension safety scan failed"));
    }

    let proof = extension_proof_packet(
        &selection,
        &proof_claims,
        &evidence,
        &replay,
        &deferrals,
        &extension_scan,
    );
    let proof_path = write_json(out_dir, "extension_proof_packet.json", &proof)?;
    Ok(vec![readme_path, walkthrough_path, scan_path, proof_path])
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
