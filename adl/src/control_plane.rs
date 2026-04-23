//! Workflow tooling helpers for ADL issue and worktree path resolution.
//!
//! This module converts issue identifiers into concrete task cards, branch names,
//! and worktree checkout locations used across the ADL PR lifecycle.

use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

/// Canonical reference to an ADL issue and its workflow-local artifacts.
///
/// The helper exposes validated identifiers and deterministic path derivation for
/// issue prompts, task bundles, and branch names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IssueRef {
    issue_number: u32,
    scope: String,
    slug: String,
}

impl IssueRef {
    /// Build a validated `IssueRef` from user-supplied metadata.
    ///
    /// `scope` must be non-empty, and `slug` is normalized to a workflow-safe
    /// form before being stored. The error contract is intentionally strict for
    /// deterministic path construction.
    pub fn new(
        issue_number: u32,
        scope: impl Into<String>,
        slug: impl Into<String>,
    ) -> Result<Self> {
        let scope = scope.into();
        let slug = sanitize_slug(slug.into());
        if scope.trim().is_empty() {
            return Err(anyhow!("scope must not be empty"));
        }
        if slug.is_empty() {
            return Err(anyhow!("slug must not be empty after sanitization"));
        }
        Ok(Self {
            issue_number,
            scope,
            slug,
        })
    }

    pub fn issue_number(&self) -> u32 {
        self.issue_number
    }

    /// Return the issue scope token associated with this issue record.
    pub fn scope(&self) -> &str {
        &self.scope
    }

    /// Return the normalized slug used in branch/task filenames.
    pub fn slug(&self) -> &str {
        &self.slug
    }

    /// Return a zero-padded issue number, used for legacy ADL task directory names.
    pub fn padded_issue_number(&self) -> String {
        format!("{:04}", self.issue_number)
    }

    /// Derive the standard `issue-xxxx` task identifier.
    pub fn task_issue_id(&self) -> String {
        format!("issue-{}", self.padded_issue_number())
    }

    /// Return the canonical task bundle directory name used by the lifecycle tools.
    pub fn task_bundle_dir_name(&self) -> String {
        format!("{}__{}", self.task_issue_id(), self.slug)
    }

    /// Resolve the source issue prompt path in the workflow scope.
    pub fn issue_prompt_path(&self, repo_root: &Path) -> PathBuf {
        repo_root
            .join(".adl")
            .join(&self.scope)
            .join("bodies")
            .join(format!("issue-{}-{}.md", self.issue_number, self.slug))
    }

    pub fn legacy_issue_prompt_path(&self, repo_root: &Path) -> PathBuf {
        repo_root
            .join(".adl")
            .join("issues")
            .join(&self.scope)
            .join("bodies")
            .join(format!("issue-{}-{}.md", self.issue_number, self.slug))
    }

    pub fn task_bundle_dir_path(&self, primary_checkout_root: &Path) -> PathBuf {
        primary_checkout_root
            .join(".adl")
            .join(&self.scope)
            .join("tasks")
            .join(self.task_bundle_dir_name())
    }

    pub fn task_bundle_stp_path(&self, primary_checkout_root: &Path) -> PathBuf {
        self.task_bundle_dir_path(primary_checkout_root)
            .join("stp.md")
    }

    pub fn task_bundle_input_path(&self, primary_checkout_root: &Path) -> PathBuf {
        self.task_bundle_dir_path(primary_checkout_root)
            .join("sip.md")
    }

    pub fn task_bundle_output_path(&self, primary_checkout_root: &Path) -> PathBuf {
        self.task_bundle_dir_path(primary_checkout_root)
            .join("sor.md")
    }

    pub fn branch_name(&self, prefix: &str) -> String {
        format!("{prefix}/{}-{}", self.issue_number, self.slug)
    }

    /// Return the default worktree directory for this issue.
    pub fn default_worktree_path(
        &self,
        primary_checkout_root: &Path,
        managed_worktree_root: Option<&Path>,
    ) -> PathBuf {
        let managed_root = managed_worktree_root
            .map(PathBuf::from)
            .unwrap_or_else(|| primary_checkout_root.join(".worktrees"));
        managed_root.join(format!("adl-wp-{}", self.issue_number))
    }

    pub fn worktree_task_bundle_dir_path(&self, worktree_root: &Path) -> PathBuf {
        worktree_root
            .join(".adl")
            .join(&self.scope)
            .join("tasks")
            .join(self.task_bundle_dir_name())
    }
}

/// Normalize a human-facing slug into a workflow-safe path segment.
///
/// The function keeps only lower-case alphanumerics and collapses all other
/// characters to single dash separators, trimming leading/trailing separators.
pub fn sanitize_slug(raw: impl AsRef<str>) -> String {
    let mut out = String::new();
    let mut last_was_dash = false;

    for ch in raw.as_ref().chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_lowercase() || ch.is_ascii_digit() {
            out.push(ch);
            last_was_dash = false;
        } else if !last_was_dash && !out.is_empty() {
            out.push('-');
            last_was_dash = true;
        }
    }

    while out.ends_with('-') {
        out.pop();
    }

    out
}

pub fn resolve_primary_checkout_root(current_top: &Path, git_common_dir: Option<&Path>) -> PathBuf {
    let Some(common_dir) = git_common_dir else {
        return current_top.to_path_buf();
    };

    let common_abs = if common_dir.is_absolute() {
        common_dir.to_path_buf()
    } else {
        current_top.join(common_dir)
    };

    if common_abs.file_name().and_then(|name| name.to_str()) == Some(".git") {
        return common_abs
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| current_top.to_path_buf());
    }

    let worktrees_dir = common_abs.parent();
    let git_dir = worktrees_dir.and_then(Path::parent);
    if worktrees_dir
        .and_then(|path| path.file_name())
        .and_then(|name| name.to_str())
        == Some("worktrees")
        && git_dir
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            == Some(".git")
    {
        return git_dir
            .and_then(Path::parent)
            .map(Path::to_path_buf)
            .unwrap_or_else(|| current_top.to_path_buf());
    }

    current_top.to_path_buf()
}

/// Resolve cards root from explicit override or default `.adl/cards`.
pub fn resolve_cards_root(
    primary_checkout_root: &Path,
    cards_root_override: Option<&Path>,
) -> PathBuf {
    match cards_root_override {
        Some(path) if path.is_absolute() => path.to_path_buf(),
        Some(path) => primary_checkout_root.join(path),
        None => primary_checkout_root.join(".adl").join("cards"),
    }
}

pub fn card_dir_path(cards_root: &Path, issue_number: u32) -> PathBuf {
    cards_root.join(issue_number.to_string())
}

pub fn card_input_path(cards_root: &Path, issue_number: u32) -> PathBuf {
    card_dir_path(cards_root, issue_number).join(format!("input_{issue_number}.md"))
}

pub fn card_stp_path(cards_root: &Path, issue_number: u32) -> PathBuf {
    card_dir_path(cards_root, issue_number).join(format!("stp_{issue_number}.md"))
}

pub fn card_output_path(cards_root: &Path, issue_number: u32) -> PathBuf {
    card_dir_path(cards_root, issue_number).join(format!("output_{issue_number}.md"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn slug_sanitization_matches_shell_rules() {
        assert_eq!(
            sanitize_slug("Fast / Slow Thinking Paths"),
            "fast-slow-thinking-paths"
        );
        assert_eq!(
            sanitize_slug("[v0.86][WP-08] Implement bounded execution (AEE-lite)"),
            "v0-86-wp-08-implement-bounded-execution-aee-lite"
        );
        assert_eq!(sanitize_slug("___Already---clean___"), "already-clean");
        assert_eq!(sanitize_slug(""), "");
    }

    #[test]
    fn issue_ref_builds_canonical_paths_and_branch_names() {
        let issue = IssueRef::new(1150, "v0.86", "Implement Rust control-plane core models")
            .expect("valid issue ref");
        let repo_root = Path::new("/repo");

        assert_eq!(
            issue.issue_prompt_path(repo_root),
            PathBuf::from(
                "/repo/.adl/v0.86/bodies/issue-1150-implement-rust-control-plane-core-models.md"
            )
        );
        assert_eq!(issue.task_issue_id(), "issue-1150");
        assert_eq!(
            issue.task_bundle_dir_name(),
            "issue-1150__implement-rust-control-plane-core-models"
        );
        assert_eq!(
            issue.task_bundle_dir_path(repo_root),
            PathBuf::from(
                "/repo/.adl/v0.86/tasks/issue-1150__implement-rust-control-plane-core-models"
            )
        );
        assert_eq!(
            issue.branch_name("codex"),
            "codex/1150-implement-rust-control-plane-core-models"
        );
    }

    #[test]
    fn default_worktree_path_matches_repo_local_governance() {
        let issue = IssueRef::new(1148, "v0.86", "demo program").expect("valid issue ref");
        let primary_root = Path::new("/repo");

        assert_eq!(
            issue.default_worktree_path(primary_root, None),
            PathBuf::from("/repo/.worktrees/adl-wp-1148")
        );
        assert_eq!(
            issue.default_worktree_path(primary_root, Some(Path::new("/managed/worktrees"))),
            PathBuf::from("/managed/worktrees/adl-wp-1148")
        );
    }

    #[test]
    fn cards_root_respects_relative_and_absolute_overrides() {
        let primary_root = Path::new("/repo");

        assert_eq!(
            resolve_cards_root(primary_root, None),
            PathBuf::from("/repo/.adl/cards")
        );
        assert_eq!(
            resolve_cards_root(primary_root, Some(Path::new("custom/cards"))),
            PathBuf::from("/repo/custom/cards")
        );
        assert_eq!(
            resolve_cards_root(primary_root, Some(Path::new("/tmp/adl-cards"))),
            PathBuf::from("/tmp/adl-cards")
        );
    }

    #[test]
    fn primary_checkout_root_resolves_primary_checkout_for_repo_local_worktrees() {
        let current_top = Path::new("/repo/.worktrees/adl-wp-200");
        let common = Path::new("/repo/.git/worktrees/adl-wp-200");

        assert_eq!(
            resolve_primary_checkout_root(current_top, Some(common)),
            PathBuf::from("/repo")
        );
    }

    #[test]
    fn primary_checkout_root_stays_on_primary_checkout() {
        let current_top = Path::new("/repo");
        let common = Path::new("/repo/.git");

        assert_eq!(
            resolve_primary_checkout_root(current_top, Some(common)),
            PathBuf::from("/repo")
        );
    }

    #[test]
    fn worktree_task_bundle_path_matches_ready_contract() {
        let issue = IssueRef::new(1125, "v0.86", "Implement fast / slow thinking paths")
            .expect("valid issue ref");

        assert_eq!(
            issue.worktree_task_bundle_dir_path(Path::new("/repo/.worktrees/adl-wp-1125")),
            PathBuf::from(
                "/repo/.worktrees/adl-wp-1125/.adl/v0.86/tasks/issue-1125__implement-fast-slow-thinking-paths"
            )
        );
        assert_eq!(
            card_input_path(Path::new("/repo/.adl/cards"), 1125),
            PathBuf::from("/repo/.adl/cards/1125/input_1125.md")
        );
        assert_eq!(
            card_stp_path(Path::new("/repo/.adl/cards"), 1125),
            PathBuf::from("/repo/.adl/cards/1125/stp_1125.md")
        );
        assert_eq!(
            card_output_path(Path::new("/repo/.adl/cards"), 1125),
            PathBuf::from("/repo/.adl/cards/1125/output_1125.md")
        );
    }
}
