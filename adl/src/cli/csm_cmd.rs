use anyhow::{Context, Result};
use std::path::PathBuf;

use super::agent_cmd::real_csm_daemon;
use super::csm_service_cmd::real_service;
use ::adl::csm_observatory::{write_observatory_outputs, ObservatoryFormat};

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
        eprintln!("csm requires subcommand: daemon | observatory");
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
        "observatory" => real_observatory(&args[1..]),
        "--help" | "-h" => {
            println!("{}", csm_usage());
            Ok(())
        }
        other => {
            eprintln!("unknown csm subcommand: {other} (expected daemon, service, or observatory)");
            std::process::exit(2);
        }
    }
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
  csm daemon --spec <agent-spec.yaml> [--max-restarts <n>] [--checkpoint-interval-secs <n>] [--interval-secs <n>] [--recover-stale-lease] [--no-sleep] [--json]
  csm service install --spec <agent-spec.yaml> [--service-root <dir>] [--manager launchd|local] [--label <label>] [--csm-bin <path>] [--json]
  csm service start|status|stop|remove [--service-root <dir>] [--json]
  adl csm observatory --packet <visibility-packet.json> [--format bundle|json|report] [--out <dir>]  # read-only control-plane inspection
  csm observatory --packet <visibility-packet.json> [--format bundle|json|report] [--out <dir>]

Semantics:
  - csm is the dedicated runtime owner binary.
  - csm daemon owns long-lived runtime execution, partial checkpoints, restart accounting, recoverable terminal state, and runtime observability.
  - csm service owns host service-manager installation/status around csm daemon; launchd is the primary macOS target and local mode is a bounded proof fallback.
  - csm daemon emits ADL_OBSERVABILITY_LOG, ADL_OTEL_LOG, and ADL_OTEL_STATUS records through the shared observability contract.
  - Read-only CSM Observatory inspection.
  - Validates the visibility packet before emitting artifacts.
  - bundle writes visibility_packet.json, operator_report.md, console_reference.md, and demo_manifest.json.
  - json writes visibility_packet.json.
  - report writes operator_report.md.
  - No live Runtime v2 mutation is performed."
}
