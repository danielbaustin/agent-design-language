use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::PathBuf;

use ::adl::scheduler::{parse_economics_bundle_json, schedule_economics_bundle};

pub(crate) fn real_scheduler(args: &[String]) -> Result<()> {
    let Some(subcommand) = args.first().map(|arg| arg.as_str()) else {
        return Err(anyhow!("scheduler requires a subcommand: plan"));
    };

    match subcommand {
        "plan" => real_scheduler_plan(&args[1..]),
        "--help" | "-h" | "help" => {
            println!("{}", super::usage::usage());
            Ok(())
        }
        other => Err(anyhow!(
            "unknown scheduler subcommand '{other}' (expected plan)"
        )),
    }
}

fn real_scheduler_plan(args: &[String]) -> Result<()> {
    let mut input: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut compact = false;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--input" => {
                input = Some(PathBuf::from(required_value(args, i, "--input")?));
                i += 1;
            }
            "--out" => {
                out = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--json" => compact = true,
            "--help" | "-h" => {
                println!("{}", super::usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for scheduler plan: {other}")),
        }
        i += 1;
    }

    let input = input.ok_or_else(|| anyhow!("scheduler plan requires --input <bundle.json>"))?;
    let raw = fs::read_to_string(&input)
        .with_context(|| format!("failed to read scheduler input {}", input.display()))?;
    let bundle = parse_economics_bundle_json(&raw)
        .with_context(|| format!("failed to parse scheduler economics bundle {}", input.display()))?;
    let plan = schedule_economics_bundle(&bundle)
        .context("failed to build scheduler plan from economics bundle")?;
    let rendered = if compact {
        serde_json::to_string(&plan).context("serialize scheduler plan")?
    } else {
        serde_json::to_string_pretty(&plan).context("serialize scheduler plan")?
    } + "\n";

    if let Some(out) = out {
        if let Some(parent) = out.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        fs::write(&out, rendered.as_bytes())
            .with_context(|| format!("failed to write scheduler plan {}", out.display()))?;
        println!("SCHEDULER_PLAN_PATH={}", out.display());
        return Ok(());
    }

    print!("{rendered}");
    Ok(())
}

fn required_value<'a>(args: &'a [String], index: usize, flag: &str) -> Result<&'a str> {
    args.get(index + 1)
        .map(String::as_str)
        .filter(|value| !value.starts_with("--"))
        .ok_or_else(|| anyhow!("{flag} requires a value"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(label: &str) -> PathBuf {
        let unique = format!(
            "{}-{}-{}",
            label,
            std::process::id(),
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
        );
        let dir = std::env::temp_dir().join(unique);
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    #[test]
    fn scheduler_plan_writes_artifact_for_valid_fixture() {
        let root = temp_dir("scheduler-cli");
        let out = root.join("plan.json");
        let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/scheduler/economics_inputs_v1.json");
        real_scheduler(&[
            "plan".to_string(),
            "--input".to_string(),
            fixture.display().to_string(),
            "--out".to_string(),
            out.display().to_string(),
        ])
        .expect("scheduler plan should succeed");

        let written = fs::read_to_string(&out).expect("read plan");
        assert!(written.contains("\"schema_version\": \"adl.scheduler.plan.v1\""));
        assert!(written.contains("\"recommended_order\""));

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn scheduler_plan_rejects_malformed_bundle() {
        let root = temp_dir("scheduler-cli-bad");
        let input = root.join("bad.json");
        fs::write(
            &input,
            r#"{"schema_version":"adl.scheduler.economics_input_bundle.v1","source_doc_ref":"","included_concepts":[],"deferred_concepts":[],"inputs":[]}"#,
        )
        .expect("write malformed bundle");

        let err = real_scheduler(&[
            "plan".to_string(),
            "--input".to_string(),
            input.display().to_string(),
        ])
        .expect_err("malformed bundle must fail");
        assert!(err
            .to_string()
            .contains("failed to parse scheduler economics bundle"));

        let _ = fs::remove_dir_all(&root);
    }
}
