use super::support::*;
use super::*;

#[test]
fn public_prompt_packet_export_writes_manifest_readme_and_cards() {
    let repo = TempRepo::new("public-prompt-packet");
    let source = repo
        .path()
        .join(".adl/v0.91.5/tasks/issue-3472__public-card-export");
    render_sample_cards(&repo, &source);

    real_tooling(&[
        "public-prompt-packet".to_string(),
        "export".to_string(),
        "--repo-root".to_string(),
        repo.path().to_string_lossy().to_string(),
        "--issue".to_string(),
        "3472".to_string(),
        "--slug".to_string(),
        "public-card-export".to_string(),
        "--version".to_string(),
        "v0.91.5".to_string(),
        "--tracker-url".to_string(),
        "https://github.com/danielbaustin/agent-design-language/issues/3472".to_string(),
    ])
    .expect("export public prompt packet");

    let packet = repo
        .path()
        .join("docs/milestones/v0.91.5/review/evidence/csdlc/issues/issue-3472-public-card-export");
    assert!(packet.join("README.md").is_file());
    assert!(packet.join("cards/sip.md").is_file());
    assert!(packet.join("cards/stp.md").is_file());
    assert!(packet.join("cards/spp.md").is_file());
    assert!(packet.join("cards/srp.md").is_file());
    assert!(packet.join("cards/sor.md").is_file());

    let manifest: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(packet.join("manifest.json")).unwrap()).unwrap();
    assert_eq!(manifest["schema"], "adl.public_prompt_packet.v1");
    assert_eq!(manifest["version"], "v0.91.5");
    assert_eq!(manifest["tracker"]["provider"], "github");
    assert_eq!(manifest["tracker"]["issue_number"], 3472);
    assert_eq!(manifest["work_item"]["id"], "issue-3472");
    assert_eq!(manifest["work_item"]["slug"], "public-card-export");
    assert_eq!(manifest["cards"].as_array().unwrap().len(), 5);
    assert_eq!(manifest["cards"][0]["template_version"], "1.0.0");
    assert_eq!(manifest["cards"][0]["card_status"], "ready");
    assert_eq!(manifest["redaction"]["status"], "passed");

    for card in manifest["cards"].as_array().unwrap() {
        let public = card["public_rel_path"].as_str().unwrap();
        let source = card["source_rel_path"].as_str().unwrap();
        assert!(!public.starts_with('/'));
        assert!(!source.starts_with('/'));
        assert!(!public.contains(".adl/"));
    }

    fs::write(packet.join("stale.txt"), "stale output should be removed").unwrap();
    real_tooling(&[
        "public-prompt-packet".to_string(),
        "export".to_string(),
        "--repo-root".to_string(),
        repo.path().to_string_lossy().to_string(),
        "--issue".to_string(),
        "3472".to_string(),
        "--slug".to_string(),
        "public-card-export".to_string(),
        "--version".to_string(),
        "v0.91.5".to_string(),
    ])
    .expect("repeat export should replace packet directory");
    assert!(!packet.join("stale.txt").exists());
}

#[test]
fn public_prompt_packet_export_refuses_host_paths_and_secret_markers() {
    let repo = TempRepo::new("public-prompt-packet-refuse");
    let source = repo
        .path()
        .join(".adl/v0.91.5/tasks/issue-3472__unsafe-card-export");
    render_sample_cards(&repo, &source);
    append_to_card(&source, "sip", "Leaked path: /Users/example/private\n");

    let err = real_tooling(&[
        "public-prompt-packet".to_string(),
        "export".to_string(),
        "--repo-root".to_string(),
        repo.path().to_string_lossy().to_string(),
        "--issue".to_string(),
        "3472".to_string(),
        "--slug".to_string(),
        "unsafe-card-export".to_string(),
        "--version".to_string(),
        "v0.91.5".to_string(),
    ])
    .expect_err("host path should fail closed");
    assert!(err
        .to_string()
        .contains("contains disallowed public-packet content"));

    let source = repo
        .path()
        .join(".adl/v0.91.5/tasks/issue-3473__secret-card-export");
    render_sample_cards(&repo, &source);
    append_to_card(&source, "sip", "Leaked token: ghp_exampletoken\n");

    let err = real_tooling(&[
        "public-prompt-packet".to_string(),
        "export".to_string(),
        "--repo-root".to_string(),
        repo.path().to_string_lossy().to_string(),
        "--issue".to_string(),
        "3473".to_string(),
        "--slug".to_string(),
        "secret-card-export".to_string(),
        "--version".to_string(),
        "v0.91.5".to_string(),
    ])
    .expect_err("secret marker should fail closed");
    assert!(err
        .to_string()
        .contains("contains disallowed public-packet content"));
}

fn render_sample_cards(repo: &TempRepo, source: &Path) {
    let values_dir = repo.path().join("values");
    real_tooling(&[
        "prompt-template".to_string(),
        "write-sample-values".to_string(),
        "--out-dir".to_string(),
        values_dir.to_string_lossy().to_string(),
    ])
    .expect("write sample values");

    real_tooling(&[
        "prompt-template".to_string(),
        "render-all".to_string(),
        "--repo-root".to_string(),
        repo_root_for_tests().to_string_lossy().to_string(),
        "--values-dir".to_string(),
        values_dir.to_string_lossy().to_string(),
        "--out-dir".to_string(),
        source.to_string_lossy().to_string(),
    ])
    .expect("render public packet source cards");
}

fn append_to_card(source: &Path, kind: &str, text: &str) {
    let path = source.join(format!("{kind}.md"));
    let mut existing = fs::read_to_string(&path).expect("read card");
    existing.push_str(text);
    fs::write(path, existing).expect("append card text");
}
