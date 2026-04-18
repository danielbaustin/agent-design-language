use super::*;

#[test]
fn identity_requires_subcommand_and_rejects_unknown_subcommand() {
    let repo = temp_repo("identity-subcommands");

    let err = real_identity_in_repo(&[], &repo).expect_err("missing subcommand should fail");
    assert!(err
        .to_string()
        .contains("identity requires a subcommand: init | show | now | foundation | adversarial-runtime | red-blue-architecture | adversarial-runner | exploit-replay | continuous-verification | operational-skills | skill-composition | delegation-refusal-coordination | provider-extension-packaging | demo-proof-entry-points | schema"));
    assert!(err.to_string().contains("continuity"));

    let err = real_identity_in_repo(&["nope".to_string()], &repo)
        .expect_err("unknown subcommand should fail");
    assert!(err
        .to_string()
        .contains("unknown identity subcommand 'nope'"));
}

#[test]
fn identity_top_level_help_and_subcommand_help_succeed() {
    let repo = temp_repo("identity-help");

    real_identity_in_repo(&["help".to_string()], &repo).expect("top-level help");
    real_identity_in_repo(&["init".to_string(), "--help".to_string()], &repo).expect("init help");
    real_identity_in_repo(&["now".to_string(), "--help".to_string()], &repo).expect("now help");
    real_identity_in_repo(&["foundation".to_string(), "--help".to_string()], &repo)
        .expect("foundation help");
    real_identity_in_repo(
        &["adversarial-runtime".to_string(), "--help".to_string()],
        &repo,
    )
    .expect("adversarial-runtime help");
    real_identity_in_repo(
        &["red-blue-architecture".to_string(), "--help".to_string()],
        &repo,
    )
    .expect("red-blue-architecture help");
    real_identity_in_repo(
        &["adversarial-runner".to_string(), "--help".to_string()],
        &repo,
    )
    .expect("adversarial-runner help");
    real_identity_in_repo(&["exploit-replay".to_string(), "--help".to_string()], &repo)
        .expect("exploit-replay help");
    real_identity_in_repo(
        &["continuous-verification".to_string(), "--help".to_string()],
        &repo,
    )
    .expect("continuous-verification help");
    real_identity_in_repo(
        &["demo-proof-entry-points".to_string(), "--help".to_string()],
        &repo,
    )
    .expect("demo-proof-entry-points help");
    real_identity_in_repo(
        &["operational-skills".to_string(), "--help".to_string()],
        &repo,
    )
    .expect("operational-skills help");
    real_identity_in_repo(
        &["skill-composition".to_string(), "--help".to_string()],
        &repo,
    )
    .expect("skill-composition help");
    real_identity_in_repo(&["schema".to_string(), "--help".to_string()], &repo)
        .expect("schema help");
    real_identity_in_repo(&["continuity".to_string(), "--help".to_string()], &repo)
        .expect("continuity help");
}
