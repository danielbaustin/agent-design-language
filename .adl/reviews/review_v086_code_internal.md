# v0.86 Code Review (Internal)

Verdict

BLOCK

Findings

1. [P1] `pr finish --allow-gitignore` is still not implemented truthfully in the shipped control plane. In Rust, [`real_pr_finish`](\/Users\/daniel\/git\/agent-design-language\/adl\/src\/cli\/pr_cmd.rs#L477) calls [`stage_selected_paths_rust`](\/Users\/daniel\/git\/agent-design-language\/adl\/src\/cli\/pr_cmd.rs#L938) before it checks `parsed.allow_gitignore`, and the staging helper always runs `git add -- <paths>`. In shell, [`stage_selected_paths`](\/Users\/daniel\/git\/agent-design-language\/adl\/tools\/pr.sh#L804) unconditionally skips ignored paths and then errors if they were the only requested paths. That means the public flag exists, but the code path still cannot legitimately finish with ignored review artifacts. This is not hypothetical; it is exactly the failure we hit while publishing `#1224`.  
   Exact replacement direction: thread `allow_gitignore` into both staging helpers and make the flag control staging behavior, not just post-stage `.gitignore` detection.

2. [P2] [`adl/src/cli/pr_cmd.rs`](\/Users\/daniel\/git\/agent-design-language\/adl\/src\/cli\/pr_cmd.rs#L1) is still too large to be a trustworthy review surface. It is `5382` lines, owns the public `create/init/start/ready/preflight/finish` command lifecycle, and then embeds its in-file test module starting at [`#L2041`](\/Users\/daniel\/git\/agent-design-language\/adl\/src\/cli\/pr_cmd.rs#L2041). That means one file is simultaneously carrying command dispatch, Git/GitHub control-plane behavior, prompt generation, validation, publication, and thousands of lines of tests. This is now a real maintenance risk, not just an aesthetic problem: it is conflict-prone, hard to audit, and too large for reliable internal review.
   Exact replacement direction: split it into focused modules for bootstrap, readiness, finish/publication, Git/GitHub integration, and tests before adding more lifecycle behavior.

3. [P3] [`adl/src/cli/run_artifacts.rs`](\/Users\/daniel\/git\/agent-design-language\/adl\/src\/cli\/run_artifacts.rs#L1) is also beyond a healthy review size. At `3242` lines it mixes schema/version constants, many artifact structs, production artifact builders, and test-only helper code such as [`build_cognitive_signals_state`](\/Users\/daniel\/git\/agent-design-language\/adl\/src\/cli\/run_artifacts.rs#L1401). This makes the artifact contract hard to reason about and too easy to change accidentally while touching unrelated proof surfaces.
   Exact replacement direction: separate artifact schemas/types, production builders, and test scaffolding into distinct modules grouped by artifact family.

4. [P4] [`adl/tools/pr.sh`](\/Users\/daniel\/git\/agent-design-language\/adl\/tools\/pr.sh#L1) still owns too much behavior for a shell wrapper. The public usage contract is still declared there at [`#L15-L24`](\/Users\/daniel\/git\/agent-design-language\/adl\/tools\/pr.sh#L15), and the shell continues to implement meaningful workflow behavior such as staging policy at [`#L804-L829`](\/Users\/daniel\/git\/agent-design-language\/adl\/tools\/pr.sh#L804). After the amount of control-plane churn we just went through, this is still too much shell ownership.
   Exact replacement direction: reduce `pr.sh` to argument normalization plus `exec adl pr ...`, with behaviorally significant policy living only in Rust.

5. [P5] The CLI test surface is also getting too large to audit comfortably. [`adl/src/cli/tests/artifact_builders.rs`](\/Users\/daniel\/git\/agent-design-language\/adl\/src\/cli\/tests\/artifact_builders.rs#L1) is `2518` lines and [`adl/src/cli/tests/run_state.rs`](\/Users\/daniel\/git\/agent-design-language\/adl\/src\/cli\/tests\/run_state.rs#L1) is `1030` lines. That does not block Sprint 7 by itself, but it makes it harder to understand which tests protect which runtime/proof-surface contract and increases friction for targeted fixes.
   Exact replacement direction: split the test files by artifact family or runtime feature slice so failures and ownership stay local.

Code Size Notes

- `adl/src/cli/pr_cmd.rs`: `5382` lines
- `adl/src/cli/run_artifacts.rs`: `3242` lines
- `adl/tools/pr.sh`: `2453` lines
- `adl/src/cli/tests/artifact_builders.rs`: `2518` lines
- `adl/src/demo.rs`: `1824` lines
- `adl/src/remote_exec.rs`: `1587` lines
- `adl/src/execute/state.rs`: `1417` lines

Assessment

The repo code is real and materially implemented, but the internal review should not call it clean yet. The blocking issue is the still-broken `--allow-gitignore` finish path. Beyond that, the control-plane and artifact modules remain oversized enough to materially weaken auditability, merge safety, and future reviewer trust. Once the `allow-gitignore` staging behavior is fixed, the remaining findings are serious but non-blocking architecture debt rather than reasons to stop Sprint 7.
