use super::*;

#[test]
fn select_open_artifact_returns_none_without_html() {
    let artifacts = vec![PathBuf::from("out/one.txt"), PathBuf::from("out/two.md")];
    assert!(select_open_artifact(&artifacts).is_none());
}

#[test]
fn run_artifacts_root_points_to_repo_adl_runs() {
    let _runs_guard = EnvGuard::set("ADL_RUNS_ROOT", "");
    let root = artifacts::runs_root().expect("run artifacts root");
    let s = root.to_string_lossy();
    assert!(s.ends_with(".adl/runs"), "unexpected path: {s}");
}

#[test]
fn enforce_signature_policy_skips_when_not_running_or_not_v0_5() {
    let mk_doc = |version: &str| adl::AdlDoc {
        version: version.to_string(),
        providers: HashMap::new(),
        tools: HashMap::new(),
        agents: HashMap::new(),
        tasks: HashMap::new(),
        workflows: HashMap::new(),
        patterns: vec![],
        signature: None,
        run: adl::RunSpec {
            id: None,
            name: None,
            created_at: None,
            defaults: adl::RunDefaults::default(),
            workflow_ref: None,
            workflow: Some(adl::WorkflowSpec {
                id: None,
                kind: adl::WorkflowKind::Sequential,
                max_concurrency: None,
                steps: vec![],
            }),
            pattern_ref: None,
            inputs: HashMap::new(),
            placement: None,
            remote: None,
            delegation_policy: None,
        },
    };

    enforce_signature_policy(&mk_doc("0.5"), false, false, None, false)
        .expect("do_run=false should skip");
    enforce_signature_policy(&mk_doc("0.4"), true, false, None, false).expect("v0.4 should skip");
    enforce_signature_policy(&mk_doc("0.5"), true, true, None, false)
        .expect("allow_unsigned should skip");
}
