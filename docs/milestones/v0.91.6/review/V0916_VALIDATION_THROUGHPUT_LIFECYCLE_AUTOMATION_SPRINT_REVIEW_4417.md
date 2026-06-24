# v0.91.6 Sprint Review: #4417 Validation Throughput And Lifecycle Automation

Status: reviewed with remediation in #4499
Date: 2026-06-24
Scope: #4417 umbrella and child issues #4418, #4419, #4420, #4421

## Findings Reviewed

### P1 Retained dirty worktree carried real Rust changes

The retained `.worktrees/adl-wp-4417` worktree was not disposable residue. It was 30 commits behind `origin/main` and contained modified Rust GitHub transport files:

- `adl/src/cli/pr_cmd/github.rs`
- `adl/src/cli/pr_cmd/github/transport.rs`
- `adl/src/cli/pr_cmd/github/tests/transport.rs`

The diff changed PR-list semantics so `pr.list.open_wave` uses open-only PR listing while `pr.list.wave` can still request all-state PR listing. This was preserved below before the stale worktree was pruned.

Disposition: routed into the active `pr.sh`/GitHub wave-liveness hardening stream. The patch is retained in this review packet as evidence and migration input; the stale worktree must not remain the only copy.

### P1 Umbrella SRP still claimed review had not run

The local `#4417` SRP still recorded `findings_status: not_run` after the sprint review was complete.

Disposition: repaired in the local root-authoritative `#4417` SRP during #4499 closeout cleanup.

### P2 Sprint review was local ignored evidence only

The sprint review existed under ignored `.adl` state but did not have a tracked milestone review packet.

Disposition: this tracked review packet is the durable review surface.

### P2 Manager-backed changed-files execution could delete a substituted path

The finish-validation path parsed manager-emitted `run_pr_fast_test_lane.sh --changed-files <path>` commands and removed the parsed path after execution without checking that it was the ADL-created retained temp manifest.

Disposition: #4499 hardens the manager-backed profile loader to require the exact ADL-created retained manifest path before execution, rejects same-pattern substituted paths, and adds focused regression coverage for both malformed and wrong-path substitution.

### P3 Child issue goal metrics remain incomplete

Child issue token/time splits are still mostly unknown because the active goal substrate exposed only umbrella-level metrics during execution.

Disposition: accepted as truthful residual risk for #4417; broader issue/sprint metrics work remains in the C-SDLC metrics stream.

## Retained Worktree Diff Summary

The retained `adl-wp-4417` diff was preserved before worktree cleanup. Summary:

```diff
diff --git a/adl/src/cli/pr_cmd/github.rs b/adl/src/cli/pr_cmd/github.rs
index d71c0d20..274f72c3 100644
--- a/adl/src/cli/pr_cmd/github.rs
+++ b/adl/src/cli/pr_cmd/github.rs
@@ -424,12 +424,12 @@ pub(crate) fn linked_prs_for_issue(
     let prs = if let Some(branch_hint) = branch_hint {
         let prs = list_prs_by_head_ref_octocrab(repo, branch_hint)?;
         if prs.is_empty() {
-            list_prs_octocrab(repo)?
+            list_prs_octocrab(repo, octocrab::params::State::All, "pr.list.wave")?
         } else {
             prs
         }
     } else {
-        list_prs_octocrab(repo)?
+        list_prs_octocrab(repo, octocrab::params::State::All, "pr.list.wave")?
     };
     for pr in prs {
         let pr_ref = pr.number.to_string();
@@ -979,9 +979,14 @@ fn run_octocrab_capture(operation: &str, args: &[&str]) -> Result<String> {
             let branch = arg_after(args, "--head")?;
             current_pr_url_octocrab(repo, branch).map(|url| url.unwrap_or_default())
         }
-        "pr.list.open_wave" | "pr.list.wave" => {
+        "pr.list.open_wave" => {
             let repo = arg_after(args, "-R")?;
-            let prs = list_prs_octocrab(repo)?;
+            let prs = list_prs_octocrab(repo, octocrab::params::State::Open, operation)?;
+            serde_json::to_string(&prs).context("failed to serialize octocrab PR list")
+        }
+        "pr.list.wave" => {
+            let repo = arg_after(args, "-R")?;
+            let prs = list_prs_octocrab(repo, octocrab::params::State::All, operation)?;
             serde_json::to_string(&prs).context("failed to serialize octocrab PR list")
         }
         "pr.view.body" => {
@@ -1162,6 +1167,7 @@ pub(super) fn unresolved_milestone_pr_wave(
     let prs: Vec<OpenPullRequest> =
         serde_json::from_str(&out).with_context(|| "failed to parse GitHub PR list JSON")?;
     prs.into_iter()
+        .filter(|pr| pr.state == "OPEN")
         .filter(|pr| {
             pr_matches_main_version_wave(
                 &PullRequestMetadataSnapshot::new(
diff --git a/adl/src/cli/pr_cmd/github/tests/transport.rs b/adl/src/cli/pr_cmd/github/tests/transport.rs
index 9c938974..8870494b 100644
--- a/adl/src/cli/pr_cmd/github/tests/transport.rs
+++ b/adl/src/cli/pr_cmd/github/tests/transport.rs
@@ -240,6 +240,11 @@ fn octocrab_transport_covers_pr_and_issue_operations_against_mock_github() {
 
     let seen = server.join().expect("server join");
     assert_eq!(seen.len(), 27, "unexpected mock GitHub calls: {seen:#?}");
+    assert!(seen.iter().any(|call| {
+        call.starts_with("GET /repos/owner/repo/pulls?")
+            && call.contains("state=open")
+            && call.contains("per_page=100")
+    }));
     assert!(seen
         .iter()
         .any(|call| call.starts_with("POST /repos/owner/repo/pulls ")));
@@ -467,7 +472,12 @@ fn list_prs_octocrab_paginates_rest_results() {
         std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
     }
 
-    let prs = list_prs_octocrab("owner/repo").expect("paginated PRs");
+    let prs = list_prs_octocrab(
+        "owner/repo",
+        octocrab::params::State::Open,
+        "pr.list.open_wave",
+    )
+    .expect("paginated PRs");
     assert_eq!(prs.len(), 2);
     assert_eq!(prs[0].number, 2101);
     assert_eq!(prs[1].number, 2102);
diff --git a/adl/src/cli/pr_cmd/github/transport.rs b/adl/src/cli/pr_cmd/github/transport.rs
index 57cf6dc8..b1556e78 100644
--- a/adl/src/cli/pr_cmd/github/transport.rs
+++ b/adl/src/cli/pr_cmd/github/transport.rs
@@ -182,15 +182,19 @@ pub(super) fn current_pr_url_octocrab(repo: &str, branch: &str) -> Result<Option
     })
 }
 
-pub(super) fn list_prs_octocrab(repo: &str) -> Result<Vec<OpenPullRequest>> {
+pub(super) fn list_prs_octocrab(
+    repo: &str,
+    state: octocrab::params::State,
+    operation: &str,
+) -> Result<Vec<OpenPullRequest>> {
     let repo_parts = parse_repo(repo)?;
-    with_octocrab("pr.list.wave", |runtime, octo| {
+    with_octocrab(operation, |runtime, octo| {
         let owner = repo_parts.owner.clone();
         let name = repo_parts.name.clone();
-        let mut page = block_on_octocrab(runtime, "pr.list.wave", || async {
+        let mut page = block_on_octocrab(runtime, operation, || async {
             octo.pulls(&owner, &name)
                 .list()
-                .state(octocrab::params::State::All)
+                .state(state.clone())
                 .per_page(100)
                 .send()
                 .await
@@ -223,7 +227,7 @@ pub(super) fn list_prs_octocrab(repo: &str) -> Result<Vec<OpenPullRequest>> {
             let Some(next) = page.next.clone() else {
                 break;
             };
-            page = block_on_octocrab(runtime, "pr.list.wave", || async {
+            page = block_on_octocrab(runtime, operation, || async {
                 octo.get_page::<octocrab::models::pulls::PullRequest>(&Some(next.clone()))
                     .await
             })?
```

## Review Result

The sprint delivered meaningful validation-throughput and lifecycle automation improvements, but it was not closeout-clean until #4499 resolved the review findings above.

After #4499 remediation, the required closure bar is:

- retained dirty worktree diff preserved and stale worktree removed
- local #4417 SRP/SOR truth normalized
- tracked review packet present
- manager-backed changed-files hardening implemented and covered by focused tests
