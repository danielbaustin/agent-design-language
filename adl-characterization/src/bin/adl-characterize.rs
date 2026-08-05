use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use adl_characterization::{
    capture_corpus, load_corpus, load_shadow_manifest, run_shadow, verify_corpus, ShadowInputs,
};

#[derive(Debug, Parser)]
#[command(
    name = "adl-characterize",
    version,
    about = "Independent ADL v1 characterization harness"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Capture {
        #[arg(long)]
        binary: PathBuf,
        #[arg(long)]
        corpus: PathBuf,
        #[arg(long)]
        observations: PathBuf,
        #[arg(long)]
        report: Option<PathBuf>,
    },
    Verify {
        #[arg(long)]
        corpus: PathBuf,
        #[arg(long)]
        observations: PathBuf,
        #[arg(long)]
        report: Option<PathBuf>,
    },
    Shadow {
        #[arg(long)]
        binary: PathBuf,
        #[arg(long)]
        lockfile: PathBuf,
        #[arg(long)]
        install_receipt: PathBuf,
        #[arg(long)]
        selector: PathBuf,
        #[arg(long)]
        repo_root: PathBuf,
        #[arg(long)]
        receipt_root: PathBuf,
        #[arg(long)]
        runtime_plan: PathBuf,
        #[arg(long)]
        corpus: PathBuf,
        #[arg(long)]
        observations: PathBuf,
        #[arg(long)]
        work_root: PathBuf,
        #[arg(long)]
        manifest: PathBuf,
        #[arg(long)]
        report: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Capture {
            binary,
            corpus,
            observations,
            report,
        } => {
            let manifest = load_corpus(&corpus)?;
            let result = capture_corpus(&binary, &corpus, &manifest, &observations)?;
            write_report(report.as_ref(), &result)?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Command::Verify {
            corpus,
            observations,
            report,
        } => {
            let manifest = load_corpus(&corpus)?;
            let result = verify_corpus(&corpus, &manifest, &observations)?;
            write_report(report.as_ref(), &result)?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Command::Shadow {
            binary,
            lockfile,
            install_receipt,
            selector,
            repo_root,
            receipt_root,
            runtime_plan,
            corpus,
            observations,
            work_root,
            manifest,
            report,
        } => {
            let manifest = load_shadow_manifest(&manifest)?;
            let result = run_shadow(
                &ShadowInputs {
                    binary: &binary,
                    lockfile: &lockfile,
                    install_receipt: &install_receipt,
                    selector: &selector,
                    repo_root: &repo_root,
                    receipt_root: &receipt_root,
                    runtime_plan: &runtime_plan,
                    corpus_path: &corpus,
                    observations: &observations,
                    work_root: &work_root,
                },
                &manifest,
            )?;
            write_json_report(&report, &result)?;
            println!("{}", serde_json::to_string_pretty(&result)?);
            if result.status != "pass" {
                anyhow::bail!("shadow parity contains blocker dispositions");
            }
        }
    }
    Ok(())
}

fn write_report(
    path: Option<&PathBuf>,
    report: &adl_characterization::VerificationReport,
) -> Result<()> {
    if let Some(path) = path {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut bytes = serde_json::to_vec_pretty(report)?;
        bytes.push(b'\n');
        std::fs::write(path, bytes)?;
    }
    Ok(())
}

fn write_json_report(path: &PathBuf, report: &adl_characterization::ShadowReport) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut bytes = serde_json::to_vec_pretty(report)?;
    bytes.push(b'\n');
    std::fs::write(path, bytes)?;
    Ok(())
}
