use std::fs;
use std::path::Path;

use csdlc_v2::LifecyclePhase;

fn repo() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repository root")
}

#[test]
fn competing_closeout_binary_and_skill_are_absent() {
    let root = repo();
    assert!(!root.join("csdlc-v2/src/bin/csdlc-closeout.rs").exists());
    assert!(!root.join("csdlc-v2/src/bin/csdlc-merge.rs").exists());
    assert!(!root.join("csdlc-v2/src/merge.rs").exists());
    assert!(!root
        .join("csdlc-v2/operator/skills/csdlc-v2-closeout/SKILL.md")
        .exists());

    let cargo = fs::read_to_string(root.join("csdlc-v2/Cargo.toml")).unwrap();
    let skills = fs::read_to_string(root.join("csdlc-v2/operator/skills.json")).unwrap();
    let coexistence = fs::read_to_string(root.join("csdlc-v2/operator/coexistence.json")).unwrap();
    let agent_contract = fs::read_to_string(root.join("csdlc-v2/AGENTS.md")).unwrap();
    let install_docs =
        fs::read_to_string(root.join("docs/tooling/OWNER_BINARY_INSTALLATION.md")).unwrap();
    let github_boundary =
        fs::read_to_string(root.join("docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md")).unwrap();
    let pr_tail_playbook =
        fs::read_to_string(root.join("docs/tooling/C_SDLC_V2_V1_ORIGIN_PR_TAIL_PLAYBOOK.md"))
            .unwrap();
    let editor_adapter =
        fs::read_to_string(root.join("docs/tooling/editor/command_adapter.md")).unwrap();
    let attach_post_merge =
        fs::read_to_string(root.join("adl/tools/attach_post_merge_closeout.sh")).unwrap();
    let main_sync =
        fs::read_to_string(root.join("adl/tools/fix_git_main_sync_preserve_local_adl.sh")).unwrap();
    let active_command_scan =
        fs::read_to_string(root.join("adl/tools/generate_active_command_reference_scan.py"))
            .unwrap();
    let editor_action = fs::read_to_string(root.join("adl/tools/editor_action.sh")).unwrap();
    let operational_skills =
        fs::read_to_string(root.join("adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md")).unwrap();
    let gate4 = fs::read_to_string(root.join("csdlc-v2/tests/gate4.rs")).unwrap();
    for surface in [
        cargo,
        skills,
        coexistence,
        agent_contract,
        install_docs,
        github_boundary,
        pr_tail_playbook,
        editor_adapter,
        attach_post_merge,
        main_sync,
        active_command_scan,
        editor_action,
        operational_skills,
        gate4,
    ] {
        assert!(!surface.contains("csdlc-closeout"));
        assert!(!surface.contains("csdlc-merge"));
    }
}

#[test]
fn finish_is_the_only_public_merge_authority() {
    let root = repo();
    let library = fs::read_to_string(root.join("csdlc-v2/src/lib.rs")).unwrap();
    let schemas = fs::read_to_string(root.join("csdlc-v2/src/schema.rs")).unwrap();
    let finish = fs::read_to_string(root.join("csdlc-v2/src/finish.rs")).unwrap();

    assert!(!library.contains("pub mod merge"));
    assert!(!library.contains("MergeRequest"));
    assert!(!library.contains("MergeResult"));
    assert!(!schemas.contains("merge_request"));
    assert!(!schemas.contains("merge_result"));
    assert!(finish.contains("pub async fn execute_finish"));
    assert!(!finish.contains("pub async fn execute_remote_merge"));
}

#[test]
fn publication_and_store_expose_no_terminal_mutation_route() {
    let root = repo();
    let publish = fs::read_to_string(root.join("csdlc-v2/src/bin/csdlc-publish.rs")).unwrap();
    let store = fs::read_to_string(root.join("csdlc-v2/src/store.rs")).unwrap();
    let model = fs::read_to_string(root.join("csdlc-v2/src/model.rs")).unwrap();

    for removed in [
        "ReconcileMerged",
        "ReconcileReady",
        "record_readiness",
        "commit_terminal",
        "retain_terminal_receipt",
        "reconcile_terminal(",
        "repair_terminal_",
        "TerminalReceiptTransportRequest",
        "ReconcileTerminalRequest",
    ] {
        assert!(
            !publish.contains(removed) && !store.contains(removed) && !model.contains(removed),
            "removed terminal writer remains reachable: {removed}"
        );
    }
}

#[test]
fn historical_phase_and_receipt_shapes_remain_readable_only() {
    for (encoded, expected) in [
        ("\"merge_ready\"", LifecyclePhase::MergeReady),
        ("\"merged\"", LifecyclePhase::Merged),
        ("\"closed_out\"", LifecyclePhase::ClosedOut),
    ] {
        assert_eq!(
            serde_json::from_str::<LifecyclePhase>(encoded).unwrap(),
            expected
        );
    }

    assert!(!LifecyclePhase::Published.allows(LifecyclePhase::MergeReady));
    assert!(!LifecyclePhase::MergeReady.allows(LifecyclePhase::Merged));
    assert!(!LifecyclePhase::Merged.allows(LifecyclePhase::ClosedOut));

    let schemas = csdlc_v2::public_schema_bundle();
    assert!(schemas.get("terminal_receipt").is_some());
    assert!(schemas.get("finish_request").is_some());
    assert!(schemas.get("derived_terminal_envelope").is_some());
    assert!(schemas.get("terminal_reconciliation_request").is_none());
    assert!(schemas.get("terminal_receipt_transport_request").is_none());
}
