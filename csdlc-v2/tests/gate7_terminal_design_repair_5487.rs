use csdlc_v2::cards::digest;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
struct RepairRequest {
    source_design_path: String,
    source_diagram_path: String,
    expected_design_digest: String,
    expected_diagram_digest: String,
}

#[test]
fn repair_request_covers_committed_5467_artifacts_and_mermaid_shape() {
    let repo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let request: RepairRequest = serde_json::from_str(
        &std::fs::read_to_string(repo.join(".csdlc/requests/5487-repair-5467.json")).unwrap(),
    )
    .unwrap();
    for path in [&request.source_design_path, &request.source_diagram_path] {
        assert!(Path::new(path).is_relative());
        assert!(!path.contains(".."));
    }
    let design = std::fs::read(repo.join(&request.source_design_path)).unwrap();
    let diagram = std::fs::read_to_string(repo.join(&request.source_diagram_path)).unwrap();
    assert_eq!(digest(&design), request.expected_design_digest);
    assert_eq!(digest(diagram.as_bytes()), request.expected_diagram_digest);
    assert!(diagram.lines().next().unwrap().starts_with("flowchart "));
    assert!(diagram.lines().count() >= 2);
}
