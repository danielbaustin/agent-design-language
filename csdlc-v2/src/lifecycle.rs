use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::cards::validate_initial_input;
use crate::doctor::DoctorStatus;
use crate::error::{ErrorCode, Result, V2Error};
use crate::git;
use crate::model::AuditEvent;
use crate::store::{bootstrap_issue, validate_bootstrap_request, BootstrapRequest, Store};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BindRequest {
    pub issue: u64,
    pub base_branch: String,
    pub branch: String,
    pub worktree: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BindResult {
    pub created: bool,
    pub branch: String,
    pub worktree: String,
}

fn clean_relative(value: &str) -> bool {
    !value.is_empty()
        && Path::new(value)
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
}

fn requested_worktree(root: &Path, value: &str) -> Result<PathBuf> {
    if value == "." {
        return Ok(root.canonicalize()?);
    }
    let path = Path::new(value);
    if !path.is_absolute() && !clean_relative(value) {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "worktree must be an absolute path or a clean repository-relative path",
        ));
    }
    let requested = if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    };
    if requested.exists() {
        return Ok(requested.canonicalize()?);
    }
    let mut ancestor = requested.as_path();
    let mut suffix = Vec::new();
    while !ancestor.exists() {
        suffix.push(ancestor.file_name().ok_or_else(|| {
            V2Error::new(
                ErrorCode::InvalidInput,
                "worktree path has no existing ancestor",
            )
        })?);
        ancestor = ancestor.parent().ok_or_else(|| {
            V2Error::new(
                ErrorCode::InvalidInput,
                "worktree path has no existing ancestor",
            )
        })?;
    }
    let mut normalized = ancestor.canonicalize()?;
    for component in suffix.into_iter().rev() {
        normalized.push(component);
    }
    Ok(normalized)
}

fn valid_branch(root: &Path, branch: &str) -> bool {
    !branch.trim().is_empty()
        && Command::new("git")
            .current_dir(root)
            .args(["check-ref-format", "--branch", branch])
            .output()
            .is_ok_and(|output| output.status.success())
}

fn branch_exists(root: &Path, branch: &str) -> bool {
    Command::new("git")
        .current_dir(root)
        .args([
            "show-ref",
            "--verify",
            "--quiet",
            &format!("refs/heads/{branch}"),
        ])
        .status()
        .is_ok_and(|status| status.success())
}

fn same_worktree(root: &Path, recorded: &str, requested: &Path) -> bool {
    requested_worktree(root, recorded).is_ok_and(|path| path == requested)
}

fn issue_records(store: &Store) -> Result<Vec<crate::IssueRecord>> {
    let issues = store.root().join(".csdlc/issues");
    if !issues.exists() {
        return Ok(Vec::new());
    }
    let mut records = Vec::new();
    for entry in fs::read_dir(issues)? {
        let entry = entry?;
        let Some(issue) = entry
            .file_name()
            .to_str()
            .and_then(|value| value.parse().ok())
        else {
            continue;
        };
        if entry.path().join("index.json").exists() {
            let record = store.load_record(issue)?;
            crate::store::verify_cards(store, &record, &store.load_cards(issue)?)?;
            records.push(record);
        }
    }
    Ok(records)
}

fn copy_tree(source: &Path, destination: &Path) -> Result<()> {
    let metadata = fs::symlink_metadata(source)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "issue projection source must be a real directory",
        ));
    }
    if fs::symlink_metadata(destination).is_ok_and(|value| value.file_type().is_symlink()) {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "issue projection target cannot be a symlink",
        ));
    }
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let from = entry.path();
        let to = destination.join(entry.file_name());
        let metadata = fs::symlink_metadata(&from)?;
        if metadata.file_type().is_symlink() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "issue projection cannot contain symlinks",
            ));
        }
        if metadata.is_dir() {
            copy_tree(&from, &to)?;
        } else {
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(from, to)?;
        }
    }
    Ok(())
}

fn copy_authored_file(source: &Store, target: &Store, relative: &str) -> Result<()> {
    let from = source.root().join(relative);
    let to = target.root().join(relative);
    if to.starts_with(target.issue_dir(0).parent().expect("issue root")) {
        return Ok(());
    }
    let bytes = fs::read(&from)?;
    if let Ok(existing) = fs::read(&to) {
        if existing == bytes {
            return Ok(());
        }
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            format!("bound worktree has different authored file {relative}"),
        ));
    }
    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(to, bytes)?;
    Ok(())
}

struct MaterializedIssue {
    store: Store,
    issue: u64,
    source_is_target: bool,
    created_issue: bool,
    created_design: bool,
    created_diagram: bool,
    design_path: String,
    diagram_path: String,
}

impl MaterializedIssue {
    fn rollback(&self) {
        if self.created_issue {
            let _ = fs::remove_dir_all(self.store.issue_dir(self.issue));
        }
        if self.created_design {
            let _ = fs::remove_file(self.store.root().join(&self.design_path));
        }
        if self.created_diagram {
            let _ = fs::remove_file(self.store.root().join(&self.diagram_path));
        }
    }
}

fn materialize_issue(source: &Store, target_root: &Path, issue: u64) -> Result<MaterializedIssue> {
    let target = Store::new(target_root.to_path_buf());
    let source_record = source.load_record(issue)?;
    let source_is_target = source.root().canonicalize()? == target.root().canonicalize()?;
    if source_is_target {
        return Ok(MaterializedIssue {
            store: target,
            issue,
            source_is_target,
            created_issue: false,
            created_design: false,
            created_diagram: false,
            design_path: source_record.design_path,
            diagram_path: source_record.diagram_path,
        });
    }
    let mut created_issue = false;
    let mut created_design = false;
    let mut created_diagram = false;
    if target.issue_dir(issue).exists() {
        let target_record = target.load_record(issue)?;
        if target_record.issue != source_record.issue
            || target_record.repository != source_record.repository
            || target_record.initialization_digest != source_record.initialization_digest
            || target_record.digest != source_record.digest
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "bound worktree contains different or stale issue truth",
            ));
        }
        crate::store::verify_cards(&target, &target_record, &target.load_cards(issue)?)?;
    } else {
        for relative in [&source_record.design_path, &source_record.diagram_path] {
            let from = source.root().join(relative);
            let to = target.root().join(relative);
            if let Ok(existing) = fs::read(&to) {
                if existing != fs::read(&from)? {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        format!("bound worktree has different authored file {relative}"),
                    ));
                }
            }
        }
        created_design = !target.root().join(&source_record.design_path).exists();
        created_diagram = !target.root().join(&source_record.diagram_path).exists();
        let result = (|| {
            copy_tree(&source.issue_dir(issue), &target.issue_dir(issue))?;
            created_issue = true;
            copy_authored_file(source, &target, &source_record.design_path)?;
            copy_authored_file(source, &target, &source_record.diagram_path)?;
            Ok(())
        })();
        if let Err(error) = result {
            if created_issue {
                let _ = fs::remove_dir_all(target.issue_dir(issue));
            }
            if created_design {
                let _ = fs::remove_file(target.root().join(&source_record.design_path));
            }
            if created_diagram {
                let _ = fs::remove_file(target.root().join(&source_record.diagram_path));
            }
            return Err(error);
        }
    }
    Ok(MaterializedIssue {
        store: target,
        issue,
        source_is_target,
        created_issue,
        created_design,
        created_diagram,
        design_path: source_record.design_path,
        diagram_path: source_record.diagram_path,
    })
}

pub(crate) fn initialize_issue(
    store: &Store,
    mut request: BootstrapRequest,
) -> Result<crate::IssueRecord> {
    let _creation_lock = store.binding_lock()?;
    if !clean_relative(&request.design_path)
        || !clean_relative(&request.diagram_path)
        || request.design_path == request.diagram_path
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "design and diagram paths must be distinct and repository-relative",
        ));
    }
    validate_bootstrap_request(&request)?;
    validate_initial_input(&request.initial)?;
    let issue_dir = store.issue_dir(request.issue);
    for authored_path in [&request.design_path, &request.diagram_path] {
        let path = store.root().join(authored_path);
        if path == issue_dir.join("index.json")
            || path == issue_dir.join("audit.jsonl")
            || path.starts_with(issue_dir.join("cards"))
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "design and diagram paths cannot target issue control files",
            ));
        }
    }
    if issue_dir.join("index.json").exists() {
        return bootstrap_issue(store, request);
    }
    let design = store.root().join(&request.design_path);
    let diagram = store.root().join(&request.diagram_path);
    let created_design = !design.exists();
    let created_diagram = !diagram.exists();
    let result = (|| {
        if created_design {
            if let Some(parent) = design.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(
                &design,
                format!(
                    "# Issue {} design\n\nStatus: design required before Ready.\n",
                    request.issue
                ),
            )?;
            request.design_approved = false;
        }
        if created_diagram {
            if let Some(parent) = diagram.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(
                &diagram,
                format!(
                    "flowchart LR\n  I[\"Issue {}\"] --> D[\"Design required\"]\n",
                    request.issue
                ),
            )?;
        }
        bootstrap_issue(store, request)
    })();
    if result.is_err() {
        if created_diagram && diagram.exists() {
            fs::remove_file(&diagram)?;
        }
        if created_design && design.exists() {
            fs::remove_file(&design)?;
        }
    }
    result
}

pub fn initialize_native_json(store: &Store, bytes: &[u8]) -> Result<crate::IssueRecord> {
    let value: serde_json::Value = serde_json::from_slice(bytes)?;
    let initial = value
        .get("initial")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "native initial input is missing"))?;
    if !initial.contains_key("operator_constraints") || !initial.contains_key("review_scope") {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "native bootstrap requires explicit operator_constraints and review_scope",
        ));
    }
    let request: BootstrapRequest = serde_json::from_value(value)?;
    crate::registry::validate_native_registry(store.root())?;
    initialize_issue(store, request)
}

pub fn bind_issue(store: &Store, request: BindRequest) -> Result<BindResult> {
    if request.issue == 0
        || request.branch == "main"
        || request.branch == request.base_branch
        || !valid_branch(store.root(), &request.branch)
        || !valid_branch(store.root(), &request.base_branch)
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "issue, distinct safe base/issue branches, and worktree are required",
        ));
    }
    let wanted = requested_worktree(store.root(), &request.worktree)?;
    let wanted_text = wanted.to_string_lossy().into_owned();
    let current_root = store.root().canonicalize()?;
    let issue_local = wanted.exists()
        && wanted.canonicalize().ok().as_ref() == Some(&current_root)
        && git::current_branch(store.root())? == request.branch;

    let _lock = store.binding_lock()?;
    let _issue_lock = store.authority_projection_lock(request.issue)?;
    let source_diagnosis = crate::diagnose(store, request.issue);
    let source_phase = source_diagnosis.phase;
    let source_is_bindable = source_diagnosis.status == DoctorStatus::Pass
        && matches!(
            source_phase,
            Some(
                crate::LifecyclePhase::Initialized
                    | crate::LifecyclePhase::Ready
                    | crate::LifecyclePhase::Bound
            )
        )
        && (source_phase != Some(crate::LifecyclePhase::Initialized) || source_diagnosis.ready);
    if !source_is_bindable {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "source issue is not execution-ready for binding",
        ));
    }

    if !issue_local && git::current_branch(store.root())? != request.base_branch {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "binding must start from the declared base branch or the exact issue worktree",
        ));
    }
    let listed = git::worktrees(store.root())?;
    for (branch, path) in &listed {
        let path = Path::new(path);
        if !path.exists() {
            continue;
        }
        for record in issue_records(&Store::new(path))? {
            if record.branch.is_none() && record.worktree.is_none() {
                continue;
            }
            if record.issue == request.issue {
                if branch == &request.branch
                    && path == wanted
                    && record.branch.as_deref() == Some(request.branch.as_str())
                    && record
                        .worktree
                        .as_deref()
                        .is_some_and(|value| same_worktree(store.root(), value, &wanted))
                    && record.phase == crate::LifecyclePhase::Bound
                {
                    return Ok(BindResult {
                        created: false,
                        branch: request.branch,
                        worktree: wanted_text,
                    });
                }
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "issue is already bound to different Git topology",
                ));
            }
            if branch == &request.branch || path == wanted {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "requested Git topology is already bound to another issue",
                ));
            }
        }
    }
    if let Some((branch, _)) = listed.iter().find(|(_, path)| path == &wanted_text) {
        if branch != &request.branch {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "requested worktree belongs to a different branch",
            ));
        }
    } else if wanted.exists() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "requested path exists but is not a Git worktree",
        ));
    }
    if let Some((_, path)) = listed.iter().find(|(branch, _)| branch == &request.branch) {
        if path != &wanted_text {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "requested branch belongs to a different worktree",
            ));
        }
    }

    let new_branch = !branch_exists(store.root(), &request.branch);
    let created = !wanted.exists();
    if created {
        if new_branch {
            git::run(
                store.root(),
                &[
                    "worktree",
                    "add",
                    "-b",
                    &request.branch,
                    &wanted_text,
                    &request.base_branch,
                ],
            )?;
        } else {
            git::run(
                store.root(),
                &["worktree", "add", &wanted_text, &request.branch],
            )?;
        }
    }

    let materialized = match materialize_issue(store, &wanted, request.issue) {
        Ok(materialized) => materialized,
        Err(error) => {
            if created {
                let _ = git::run(
                    store.root(),
                    &["worktree", "remove", "--force", &wanted_text],
                );
                if new_branch {
                    let _ = git::run(store.root(), &["branch", "-D", &request.branch]);
                }
            }
            return Err(error);
        }
    };
    let target_lock = wanted.join(format!(".csdlc/locks/{}.lock", request.issue));
    let target_lock_created = !target_lock.exists();
    let commit = (|| {
        let target = &materialized.store;
        let mut record = target.load_record(request.issue)?;
        let expected_digest = record.digest.clone();
        if record.phase == crate::LifecyclePhase::Initialized {
            record.advance(
                crate::LifecyclePhase::Ready,
                "csdlc-bind".into(),
                "validated issue readiness".into(),
            )?;
        }
        if record.phase == crate::LifecyclePhase::Ready {
            record.advance(
                crate::LifecyclePhase::Bound,
                "csdlc-bind".into(),
                "bound issue branch and worktree".into(),
            )?;
        } else if record.phase != crate::LifecyclePhase::Bound {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "issue phase cannot be bound",
            ));
        }
        record.branch = Some(request.branch.clone());
        record.worktree = Some(wanted_text.clone());
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor: "csdlc-bind".into(),
            reason: "record Git branch/worktree topology".into(),
            operation: "bind".into(),
        });
        record.digest = crate::store::record_digest(&record)?;
        if materialized.source_is_target {
            target.replace_record_locked(request.issue, &expected_digest, &record)
        } else {
            target.replace_record(request.issue, &expected_digest, &record)
        }
    })();

    if let Err(error) = commit {
        materialized.rollback();
        if target_lock_created {
            let _ = fs::remove_file(target_lock);
        }
        if created {
            let _ = git::run(
                store.root(),
                &["worktree", "remove", "--force", &wanted_text],
            );
            if new_branch {
                let _ = git::run(store.root(), &["branch", "-D", &request.branch]);
            }
        }
        return Err(error);
    }

    Ok(BindResult {
        created,
        branch: request.branch,
        worktree: wanted_text,
    })
}
