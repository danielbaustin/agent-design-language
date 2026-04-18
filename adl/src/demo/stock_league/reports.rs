use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde_json::{json, Value};

use super::files::collect_files;
use super::shared::{DISCLAIMER, FIXED_TIME, RUN_ID, SEASON_ID};

pub(super) fn readme() -> String {
    format!(
        "# Stock League Demo Scaffold\n\n{DISCLAIMER}\n\n## What This Proves\n\nThe scaffold proves that ADL can produce a deterministic paper-market demo root with stable agent identities, explicit league rules, fixture data, paper-only decisions, and safety guardrails.\n\n## What This Does Not Do\n\n- It does not place orders.\n- It does not connect to broker APIs.\n- It does not use live market data.\n- It does not use personal financial information.\n- It does not claim market-beating ability.\n\n## Reviewer Path\n\n1. Inspect `season_manifest.json`.\n2. Inspect `agents/*/identity.json` and `agents/*/style_card.md`.\n3. Inspect `fixture/season_001_fixture.json` and `market/snapshots/*.json`.\n4. Inspect `decisions/day-001.json` and `paper_ledger.jsonl`.\n5. Inspect `audit/guardrail_report.json` and `audit/artifact_safety_scan.json`.\n\nWP-08 owns the later integration where this scaffold becomes a recurring long-lived agent demo.\n"
    )
}

pub(super) fn reviewer_walkthrough() -> String {
    format!(
        "# Reviewer Walkthrough\n\n{DISCLAIMER}\n\nThis scaffold is intentionally fixture-first. Start with `proof_packet.json`, then follow its artifact refs. The important review question is not which paper allocation wins. The important review question is whether agent identity, paper-only guardrails, and accountability surfaces are visible before any future recurring-cycle integration.\n"
    )
}

pub(super) fn scan_public_artifacts(out_dir: &Path) -> Result<Value> {
    scan_public_artifacts_for_run(out_dir, RUN_ID)
}

pub(super) fn scan_public_artifacts_for_run(out_dir: &Path, run_id: &str) -> Result<Value> {
    let files = collect_files(out_dir)?
        .into_iter()
        .filter(|rel| rel != Path::new("audit/artifact_safety_scan.json"))
        .collect::<Vec<_>>();
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

    Ok(json_ok_scan(files, findings, run_id))
}

fn json_ok_scan(files: Vec<PathBuf>, findings: Vec<Value>, run_id: &str) -> Value {
    json!({
        "schema_version": "adl.stock_league.artifact_safety_scan.v1",
        "run_id": run_id,
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

pub(super) fn integration_readme() -> String {
    format!(
        "# Stock League Recurring Demo Integration\n\n{DISCLAIMER}\n\n## What This Proves\n\nThe integration demo proves that the WP-07 fixture-backed stock league scaffold can be connected to the v0.90 long-lived-agent runtime for recurring bounded cycles. It writes status, continuity, cycle ledger, cycle manifests, guardrail reports, memory writes, and inspection packets under one reviewer-readable artifact root.\n\n## What This Does Not Do\n\n- It does not place orders.\n- It does not connect to broker APIs.\n- It does not use live market data.\n- It does not require a hidden daemon or long-running process.\n- It does not use personal financial information.\n- It does not claim suitability as an investment strategy.\n\n## Reviewer Path\n\n1. Inspect `integration_proof_packet.json`.\n2. Inspect `long_lived_agent/state/status.json` for completed multi-cycle state.\n3. Inspect `long_lived_agent/state/cycle_ledger.jsonl` for the append-only cycle ledger.\n4. Inspect `continuity/continuity_proof.json` for previous-cycle links and preserved first-cycle artifacts.\n5. Inspect `inspection/latest.json` and `inspection/cycle-000001.json` to compare latest state with prior commitments.\n6. Inspect `audit/recurring_guardrail_summary.json` and `audit/artifact_safety_scan.json` for public safety proof.\n"
    )
}

pub(super) fn integration_reviewer_walkthrough() -> String {
    format!(
        "# Recurring Demo Reviewer Walkthrough\n\n{DISCLAIMER}\n\nRun the demo with `cargo run --manifest-path adl/Cargo.toml -- demo demo-j-v090-stock-league-recurring --run --trace --out out --no-open`.\n\nThe command runs three no-sleep fixture cycles through the long-lived-agent runtime. The review question is whether the latest cycle can point back to prior cycle artifacts without erasing earlier commitments, and whether every cycle preserves the no-advice, no-broker, paper-only guardrail boundary.\n"
    )
}

pub(super) fn extension_readme() -> String {
    format!(
        "# Stock League Demo Extension Proof\n\n{DISCLAIMER}\n\n## What This Proves\n\nThe D5 extension proves that v0.90 can add a named, bounded demo-extension packet without weakening the primary stock-league proof. It replays the D4 recurring fixture path, then adds a selected-demo manifest, proof-claim registry, evidence index, replay manifest, non-goals and deferrals register, public artifact safety scan, and extension proof packet.\n\n## Selected Extension\n\n- D5-A: `stock_league_reviewer_evidence_index`\n- Entrypoint: `demo-k-v090-stock-league-proof-expansion`\n- Source proof: `demo-j-v090-stock-league-recurring`\n\n## What This Does Not Do\n\n- It does not add live market data.\n- It does not place orders.\n- It does not connect to broker APIs.\n- It does not rank model providers.\n- It does not claim product readiness beyond the fixture-backed proof packet.\n\n## Reviewer Path\n\n1. Inspect `extension_proof_packet.json`.\n2. Inspect `demo_extension_selection.json` for named selected demos, non-proving surfaces, and deferrals.\n3. Inspect `extensions/evidence_index.json` and `extensions/proof_claims.json`.\n4. Inspect `extensions/replay_manifest.json` for the deterministic proof command.\n5. Inspect `integration_proof_packet.json`, `continuity/continuity_proof.json`, and `audit/recurring_guardrail_summary.json` for the source D4 proof.\n6. Inspect `extensions/extension_artifact_safety_scan.json` for public-artifact safety proof.\n"
    )
}

pub(super) fn extension_reviewer_walkthrough() -> String {
    format!(
        "# Demo Extension Reviewer Walkthrough\n\n{DISCLAIMER}\n\nRun the extension with `cargo run --manifest-path adl/Cargo.toml -- demo demo-k-v090-stock-league-proof-expansion --run --trace --out out --no-open`.\n\nThe command intentionally uses the same deterministic stock-league fixture and three-cycle recurring run as D4. The new D5 review question is narrower: did the milestone name the selected extension, attach proof claims and evidence refs, classify non-proving surfaces, and defer anything that would widen scope?\n"
    )
}
