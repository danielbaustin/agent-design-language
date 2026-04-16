use anyhow::{anyhow, Result};
use std::path::Path;

use super::contracts::{
    real_identity_adversarial_runner, real_identity_adversarial_runtime, real_identity_causality,
    real_identity_commitments, real_identity_continuity, real_identity_continuous_verification,
    real_identity_cost, real_identity_exploit_replay, real_identity_foundation,
    real_identity_instinct, real_identity_instinct_runtime, real_identity_phi,
    real_identity_red_blue_architecture, real_identity_retrieval, real_identity_schema,
};
use super::helpers::repo_root;
use super::profile::{real_identity_init, real_identity_now, real_identity_show};

pub(crate) fn real_identity(args: &[String]) -> Result<()> {
    let repo_root = repo_root()?;
    real_identity_in_repo(args, &repo_root)
}

pub(super) fn real_identity_in_repo(args: &[String], repo_root: &Path) -> Result<()> {
    let Some(subcommand) = args.first().map(|arg| arg.as_str()) else {
        return Err(anyhow!(
            "identity requires a subcommand: init | show | now | foundation | adversarial-runtime | red-blue-architecture | adversarial-runner | exploit-replay | continuous-verification | schema | continuity | retrieval | commitments | causality | cost | phi | instinct | instinct-runtime"
        ));
    };

    match subcommand {
        "init" => real_identity_init(repo_root, &args[1..]),
        "show" => real_identity_show(repo_root, &args[1..]),
        "now" => real_identity_now(repo_root, &args[1..]),
        "foundation" => real_identity_foundation(repo_root, &args[1..]),
        "adversarial-runtime" => real_identity_adversarial_runtime(repo_root, &args[1..]),
        "red-blue-architecture" => real_identity_red_blue_architecture(repo_root, &args[1..]),
        "adversarial-runner" => real_identity_adversarial_runner(repo_root, &args[1..]),
        "exploit-replay" => real_identity_exploit_replay(repo_root, &args[1..]),
        "continuous-verification" => real_identity_continuous_verification(repo_root, &args[1..]),
        "schema" => real_identity_schema(repo_root, &args[1..]),
        "continuity" => real_identity_continuity(repo_root, &args[1..]),
        "retrieval" => real_identity_retrieval(repo_root, &args[1..]),
        "commitments" => real_identity_commitments(repo_root, &args[1..]),
        "causality" => real_identity_causality(repo_root, &args[1..]),
        "cost" => real_identity_cost(repo_root, &args[1..]),
        "phi" => real_identity_phi(repo_root, &args[1..]),
        "instinct" => real_identity_instinct(repo_root, &args[1..]),
        "instinct-runtime" => real_identity_instinct_runtime(repo_root, &args[1..]),
        "--help" | "-h" | "help" => {
            println!("{}", super::super::usage::usage());
            Ok(())
        }
        _ => Err(anyhow!(
            "unknown identity subcommand '{subcommand}' (expected init | show | now | foundation | adversarial-runtime | red-blue-architecture | adversarial-runner | exploit-replay | continuous-verification | schema | continuity | retrieval | commitments | causality | cost | phi | instinct | instinct-runtime)"
        )),
    }
}
