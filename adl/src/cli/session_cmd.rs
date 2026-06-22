use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use chrono::Utc;

use adl::session_ledger::{
    acquire_ledger_lock, default_ledger_path, load_ledger, parse_mode, parse_resource, save_ledger,
    ClaimInput, GithubRef, DEFAULT_TTL_SECS,
};

pub(crate) fn real_session(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("status") => real_status(&args[1..]),
        Some("claim") => real_claim(&args[1..]),
        Some("heartbeat") => real_heartbeat(&args[1..]),
        Some("release") => real_release(&args[1..]),
        Some("--help" | "-h" | "help") | None => {
            println!("{}", session_usage());
            Ok(())
        }
        Some(other) => Err(anyhow!(
            "unknown session command '{other}'\n\n{}",
            session_usage()
        )),
    }
}

fn real_status(args: &[String]) -> Result<()> {
    if help_requested(args) {
        println!("{}", status_usage());
        return Ok(());
    }
    let parsed = parse_common(args)?;
    let now = Utc::now();
    let ledger = load_ledger(&parsed.ledger_path, now)?;
    let status = ledger.status(&parsed.ledger_path, now);
    if parsed.json {
        println!("{}", serde_json::to_string_pretty(&status)?);
    } else {
        println!("Session ledger: {}", status.ledger_path);
        println!("Global freeze: {}", status.global_freeze_active);
        println!(
            "Claims: active={} stale={} released={}",
            status.active_claims, status.stale_claims, status.released_claims
        );
        for claim in status.claims {
            println!(
                "- {} {:?} {:?} {}:{} owner={} session={} worktree={}",
                claim.claim_id,
                claim.classification,
                claim.mode,
                claim.resource.kind,
                claim.resource.id,
                claim.owner,
                claim.session_id,
                claim.worktree_path.unwrap_or_else(|| "none".to_string())
            );
        }
    }
    Ok(())
}

fn real_claim(args: &[String]) -> Result<()> {
    if help_requested(args) {
        println!("{}", claim_usage());
        return Ok(());
    }
    let parsed = parse_claim(args)?;
    let now = Utc::now();
    let _lock = acquire_ledger_lock(&parsed.common.ledger_path)?;
    let mut ledger = load_ledger(&parsed.common.ledger_path, now)?;
    let claim = ledger.claim(parsed.input, now)?;
    save_ledger(&parsed.common.ledger_path, &ledger)?;
    if parsed.common.json {
        println!("{}", serde_json::to_string_pretty(&claim)?);
    } else {
        println!("claimed {}", claim.claim_id);
    }
    Ok(())
}

fn real_heartbeat(args: &[String]) -> Result<()> {
    if help_requested(args) {
        println!("{}", heartbeat_usage());
        return Ok(());
    }
    let parsed = parse_claim_id_command(args)?;
    let now = Utc::now();
    let _lock = acquire_ledger_lock(&parsed.common.ledger_path)?;
    let mut ledger = load_ledger(&parsed.common.ledger_path, now)?;
    let claim = ledger.heartbeat(&parsed.claim_id, parsed.ttl_secs, now)?;
    save_ledger(&parsed.common.ledger_path, &ledger)?;
    if parsed.common.json {
        println!("{}", serde_json::to_string_pretty(&claim)?);
    } else {
        println!("heartbeat {}", claim.claim_id);
    }
    Ok(())
}

fn real_release(args: &[String]) -> Result<()> {
    if help_requested(args) {
        println!("{}", release_usage());
        return Ok(());
    }
    let parsed = parse_release(args)?;
    let now = Utc::now();
    let _lock = acquire_ledger_lock(&parsed.common.ledger_path)?;
    let mut ledger = load_ledger(&parsed.common.ledger_path, now)?;
    let claim = ledger.release(&parsed.claim_id, parsed.reason, now)?;
    save_ledger(&parsed.common.ledger_path, &ledger)?;
    if parsed.common.json {
        println!("{}", serde_json::to_string_pretty(&claim)?);
    } else {
        println!("released {}", claim.claim_id);
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct CommonArgs {
    ledger_path: PathBuf,
    json: bool,
}

#[derive(Debug, Clone)]
struct ClaimArgs {
    common: CommonArgs,
    input: ClaimInput,
}

#[derive(Debug, Clone)]
struct ClaimIdArgs {
    common: CommonArgs,
    claim_id: String,
    ttl_secs: i64,
}

#[derive(Debug, Clone)]
struct ReleaseArgs {
    common: CommonArgs,
    claim_id: String,
    reason: Option<String>,
}

fn parse_common(args: &[String]) -> Result<CommonArgs> {
    let mut ledger_path: Option<PathBuf> = None;
    let mut json = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => {
                json = true;
                i += 1;
            }
            "--ledger" => {
                ledger_path = Some(PathBuf::from(take_value(args, i, "--ledger")?));
                i += 2;
            }
            other => {
                return Err(anyhow!(
                    "unknown session status option '{other}'\n\n{}",
                    status_usage()
                ));
            }
        }
    }
    Ok(CommonArgs {
        ledger_path: ledger_path.unwrap_or_else(default_current_ledger_path),
        json,
    })
}

fn parse_claim(args: &[String]) -> Result<ClaimArgs> {
    let mut common = CommonArgs {
        ledger_path: default_current_ledger_path(),
        json: false,
    };
    let mut session_id: Option<String> = None;
    let mut owner: Option<String> = None;
    let mut resource = None;
    let mut purpose: Option<String> = None;
    let mut mode = None;
    let mut lifecycle_phase = None;
    let mut policy_ref = None;
    let mut issue = None;
    let mut pull_request = None;
    let mut repository = None;
    let mut last_state = None;
    let mut branch = None;
    let mut worktree_path = None;
    let mut do_not_touch_paths = Vec::new();
    let mut blockers = Vec::new();
    let mut ttl_secs = DEFAULT_TTL_SECS;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => {
                common.json = true;
                i += 1;
            }
            "--ledger" => {
                common.ledger_path = PathBuf::from(take_value(args, i, "--ledger")?);
                i += 2;
            }
            "--session-id" => {
                session_id = Some(take_value(args, i, "--session-id")?.to_string());
                i += 2;
            }
            "--owner" => {
                owner = Some(take_value(args, i, "--owner")?.to_string());
                i += 2;
            }
            "--resource" => {
                resource = Some(parse_resource(take_value(args, i, "--resource")?)?);
                i += 2;
            }
            "--purpose" => {
                purpose = Some(take_value(args, i, "--purpose")?.to_string());
                i += 2;
            }
            "--mode" => {
                mode = Some(parse_claim_mode(take_value(args, i, "--mode")?)?);
                i += 2;
            }
            "--lifecycle-phase" => {
                lifecycle_phase = Some(take_value(args, i, "--lifecycle-phase")?.to_string());
                i += 2;
            }
            "--policy-ref" => {
                policy_ref = Some(take_value(args, i, "--policy-ref")?.to_string());
                i += 2;
            }
            "--issue" => {
                issue = Some(parse_u64(take_value(args, i, "--issue")?, "--issue")?);
                i += 2;
            }
            "--pr" => {
                pull_request = Some(parse_u64(take_value(args, i, "--pr")?, "--pr")?);
                i += 2;
            }
            "--repository" => {
                repository = Some(take_value(args, i, "--repository")?.to_string());
                i += 2;
            }
            "--last-state" => {
                last_state = Some(take_value(args, i, "--last-state")?.to_string());
                i += 2;
            }
            "--branch" => {
                branch = Some(take_value(args, i, "--branch")?.to_string());
                i += 2;
            }
            "--worktree" => {
                worktree_path = Some(take_value(args, i, "--worktree")?.to_string());
                i += 2;
            }
            "--do-not-touch" => {
                do_not_touch_paths.push(take_value(args, i, "--do-not-touch")?.to_string());
                i += 2;
            }
            "--blocker" => {
                blockers.push(take_value(args, i, "--blocker")?.to_string());
                i += 2;
            }
            "--ttl-secs" => {
                ttl_secs = parse_i64(take_value(args, i, "--ttl-secs")?, "--ttl-secs")?;
                i += 2;
            }
            other => {
                return Err(anyhow!(
                    "unknown session claim option '{other}'\n\n{}",
                    claim_usage()
                ));
            }
        }
    }

    Ok(ClaimArgs {
        common,
        input: ClaimInput {
            session_id: session_id.ok_or_else(|| anyhow!("--session-id is required"))?,
            owner: owner.ok_or_else(|| anyhow!("--owner is required"))?,
            resource: resource.ok_or_else(|| anyhow!("--resource is required"))?,
            purpose: purpose.ok_or_else(|| anyhow!("--purpose is required"))?,
            mode: mode.unwrap_or(adl::session_ledger::ClaimMode::Active),
            lifecycle_phase,
            policy_ref,
            github: GithubRef {
                issue,
                pull_request,
                repository,
                last_state,
            },
            branch,
            worktree_path,
            do_not_touch_paths,
            blockers,
            ttl_secs,
        },
    })
}

fn parse_claim_id_command(args: &[String]) -> Result<ClaimIdArgs> {
    let mut common = CommonArgs {
        ledger_path: default_current_ledger_path(),
        json: false,
    };
    let mut claim_id = None;
    let mut ttl_secs = DEFAULT_TTL_SECS;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => {
                common.json = true;
                i += 1;
            }
            "--ledger" => {
                common.ledger_path = PathBuf::from(take_value(args, i, "--ledger")?);
                i += 2;
            }
            "--claim-id" => {
                claim_id = Some(take_value(args, i, "--claim-id")?.to_string());
                i += 2;
            }
            "--ttl-secs" => {
                ttl_secs = parse_i64(take_value(args, i, "--ttl-secs")?, "--ttl-secs")?;
                i += 2;
            }
            other => {
                return Err(anyhow!(
                    "unknown session heartbeat option '{other}'\n\n{}",
                    heartbeat_usage()
                ));
            }
        }
    }
    Ok(ClaimIdArgs {
        common,
        claim_id: claim_id.ok_or_else(|| anyhow!("--claim-id is required"))?,
        ttl_secs,
    })
}

fn parse_release(args: &[String]) -> Result<ReleaseArgs> {
    let mut common = CommonArgs {
        ledger_path: default_current_ledger_path(),
        json: false,
    };
    let mut claim_id = None;
    let mut reason = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => {
                common.json = true;
                i += 1;
            }
            "--ledger" => {
                common.ledger_path = PathBuf::from(take_value(args, i, "--ledger")?);
                i += 2;
            }
            "--claim-id" => {
                claim_id = Some(take_value(args, i, "--claim-id")?.to_string());
                i += 2;
            }
            "--reason" => {
                reason = Some(take_value(args, i, "--reason")?.to_string());
                i += 2;
            }
            other => {
                return Err(anyhow!(
                    "unknown session release option '{other}'\n\n{}",
                    release_usage()
                ));
            }
        }
    }
    Ok(ReleaseArgs {
        common,
        claim_id: claim_id.ok_or_else(|| anyhow!("--claim-id is required"))?,
        reason,
    })
}

fn default_current_ledger_path() -> PathBuf {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    default_ledger_path(&discover_repo_root(&current_dir))
}

fn discover_repo_root(start: &Path) -> PathBuf {
    for candidate in start.ancestors() {
        if candidate.join(".git").exists()
            || (candidate.join("AGENTS.md").exists()
                && candidate.join("adl").join("Cargo.toml").exists())
        {
            return candidate.to_path_buf();
        }
    }
    start.to_path_buf()
}

fn help_requested(args: &[String]) -> bool {
    matches!(
        args.first().map(String::as_str),
        Some("--help" | "-h" | "help")
    )
}

fn take_value<'a>(args: &'a [String], index: usize, flag: &str) -> Result<&'a str> {
    args.get(index + 1)
        .map(String::as_str)
        .filter(|value| !value.starts_with("--"))
        .ok_or_else(|| anyhow!("{flag} requires a value"))
}

fn parse_u64(raw: &str, flag: &str) -> Result<u64> {
    raw.parse::<u64>()
        .with_context(|| format!("invalid {flag} value '{raw}'"))
}

fn parse_i64(raw: &str, flag: &str) -> Result<i64> {
    raw.parse::<i64>()
        .with_context(|| format!("invalid {flag} value '{raw}'"))
}

fn parse_claim_mode(raw: &str) -> Result<adl::session_ledger::ClaimMode> {
    let mode = parse_mode(Some(raw))?;
    match mode {
        adl::session_ledger::ClaimMode::Active
        | adl::session_ledger::ClaimMode::Watching
        | adl::session_ledger::ClaimMode::Paused => Ok(mode),
        adl::session_ledger::ClaimMode::Stale | adl::session_ledger::ClaimMode::Released => Err(
            anyhow!("claim mode '{raw}' cannot be set directly; use expiry or release"),
        ),
    }
}

fn session_usage() -> &'static str {
    "Usage:
  adl session status [--ledger <path>] [--json]
  adl session claim --session-id <id> --owner <name> --resource <kind:id> --purpose <text> [--issue <n>] [--pr <n>] [--branch <name>] [--worktree <path>] [--policy-ref <path>] [--lifecycle-phase <phase>] [--mode active|watching|paused] [--ttl-secs <n>] [--do-not-touch <path>]... [--blocker <text>]... [--ledger <path>] [--json]
  adl session heartbeat --claim-id <id> [--ttl-secs <n>] [--ledger <path>] [--json]
  adl session release --claim-id <id> [--reason <text>] [--ledger <path>] [--json]"
}

fn status_usage() -> &'static str {
    "Usage:
  adl session status [--ledger <path>] [--json]"
}

fn claim_usage() -> &'static str {
    "Usage:
  adl session claim --session-id <id> --owner <name> --resource <kind:id> --purpose <text> [--issue <n>] [--pr <n>] [--branch <name>] [--worktree <path>] [--policy-ref <path>] [--lifecycle-phase <phase>] [--mode active|watching|paused] [--ttl-secs <n>] [--do-not-touch <path>]... [--blocker <text>]... [--ledger <path>] [--json]"
}

fn heartbeat_usage() -> &'static str {
    "Usage:
  adl session heartbeat --claim-id <id> [--ttl-secs <n>] [--ledger <path>] [--json]"
}

fn release_usage() -> &'static str {
    "Usage:
  adl session release --claim-id <id> [--reason <text>] [--ledger <path>] [--json]"
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn claim_parser_accepts_generic_resource_and_csdlc_metadata() {
        let parsed = parse_claim(&[
            "--session-id".to_string(),
            "thread-a".to_string(),
            "--owner".to_string(),
            "codex".to_string(),
            "--resource".to_string(),
            "csdlc_issue:4412".to_string(),
            "--purpose".to_string(),
            "implement ledger".to_string(),
            "--issue".to_string(),
            "4412".to_string(),
            "--branch".to_string(),
            "codex/4412".to_string(),
            "--worktree".to_string(),
            ".worktrees/adl-wp-4412".to_string(),
            "--do-not-touch".to_string(),
            "docs/planning".to_string(),
        ])
        .expect("parse claim");
        assert_eq!(parsed.input.resource.kind, "csdlc_issue");
        assert_eq!(parsed.input.github.issue, Some(4412));
        assert_eq!(
            parsed.input.do_not_touch_paths,
            vec!["docs/planning".to_string()]
        );
    }

    #[test]
    fn claim_parser_rejects_lifecycle_outcome_modes() {
        let err = parse_claim(&[
            "--session-id".to_string(),
            "thread-a".to_string(),
            "--owner".to_string(),
            "codex".to_string(),
            "--resource".to_string(),
            "csdlc_issue:4412".to_string(),
            "--purpose".to_string(),
            "implement ledger".to_string(),
            "--mode".to_string(),
            "released".to_string(),
        ])
        .unwrap_err();
        assert!(err.to_string().contains("cannot be set directly"));
    }

    #[test]
    fn parse_common_accepts_json_and_custom_ledger() {
        let parsed = parse_common(&[
            "--ledger".to_string(),
            "tmp/session-ledger.json".to_string(),
            "--json".to_string(),
        ])
        .expect("parse common");
        assert_eq!(parsed.ledger_path, PathBuf::from("tmp/session-ledger.json"));
        assert!(parsed.json);
    }

    #[test]
    fn parse_common_rejects_unknown_options() {
        let err = parse_common(&["--bogus".to_string()]).unwrap_err();
        assert!(err.to_string().contains("unknown session status option"));
    }

    #[test]
    fn claim_parser_accepts_extended_optional_metadata() {
        let parsed = parse_claim(&[
            "--ledger".to_string(),
            "tmp/session-ledger.json".to_string(),
            "--json".to_string(),
            "--session-id".to_string(),
            "thread-b".to_string(),
            "--owner".to_string(),
            "watcher".to_string(),
            "--resource".to_string(),
            "worktree:adl-wp-4412".to_string(),
            "--purpose".to_string(),
            "watch checks".to_string(),
            "--mode".to_string(),
            "watching".to_string(),
            "--lifecycle-phase".to_string(),
            "pr_janitor".to_string(),
            "--policy-ref".to_string(),
            "docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md".to_string(),
            "--pr".to_string(),
            "4415".to_string(),
            "--repository".to_string(),
            "danielbaustin/agent-design-language".to_string(),
            "--last-state".to_string(),
            "waiting_for_checks".to_string(),
            "--ttl-secs".to_string(),
            "120".to_string(),
            "--blocker".to_string(),
            "adl-ci failing".to_string(),
            "--blocker".to_string(),
            "coverage below threshold".to_string(),
        ])
        .expect("parse extended claim");

        assert!(parsed.common.json);
        assert_eq!(
            parsed.common.ledger_path,
            PathBuf::from("tmp/session-ledger.json")
        );
        assert_eq!(parsed.input.mode, adl::session_ledger::ClaimMode::Watching);
        assert_eq!(parsed.input.lifecycle_phase.as_deref(), Some("pr_janitor"));
        assert_eq!(
            parsed.input.policy_ref.as_deref(),
            Some("docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md")
        );
        assert_eq!(parsed.input.github.pull_request, Some(4415));
        assert_eq!(
            parsed.input.github.repository.as_deref(),
            Some("danielbaustin/agent-design-language")
        );
        assert_eq!(
            parsed.input.github.last_state.as_deref(),
            Some("waiting_for_checks")
        );
        assert_eq!(parsed.input.ttl_secs, 120);
        assert_eq!(
            parsed.input.blockers,
            vec![
                "adl-ci failing".to_string(),
                "coverage below threshold".to_string()
            ]
        );
    }

    #[test]
    fn claim_parser_requires_required_fields() {
        let err = parse_claim(&[
            "--owner".to_string(),
            "codex".to_string(),
            "--resource".to_string(),
            "csdlc_issue:4412".to_string(),
            "--purpose".to_string(),
            "implement ledger".to_string(),
        ])
        .unwrap_err();
        assert!(err.to_string().contains("--session-id is required"));
    }

    #[test]
    fn claim_id_parser_accepts_json_ledger_and_ttl() {
        let parsed = parse_claim_id_command(&[
            "--ledger".to_string(),
            "tmp/session-ledger.json".to_string(),
            "--json".to_string(),
            "--claim-id".to_string(),
            "claim-123".to_string(),
            "--ttl-secs".to_string(),
            "45".to_string(),
        ])
        .expect("parse claim-id args");

        assert!(parsed.common.json);
        assert_eq!(
            parsed.common.ledger_path,
            PathBuf::from("tmp/session-ledger.json")
        );
        assert_eq!(parsed.claim_id, "claim-123");
        assert_eq!(parsed.ttl_secs, 45);
    }

    #[test]
    fn claim_id_parser_rejects_unknown_options() {
        let err = parse_claim_id_command(&[
            "--claim-id".to_string(),
            "claim-123".to_string(),
            "--bogus".to_string(),
        ])
        .unwrap_err();
        assert!(err.to_string().contains("unknown session heartbeat option"));
    }

    #[test]
    fn release_parser_accepts_reason_json_and_ledger() {
        let parsed = parse_release(&[
            "--ledger".to_string(),
            "tmp/session-ledger.json".to_string(),
            "--json".to_string(),
            "--claim-id".to_string(),
            "claim-123".to_string(),
            "--reason".to_string(),
            "checks passed".to_string(),
        ])
        .expect("parse release");

        assert!(parsed.common.json);
        assert_eq!(
            parsed.common.ledger_path,
            PathBuf::from("tmp/session-ledger.json")
        );
        assert_eq!(parsed.claim_id, "claim-123");
        assert_eq!(parsed.reason.as_deref(), Some("checks passed"));
    }

    #[test]
    fn release_parser_rejects_unknown_options() {
        let err = parse_release(&[
            "--claim-id".to_string(),
            "claim-123".to_string(),
            "--bogus".to_string(),
        ])
        .unwrap_err();
        assert!(err.to_string().contains("unknown session release option"));
    }

    #[test]
    fn parser_helpers_and_dispatch_reject_invalid_inputs() {
        assert!(help_requested(&["--help".to_string()]));
        assert!(help_requested(&["help".to_string()]));
        assert!(!help_requested(&["status".to_string()]));

        let missing = ["--ledger".to_string()];
        assert!(take_value(&missing, 0, "--ledger")
            .unwrap_err()
            .to_string()
            .contains("--ledger requires a value"));

        assert!(parse_u64("abc", "--issue")
            .unwrap_err()
            .to_string()
            .contains("invalid --issue value"));
        assert!(parse_i64("abc", "--ttl-secs")
            .unwrap_err()
            .to_string()
            .contains("invalid --ttl-secs value"));
        assert_eq!(
            parse_claim_mode("paused").expect("paused mode"),
            adl::session_ledger::ClaimMode::Paused
        );
        assert!(parse_claim_mode("released")
            .unwrap_err()
            .to_string()
            .contains("cannot be set directly"));
        assert!(real_session(&["bogus".to_string()])
            .unwrap_err()
            .to_string()
            .contains("unknown session command"));
    }

    #[test]
    fn repo_root_discovery_keeps_subdir_commands_on_shared_ledger() {
        let repo =
            std::env::temp_dir().join(format!("adl-session-repo-root-test-{}", std::process::id()));
        let subdir = repo.join("adl").join("src");
        fs::create_dir_all(&subdir).expect("create test repo dirs");
        fs::write(repo.join("AGENTS.md"), "# test\n").expect("write agents");
        fs::write(repo.join("adl").join("Cargo.toml"), "[package]\n").expect("write cargo");

        assert_eq!(discover_repo_root(&subdir), repo);
        let _ = fs::remove_dir_all(discover_repo_root(&subdir));
    }

    #[test]
    fn repo_root_discovery_falls_back_to_start_path_without_repo_markers() {
        let dir = std::env::temp_dir().join(format!(
            "adl-session-repo-root-fallback-{}",
            std::process::id()
        ));
        fs::create_dir_all(&dir).expect("create fallback dir");
        assert_eq!(discover_repo_root(&dir), dir);
        let _ = fs::remove_dir_all(&dir);
    }
}
