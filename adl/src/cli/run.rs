use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use ::adl::{adl, artifacts, execute, obsmem_demo, overlay, prompt, resolve, signing, trace};

use super::open::{open_artifact, select_open_artifact, RealCommandRunner};
use super::run_artifacts::{
    load_pause_state_artifact, load_resume_state, load_steering_patch,
    resume_state_path_for_run_id, validate_pause_artifact_basic,
    validate_pause_artifact_for_resume, write_run_state_artifacts,
};
use super::{resume_usage, usage};

pub(crate) fn run_workflow(args: &[String]) -> Result<()> {
    let adl_path: PathBuf = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            eprintln!("missing ADL yaml path");
            eprintln!("Try: adl examples/v0-3-concurrency-fork-join.adl.yaml --print-plan");
            eprintln!("{}", usage());
            std::process::exit(2);
        }
    };

    let mut print_plan = false;
    let mut print_prompts = false;
    let mut do_trace = false;
    let mut do_run = false;
    let mut out_dir = PathBuf::from("out");
    let mut quiet = false;
    let mut do_open = false;
    let mut allow_unsigned = false;
    let mut resume_path: Option<PathBuf> = None;
    let mut steer_path: Option<PathBuf> = None;
    let mut overlay_path: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        let a = &args[i];
        match a.as_str() {
            "--print-plan" => print_plan = true,
            "--print-prompts" | "--print-prompt" => print_prompts = true,
            "--trace" => do_trace = true,
            "--run" => do_run = true,
            "--resume" => {
                let Some(path) = args.get(i + 1) else {
                    eprintln!("--resume requires a run.json path");
                    eprintln!("{}", usage());
                    std::process::exit(2);
                };
                resume_path = Some(PathBuf::from(path));
                i += 1;
            }
            "--steer" => {
                let Some(path) = args.get(i + 1) else {
                    eprintln!("--steer requires a path to steering patch JSON");
                    eprintln!("{}", usage());
                    std::process::exit(2);
                };
                steer_path = Some(PathBuf::from(path));
                i += 1;
            }
            "--overlay" => {
                let Some(path) = args.get(i + 1) else {
                    eprintln!("--overlay requires a path to overlay.json");
                    eprintln!("{}", usage());
                    std::process::exit(2);
                };
                overlay_path = Some(PathBuf::from(path));
                i += 1;
            }
            "--out" => {
                let Some(dir) = args.get(i + 1) else {
                    eprintln!("--out requires a directory path");
                    eprintln!("{}", usage());
                    std::process::exit(2);
                };
                out_dir = PathBuf::from(dir);
                i += 1;
            }
            "--quiet" | "--no-step-output" => quiet = true,
            "--open" | "--open-artifacts" => do_open = true,
            "--allow-unsigned" => allow_unsigned = true,
            "--help" | "-h" => {
                println!("{}", usage());
                return Ok(());
            }
            _ => {
                eprintln!("Unknown arg: {a}");
                eprintln!("Run 'adl --help' for usage.");
                eprintln!("{}", usage());
                std::process::exit(2);
            }
        }
        i += 1;
    }

    let adl_path_str = adl_path.to_str().context("ADL path must be valid UTF-8")?;
    let adl_base_dir: PathBuf = adl_path.parent().unwrap_or(Path::new(".")).to_path_buf();

    if steer_path.is_some() && resume_path.is_none() {
        return Err(anyhow::anyhow!(
            "--steer requires --resume <run.json> so steering is only applied at a checkpoint boundary"
        ));
    }

    let mut doc = match adl::AdlDoc::load_from_file(adl_path_str)
        .with_context(|| format!("failed to load ADL document: {adl_path_str}"))
    {
        Ok(doc) => doc,
        Err(err) => {
            if do_trace {
                let mut tr = trace::Trace::new("unknown", "unknown", "unknown");
                tr.run_failed(&err.to_string());
                trace::print_trace(&tr);
            }
            return Err(err);
        }
    };

    let applied_overlay = if let Some(path) = overlay_path.as_deref() {
        let spec = overlay::load_overlay(path)?;
        Some(overlay::apply_overlay_to_doc(&mut doc, &spec)?)
    } else {
        None
    };

    let allow_unsigned = allow_unsigned
        || std::env::var("ADL_ALLOW_UNSIGNED")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
    enforce_signature_policy(&doc, do_run, allow_unsigned)?;

    let resolved = match resolve::resolve_run(&doc) {
        Ok(resolved) => resolved,
        Err(err) => {
            if do_trace {
                let run_id = doc.run.name.clone().unwrap_or_else(|| "run".to_string());
                let mut tr = trace::Trace::new(run_id, "workflow", doc.version.clone());
                tr.run_failed(&err.to_string());
                trace::print_trace(&tr);
            }
            return Err(err);
        }
    };

    if let Some(applied) = applied_overlay.as_ref() {
        persist_overlay_audit(&resolved.run_id, applied, overlay_path.as_deref())?;
    }

    let mode_requested = print_plan || print_prompts || do_trace || do_run;
    if !mode_requested {
        match doc.version.trim() {
            "0.1" => print_plan = true,
            _ => do_run = true,
        }
    }

    if print_plan {
        resolve::print_resolved_plan(&resolved);
    }

    if print_prompts {
        prompt::print_prompts(&resolved)?;
    }

    if do_run {
        let print_outputs = !quiet;
        let run_started_ms = now_ms();
        let mut tr = trace::Trace::new(
            resolved.run_id.clone(),
            resolved.workflow_id.clone(),
            resolved.doc.version.clone(),
        );
        if !quiet {
            eprintln!(
                "RUN start {} run_id={} workflow={}",
                trace::format_iso_utc_ms(tr.current_ts_ms()),
                resolved.run_id,
                resolved.workflow_id
            );
        }

        let resume_state = match resume_path.as_deref() {
            Some(path) => {
                let mut resume = load_resume_state(path, &resolved)?;
                if let Some(steer_path) = steer_path.as_deref() {
                    let (patch, fingerprint) = load_steering_patch(steer_path)?;
                    let record = execute::apply_steering_patch(&mut resume, &patch, fingerprint)?;
                    if !quiet {
                        eprintln!(
                            "STEER apply sequence={} apply_at={} keys_set={} keys_removed={}",
                            record.sequence,
                            record.apply_at,
                            record.set_state_keys.len(),
                            record.removed_state_keys.len()
                        );
                    }
                }
                Some(resume)
            }
            None => None,
        };
        let resume_completed_ids = resume_state.as_ref().map(|r| r.completed_step_ids.clone());
        let steering_history = resume_state
            .as_ref()
            .map(|r| r.steering_history.clone())
            .unwrap_or_default();

        let result = execute::execute_sequential_with_resume(
            &resolved,
            &mut tr,
            print_outputs,
            !quiet,
            &adl_base_dir,
            &out_dir,
            resume_state,
        );
        let result = match result {
            Ok(result) => result,
            Err(err) => {
                let run_finished_ms = now_ms();
                let run_dir = write_run_state_artifacts(
                    &resolved,
                    &tr,
                    &adl_path,
                    &out_dir,
                    run_started_ms,
                    run_finished_ms,
                    "failure",
                    None,
                    &steering_history,
                    &execute::derive_runtime_control_state("failure", &[], &tr),
                    resume_completed_ids.as_ref(),
                    Some(&err),
                )?;
                if !quiet {
                    eprintln!(
                        "RUN done (+{}ms) fail artifacts={}",
                        tr.current_elapsed_ms(),
                        run_dir.display()
                    );
                }
                if resolved.doc.version.trim() == "0.2" {
                    tr.run_finished(false);
                }
                if do_trace {
                    trace::print_trace(&tr);
                }
                return Err(err);
            }
        };
        let _outputs = result.outputs;
        let artifacts = result.artifacts;
        let records = result.records;
        let pause_state = result.pause;
        let run_finished_ms = now_ms();
        let status = if pause_state.is_some() {
            "paused"
        } else {
            "success"
        };
        let run_dir = write_run_state_artifacts(
            &resolved,
            &tr,
            &adl_path,
            &out_dir,
            run_started_ms,
            run_finished_ms,
            status,
            pause_state.as_ref(),
            &result.steering_history,
            &result.runtime_control,
            resume_completed_ids.as_ref(),
            None,
        )?;
        if !quiet {
            let status_label = if pause_state.is_some() {
                "paused"
            } else {
                "ok"
            };
            eprintln!(
                "RUN done (+{}ms) {} artifacts={}",
                tr.current_elapsed_ms(),
                status_label,
                run_dir.display()
            );
        }

        if pause_state.is_none() {
            if let Some(paths) = obsmem_demo::maybe_emit_obsmem_demo_artifacts(&resolved.run_id)? {
                if !quiet {
                    eprintln!(
                        "OBSMEM artifacts index={} query={}",
                        paths.index_summary.display(),
                        paths.query_result.display()
                    );
                }
            }
        }

        println!("RUN SUMMARY: {} step(s)", records.len());
        if let Some((max_concurrency, source)) = execute::scheduler_policy_for_run(&resolved)? {
            println!(
                "SCHEDULER POLICY: max_concurrency={} source={}",
                max_concurrency,
                source.as_str()
            );
        }
        for r in &records {
            println!(
                "  step={} provider={} status={} attempts={} bytes={}",
                r.step_id, r.provider_id, r.status, r.attempts, r.output_bytes
            );
        }

        if do_trace {
            if resolved.doc.version.trim() == "0.2" {
                tr.run_finished(pause_state.is_none());
            }
            trace::print_trace(&tr);
        }

        if pause_state.is_none() && do_open {
            if let Some(path) = select_open_artifact(&artifacts) {
                let runner = RealCommandRunner;
                open_artifact(&runner, &path)?;
                println!("OPEN path={}", path.display());
            }
        }
    } else if do_trace {
        let mut tr = trace::Trace::new(
            resolved.run_id.clone(),
            resolved.workflow_id.clone(),
            resolved.doc.version.clone(),
        );

        for step in &resolved.steps {
            let step_id = step.id.as_str();
            let agent_id = step.agent.as_deref().unwrap_or("<unresolved-agent>");
            let task_id = step.task.as_deref().unwrap_or("<unresolved-task>");
            let provider_id = step.provider.as_deref().unwrap_or("<unresolved-provider>");

            tr.step_started(step_id, agent_id, provider_id, task_id, None);

            if let Some(p) = step.effective_prompt_with_defaults(&resolved) {
                let inputs: HashMap<String, String> = step
                    .inputs
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();

                let prompt_text = prompt::trace_prompt_assembly(&p, &inputs);
                let hash = prompt::hash_prompt(&prompt_text);
                tr.prompt_assembled(step_id, &hash);
            }

            tr.step_finished(step_id, true);
        }

        if resolved.doc.version.trim() == "0.2" {
            tr.run_finished(true);
        }

        trace::print_trace(&tr);
    }

    Ok(())
}

pub(crate) fn real_resume(args: &[String]) -> Result<()> {
    if matches!(args.first().map(|s| s.as_str()), Some("--help" | "-h")) {
        println!("{}", resume_usage());
        return Ok(());
    }

    let Some(run_id) = args.first() else {
        eprintln!("resume requires <run_id>");
        eprintln!("{}", resume_usage());
        std::process::exit(2);
    };
    let mut adl_path: Option<PathBuf> = None;
    let mut steer_path: Option<PathBuf> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--adl" => {
                let Some(path) = args.get(i + 1) else {
                    eprintln!("resume --adl requires a path to the ADL document");
                    eprintln!("{}", resume_usage());
                    std::process::exit(2);
                };
                adl_path = Some(PathBuf::from(path));
                i += 1;
            }
            "--steer" => {
                let Some(path) = args.get(i + 1) else {
                    eprintln!("resume --steer requires a path to steering patch JSON");
                    eprintln!("{}", resume_usage());
                    std::process::exit(2);
                };
                steer_path = Some(PathBuf::from(path));
                i += 1;
            }
            other => {
                if args.len() > 1
                    && args
                        .iter()
                        .skip(1)
                        .all(|arg| arg != "--adl" && arg != "--steer")
                {
                    eprintln!("resume requires <run_id> plus --adl <path>");
                } else {
                    eprintln!(
                        "resume accepts <run_id> --adl <path> [--steer <steering.json>] (unknown arg: {other})"
                    );
                }
                eprintln!("{}", resume_usage());
                std::process::exit(2);
            }
        }
        i += 1;
    }
    let pause_path = resume_state_path_for_run_id(run_id)?;
    if !pause_path.exists() {
        return Err(anyhow::anyhow!(
            "pause state not found for run_id '{}': expected '{}'",
            run_id,
            pause_path.display()
        ));
    }
    let Some(adl_path) = adl_path else {
        eprintln!(
            "resume requires --adl <path> so the ADL document comes from a trusted explicit source"
        );
        eprintln!("{}", resume_usage());
        std::process::exit(2);
    };
    let pause_artifact = load_pause_state_artifact(&pause_path)?;
    validate_pause_artifact_basic(&pause_artifact, run_id)?;

    let adl_path_str = adl_path
        .to_str()
        .context("resume ADL path must be valid UTF-8")?;
    let adl_base_dir: PathBuf = adl_path.parent().unwrap_or(Path::new(".")).to_path_buf();
    let doc = adl::AdlDoc::load_from_file(adl_path_str).with_context(|| {
        format!(
            "failed to load ADL document for resume: {}",
            adl_path.display()
        )
    })?;

    let allow_unsigned = std::env::var("ADL_ALLOW_UNSIGNED")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    enforce_signature_policy(&doc, true, allow_unsigned)?;

    let resolved = resolve::resolve_run(&doc)?;
    validate_pause_artifact_for_resume(&pause_artifact, run_id, &resolved)?;

    let mut resume_state = execute::ResumeState {
        completed_step_ids: pause_artifact
            .pause
            .completed_step_ids
            .into_iter()
            .collect(),
        saved_state: pause_artifact.pause.saved_state,
        completed_outputs: pause_artifact.pause.completed_outputs,
        steering_history: pause_artifact.steering_history,
    };
    if let Some(steer_path) = steer_path.as_deref() {
        let (patch, fingerprint) = load_steering_patch(steer_path)?;
        let record = execute::apply_steering_patch(&mut resume_state, &patch, fingerprint)?;
        eprintln!(
            "STEER apply sequence={} apply_at={} keys_set={} keys_removed={}",
            record.sequence,
            record.apply_at,
            record.set_state_keys.len(),
            record.removed_state_keys.len()
        );
    }
    let resume_completed_ids = resume_state.completed_step_ids.clone();

    let out_dir = PathBuf::from("out");
    let run_started_ms = now_ms();
    let mut tr = trace::Trace::new(
        resolved.run_id.clone(),
        resolved.workflow_id.clone(),
        resolved.doc.version.clone(),
    );
    eprintln!(
        "RUN resume {} run_id={} workflow={}",
        trace::format_iso_utc_ms(tr.current_ts_ms()),
        resolved.run_id,
        resolved.workflow_id
    );

    let result = execute::execute_sequential_with_resume(
        &resolved,
        &mut tr,
        true,
        true,
        &adl_base_dir,
        &out_dir,
        Some(resume_state),
    )?;
    let run_finished_ms = now_ms();
    let run_dir = write_run_state_artifacts(
        &resolved,
        &tr,
        &adl_path,
        &out_dir,
        run_started_ms,
        run_finished_ms,
        if result.pause.is_some() {
            "paused"
        } else {
            "success"
        },
        result.pause.as_ref(),
        &result.steering_history,
        &result.runtime_control,
        Some(&resume_completed_ids),
        None,
    )?;
    eprintln!(
        "RUN done (+{}ms) {} artifacts={}",
        tr.current_elapsed_ms(),
        if result.pause.is_some() {
            "paused"
        } else {
            "ok"
        },
        run_dir.display()
    );
    println!("RUN SUMMARY: {} step(s)", result.records.len());
    if let Some((max_concurrency, source)) = execute::scheduler_policy_for_run(&resolved)? {
        println!(
            "SCHEDULER POLICY: max_concurrency={} source={}",
            max_concurrency,
            source.as_str()
        );
    }
    for r in &result.records {
        println!(
            "  step={} provider={} status={} attempts={} bytes={}",
            r.step_id, r.provider_id, r.status, r.attempts, r.output_bytes
        );
    }
    Ok(())
}

pub(crate) fn persist_overlay_audit(
    run_id: &str,
    applied: &overlay::AppliedOverlayAudit,
    overlay_path: Option<&Path>,
) -> Result<()> {
    let run_paths = artifacts::RunArtifactPaths::for_run(run_id)?;
    run_paths.ensure_layout()?;
    run_paths.write_model_marker()?;

    let mut audit = applied.clone();
    if let Some(path) = overlay_path {
        audit.source_path = path.display().to_string();
    }
    let audit_bytes =
        serde_json::to_vec_pretty(&audit).context("serialize applied overlay audit")?;
    artifacts::atomic_write(
        &run_paths.overlays_dir().join("applied_overlay.json"),
        &audit_bytes,
    )?;

    if let Some(path) = overlay_path {
        let raw = std::fs::read(path)
            .with_context(|| format!("read overlay source '{}'", path.display()))?;
        artifacts::atomic_write(&run_paths.overlays_dir().join("source_overlay.json"), &raw)?;
    }
    Ok(())
}

pub(crate) fn enforce_signature_policy(
    doc: &adl::AdlDoc,
    do_run: bool,
    allow_unsigned: bool,
) -> Result<()> {
    if do_run && doc.version.trim() == "0.5" && !allow_unsigned {
        signing::verify_doc(doc, None)
            .with_context(|| "signature enforcement failed (use --allow-unsigned for dev)")?;
    }
    Ok(())
}

pub(crate) fn now_ms() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}
