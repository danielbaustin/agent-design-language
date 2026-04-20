use anyhow::{Context, Result};
use std::path::PathBuf;

use ::adl::csm_observatory::{write_observatory_outputs, ObservatoryFormat};

pub(crate) fn real_csm(args: &[String]) -> Result<()> {
    let Some(cmd) = args.first().map(|value| value.as_str()) else {
        eprintln!("csm requires subcommand: observatory");
        std::process::exit(2);
    };

    match cmd {
        "observatory" => real_observatory(&args[1..]),
        "--help" | "-h" => {
            println!("{}", csm_usage());
            Ok(())
        }
        other => {
            eprintln!("unknown csm subcommand: {other}");
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

fn csm_usage() -> &'static str {
    "Usage:
  adl csm observatory --packet <visibility-packet.json> [--format bundle|json|report] [--out <dir>]

Semantics:
  - Read-only CSM Observatory inspection.
  - Validates the visibility packet before emitting artifacts.
  - bundle writes visibility_packet.json, operator_report.md, console_reference.md, and demo_manifest.json.
  - json writes visibility_packet.json.
  - report writes operator_report.md.
  - No live Runtime v2 mutation is performed."
}
