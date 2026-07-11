use anyhow::{Context, Result};
use std::path::PathBuf;

use super::agent_cmd::real_csm_daemon;
use super::csm_service_cmd::real_service;
use ::adl::csm_backpressure::{prove_backpressure, BackpressureProofOptions};
use ::adl::csm_cav_red_blue::{prove_cav_red_blue, CavRedBlueProofOptions};
use ::adl::csm_cloud_control::{prove_cloudfront_status, CloudFrontStatusOptions};
use ::adl::csm_continuity_capsule::{
    capture_capsule, fire_drill_capsule, restore_capsule, stage_capsule, ContinuityCaptureOptions,
    ContinuityFireDrillOptions, ContinuityRestoreOptions, ContinuityStageOptions,
};
use ::adl::csm_credential_policy::{prove_credential_policy, CredentialPolicyProofOptions};
use ::adl::csm_godel_snapshot::{prove_godel_snapshot_diff, GodelSnapshotProofOptions};
use ::adl::csm_observatory::{write_observatory_outputs, ObservatoryFormat};
use ::adl::csm_polis_storage::{prove_polis_storage, PolisStorageProofOptions};
use ::adl::csm_runtime_api::{prove_api_gateway_bridge, ApiGatewayBridgeOptions};
use ::adl::long_lived_agent::{governed_stop, GovernedStopRequest};
use ::adl::wp08_acip_sns_proof::run_wp08_acip_sns_live_proof;
use chrono::{DateTime, Utc};

pub(crate) enum CsmDispatchMode {
    StandaloneRuntime,
    AdlControlPlane,
}

pub(crate) fn real_csm(args: &[String]) -> Result<()> {
    real_csm_with_mode(args, CsmDispatchMode::AdlControlPlane)
}

pub(crate) fn real_csm_standalone(args: &[String]) -> Result<()> {
    real_csm_with_mode(args, CsmDispatchMode::StandaloneRuntime)
}

fn real_csm_with_mode(args: &[String], mode: CsmDispatchMode) -> Result<()> {
    let Some(cmd) = args.first().map(|value| value.as_str()) else {
        eprintln!(
            "csm requires subcommand: daemon | service | governed-stop | credential-policy | continuity | godel-snapshot | cav | backpressure | aws-signal | storage | cloud-control | observatory"
        );
        std::process::exit(2);
    };

    match cmd {
        "daemon" => match mode {
            CsmDispatchMode::StandaloneRuntime => real_csm_daemon(&args[1..]),
            CsmDispatchMode::AdlControlPlane => Err(anyhow::anyhow!(
                "csm daemon is owned by the standalone csm runtime binary; use `csm daemon`, not `adl csm daemon`"
            )),
        },
        "service" => match mode {
            CsmDispatchMode::StandaloneRuntime => real_service(&args[1..]),
            CsmDispatchMode::AdlControlPlane => Err(anyhow::anyhow!(
                "csm service is owned by the standalone csm runtime binary; use `csm service`, not `adl csm service`"
            )),
        },
        "governed-stop" => match mode {
            CsmDispatchMode::StandaloneRuntime => real_governed_stop(&args[1..]),
            CsmDispatchMode::AdlControlPlane => Err(anyhow::anyhow!(
                "csm governed-stop is owned by the standalone csm runtime binary; use `csm governed-stop`, not `adl csm governed-stop`"
            )),
        },
        "credential-policy" => match mode {
            CsmDispatchMode::StandaloneRuntime => real_credential_policy(&args[1..]),
            CsmDispatchMode::AdlControlPlane => Err(anyhow::anyhow!(
                "csm credential-policy is owned by the standalone csm runtime binary; use `csm credential-policy`, not `adl csm credential-policy`"
            )),
        },
        "continuity" => match mode {
            CsmDispatchMode::StandaloneRuntime => real_continuity(&args[1..]),
            CsmDispatchMode::AdlControlPlane => Err(anyhow::anyhow!(
                "csm continuity is owned by the standalone csm runtime binary; use `csm continuity`, not `adl csm continuity`"
            )),
        },
        "backpressure" => match mode {
            CsmDispatchMode::StandaloneRuntime => real_backpressure(&args[1..]),
            CsmDispatchMode::AdlControlPlane => Err(anyhow::anyhow!(
                "csm backpressure is owned by the standalone csm runtime binary; use `csm backpressure`, not `adl csm backpressure`"
            )),
        },
        "godel-snapshot" => match mode {
            CsmDispatchMode::StandaloneRuntime => real_godel_snapshot(&args[1..]),
            CsmDispatchMode::AdlControlPlane => Err(anyhow::anyhow!(
                "csm godel-snapshot is owned by the standalone csm runtime binary; use `csm godel-snapshot`, not `adl csm godel-snapshot`"
            )),
        },
        "cav" => match mode {
            CsmDispatchMode::StandaloneRuntime => real_cav(&args[1..]),
            CsmDispatchMode::AdlControlPlane => Err(anyhow::anyhow!(
                "csm cav is owned by the standalone csm runtime binary; use `csm cav`, not `adl csm cav`"
            )),
        },
        "aws-signal" => match mode {
            CsmDispatchMode::StandaloneRuntime => real_aws_signal(&args[1..]),
            CsmDispatchMode::AdlControlPlane => Err(anyhow::anyhow!(
                "csm aws-signal is owned by the standalone csm runtime binary; use `csm aws-signal`, not `adl csm aws-signal`"
            )),
        },
        "storage" => match mode {
            CsmDispatchMode::StandaloneRuntime => real_storage(&args[1..]),
            CsmDispatchMode::AdlControlPlane => Err(anyhow::anyhow!(
                "csm storage is owned by the standalone csm runtime binary; use `csm storage`, not `adl csm storage`"
            )),
        },
        "cloud-control" => match mode {
            CsmDispatchMode::StandaloneRuntime => real_cloud_control(&args[1..]),
            CsmDispatchMode::AdlControlPlane => Err(anyhow::anyhow!(
                "csm cloud-control is owned by the standalone csm runtime binary; use `csm cloud-control`, not `adl csm cloud-control`"
            )),
        },
        "observatory" => real_observatory(&args[1..]),
        "--help" | "-h" => {
            println!("{}", csm_usage());
            Ok(())
        }
        other => {
            eprintln!(
                "unknown csm subcommand: {other} (expected daemon, service, governed-stop, credential-policy, continuity, godel-snapshot, cav, backpressure, aws-signal, storage, cloud-control, or observatory)"
            );
            std::process::exit(2);
        }
    }
}

fn real_cav(args: &[String]) -> Result<()> {
    let Some(cmd) = args.first().map(|value| value.as_str()) else {
        eprintln!("csm cav requires subcommand: red-blue");
        std::process::exit(2);
    };
    match cmd {
        "red-blue" => real_cav_red_blue(&args[1..]),
        "--help" | "-h" => {
            println!("{}", csm_usage());
            Ok(())
        }
        other => {
            eprintln!("unknown csm cav subcommand: {other} (expected red-blue)");
            std::process::exit(2);
        }
    }
}

fn real_cav_red_blue(args: &[String]) -> Result<()> {
    let Some(cmd) = args.first().map(|value| value.as_str()) else {
        eprintln!("csm cav red-blue requires subcommand: prove");
        std::process::exit(2);
    };
    match cmd {
        "prove" => real_cav_red_blue_prove(&args[1..]),
        "--help" | "-h" => {
            println!("{}", csm_usage());
            Ok(())
        }
        other => {
            eprintln!("unknown csm cav red-blue subcommand: {other} (expected prove)");
            std::process::exit(2);
        }
    }
}

fn real_cav_red_blue_prove(args: &[String]) -> Result<()> {
    let mut out_dir: Option<PathBuf> = None;
    let mut run_id = "wp12-4914-cav-red-blue".to_string();
    let mut operator = "local-operator".to_string();
    let mut requested_at: Option<DateTime<Utc>> = None;
    let mut json_output = false;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_dir = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--run-id" => {
                run_id = required_value(args, i, "--run-id")?.to_string();
                i += 1;
            }
            "--operator" => {
                operator = required_value(args, i, "--operator")?.to_string();
                i += 1;
            }
            "--requested-at" => {
                let raw = required_value(args, i, "--requested-at")?;
                requested_at = Some(
                    DateTime::parse_from_rfc3339(raw)
                        .with_context(|| {
                            format!(
                                "csm cav red-blue prove requires --requested-at to be RFC3339, got {raw}"
                            )
                        })?
                        .with_timezone(&Utc),
                );
                i += 1;
            }
            "--json" => json_output = true,
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm cav red-blue prove arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }

    let proof = prove_cav_red_blue(CavRedBlueProofOptions {
        out_dir: out_dir.context("csm cav red-blue prove requires --out <proof-dir>")?,
        run_id,
        operator,
        requested_at,
    })?;
    if json_output {
        println!("{}", serde_json::to_string_pretty(&proof)?);
    } else {
        println!(
            "CSM_CAV_RED_BLUE ok status={} run_id={}",
            proof.status, proof.run_id
        );
    }
    Ok(())
}

fn real_credential_policy(args: &[String]) -> Result<()> {
    let Some(cmd) = args.first().map(|value| value.as_str()) else {
        eprintln!("csm credential-policy requires subcommand: prove");
        std::process::exit(2);
    };
    match cmd {
        "prove" => real_credential_policy_prove(&args[1..]),
        "--help" | "-h" => {
            println!("{}", csm_usage());
            Ok(())
        }
        other => {
            eprintln!("unknown csm credential-policy subcommand: {other} (expected prove)");
            std::process::exit(2);
        }
    }
}

fn real_credential_policy_prove(args: &[String]) -> Result<()> {
    let mut out_dir: Option<PathBuf> = None;
    let mut run_id = "wp12-4920-credential-policy".to_string();
    let mut operator = "local-operator".to_string();
    let mut requested_at: Option<DateTime<Utc>> = None;
    let mut json_output = false;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_dir = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--run-id" => {
                run_id = required_value(args, i, "--run-id")?.to_string();
                i += 1;
            }
            "--operator" => {
                operator = required_value(args, i, "--operator")?.to_string();
                i += 1;
            }
            "--requested-at" => {
                let raw = required_value(args, i, "--requested-at")?;
                requested_at = Some(
                    DateTime::parse_from_rfc3339(raw)
                        .with_context(|| {
                            format!(
                                "csm credential-policy prove requires --requested-at to be RFC3339, got {raw}"
                            )
                        })?
                        .with_timezone(&Utc),
                );
                i += 1;
            }
            "--json" => json_output = true,
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm credential-policy prove arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }

    let proof = prove_credential_policy(CredentialPolicyProofOptions {
        out_dir: out_dir.context("csm credential-policy prove requires --out <proof-dir>")?,
        run_id,
        operator,
        requested_at,
    })?;
    if json_output {
        println!("{}", serde_json::to_string_pretty(&proof)?);
    } else {
        println!(
            "CSM_CREDENTIAL_POLICY ok status={} run_id={}",
            proof.status, proof.run_id
        );
    }
    Ok(())
}

fn real_governed_stop(args: &[String]) -> Result<()> {
    let mut spec: Option<PathBuf> = None;
    let mut reason: Option<String> = None;
    let mut operator_identity: Option<String> = None;
    let mut authorization: Option<String> = None;
    let mut intent: Option<String> = None;
    let mut requested_at: Option<DateTime<Utc>> = None;
    let mut json_output = false;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--spec" => {
                spec = Some(PathBuf::from(required_value(args, i, "--spec")?));
                i += 1;
            }
            "--reason" => {
                reason = Some(required_value(args, i, "--reason")?.to_string());
                i += 1;
            }
            "--operator" => {
                operator_identity = Some(required_value(args, i, "--operator")?.to_string());
                i += 1;
            }
            "--authorization" => {
                authorization = Some(required_value(args, i, "--authorization")?.to_string());
                i += 1;
            }
            "--intent" => {
                intent = Some(required_value(args, i, "--intent")?.to_string());
                i += 1;
            }
            "--requested-at" => {
                let raw = required_value(args, i, "--requested-at")?;
                let parsed = DateTime::parse_from_rfc3339(raw)
                    .with_context(|| {
                        format!(
                            "csm governed-stop requires --requested-at to be RFC3339, got {raw}"
                        )
                    })?
                    .with_timezone(&Utc);
                requested_at = Some(parsed);
                i += 1;
            }
            "--json" => json_output = true,
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm governed-stop arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    let result = governed_stop(
        &spec.context("csm governed-stop requires --spec <agent-spec.yaml>")?,
        GovernedStopRequest {
            reason: reason.context("csm governed-stop requires --reason <text>")?,
            operator_identity: operator_identity
                .context("csm governed-stop requires --operator <identity>")?,
            authorization: authorization
                .context("csm governed-stop requires --authorization <metadata>")?,
            intent: intent.context("csm governed-stop requires --intent <intent>")?,
            requested_at: requested_at
                .context("csm governed-stop requires --requested-at <RFC3339>")?,
        },
    )?;
    if json_output {
        println!("{}", serde_json::to_string(&result)?);
    } else {
        println!(
            "governed stop recorded: {}",
            result["governed_stop_id"].as_str().unwrap_or("unknown")
        );
    }
    Ok(())
}

fn real_cloud_control(args: &[String]) -> Result<()> {
    let Some(cmd) = args.first().map(|value| value.as_str()) else {
        eprintln!("csm cloud-control requires subcommand: cloudfront-status | api-gateway-bridge");
        std::process::exit(2);
    };
    match cmd {
        "api-gateway-bridge" => real_api_gateway_bridge(&args[1..]),
        "cloudfront-status" => real_cloudfront_status(&args[1..]),
        "--help" | "-h" => {
            println!("{}", csm_usage());
            Ok(())
        }
        other => {
            eprintln!("unknown csm cloud-control subcommand: {other} (expected cloudfront-status or api-gateway-bridge)");
            std::process::exit(2);
        }
    }
}

fn real_api_gateway_bridge(args: &[String]) -> Result<()> {
    let mut out_dir: Option<PathBuf> = None;
    let mut run_id = "wp07-5039-api-gateway-bridge".to_string();
    let mut polis_id = std::env::var("ADL_CSM_POLIS_ID").unwrap_or_default();
    let mut profile = std::env::var("ADL_AWS_PROFILE")
        .or_else(|_| std::env::var("AWS_PROFILE"))
        .unwrap_or_else(|_| "agent-logic-admin".to_string());
    let mut region = std::env::var("ADL_AWS_REGION").unwrap_or_else(|_| "us-west-2".to_string());
    let mut expected_account_sha256 =
        std::env::var("ADL_AWS_CSM_API_GATEWAY_ACCOUNT_SHA256").unwrap_or_default();
    let mut api_id = std::env::var("ADL_CSM_API_GATEWAY_API_ID").ok();
    let mut stage_name = std::env::var("ADL_CSM_API_GATEWAY_STAGE").ok();
    let mut invoke_url = std::env::var("ADL_CSM_API_GATEWAY_INVOKE_URL").unwrap_or_default();
    let mut operator_token =
        std::env::var("ADL_CSM_API_GATEWAY_OPERATOR_TOKEN").unwrap_or_default();
    let mut operator_token_file = std::env::var("ADL_CSM_API_GATEWAY_OPERATOR_TOKEN_FILE").ok();
    let mut cloudwatch_log_group =
        std::env::var("ADL_CSM_API_GATEWAY_CLOUDWATCH_LOG_GROUP").unwrap_or_default();
    let mut eventbridge_bus =
        std::env::var("ADL_CSM_API_GATEWAY_EVENTBRIDGE_BUS").unwrap_or_default();
    let mut aws_bin = std::env::var("AWS_BIN").unwrap_or_else(|_| "aws".to_string());
    let mut http_bin = std::env::var("CURL_BIN").unwrap_or_else(|_| "curl".to_string());
    let mut json_output = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_dir = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--run-id" => {
                run_id = required_value(args, i, "--run-id")?.to_string();
                i += 1;
            }
            "--polis-id" => {
                polis_id = required_value(args, i, "--polis-id")?.to_string();
                i += 1;
            }
            "--profile" => {
                profile = required_value(args, i, "--profile")?.to_string();
                i += 1;
            }
            "--region" => {
                region = required_value(args, i, "--region")?.to_string();
                i += 1;
            }
            "--expected-account-sha256" => {
                expected_account_sha256 =
                    required_value(args, i, "--expected-account-sha256")?.to_string();
                i += 1;
            }
            "--api-id" => {
                api_id = Some(required_value(args, i, "--api-id")?.to_string());
                i += 1;
            }
            "--stage" => {
                stage_name = Some(required_value(args, i, "--stage")?.to_string());
                i += 1;
            }
            "--invoke-url" => {
                invoke_url = required_value(args, i, "--invoke-url")?.to_string();
                i += 1;
            }
            "--operator-token" => {
                operator_token = required_value(args, i, "--operator-token")?.to_string();
                i += 1;
            }
            "--operator-token-file" => {
                operator_token_file =
                    Some(required_value(args, i, "--operator-token-file")?.to_string());
                i += 1;
            }
            "--cloudwatch-log-group" => {
                cloudwatch_log_group =
                    required_value(args, i, "--cloudwatch-log-group")?.to_string();
                i += 1;
            }
            "--eventbridge-bus" => {
                eventbridge_bus = required_value(args, i, "--eventbridge-bus")?.to_string();
                i += 1;
            }
            "--aws-bin" => {
                aws_bin = required_value(args, i, "--aws-bin")?.to_string();
                i += 1;
            }
            "--http-bin" => {
                http_bin = required_value(args, i, "--http-bin")?.to_string();
                i += 1;
            }
            "--json" => json_output = true,
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm cloud-control api-gateway-bridge arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }

    let summary = prove_api_gateway_bridge(ApiGatewayBridgeOptions {
        out_dir: out_dir
            .context("csm cloud-control api-gateway-bridge requires --out <proof-dir>")?,
        run_id,
        polis_id,
        profile,
        region,
        expected_account_sha256,
        api_id,
        stage_name,
        invoke_url,
        operator_token: resolve_operator_token(operator_token, operator_token_file)?,
        cloudwatch_log_group,
        eventbridge_bus,
        aws_bin,
        http_bin,
    })?;
    if json_output {
        println!("{}", serde_json::to_string_pretty(&summary)?);
    } else {
        println!(
            "CSM_API_GATEWAY_BRIDGE ok status={} correlation_id={}",
            summary.status, summary.bridge.correlation_id
        );
    }
    Ok(())
}

fn resolve_operator_token(token: String, token_file: Option<String>) -> Result<String> {
    if !token.trim().is_empty() {
        return Ok(token);
    }
    let Some(path) = token_file else {
        return Ok(token);
    };
    Ok(std::fs::read_to_string(&path)
        .with_context(|| format!("read CSM API Gateway operator token file {path}"))?
        .trim()
        .to_string())
}

fn real_cloudfront_status(args: &[String]) -> Result<()> {
    let mut out_dir: Option<PathBuf> = None;
    let mut run_id = "wp08-4915-cloudfront".to_string();
    let mut profile = std::env::var("ADL_AWS_PROFILE")
        .or_else(|_| std::env::var("AWS_PROFILE"))
        .unwrap_or_else(|_| "agent-logic-admin".to_string());
    let mut region = std::env::var("ADL_AWS_REGION").unwrap_or_else(|_| "us-west-2".to_string());
    let mut expected_account_sha256 =
        std::env::var("ADL_AWS_CLOUD_CONTROL_ACCOUNT_SHA256").unwrap_or_default();
    let mut distribution_id = std::env::var("ADL_AWS_CLOUDFRONT_DISTRIBUTION_ID").ok();
    let mut negative_distribution_id = Some(
        std::env::var("ADL_AWS_CLOUDFRONT_NEGATIVE_DISTRIBUTION_ID")
            .unwrap_or_else(|_| "E0000000000000".to_string()),
    );
    let mut aws_bin = std::env::var("AWS_BIN").unwrap_or_else(|_| "aws".to_string());

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_dir = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--run-id" => {
                run_id = required_value(args, i, "--run-id")?.to_string();
                i += 1;
            }
            "--profile" => {
                profile = required_value(args, i, "--profile")?.to_string();
                i += 1;
            }
            "--region" => {
                region = required_value(args, i, "--region")?.to_string();
                i += 1;
            }
            "--expected-account-sha256" => {
                expected_account_sha256 =
                    required_value(args, i, "--expected-account-sha256")?.to_string();
                i += 1;
            }
            "--distribution-id" => {
                distribution_id = Some(required_value(args, i, "--distribution-id")?.to_string());
                i += 1;
            }
            "--negative-distribution-id" => {
                negative_distribution_id =
                    Some(required_value(args, i, "--negative-distribution-id")?.to_string());
                i += 1;
            }
            "--skip-negative-distribution" => {
                negative_distribution_id = None;
            }
            "--aws-bin" => {
                aws_bin = required_value(args, i, "--aws-bin")?.to_string();
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm cloud-control cloudfront-status arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }

    let out_dir =
        out_dir.context("csm cloud-control cloudfront-status requires --out <proof-dir>")?;
    if expected_account_sha256.trim().is_empty() {
        anyhow::bail!(
            "csm cloud-control cloudfront-status requires --expected-account-sha256 or ADL_AWS_CLOUD_CONTROL_ACCOUNT_SHA256"
        );
    }
    let summary = prove_cloudfront_status(CloudFrontStatusOptions {
        out_dir,
        run_id,
        profile,
        region,
        expected_account_sha256,
        distribution_id,
        negative_distribution_id,
        aws_bin,
    })?;
    println!("{}", serde_json::to_string(&summary)?);
    Ok(())
}

fn real_aws_signal(args: &[String]) -> Result<()> {
    let Some(cmd) = args.first().map(|value| value.as_str()) else {
        eprintln!("csm aws-signal requires subcommand: acip-sns-proof");
        std::process::exit(2);
    };
    match cmd {
        "acip-sns-proof" => run_wp08_acip_sns_live_proof(&args[1..]),
        "--help" | "-h" => {
            println!("{}", csm_usage());
            Ok(())
        }
        other => {
            eprintln!("unknown csm aws-signal subcommand: {other} (expected acip-sns-proof)");
            std::process::exit(2);
        }
    }
}

fn real_storage(args: &[String]) -> Result<()> {
    let Some(cmd) = args.first().map(|value| value.as_str()) else {
        eprintln!("csm storage requires subcommand: prove-s3");
        std::process::exit(2);
    };
    match cmd {
        "prove-s3" => real_storage_prove_s3(&args[1..]),
        "--help" | "-h" => {
            println!("{}", csm_usage());
            Ok(())
        }
        other => {
            eprintln!("unknown csm storage subcommand: {other} (expected prove-s3)");
            std::process::exit(2);
        }
    }
}

fn real_storage_prove_s3(args: &[String]) -> Result<()> {
    let mut out_dir: Option<PathBuf> = None;
    let mut bucket: Option<String> = None;
    let mut prefix = "community-memory/".to_string();
    let mut profile = std::env::var("ADL_AWS_PROFILE")
        .or_else(|_| std::env::var("AWS_PROFILE"))
        .unwrap_or_else(|_| "agent-logic-admin".to_string());
    let mut region = std::env::var("ADL_AWS_REGION").unwrap_or_else(|_| "us-west-2".to_string());
    let mut expected_account_sha256 =
        std::env::var("ADL_AWS_POLIS_STORAGE_ACCOUNT_SHA256").unwrap_or_default();
    let mut run_id = "wp08-4913-polis-storage".to_string();
    let mut aws_bin = std::env::var("AWS_BIN").unwrap_or_else(|_| "aws".to_string());
    let mut json_output = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_dir = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--bucket" => {
                bucket = Some(required_value(args, i, "--bucket")?.to_string());
                i += 1;
            }
            "--prefix" => {
                prefix = required_value(args, i, "--prefix")?.to_string();
                i += 1;
            }
            "--profile" => {
                profile = required_value(args, i, "--profile")?.to_string();
                i += 1;
            }
            "--region" => {
                region = required_value(args, i, "--region")?.to_string();
                i += 1;
            }
            "--expected-account-sha256" => {
                expected_account_sha256 =
                    required_value(args, i, "--expected-account-sha256")?.to_string();
                i += 1;
            }
            "--run-id" => {
                run_id = required_value(args, i, "--run-id")?.to_string();
                i += 1;
            }
            "--aws-bin" => {
                aws_bin = required_value(args, i, "--aws-bin")?.to_string();
                i += 1;
            }
            "--json" => json_output = true,
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm storage prove-s3 arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }

    let result = prove_polis_storage(PolisStorageProofOptions {
        out_dir: out_dir.context("csm storage prove-s3 requires --out <proof-dir>")?,
        bucket: bucket.context("csm storage prove-s3 requires --bucket <bucket>")?,
        prefix,
        profile,
        region,
        expected_account_sha256,
        run_id,
        aws_bin,
    })?;
    if json_output {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!(
            "CSM_POLIS_STORAGE ok status={} key={}",
            result.status, result.object.key
        );
    }
    Ok(())
}

fn real_backpressure(args: &[String]) -> Result<()> {
    let Some(cmd) = args.first().map(|value| value.as_str()) else {
        eprintln!("csm backpressure requires subcommand: prove");
        std::process::exit(2);
    };
    match cmd {
        "prove" => real_backpressure_prove(&args[1..]),
        "--help" | "-h" => {
            println!("{}", csm_usage());
            Ok(())
        }
        other => {
            eprintln!("unknown csm backpressure subcommand: {other} (expected prove)");
            std::process::exit(2);
        }
    }
}

fn real_backpressure_prove(args: &[String]) -> Result<()> {
    let mut spec: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut profile = "local".to_string();
    let mut json_output = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--spec" => {
                spec = Some(PathBuf::from(required_value(args, i, "--spec")?));
                i += 1;
            }
            "--out" => {
                out_dir = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--profile" => {
                profile = required_value(args, i, "--profile")?.to_string();
                i += 1;
            }
            "--json" => json_output = true,
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm backpressure prove arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    let result = prove_backpressure(BackpressureProofOptions {
        spec_path: spec.context("csm backpressure prove requires --spec <agent-spec.yaml>")?,
        out_dir: out_dir.context("csm backpressure prove requires --out <proof-dir>")?,
        profile,
    })?;
    if json_output {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!(
            "CSM_BACKPRESSURE ok status={} report={}",
            result.status, result.report_ref
        );
    }
    Ok(())
}

fn real_godel_snapshot(args: &[String]) -> Result<()> {
    let Some(cmd) = args.first().map(|value| value.as_str()) else {
        eprintln!("csm godel-snapshot requires subcommand: proof");
        std::process::exit(2);
    };
    match cmd {
        "proof" => real_godel_snapshot_proof(&args[1..]),
        "--help" | "-h" => {
            println!("{}", csm_usage());
            Ok(())
        }
        other => {
            eprintln!("unknown csm godel-snapshot subcommand: {other} (expected proof)");
            std::process::exit(2);
        }
    }
}

fn real_godel_snapshot_proof(args: &[String]) -> Result<()> {
    let mut spec: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut run_id = "issue-4912-godel-snapshot-diff".to_string();
    let mut json_output = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--spec" => {
                spec = Some(PathBuf::from(required_value(args, i, "--spec")?));
                i += 1;
            }
            "--out" => {
                out_dir = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--run-id" => {
                run_id = required_value(args, i, "--run-id")?.to_string();
                i += 1;
            }
            "--json" => json_output = true,
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm godel-snapshot proof arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    let result = prove_godel_snapshot_diff(GodelSnapshotProofOptions {
        spec_path: spec,
        out_dir: out_dir.context("csm godel-snapshot proof requires --out <proof-dir>")?,
        run_id,
    })?;
    if json_output {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!(
            "CSM_GODEL_SNAPSHOT ok chain={} lkg={}",
            result.positive_case.chain_ref, result.positive_case.last_known_good_ref
        );
    }
    Ok(())
}

fn real_continuity(args: &[String]) -> Result<()> {
    let Some(cmd) = args.first().map(|value| value.as_str()) else {
        eprintln!("csm continuity requires subcommand: capture | stage | restore | drill");
        std::process::exit(2);
    };
    match cmd {
        "capture" => real_continuity_capture(&args[1..]),
        "stage" => real_continuity_stage(&args[1..]),
        "restore" => real_continuity_restore(&args[1..]),
        "drill" => real_continuity_drill(&args[1..]),
        "--help" | "-h" => {
            println!("{}", csm_usage());
            Ok(())
        }
        other => {
            eprintln!(
                "unknown csm continuity subcommand: {other} (expected capture, stage, restore, or drill)"
            );
            std::process::exit(2);
        }
    }
}

fn real_continuity_capture(args: &[String]) -> Result<()> {
    let mut spec: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut source_host = "wuji".to_string();
    let mut target_host = "ec2-staging".to_string();
    let mut json_output = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--spec" => {
                spec = Some(PathBuf::from(required_value(args, i, "--spec")?));
                i += 1;
            }
            "--out" => {
                out_dir = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--source-host" => {
                source_host = required_value(args, i, "--source-host")?.to_string();
                i += 1;
            }
            "--target-host" => {
                target_host = required_value(args, i, "--target-host")?.to_string();
                i += 1;
            }
            "--json" => json_output = true,
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm continuity capture arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    let result = capture_capsule(ContinuityCaptureOptions {
        spec_path: spec.context("csm continuity capture requires --spec <agent-spec.yaml>")?,
        out_dir: out_dir.context("csm continuity capture requires --out <bundle-dir>")?,
        source_host,
        target_host,
    })?;
    print_continuity_result(&result, json_output)
}

fn real_continuity_stage(args: &[String]) -> Result<()> {
    let mut bundle: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut target_host = "ec2-staging".to_string();
    let mut json_output = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--bundle" => {
                bundle = Some(PathBuf::from(required_value(args, i, "--bundle")?));
                i += 1;
            }
            "--out" => {
                out_dir = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--target-host" => {
                target_host = required_value(args, i, "--target-host")?.to_string();
                i += 1;
            }
            "--json" => json_output = true,
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm continuity stage arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    let result = stage_capsule(ContinuityStageOptions {
        bundle_dir: bundle.context("csm continuity stage requires --bundle <bundle-dir>")?,
        out_dir: out_dir.context("csm continuity stage requires --out <stage-dir>")?,
        target_host,
    })?;
    print_continuity_result(&result, json_output)
}

fn real_continuity_restore(args: &[String]) -> Result<()> {
    let mut bundle: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut target_host = "ec2-staging".to_string();
    let mut json_output = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--bundle" => {
                bundle = Some(PathBuf::from(required_value(args, i, "--bundle")?));
                i += 1;
            }
            "--out" => {
                out_dir = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--target-host" => {
                target_host = required_value(args, i, "--target-host")?.to_string();
                i += 1;
            }
            "--json" => json_output = true,
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm continuity restore arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    let result = restore_capsule(ContinuityRestoreOptions {
        bundle_dir: bundle.context("csm continuity restore requires --bundle <bundle-dir>")?,
        out_dir: out_dir.context("csm continuity restore requires --out <runtime-dir>")?,
        target_host,
    })?;
    print_continuity_result(&result, json_output)
}

fn real_continuity_drill(args: &[String]) -> Result<()> {
    let mut bundle: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut target_host = "local".to_string();
    let mut cadence = "manual".to_string();
    let mut json_output = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--bundle" => {
                bundle = Some(PathBuf::from(required_value(args, i, "--bundle")?));
                i += 1;
            }
            "--out" => {
                out_dir = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--target-host" => {
                target_host = required_value(args, i, "--target-host")?.to_string();
                i += 1;
            }
            "--cadence" => {
                cadence = required_value(args, i, "--cadence")?.to_string();
                i += 1;
            }
            "--json" => json_output = true,
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm continuity drill arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    let result = fire_drill_capsule(ContinuityFireDrillOptions {
        bundle_dir: bundle.context("csm continuity drill requires --bundle <bundle-dir>")?,
        out_dir: out_dir.context("csm continuity drill requires --out <drill-dir>")?,
        target_host,
        cadence,
    })?;
    print_continuity_result(&result, json_output)
}

fn required_value<'a>(args: &'a [String], i: usize, flag: &str) -> Result<&'a str> {
    args.get(i + 1)
        .map(|value| value.as_str())
        .ok_or_else(|| anyhow::anyhow!("{flag} requires a value"))
}

fn print_continuity_result(
    result: &::adl::csm_continuity_capsule::ContinuityCommandResult,
    json_output: bool,
) -> Result<()> {
    if json_output {
        println!("{}", serde_json::to_string_pretty(result)?);
    } else {
        println!(
            "CSM_CONTINUITY ok operation={} status={} bundle={}",
            result.operation,
            result.status,
            result.bundle_dir.display()
        );
    }
    Ok(())
}

fn real_observatory(args: &[String]) -> Result<()> {
    let mut packet: Option<PathBuf> = None;
    let mut out_dir = PathBuf::from("out/csm-observatory");
    let mut format = ObservatoryFormat::Bundle;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--packet" => {
                let Some(value) = args.get(i + 1) else {
                    eprintln!("csm observatory requires --packet <visibility-packet.json>");
                    std::process::exit(2);
                };
                packet = Some(PathBuf::from(value));
                i += 1;
            }
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    eprintln!("csm observatory requires --out <dir>");
                    std::process::exit(2);
                };
                out_dir = PathBuf::from(value);
                i += 1;
            }
            "--format" => {
                let Some(value) = args.get(i + 1) else {
                    eprintln!("csm observatory requires --format <bundle|json|report>");
                    std::process::exit(2);
                };
                format = ObservatoryFormat::parse(value)?;
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm observatory arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }

    let packet = packet.context("csm observatory requires --packet <visibility-packet.json>")?;
    let output = write_observatory_outputs(&packet, &out_dir, format)?;

    println!(
        "CSM_OBSERVATORY ok format={format:?} out={}",
        out_dir.display()
    );
    if let Some(path) = output.packet_path {
        println!("  packet={}", path.display());
    }
    if let Some(path) = output.report_path {
        println!("  report={}", path.display());
    }
    if let Some(path) = output.console_reference_path {
        println!("  console_reference={}", path.display());
    }
    if let Some(path) = output.manifest_path {
        println!("  manifest={}", path.display());
    }
    Ok(())
}

pub(crate) fn csm_usage() -> &'static str {
    "Usage:
  csm daemon --spec <agent-spec.yaml> [--checkpoint-interval-secs <n>] [--interval-secs <n>] [--api-bind 127.0.0.1:19997] [--recover-stale-lease] [--no-sleep] [--json]
  csm service install --spec <agent-spec.yaml> [--service-root <dir>] [--manager launchd|local] [--label <label>] [--csm-bin <path>] [--api-bind 127.0.0.1:19997] [--json]
  csm service start|status|stop|remove [--service-root <dir>] [--json]
  csm governed-stop --spec <agent-spec.yaml> --reason <text> --operator <identity> --authorization <metadata> --intent emergency_polis_stop|operator_safety_stop|recoverability_drill --requested-at <RFC3339> [--json]
  csm credential-policy prove --out <proof-dir> [--run-id <id>] [--operator <identity>] [--requested-at <RFC3339>] [--json]
  csm cav red-blue prove --out <proof-dir> [--run-id <id>] [--operator <identity>] [--requested-at <RFC3339>] [--json]
  csm aws-signal acip-sns-proof --out <proof-dir> [--run-id <id>] [--projection-level delivery_metadata|content_summary]
  csm cloud-control cloudfront-status --out <proof-dir> [--profile agent-logic-admin] [--region us-west-2] [--distribution-id <id>] [--expected-account-sha256 <hash>]
  csm cloud-control api-gateway-bridge --out <proof-dir> --polis-id <id> --api-id <id> --stage <name> --invoke-url <url> --operator-token-file <path> --cloudwatch-log-group <name> --eventbridge-bus <name> [--expected-account-sha256 <hash>] [--json]
  csm backpressure prove --spec <agent-spec.yaml> --out <proof-dir> [--profile local|soak2|pre-v0.92] [--json]
  csm godel-snapshot proof --out <proof-dir> [--spec <agent-spec.yaml>] [--run-id <id>] [--json]
  csm storage prove-s3 --out <proof-dir> --bucket <bucket> --expected-account-sha256 <sha256> [--prefix community-memory/] [--profile agent-logic-admin] [--region us-west-2] [--run-id <id>] [--json]
  csm continuity capture --spec <agent-spec.yaml> --out <bundle-dir> [--source-host wuji] [--target-host ec2-staging|ec2|local] [--json]
  csm continuity stage --bundle <bundle-dir> --out <stage-dir> [--target-host ec2-staging|ec2|local] [--json]
  csm continuity restore --bundle <bundle-dir> --out <runtime-dir> [--target-host ec2-staging|ec2|local] [--json]
  csm continuity drill --bundle <bundle-dir> --out <drill-dir> [--target-host local|ec2-staging] [--cadence daily|per-release|pre-v0.92|manual] [--json]
  adl csm observatory --packet <visibility-packet.json> [--format bundle|json|report] [--out <dir>]  # read-only control-plane inspection
  csm observatory --packet <visibility-packet.json> [--format bundle|json|report] [--out <dir>]

Semantics:
  - csm is the dedicated runtime owner binary.
  - csm daemon owns permanent restart-always runtime execution, partial checkpoints, restart accounting telemetry, recoverable terminal state, and runtime observability.
  - csm daemon owns the runtime API as an embedded module in the daemon process; the API is not a separate service process.
  - csm daemon service mode has no cycle-count lifetime boundary; --no-sleep is a test-only bounded harness boundary.
  - csm service owns CSM runtime supervision around csm daemon; local mode is the portable Rust supervisor path, while launchd/systemd metadata are host integration targets.
  - csm governed-stop is the only emergency polis stop path; it requires explicit operator metadata, checkpoints and safe-fail serialization before stop, lifecycle lifelog DB rows, and governed notice fan-out.
  - csm credential-policy proves no-secret credential class inventory, rotation cadence, break-glass audit events, revocation, and failed-closed negative cases for missing, expired, denied, and stale bindings.
  - csm cav red-blue proves bounded red-team fixtures and blue-team detection/response for CSM runtime security surfaces without retaining secrets or performing destructive cloud actions.
  - csm daemon embeds the local-by-default runtime API at --api-bind and exposes /status, /health, /ready, /metrics, /events, /chronosense, /shepherd, /cav, /freedom-gate, /curiosity, /acip, /reasoning, /constructability, /persistence, and /api-gateway-bridge from retained runtime artifacts without leaking host-private paths or secrets.
  - csm daemon defaults its embedded runtime API to listener_role=main_runtime_api on 127.0.0.1:19997; 19950-19999 is reserved for local CSM runtime/dev/test listeners, and 127.0.0.1:0 is accepted only for explicit bounded test harness flags.
  - csm aws-signal owns runtime AWS signal proof execution, including ACIP-to-SNS live publication under the Agent Logic account guard.
  - csm cloud-control owns read-only AWS cloud-control observation hooks, including CloudFront status and governed per-polis API Gateway bridge validation of the CSM runtime API /api-gateway-bridge path under the Agent Logic account guard.
  - csm backpressure proves bounded overload policy, retained metrics, and safe-fail serialization triggers for capacity-degraded runtime paths.
  - csm godel-snapshot proves per-agent versioned snapshot/diff writes, chain validation, recovery-read posture, and negative cases.
  - csm storage proves Polis durable-state write/read/restore semantics against the approved S3 backend with checksum, immutable reference, and negative-case evidence.
  - csm continuity captures, stages, restores, and fire-drills portable continuity capsules with secrets excluded and host bindings explicit.
  - csm daemon emits ADL_OBSERVABILITY_LOG, ADL_OTEL_LOG, and ADL_OTEL_STATUS records through the shared observability contract.
  - Read-only CSM Observatory inspection.
  - Validates the visibility packet before emitting artifacts.
  - bundle writes visibility_packet.json, operator_report.md, console_reference.md, and demo_manifest.json.
  - json writes visibility_packet.json.
  - report writes operator_report.md.
  - No live Runtime v2 mutation is performed."
}

#[cfg(test)]
mod tests {
    use super::{csm_usage, real_csm, real_csm_standalone, required_value, resolve_operator_token};
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_SEQ: AtomicU64 = AtomicU64::new(0);

    fn temp_root(prefix: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "adl-csm-cmd-{prefix}-{}-{}",
            std::process::id(),
            TEMP_SEQ.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&root).expect("create temp root");
        root
    }

    fn write_runtime_spec(root: &Path) -> PathBuf {
        let spec = root.join("agent.yaml");
        fs::write(
            &spec,
            r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: csm-cmd-agent
display_name: CSM Cmd Agent
state_root: state
workflow:
  kind: demo_adapter
  name: csm_cmd_probe
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 1
  stale_lease_after_secs: 60
safety: {}
memory: {}
"#,
        )
        .expect("write spec");
        spec
    }

    fn write_runtime_state(root: &Path) {
        let state = root.join("state");
        fs::create_dir_all(&state).expect("create state");
        for (name, body) in [
            (
                "agent_spec.locked.json",
                r#"{"schema":"adl.long_lived_agent_spec.v1","agent_instance_id":"csm-cmd-agent","display_name":"CSM Cmd Agent","state_root":"state","workflow":{"kind":"demo_adapter","name":"csm_cmd_probe","path":null,"run_args":{}},"heartbeat":{"interval_secs":1,"max_cycles":1,"stale_lease_after_secs":60},"checkpoint":{},"safety":{},"memory":{}}"#,
            ),
            (
                "status.json",
                r#"{"schema":"adl.long_lived_agent_status.v1","agent_instance_id":"csm-cmd-agent","state":"idle","last_cycle_id":"cycle-1","last_cycle_status":"success","completed_cycle_count":1,"consecutive_failure_count":0,"active_lease":null,"stop_requested":false,"last_error":null,"safety_policy":{},"updated_at":"2026-07-07T00:00:00Z"}"#,
            ),
            (
                "daemon_status.json",
                r#"{"schema":"adl.csm.daemon_status.v1","runtime_capabilities":{"scheduler_watcher":{"status":"integrated"},"chronosense":{"status":"integrated","time_sync":{"schema_version":"chronosense_time_sync_status.v1","substrate":"ntpd-rs","health":"synced","reason":"fixture_synced"}},"aee":{"status":"integrated"},"resilience_middleware":{"status":"integrated"}}}"#,
            ),
            ("continuity.json", r#"{"schema":"adl.csm.continuity.v1"}"#),
            (
                "continuity_checkpoint.json",
                r#"{"schema":"adl.csm.continuity_checkpoint.v1","checkpoint_id":"checkpoint-1","agent_state":"idle"}"#,
            ),
            (
                "continuity_replay_manifest.json",
                r#"{"schema":"adl.csm.continuity_replay_manifest.v1","entries":[]}"#,
            ),
            (
                "cycle_ledger.jsonl",
                r#"{"cycle_id":"cycle-1","status":"success"}"#,
            ),
            (
                "memory_index.json",
                r#"{"schema":"adl.csm.memory_index.v1","entries":[]}"#,
            ),
            (
                "provider_binding_history.jsonl",
                r#"{"provider":"local","status":"bound"}"#,
            ),
            ("operator_events.jsonl", r#"{"event":"daemon_started"}"#),
            (
                "safe_fail_bundle.json",
                r#"{"schema":"adl.csm.safe_fail_bundle.v1","runtime_owner":"csm","agent_outcome":{"state":"sleeping"},"recoverability":{"class":"recoverable_sleeping"}}"#,
            ),
        ] {
            fs::write(state.join(name), body).unwrap_or_else(|error| {
                panic!("write runtime fixture {name}: {error}");
            });
        }
    }

    struct EnvVarGuard {
        key: &'static str,
        previous: Option<String>,
    }

    impl EnvVarGuard {
        fn set(key: &'static str, value: &str) -> Self {
            let previous = std::env::var(key).ok();
            std::env::set_var(key, value);
            Self { key, previous }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            match &self.previous {
                Some(value) => std::env::set_var(self.key, value),
                None => std::env::remove_var(self.key),
            }
        }
    }

    fn custody_public_key_from_private_key(private_key_b64: &str) -> String {
        use base64::Engine;

        let key_bytes = base64::engine::general_purpose::STANDARD
            .decode(private_key_b64.as_bytes())
            .expect("decode fixture custody private key");
        let signing = p256::ecdsa::SigningKey::from_slice(&key_bytes)
            .expect("fixture custody private key must be valid P-256");
        base64::engine::general_purpose::STANDARD
            .encode(signing.verifying_key().to_encoded_point(false).as_bytes())
    }

    fn write_executable(path: &Path, body: &str) {
        fs::write(path, body).expect("write executable fixture");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = fs::metadata(path).unwrap().permissions();
            permissions.set_mode(0o755);
            fs::set_permissions(path, permissions).unwrap();
        }
    }

    #[test]
    fn standalone_csm_accepts_aws_signal_help() {
        let args = vec!["aws-signal".to_string(), "--help".to_string()];
        real_csm_standalone(&args).expect("standalone csm owns aws-signal");
    }

    #[test]
    fn standalone_csm_accepts_cloud_control_help() {
        let args = vec!["cloud-control".to_string(), "--help".to_string()];
        real_csm_standalone(&args).expect("standalone csm owns cloud-control");
    }

    #[test]
    fn adl_control_plane_rejects_aws_signal_runtime_surface() {
        let args = vec!["aws-signal".to_string(), "--help".to_string()];
        let error = real_csm(&args).expect_err("adl csm must not own aws-signal runtime surface");
        assert!(
            error.to_string().contains("standalone csm runtime binary"),
            "{error}"
        );
    }

    #[test]
    fn standalone_csm_accepts_storage_help() {
        let args = vec!["storage".to_string(), "--help".to_string()];
        real_csm_standalone(&args).expect("standalone csm owns storage");
    }

    #[test]
    fn adl_control_plane_rejects_storage_runtime_surface() {
        let args = vec!["storage".to_string(), "--help".to_string()];
        let error = real_csm(&args).expect_err("adl csm must not own storage runtime surface");
        assert!(
            error.to_string().contains("standalone csm runtime binary"),
            "{error}"
        );
    }

    #[test]
    fn adl_control_plane_rejects_cloud_control_runtime_surface() {
        let args = vec!["cloud-control".to_string(), "--help".to_string()];
        let error =
            real_csm(&args).expect_err("adl csm must not own cloud-control runtime surface");
        assert!(
            error.to_string().contains("standalone csm runtime binary"),
            "{error}"
        );
    }

    #[test]
    fn standalone_csm_help_paths_cover_runtime_owned_surfaces() {
        for subcommand in [
            "backpressure",
            "credential-policy",
            "continuity",
            "cav",
            "governed-stop",
            "observatory",
            "service",
        ] {
            let args = vec![subcommand.to_string(), "--help".to_string()];
            real_csm_standalone(&args)
                .unwrap_or_else(|error| panic!("standalone csm {subcommand} help failed: {error}"));
        }
    }

    #[test]
    fn adl_control_plane_rejects_runtime_owned_surfaces() {
        for subcommand in [
            "daemon",
            "service",
            "governed-stop",
            "credential-policy",
            "continuity",
            "cav",
            "backpressure",
        ] {
            let args = vec![subcommand.to_string(), "--help".to_string()];
            let error = real_csm(&args)
                .err()
                .unwrap_or_else(|| panic!("adl csm unexpectedly accepted {subcommand}"));
            assert!(
                error.to_string().contains("standalone csm runtime binary"),
                "{subcommand}: {error}"
            );
        }
    }

    #[test]
    fn csm_usage_documents_permanent_runtime_without_public_budgets() {
        let usage = csm_usage();
        assert!(usage.contains("csm is the dedicated runtime owner binary"));
        assert!(usage.contains("permanent restart-always runtime execution"));
        assert!(usage.contains("csm governed-stop --spec"));
        assert!(usage.contains("only emergency polis stop path"));
        assert!(usage.contains("csm credential-policy prove --out"));
        assert!(usage.contains("csm cav red-blue prove --out"));
        assert!(usage.contains("break-glass audit events"));
        assert!(usage.contains("ADL_OBSERVABILITY_LOG"));
        assert!(usage.contains("ADL_OTEL_STATUS"));
        assert!(!usage.contains("csm api serve"));
        assert!(usage.contains("/chronosense"));
        assert!(!usage.contains("--max-restarts"));
        assert!(!usage.contains("--max-requests"));
        assert!(!usage.contains("--once"));
        assert!(!usage.contains("--idle-timeout-ms"));
    }

    #[test]
    fn required_value_reports_missing_cli_value() {
        let args = vec!["--spec".to_string()];
        let error = required_value(&args, 0, "--spec").expect_err("missing value must fail");
        assert_eq!(error.to_string(), "--spec requires a value");
    }

    #[test]
    fn governed_stop_parser_fails_closed_without_required_metadata() {
        let root = temp_root("governed-stop-missing");
        let spec = write_runtime_spec(&root);
        let args = vec![
            "governed-stop".to_string(),
            "--spec".to_string(),
            spec.display().to_string(),
            "--reason".to_string(),
            "operator safety".to_string(),
            "--json".to_string(),
        ];
        let error = real_csm_standalone(&args).expect_err("missing operator metadata must fail");
        assert!(
            error
                .to_string()
                .contains("csm governed-stop requires --operator"),
            "{error}"
        );
        assert!(!root.join("state/governed_stop.json").exists());
        assert!(!root.join("state/stop.json").exists());
    }

    #[test]
    fn standalone_csm_executes_local_runtime_parser_paths() {
        let root = temp_root("local-runtime-paths");
        let spec = write_runtime_spec(&root);
        write_runtime_state(&root);

        let godel_snapshot = vec![
            "godel-snapshot".to_string(),
            "proof".to_string(),
            "--out".to_string(),
            root.join("godel-snapshot").display().to_string(),
            "--run-id".to_string(),
            "csm-cmd-godel-snapshot-proof".to_string(),
            "--json".to_string(),
        ];
        real_csm_standalone(&godel_snapshot).expect("godel snapshot proof parser path");
        assert!(root
            .join("godel-snapshot")
            .join("godel_snapshot_diff_proof.json")
            .exists());

        let backpressure = vec![
            "backpressure".to_string(),
            "prove".to_string(),
            "--spec".to_string(),
            spec.display().to_string(),
            "--out".to_string(),
            root.join("backpressure").display().to_string(),
            "--profile".to_string(),
            "soak2".to_string(),
            "--json".to_string(),
        ];
        real_csm_standalone(&backpressure).expect("backpressure proof parser path");

        let credential_policy = vec![
            "credential-policy".to_string(),
            "prove".to_string(),
            "--out".to_string(),
            root.join("credential-policy").display().to_string(),
            "--run-id".to_string(),
            "wp12-4920-parser-proof".to_string(),
            "--operator".to_string(),
            "local-operator".to_string(),
            "--requested-at".to_string(),
            "2026-07-10T00:00:00Z".to_string(),
            "--json".to_string(),
        ];
        real_csm_standalone(&credential_policy).expect("credential policy parser path");
        assert!(root
            .join("credential-policy")
            .join("credential_policy_summary.json")
            .exists());

        let cav_red_blue = vec![
            "cav".to_string(),
            "red-blue".to_string(),
            "prove".to_string(),
            "--out".to_string(),
            root.join("cav-red-blue").display().to_string(),
            "--run-id".to_string(),
            "wp12-4914-parser-proof".to_string(),
            "--operator".to_string(),
            "local-operator".to_string(),
            "--requested-at".to_string(),
            "2026-07-10T00:00:00Z".to_string(),
            "--json".to_string(),
        ];
        real_csm_standalone(&cav_red_blue).expect("cav red-blue proof parser path");
        assert!(root
            .join("cav-red-blue")
            .join("cav_red_blue_summary.json")
            .exists());

        let custody_p256_signing_private_key = "CQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQk=";
        let custody_trusted_public_key =
            custody_public_key_from_private_key(custody_p256_signing_private_key);
        let _custody_private_key = EnvVarGuard::set(
            "ADL_CSM_CUSTODY_P256_SIGNING_PRIVATE_KEY_B64",
            custody_p256_signing_private_key,
        );
        let _custody_key_id =
            EnvVarGuard::set("ADL_CSM_CUSTODY_SIGNING_KEY_ID", "test-csm-cmd-custody-key");
        let _custody_public_key = EnvVarGuard::set(
            "ADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64",
            &custody_trusted_public_key,
        );

        let bundle = root.join("bundle");
        let capture = vec![
            "continuity".to_string(),
            "capture".to_string(),
            "--spec".to_string(),
            spec.display().to_string(),
            "--out".to_string(),
            bundle.display().to_string(),
            "--source-host".to_string(),
            "wuji".to_string(),
            "--target-host".to_string(),
            "ec2-staging".to_string(),
            "--json".to_string(),
        ];
        real_csm_standalone(&capture).expect("continuity capture parser path");

        let stage = vec![
            "continuity".to_string(),
            "stage".to_string(),
            "--bundle".to_string(),
            bundle.display().to_string(),
            "--out".to_string(),
            root.join("stage").display().to_string(),
            "--target-host".to_string(),
            "local".to_string(),
            "--json".to_string(),
        ];
        real_csm_standalone(&stage).expect("continuity stage parser path");

        let restore = vec![
            "continuity".to_string(),
            "restore".to_string(),
            "--bundle".to_string(),
            bundle.display().to_string(),
            "--out".to_string(),
            root.join("restore").display().to_string(),
            "--target-host".to_string(),
            "local".to_string(),
            "--json".to_string(),
        ];
        real_csm_standalone(&restore).expect("continuity restore parser path");

        let drill = vec![
            "continuity".to_string(),
            "drill".to_string(),
            "--bundle".to_string(),
            bundle.display().to_string(),
            "--out".to_string(),
            root.join("drill").display().to_string(),
            "--target-host".to_string(),
            "local".to_string(),
            "--cadence".to_string(),
            "manual".to_string(),
            "--json".to_string(),
        ];
        real_csm_standalone(&drill).expect("continuity drill parser path");
    }

    #[test]
    fn standalone_csm_fail_closed_parsers_cover_required_runtime_inputs() {
        let root = temp_root("fail-closed-parsers");
        let cases = [
            (
                vec!["credential-policy".to_string(), "prove".to_string()],
                "csm credential-policy prove requires --out <proof-dir>",
            ),
            (
                vec![
                    "cav".to_string(),
                    "red-blue".to_string(),
                    "prove".to_string(),
                ],
                "csm cav red-blue prove requires --out <proof-dir>",
            ),
            (
                vec!["storage".to_string(), "prove-s3".to_string()],
                "csm storage prove-s3 requires --out <proof-dir>",
            ),
            (
                vec![
                    "cloud-control".to_string(),
                    "cloudfront-status".to_string(),
                    "--out".to_string(),
                    root.join("cloudfront").display().to_string(),
                ],
                "requires --expected-account-sha256",
            ),
            (
                vec!["aws-signal".to_string(), "acip-sns-proof".to_string()],
                "csm aws-signal acip-sns-proof requires --out <dir>",
            ),
            (
                vec!["observatory".to_string()],
                "csm observatory requires --packet <visibility-packet.json>",
            ),
        ];

        for (args, expected) in cases {
            let error = real_csm_standalone(&args)
                .err()
                .unwrap_or_else(|| panic!("expected csm args to fail closed: {args:?}"));
            assert!(
                error.to_string().contains(expected),
                "expected {expected:?} in {error}"
            );
        }
    }

    #[test]
    fn standalone_csm_fail_closed_parsers_cover_runtime_flag_matrices() {
        let root = temp_root("flag-matrices");
        let storage = vec![
            "storage".to_string(),
            "prove-s3".to_string(),
            "--out".to_string(),
            root.join("storage").display().to_string(),
            "--bucket".to_string(),
            "adl-test-bucket".to_string(),
            "--prefix".to_string(),
            "community-memory/wp-07/".to_string(),
            "--profile".to_string(),
            "agent-logic-admin".to_string(),
            "--region".to_string(),
            "us-west-2".to_string(),
            "--expected-account-sha256".to_string(),
            "not-a-sha256".to_string(),
            "--run-id".to_string(),
            "wp07-4998-parser-proof".to_string(),
            "--aws-bin".to_string(),
            "aws".to_string(),
            "--json".to_string(),
        ];
        let storage_error =
            real_csm_standalone(&storage).expect_err("invalid hash fails before AWS");
        assert!(
            storage_error
                .to_string()
                .contains("expected-account-sha256"),
            "{storage_error}"
        );

        let cloudfront = vec![
            "cloud-control".to_string(),
            "cloudfront-status".to_string(),
            "--out".to_string(),
            root.join("cloudfront").display().to_string(),
            "--run-id".to_string(),
            "wp07-4998-cloudfront-parser".to_string(),
            "--profile".to_string(),
            "agent-logic-admin".to_string(),
            "--region".to_string(),
            "us-west-2".to_string(),
            "--distribution-id".to_string(),
            "EDFDVBD632BHDS5".to_string(),
            "--negative-distribution-id".to_string(),
            "E0000000000000".to_string(),
            "--skip-negative-distribution".to_string(),
            "--aws-bin".to_string(),
            "aws".to_string(),
        ];
        let cloudfront_error = real_csm_standalone(&cloudfront)
            .expect_err("missing expected account hash fails closed");
        assert!(
            cloudfront_error
                .to_string()
                .contains("requires --expected-account-sha256"),
            "{cloudfront_error}"
        );

        let acip = vec![
            "aws-signal".to_string(),
            "acip-sns-proof".to_string(),
            "--out".to_string(),
            root.join("acip").display().to_string(),
            "--run-id".to_string(),
            "wp07-4998-acip-parser".to_string(),
            "--projection-level".to_string(),
            "delivery_metadata".to_string(),
        ];
        let acip_error =
            real_csm_standalone(&acip).expect_err("unconfigured SNS proof must fail closed");
        assert!(
            acip_error
                .to_string()
                .contains("ACIP SNS live proof did not publish live"),
            "{acip_error}"
        );
        assert!(root.join("acip").join("acip_sns_summary.json").exists());
    }

    #[test]
    fn standalone_csm_api_gateway_bridge_parser_covers_all_runtime_flags() {
        let root = temp_root("api-gateway-parser");
        let aws = root.join("aws");
        write_executable(
            &aws,
            r#"#!/usr/bin/env bash
set -euo pipefail
if [ "$1 $2" = "sts get-caller-identity" ]; then
  printf '%s\n' '123456789012'
  exit 0
fi
echo "unexpected aws args: $*" >&2
exit 2
"#,
        );
        let args = vec![
            "cloud-control",
            "api-gateway-bridge",
            "--out",
            root.join("proof").to_str().unwrap(),
            "--run-id",
            "wp07-5122-cli-parser",
            "--polis-id",
            "polis-5122",
            "--profile",
            "agent-logic-admin",
            "--region",
            "us-west-2",
            "--expected-account-sha256",
            &"0".repeat(64),
            "--api-id",
            "api-5122",
            "--stage",
            "prod",
            "--invoke-url",
            "https://example.invalid/prod",
            "--operator-token",
            "bounded-test-token",
            "--cloudwatch-log-group",
            "/aws/apigateway/adl-csm",
            "--eventbridge-bus",
            "adl-csm-bus",
            "--aws-bin",
            aws.to_str().unwrap(),
            "--http-bin",
            "curl",
            "--json",
        ]
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
        let error = real_csm_standalone(&args)
            .expect_err("account guard must reject the bounded fake account");
        assert!(
            error
                .to_string()
                .contains("approved Agent Logic account hash"),
            "{error}"
        );
    }

    #[test]
    fn csm_operator_token_resolution_covers_direct_file_and_empty_inputs() {
        assert_eq!(
            resolve_operator_token("direct-token".to_string(), None).unwrap(),
            "direct-token"
        );
        assert_eq!(resolve_operator_token(String::new(), None).unwrap(), "");
        let root = temp_root("operator-token-file");
        let token_file = root.join("operator.token");
        fs::write(&token_file, " file-token\n").unwrap();
        assert_eq!(
            resolve_operator_token(
                String::new(),
                Some(token_file.to_string_lossy().into_owned())
            )
            .unwrap(),
            "file-token"
        );
    }

    #[test]
    fn standalone_csm_observatory_parser_writes_bundle_outputs() {
        let root = temp_root("observatory-parser");
        let packet = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/runtime_v2/observatory/visibility_packet.json");
        let out = root.join("observatory");
        let args = vec![
            "observatory".to_string(),
            "--packet".to_string(),
            packet.to_string_lossy().into_owned(),
            "--out".to_string(),
            out.to_string_lossy().into_owned(),
            "--format".to_string(),
            "bundle".to_string(),
        ];
        real_csm_standalone(&args).expect("observatory parser writes bundle");
        assert!(out.join("visibility_packet.json").exists());
        assert!(out.join("operator_report.md").exists());
        assert!(out.join("console_reference.md").exists());
        assert!(out.join("demo_manifest.json").exists());
    }
}
