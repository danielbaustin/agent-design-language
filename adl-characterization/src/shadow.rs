use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use wait_timeout::ChildExt;

use crate::compare::verify_corpus;
use crate::manifest::{corpus_bundle_sha256, load_corpus};
use crate::model::{
    CommandObservation, Corpus, IntentionalDifference, NormalizedObservation, PreAction,
    ShadowCase, ShadowDisposition, ShadowManifest, SHADOW_MANIFEST_SCHEMA, SHADOW_REPORT_SCHEMA,
};
use crate::normalize::normalize;
use crate::runner::binary_sha256;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShadowReport {
    pub schema: String,
    pub candidate_revision: String,
    pub candidate_binary_sha256: String,
    pub candidate_lock_sha256: String,
    pub candidate_install_receipt_sha256: String,
    pub candidate_selector_generation: String,
    pub candidate_selector_sha256: String,
    pub corpus_bundle_sha256: String,
    pub case_count: usize,
    pub behavior_count: usize,
    pub equivalence_group_count: usize,
    pub difference_group_count: usize,
    pub disposition_counts: BTreeMap<String, usize>,
    pub rows: Vec<ShadowRow>,
    pub behaviors: Vec<ShadowBehavior>,
    pub groups: Vec<ShadowGroup>,
    pub runtime_overlay: Vec<EvidenceOverlay>,
    pub adapter_overlay: Vec<EvidenceOverlay>,
    pub wp10a_overlay: Vec<EvidenceOverlay>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShadowRow {
    pub case_id: String,
    pub behaviors: Vec<String>,
    pub disposition: ShadowDisposition,
    pub incumbent_observation_count: usize,
    pub candidate_observation_count: usize,
    pub incumbent_normalized_sha256: String,
    pub candidate_normalized_sha256: String,
    pub candidate_commands: Vec<CommandObservation>,
    pub decision: Option<IntentionalDifference>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShadowBehavior {
    pub behavior: String,
    pub cases: Vec<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShadowGroup {
    pub id: String,
    pub kind: String,
    pub cases: Vec<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceOverlay {
    pub issue: u64,
    pub groups: Vec<u64>,
    pub disposition: String,
    pub observed_sha: Option<String>,
    pub integrated_sha: Option<String>,
    pub receipt_sha256: String,
    pub reviewed_revision: Option<String>,
    pub reviewer: Option<String>,
    pub evidence_sha256: Option<String>,
    pub status: String,
}

pub struct ShadowInputs<'a> {
    pub binary: &'a Path,
    pub lockfile: &'a Path,
    pub install_receipt: &'a Path,
    pub selector: &'a Path,
    pub repo_root: &'a Path,
    pub receipt_root: &'a Path,
    pub runtime_plan: &'a Path,
    pub corpus_path: &'a Path,
    pub observations: &'a Path,
    pub work_root: &'a Path,
}

pub fn load_shadow_manifest(path: &Path) -> Result<ShadowManifest> {
    let bytes = fs::read(path).with_context(|| format!("read {}", path.display()))?;
    let manifest: ShadowManifest =
        serde_yaml::from_slice(&bytes).with_context(|| format!("parse {}", path.display()))?;
    if manifest.schema != SHADOW_MANIFEST_SCHEMA {
        bail!("unsupported shadow manifest schema {}", manifest.schema);
    }
    Ok(manifest)
}

pub fn run_shadow(inputs: &ShadowInputs<'_>, manifest: &ShadowManifest) -> Result<ShadowReport> {
    let binary = inputs
        .binary
        .canonicalize()
        .with_context(|| format!("resolve candidate binary {}", inputs.binary.display()))?;
    let corpus = load_corpus(inputs.corpus_path)?;
    verify_corpus(inputs.corpus_path, &corpus, inputs.observations)?;
    verify_identity(
        &binary,
        inputs.lockfile,
        inputs.install_receipt,
        inputs.selector,
        manifest,
    )?;
    verify_manifest(&corpus, manifest)?;

    let root = inputs
        .corpus_path
        .parent()
        .unwrap_or_else(|| Path::new("."));
    let mut rows = Vec::with_capacity(corpus.cases.len());
    let mut candidate_by_case = BTreeMap::new();
    for source_case in &corpus.cases {
        let shadow_case = manifest
            .cases
            .iter()
            .find(|candidate| candidate.id == source_case.id)
            .expect("manifest inventory verified");
        let mut candidate_observations = Vec::new();
        let mut normalized_observations = Vec::new();
        for _ in 1..=corpus.repetitions {
            let observation = run_candidate_case(
                &binary,
                root,
                inputs.work_root,
                shadow_case,
                corpus.command_timeout_ms,
            )?;
            normalized_observations.push(normalize_candidate(&observation, shadow_case)?);
            candidate_observations.push(observation);
        }
        let first_normalized = normalized_observations
            .first()
            .expect("nonzero repetitions");
        if normalized_observations
            .iter()
            .skip(1)
            .any(|value| value.commands != first_normalized.commands)
        {
            bail!(
                "candidate case {} has repeated-run divergence",
                source_case.id
            );
        }
        let candidate_normalized_sha256 = digest(&serde_json::to_vec(&normalized_observations)?);
        let incumbent_normalized_sha256 =
            incumbent_case_digest(inputs.observations, &source_case.id, corpus.repetitions)?;
        candidate_by_case.insert(source_case.id.clone(), first_normalized.clone());
        rows.push(ShadowRow {
            case_id: source_case.id.clone(),
            behaviors: source_case.behaviors.clone(),
            disposition: shadow_case.disposition,
            incumbent_observation_count: corpus.repetitions as usize,
            candidate_observation_count: corpus.repetitions as usize,
            incumbent_normalized_sha256,
            candidate_normalized_sha256,
            candidate_commands: candidate_observations
                .into_iter()
                .next()
                .expect("nonzero repetitions"),
            decision: shadow_case.decision_ref.as_ref().map(|key| {
                manifest
                    .decisions
                    .get(key)
                    .expect("manifest verified")
                    .clone()
            }),
        });
    }

    let groups = verify_groups(&corpus, &candidate_by_case, manifest)?;
    let behaviors = behavior_rows(&corpus, manifest);
    let (runtime_overlay, adapter_overlay, wp10a_overlay) = verify_overlays(
        inputs.repo_root,
        inputs.receipt_root,
        inputs.runtime_plan,
        &manifest.candidate_revision,
    )?;
    let mut disposition_counts = BTreeMap::new();
    for row in &rows {
        *disposition_counts
            .entry(disposition_name(row.disposition).to_owned())
            .or_insert(0) += 1;
    }
    let blocking = rows.iter().any(|row| row.disposition.is_blocking())
        || groups.iter().any(|group| group.status == "blocker");
    Ok(ShadowReport {
        schema: SHADOW_REPORT_SCHEMA.into(),
        candidate_revision: manifest.candidate_revision.clone(),
        candidate_binary_sha256: manifest.candidate_binary_sha256.clone(),
        candidate_lock_sha256: manifest.candidate_lock_sha256.clone(),
        candidate_install_receipt_sha256: manifest.candidate_install_receipt_sha256.clone(),
        candidate_selector_generation: manifest.candidate_selector_generation.clone(),
        candidate_selector_sha256: manifest.candidate_selector_sha256.clone(),
        corpus_bundle_sha256: corpus_bundle_sha256(inputs.corpus_path)?,
        case_count: rows.len(),
        behavior_count: corpus.required_behaviors.len(),
        equivalence_group_count: corpus.equivalence_groups.len(),
        difference_group_count: corpus.difference_groups.len(),
        disposition_counts,
        rows,
        behaviors,
        groups,
        runtime_overlay,
        adapter_overlay,
        wp10a_overlay,
        status: if blocking { "block" } else { "pass" }.into(),
    })
}

fn verify_overlays(
    repo_root: &Path,
    receipt_root: &Path,
    runtime_plan: &Path,
    candidate_revision: &str,
) -> Result<(
    Vec<EvidenceOverlay>,
    Vec<EvidenceOverlay>,
    Vec<EvidenceOverlay>,
)> {
    let plan: serde_json::Value = serde_json::from_slice(&fs::read(runtime_plan)?)?;
    let groups = plan
        .get("proof_groups")
        .or_else(|| plan.get("groups"))
        .and_then(serde_json::Value::as_array)
        .or_else(|| plan.as_array())
        .ok_or_else(|| anyhow::anyhow!("runtime parity plan lacks proof-group array"))?;
    let mut runtime_owners = BTreeMap::<u64, Vec<u64>>::new();
    for group in groups {
        let id = group
            .get("id")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| anyhow::anyhow!("runtime proof group lacks numeric id"))?;
        let owner = group
            .get("owner_issue")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| anyhow::anyhow!("runtime proof group lacks owner_issue"))?;
        runtime_owners.entry(owner).or_default().push(id);
    }
    if runtime_owners
        .values()
        .flatten()
        .copied()
        .collect::<BTreeSet<_>>()
        != (1_u64..=10).collect()
    {
        bail!("runtime parity plan must cover proof groups 1 through 10 exactly");
    }

    let runtime = runtime_owners
        .into_iter()
        .map(|(issue, groups)| {
            receipt_overlay(
                repo_root,
                receipt_root,
                candidate_revision,
                issue,
                groups,
                None,
            )
        })
        .collect::<Result<Vec<_>>>()?;
    let adapters = [5341_u64, 5349]
        .into_iter()
        .map(|issue| {
            receipt_overlay(
                repo_root,
                receipt_root,
                candidate_revision,
                issue,
                vec![],
                None,
            )
        })
        .collect::<Result<Vec<_>>>()?;
    let live_evidence = [
        "admitted-plan.json",
        "convergence-decision.json",
        "dependency-ancestry.json",
        "live-run-manifest.json",
        "negative-case-refusal.json",
        "retained-live-proof-review.json",
        "retained-live-proof.json",
        "single-agent-comparison.json",
    ];
    let live_digest = evidence_digest(repo_root, 5501, &live_evidence)?;
    let wp10a = vec![
        receipt_overlay(
            repo_root,
            receipt_root,
            candidate_revision,
            5497,
            vec![],
            None,
        )?,
        receipt_overlay(
            repo_root,
            receipt_root,
            candidate_revision,
            5501,
            vec![],
            Some(live_digest),
        )?,
    ];
    Ok((runtime, adapters, wp10a))
}

fn receipt_overlay(
    repo_root: &Path,
    receipt_root: &Path,
    candidate_revision: &str,
    issue: u64,
    groups: Vec<u64>,
    evidence_sha256: Option<String>,
) -> Result<EvidenceOverlay> {
    let path = receipt_root.join(format!("{issue}.json"));
    let bytes = fs::read(&path).with_context(|| format!("read receipt for issue {issue}"))?;
    let value: serde_json::Value = serde_json::from_slice(&bytes)?;
    if value.get("issue").and_then(serde_json::Value::as_u64) != Some(issue)
        || value
            .pointer("/record/phase")
            .and_then(serde_json::Value::as_str)
            != Some("closed_out")
        || !value
            .pointer("/record/review/completed")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
        || value
            .pointer("/record/review/findings")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|findings| {
                findings.iter().any(|finding| {
                    finding
                        .get("actionable")
                        .and_then(serde_json::Value::as_bool)
                        .unwrap_or(true)
                        && finding
                            .get("disposition")
                            .and_then(serde_json::Value::as_str)
                            != Some("fixed")
                })
            })
    {
        bail!("issue {issue} receipt lacks clean terminal review truth");
    }
    let disposition = value
        .pointer("/record/terminal/disposition")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("issue {issue} receipt lacks terminal disposition"))?
        .to_owned();
    let observed_sha = value
        .pointer("/record/terminal/observed_sha")
        .and_then(serde_json::Value::as_str)
        .filter(|sha| !sha.is_empty())
        .map(str::to_owned);
    let integrated_sha = if disposition == "merged" {
        let sha = observed_sha
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("issue {issue} merged receipt lacks observed SHA"))?;
        let pull_request = value
            .pointer("/record/terminal/pull_request")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| anyhow::anyhow!("issue {issue} receipt lacks pull request"))?;
        Some(resolve_integration(
            repo_root,
            candidate_revision,
            sha,
            pull_request,
        )?)
    } else if issue != 5497 || disposition != "closed_no_pr" {
        bail!("issue {issue} has unsupported terminal disposition {disposition}");
    } else {
        None
    };
    Ok(EvidenceOverlay {
        issue,
        groups,
        disposition,
        observed_sha,
        integrated_sha,
        receipt_sha256: digest(&bytes),
        reviewed_revision: value
            .pointer("/record/review/reviewed_revision")
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned),
        reviewer: value
            .pointer("/record/review/reviewer")
            .and_then(serde_json::Value::as_str)
            .map(portable_reviewer),
        evidence_sha256,
        status: "pass".into(),
    })
}

fn resolve_integration(
    repo_root: &Path,
    candidate_revision: &str,
    observed_sha: &str,
    pull_request: u64,
) -> Result<String> {
    if Command::new("git")
        .args([
            "merge-base",
            "--is-ancestor",
            observed_sha,
            candidate_revision,
        ])
        .current_dir(repo_root)
        .status()?
        .success()
    {
        return Ok(observed_sha.to_owned());
    }
    let output = Command::new("git")
        .args(["log", "--format=%H%x00%s", candidate_revision])
        .current_dir(repo_root)
        .output()?;
    if !output.status.success() {
        bail!("cannot inspect candidate integration history");
    }
    let suffix = format!("(#{pull_request})");
    for line in String::from_utf8(output.stdout)?.lines() {
        let Some((sha, subject)) = line.split_once('\0') else {
            continue;
        };
        if subject.ends_with(&suffix) {
            return Ok(sha.to_owned());
        }
    }
    bail!(
        "reviewed head {} and PR #{} integration are absent from candidate history",
        observed_sha,
        pull_request
    )
}

fn evidence_digest(repo_root: &Path, issue: u64, names: &[&str]) -> Result<String> {
    let root = repo_root.join(format!(".csdlc/evidence/{issue}"));
    let mut hasher = Sha256::new();
    for name in names {
        let path = root.join(name);
        let bytes =
            fs::read(&path).with_context(|| format!("read live evidence {}", path.display()))?;
        hasher.update((name.len() as u64).to_be_bytes());
        hasher.update(name.as_bytes());
        hasher.update((bytes.len() as u64).to_be_bytes());
        hasher.update(bytes);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn verify_identity(
    binary: &Path,
    lockfile: &Path,
    install_receipt: &Path,
    selector: &Path,
    manifest: &ShadowManifest,
) -> Result<()> {
    let actual_binary = binary_sha256(binary)?;
    if actual_binary != manifest.candidate_binary_sha256 {
        bail!("candidate executable digest mismatch");
    }
    let actual_lock = format!("{:x}", Sha256::digest(fs::read(lockfile)?));
    if actual_lock != manifest.candidate_lock_sha256 {
        bail!("candidate lockfile digest mismatch");
    }
    let receipt_bytes = fs::read(install_receipt)?;
    if digest(&receipt_bytes) != manifest.candidate_install_receipt_sha256 {
        bail!("candidate install receipt digest mismatch");
    }
    let receipt: serde_json::Value = serde_json::from_slice(&receipt_bytes)?;
    if receipt.get("schema").and_then(serde_json::Value::as_str) != Some("adl.install.receipt.v1")
        || receipt.get("binary").and_then(serde_json::Value::as_str)
            != Some(manifest.candidate_selector_generation.as_str())
        || receipt.get("sha256").and_then(serde_json::Value::as_str)
            != Some(manifest.candidate_binary_sha256.as_str())
    {
        bail!("candidate install receipt identity mismatch");
    }
    let selector_bytes = fs::read(selector)?;
    if digest(&selector_bytes) != manifest.candidate_selector_sha256 {
        bail!("candidate selector digest mismatch");
    }
    let selector: serde_json::Value = serde_json::from_slice(&selector_bytes)?;
    if selector.get("schema").and_then(serde_json::Value::as_str) != Some("adl.selector.v1")
        || selector
            .pointer("/current/generation")
            .and_then(serde_json::Value::as_str)
            != Some(manifest.candidate_selector_generation.as_str())
        || selector
            .pointer("/current/digest")
            .and_then(serde_json::Value::as_str)
            != Some(manifest.candidate_binary_sha256.as_str())
    {
        bail!("candidate selector identity mismatch");
    }
    if manifest.candidate_revision.len() != 40
        || !manifest
            .candidate_revision
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        bail!("candidate revision must be an exact 40-character Git SHA");
    }
    Ok(())
}

fn verify_manifest(corpus: &Corpus, manifest: &ShadowManifest) -> Result<()> {
    let expected = corpus
        .cases
        .iter()
        .map(|case| case.id.as_str())
        .collect::<BTreeSet<_>>();
    let actual = manifest
        .cases
        .iter()
        .map(|case| case.id.as_str())
        .collect::<BTreeSet<_>>();
    if expected != actual || actual.len() != manifest.cases.len() {
        bail!("shadow manifest must classify every corpus case exactly once");
    }
    for case in &manifest.cases {
        if case.steps.is_empty() {
            bail!("shadow case {} has no executable steps", case.id);
        }
        for step in &case.steps {
            if !local_command_shape(&step.args) {
                bail!(
                    "shadow case {} step {} has a forbidden command shape",
                    case.id,
                    step.id
                );
            }
        }
        match case.disposition {
            ShadowDisposition::ApprovedIntentionalDifference if case.decision_ref.is_none() => {
                bail!(
                    "shadow case {} lacks intentional-difference authority",
                    case.id
                )
            }
            ShadowDisposition::ApprovedIntentionalDifference => {
                let key = case.decision_ref.as_ref().expect("checked");
                let decision = manifest.decisions.get(key).ok_or_else(|| {
                    anyhow::anyhow!(
                        "shadow case {} references unknown decision {}",
                        case.id,
                        key
                    )
                })?;
                if decision.owner_issue == 0
                    || [
                        &decision.authority,
                        &decision.rationale,
                        &decision.replacement_proof,
                        &decision.risk,
                        &decision.reviewer,
                        &decision.rollback_impact,
                    ]
                    .iter()
                    .any(|value| value.trim().is_empty())
                {
                    bail!(
                        "shadow case {} has incomplete difference authority",
                        case.id
                    );
                }
            }
            _ if case.decision_ref.is_some() => {
                bail!(
                    "shadow case {} has an unexpected difference decision",
                    case.id
                )
            }
            _ => {}
        }
    }
    Ok(())
}

fn local_command_shape(args: &[String]) -> bool {
    match args {
        [flag] => matches!(
            flag.as_str(),
            "--help" | "--version" | "--definitely-invalid"
        ),
        [command, path, format]
            if matches!(command.as_str(), "plan" | "validate")
                && format == "--yaml"
                && path.starts_with("{ROOT}/fixtures/")
                && path.ends_with(".adl.yaml")
                && !path.contains("..")
                && !path.contains("://") =>
        {
            true
        }
        [command, path, format] => {
            command == "run" && path == "{ROOT}/fixtures/mock-run.adl.yaml" && format == "--yaml"
        }
        [command, path, flag_a, value_a, flag_b, value_b] => match command.as_str() {
            "sign" => {
                path == "{ROOT}/../v2/event-record.json"
                    && flag_a == "--key-id"
                    && value_a == "characterization-fixed"
                    && flag_b == "--key-hex"
                    && value_b.len() == 64
                    && value_b.bytes().all(|byte| byte.is_ascii_hexdigit())
            }
            "verify" => {
                path == "{WORK}/signed.json"
                    && flag_a == "--public-key-hex"
                    && value_a.len() == 64
                    && value_a.bytes().all(|byte| byte.is_ascii_hexdigit())
                    && flag_b == "--logical-time"
                    && value_b.parse::<u64>().is_ok()
            }
            _ => false,
        },
        _ => false,
    }
}

fn portable_reviewer(reviewer: &str) -> String {
    let Some((kind, path)) = reviewer.split_once(":/") else {
        return reviewer.to_owned();
    };
    let identity = Path::new(path)
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or("redacted");
    format!("{kind}:{identity}")
}

fn run_candidate_case(
    binary: &Path,
    root: &Path,
    work_root: &Path,
    case: &ShadowCase,
    timeout_ms: u64,
) -> Result<Vec<CommandObservation>> {
    fs::create_dir_all(work_root)
        .with_context(|| format!("create shadow work root {}", work_root.display()))?;
    let work_root = work_root
        .canonicalize()
        .with_context(|| format!("resolve shadow work root {}", work_root.display()))?;
    let temp = tempfile::Builder::new()
        .prefix("adl-shadow-")
        .tempdir_in(&work_root)?;
    let work = temp.path().canonicalize()?;
    let root = root.canonicalize()?;
    let mut commands = Vec::new();
    for step in &case.steps {
        for action in &step.pre_actions {
            apply_action(action, &work)?;
        }
        let args = step
            .args
            .iter()
            .map(|arg| expand(arg, &root, &work))
            .collect::<Result<Vec<_>>>()?;
        let mut child = Command::new(binary)
            .args(&args)
            .current_dir(&work)
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("HOME", &work)
            .env("TMPDIR", &work)
            .env("NO_PROXY", "*")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| format!("execute candidate case {} step {}", case.id, step.id))?;
        let status = match child.wait_timeout(Duration::from_millis(timeout_ms))? {
            Some(status) => status,
            None => {
                child.kill()?;
                child.wait()?;
                bail!("candidate case {} step {} timed out", case.id, step.id);
            }
        };
        let mut stdout_bytes = Vec::new();
        let mut stderr_bytes = Vec::new();
        child
            .stdout
            .take()
            .expect("piped")
            .read_to_end(&mut stdout_bytes)?;
        child
            .stderr
            .take()
            .expect("piped")
            .read_to_end(&mut stderr_bytes)?;
        let exit_code = status.code().unwrap_or(-1);
        let captured_stdout_sha256 = digest(&stdout_bytes);
        let captured_stderr_sha256 = digest(&stderr_bytes);
        let stdout = portable(String::from_utf8(stdout_bytes)?, &root, &work);
        let stderr = portable(String::from_utf8(stderr_bytes)?, &root, &work);
        verify_command(case, step, exit_code, &stdout, &stderr)?;
        if let Some(path) = &step.capture_stdout_to {
            let path = work_path(path, &work)?;
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let bytes = match &step.capture_stdout_json_field {
                Some(field) => {
                    let value: serde_json::Value = serde_json::from_str(&stdout)?;
                    let selected = value.get(field).ok_or_else(|| {
                        anyhow::anyhow!(
                            "candidate case {} step {} stdout lacks JSON field {}",
                            case.id,
                            step.id,
                            field
                        )
                    })?;
                    serde_json::to_vec(selected)?
                }
                None => stdout.as_bytes().to_vec(),
            };
            fs::write(path, bytes)?;
        } else if step.capture_stdout_json_field.is_some() {
            bail!(
                "candidate case {} step {} has JSON capture field without output path",
                case.id,
                step.id
            );
        }
        commands.push(CommandObservation {
            step_id: step.id.clone(),
            declared_args: step.args.clone(),
            expanded_args: step
                .args
                .iter()
                .map(|arg| arg.replace("{ROOT}", "<ROOT>").replace("{WORK}", "<WORK>"))
                .collect(),
            exit_code,
            captured_stdout_sha256,
            captured_stderr_sha256,
            portable_stdout_sha256: digest(stdout.as_bytes()),
            portable_stderr_sha256: digest(stderr.as_bytes()),
            stdout,
            stderr,
        });
    }
    Ok(commands)
}

fn verify_command(
    case: &ShadowCase,
    step: &crate::model::ShadowStep,
    exit_code: i32,
    stdout: &str,
    stderr: &str,
) -> Result<()> {
    if exit_code != step.expected_exit {
        bail!(
            "candidate case {} step {} exit {}, expected {}",
            case.id,
            step.id,
            exit_code,
            step.expected_exit
        );
    }
    for value in &step.stdout_contains {
        if !stdout.contains(value) {
            bail!(
                "candidate case {} step {} stdout missing {:?}",
                case.id,
                step.id,
                value
            );
        }
    }
    for value in &step.stderr_contains {
        if !stderr.contains(value) {
            bail!(
                "candidate case {} step {} stderr missing {:?}",
                case.id,
                step.id,
                value
            );
        }
    }
    Ok(())
}

fn normalize_candidate(
    commands: &[CommandObservation],
    case: &ShadowCase,
) -> Result<NormalizedObservation> {
    let raw = crate::model::RawObservation {
        schema: crate::model::OBSERVATION_SCHEMA.into(),
        case_id: case.id.clone(),
        repetition: 1,
        incumbent_revision: "candidate".into(),
        binary_sha256: "candidate".into(),
        corpus_bundle_sha256: "candidate".into(),
        commands: commands.to_vec(),
        evidence_envelope_sha256: String::new(),
    };
    normalize(&raw, &case.normalization)
}

fn verify_groups(
    corpus: &Corpus,
    candidate: &BTreeMap<String, NormalizedObservation>,
    manifest: &ShadowManifest,
) -> Result<Vec<ShadowGroup>> {
    let mut groups = Vec::new();
    for group in &corpus.equivalence_groups {
        let status = group_status(group.cases.as_slice(), candidate, manifest, true)?;
        groups.push(ShadowGroup {
            id: group.id.clone(),
            kind: "equivalence".into(),
            cases: group.cases.clone(),
            status,
        });
    }
    for group in &corpus.difference_groups {
        let status = group_status(group.cases.as_slice(), candidate, manifest, false)?;
        groups.push(ShadowGroup {
            id: group.id.clone(),
            kind: "difference".into(),
            cases: group.cases.clone(),
            status,
        });
    }
    Ok(groups)
}

fn group_status(
    cases: &[String],
    candidate: &BTreeMap<String, NormalizedObservation>,
    manifest: &ShadowManifest,
    expect_equal: bool,
) -> Result<String> {
    let rows = cases
        .iter()
        .map(|id| {
            manifest
                .cases
                .iter()
                .find(|case| case.id == *id)
                .expect("inventory verified")
        })
        .collect::<Vec<_>>();
    if rows.iter().any(|row| row.disposition.is_blocking()) {
        return Ok("blocker".into());
    }
    let first = candidate.get(&cases[0]).expect("inventory verified");
    let equal = cases.iter().skip(1).all(|id| {
        semantic_commands_equal(
            &candidate.get(id).expect("inventory verified").commands,
            &first.commands,
        )
    });
    if equal != expect_equal {
        return Ok("blocker".into());
    }
    if rows
        .iter()
        .all(|row| row.disposition == ShadowDisposition::ApprovedIntentionalDifference)
    {
        return Ok("approved_intentional_difference".into());
    }
    Ok("pass".into())
}

fn semantic_commands_equal(left: &[CommandObservation], right: &[CommandObservation]) -> bool {
    left.len() == right.len()
        && left.iter().zip(right).all(|(left, right)| {
            left.exit_code == right.exit_code
                && left.stdout == right.stdout
                && left.stderr == right.stderr
        })
}

fn behavior_rows(corpus: &Corpus, manifest: &ShadowManifest) -> Vec<ShadowBehavior> {
    corpus
        .coverage
        .iter()
        .map(|coverage| {
            let cases = manifest
                .cases
                .iter()
                .filter(|case| coverage.cases.contains(&case.id))
                .collect::<Vec<_>>();
            let status = if cases.iter().any(|case| case.disposition.is_blocking()) {
                "blocker"
            } else if cases
                .iter()
                .all(|case| case.disposition == ShadowDisposition::ApprovedIntentionalDifference)
            {
                "approved_intentional_difference"
            } else {
                "pass"
            };
            ShadowBehavior {
                behavior: coverage.behavior.clone(),
                cases: coverage.cases.clone(),
                status: status.into(),
            }
        })
        .collect()
}

fn incumbent_case_digest(observations: &Path, case: &str, repetitions: u32) -> Result<String> {
    let mut hasher = Sha256::new();
    for repetition in 1..=repetitions {
        let path = observations
            .join(case)
            .join(format!("{repetition:02}.normalized.json"));
        let bytes = fs::read(&path).with_context(|| format!("read {}", path.display()))?;
        hasher.update((bytes.len() as u64).to_be_bytes());
        hasher.update(bytes);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn apply_action(action: &PreAction, work: &Path) -> Result<()> {
    match action {
        PreAction::ReplaceText { path, from, to } => {
            let path = work_path(path, work)?;
            let source = fs::read_to_string(&path)?;
            if !source.contains(from) {
                bail!("replace_text source not found in {}", path.display());
            }
            fs::write(path, source.replacen(from, to, 1))?;
        }
        PreAction::FixedEd25519Keypair { .. } => {
            bail!("candidate shadow uses explicit hexadecimal key arguments")
        }
    }
    Ok(())
}

fn expand(value: &str, root: &Path, work: &Path) -> Result<String> {
    let expanded = value
        .replace("{ROOT}", &root.display().to_string())
        .replace("{WORK}", &work.display().to_string());
    if expanded.contains('{') || expanded.contains('}') {
        bail!("unknown placeholder in candidate argument {value}");
    }
    Ok(expanded)
}

fn work_path(value: &str, work: &Path) -> Result<PathBuf> {
    let relative = value
        .strip_prefix("{WORK}/")
        .ok_or_else(|| anyhow::anyhow!("candidate work path must start with {{WORK}}/"))?;
    let relative = Path::new(relative);
    if relative.as_os_str().is_empty()
        || relative
            .components()
            .any(|part| !matches!(part, std::path::Component::Normal(_)))
    {
        bail!("candidate work path must be clean and relative");
    }
    Ok(work.join(relative))
}

fn portable(value: String, root: &Path, work: &Path) -> String {
    value
        .replace(&root.display().to_string(), "<ROOT>")
        .replace(&work.display().to_string(), "<WORK>")
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn disposition_name(value: ShadowDisposition) -> &'static str {
    match value {
        ShadowDisposition::ExactMatch => "exact_match",
        ShadowDisposition::NormalizedMatch => "normalized_match",
        ShadowDisposition::ApprovedIntentionalDifference => "approved_intentional_difference",
        ShadowDisposition::RegressionBlocker => "regression_blocker",
        ShadowDisposition::UnsupportedBlocker => "unsupported_blocker",
        ShadowDisposition::EvidenceInvalid => "evidence_invalid",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_incomplete_or_duplicate_case_inventory() {
        let corpus = Corpus {
            schema: crate::model::CORPUS_SCHEMA.into(),
            incumbent_revision: "a".repeat(40),
            binary_sha256: "b".repeat(64),
            repetitions: 1,
            command_timeout_ms: 100,
            schema_path: "schema.json".into(),
            required_behaviors: vec![],
            cases: vec![crate::model::Case {
                id: "one".into(),
                behaviors: vec![],
                steps: vec![],
                normalization: vec![],
            }],
            equivalence_groups: vec![],
            difference_groups: vec![],
            coverage: vec![],
        };
        let manifest = ShadowManifest {
            schema: SHADOW_MANIFEST_SCHEMA.into(),
            candidate_revision: "c".repeat(40),
            candidate_binary_sha256: "d".repeat(64),
            candidate_lock_sha256: "e".repeat(64),
            candidate_install_receipt_sha256: "f".repeat(64),
            candidate_selector_generation: "adl-v2".into(),
            candidate_selector_sha256: "1".repeat(64),
            decisions: BTreeMap::new(),
            cases: vec![],
        };
        assert!(verify_manifest(&corpus, &manifest).is_err());
    }

    #[test]
    fn intentional_difference_requires_complete_authority() {
        let corpus = Corpus {
            schema: crate::model::CORPUS_SCHEMA.into(),
            incumbent_revision: "a".repeat(40),
            binary_sha256: "b".repeat(64),
            repetitions: 1,
            command_timeout_ms: 100,
            schema_path: "schema.json".into(),
            required_behaviors: vec![],
            cases: vec![crate::model::Case {
                id: "one".into(),
                behaviors: vec![],
                steps: vec![],
                normalization: vec![],
            }],
            equivalence_groups: vec![],
            difference_groups: vec![],
            coverage: vec![],
        };
        let manifest = ShadowManifest {
            schema: SHADOW_MANIFEST_SCHEMA.into(),
            candidate_revision: "c".repeat(40),
            candidate_binary_sha256: "d".repeat(64),
            candidate_lock_sha256: "e".repeat(64),
            candidate_install_receipt_sha256: "f".repeat(64),
            candidate_selector_generation: "adl-v2".into(),
            candidate_selector_sha256: "1".repeat(64),
            decisions: BTreeMap::new(),
            cases: vec![ShadowCase {
                id: "one".into(),
                disposition: ShadowDisposition::ApprovedIntentionalDifference,
                steps: vec![crate::model::ShadowStep {
                    id: "step".into(),
                    args: vec!["--help".into()],
                    expected_exit: 0,
                    stdout_contains: vec![],
                    stderr_contains: vec![],
                    pre_actions: vec![],
                    capture_stdout_to: None,
                    capture_stdout_json_field: None,
                }],
                normalization: vec![],
                decision_ref: None,
            }],
        };
        assert!(verify_manifest(&corpus, &manifest).is_err());
    }

    #[test]
    fn work_paths_fail_closed_on_traversal() {
        let work = tempfile::tempdir().unwrap();
        assert!(work_path("{WORK}/../escape", work.path()).is_err());
        assert!(work_path("/tmp/escape", work.path()).is_err());
        assert!(work_path("{WORK}/safe/file", work.path()).is_ok());
    }

    #[test]
    fn intentional_difference_cannot_bypass_group_semantics() {
        let manifest = approved_manifest(&["a", "b"]);
        let unequal = BTreeMap::from([
            ("a".into(), normalized("a", "first")),
            ("b".into(), normalized("b", "second")),
        ]);
        assert_eq!(
            group_status(&["a".into(), "b".into()], &unequal, &manifest, true).unwrap(),
            "blocker"
        );

        let equal = BTreeMap::from([
            ("a".into(), normalized("a", "same")),
            ("b".into(), normalized("b", "same")),
        ]);
        assert_eq!(
            group_status(&["a".into(), "b".into()], &equal, &manifest, false).unwrap(),
            "blocker"
        );
    }

    #[test]
    fn command_policy_accepts_only_reviewed_local_shapes() {
        assert!(local_command_shape(&["--help".into()]));
        assert!(local_command_shape(&[
            "validate".into(),
            "{ROOT}/fixtures/unknown-agent.adl.yaml".into(),
            "--yaml".into()
        ]));
        assert!(local_command_shape(&[
            "run".into(),
            "{ROOT}/fixtures/mock-run.adl.yaml".into(),
            "--yaml".into()
        ]));
        assert!(!local_command_shape(&[
            "run".into(),
            "{ROOT}/fixtures/network-run.adl.yaml".into(),
            "--yaml".into()
        ]));
        assert!(!local_command_shape(&[
            "validate".into(),
            "https://example.invalid/input.yaml".into(),
            "--yaml".into()
        ]));
    }

    #[test]
    fn reviewer_identity_drops_host_paths() {
        assert_eq!(
            portable_reviewer("subagent:/root/review_5591"),
            "subagent:review_5591"
        );
        assert_eq!(
            portable_reviewer("gpt-5.5:required-pre-pr-review"),
            "gpt-5.5:required-pre-pr-review"
        );
    }

    fn approved_manifest(ids: &[&str]) -> ShadowManifest {
        ShadowManifest {
            schema: SHADOW_MANIFEST_SCHEMA.into(),
            candidate_revision: "c".repeat(40),
            candidate_binary_sha256: "d".repeat(64),
            candidate_lock_sha256: "e".repeat(64),
            candidate_install_receipt_sha256: "f".repeat(64),
            candidate_selector_generation: "adl-v2".into(),
            candidate_selector_sha256: "1".repeat(64),
            decisions: BTreeMap::new(),
            cases: ids
                .iter()
                .map(|id| ShadowCase {
                    id: (*id).into(),
                    disposition: ShadowDisposition::ApprovedIntentionalDifference,
                    steps: vec![],
                    normalization: vec![],
                    decision_ref: None,
                })
                .collect(),
        }
    }

    fn normalized(id: &str, stdout: &str) -> NormalizedObservation {
        NormalizedObservation {
            schema: "test".into(),
            case_id: id.into(),
            repetition: 1,
            incumbent_revision: "a".repeat(40),
            binary_sha256: "b".repeat(64),
            commands: vec![CommandObservation {
                step_id: "step".into(),
                declared_args: vec![],
                expanded_args: vec![],
                exit_code: 0,
                captured_stdout_sha256: String::new(),
                captured_stderr_sha256: String::new(),
                portable_stdout_sha256: String::new(),
                portable_stderr_sha256: String::new(),
                stdout: stdout.into(),
                stderr: String::new(),
            }],
        }
    }
}
