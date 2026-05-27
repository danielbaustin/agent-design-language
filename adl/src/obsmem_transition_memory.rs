use std::fs;
use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::obsmem_contract::{
    MemoryCitation, MemoryFollowOnRef, MemoryReviewFinding, MemoryTraceRef, MemoryWriteRequest,
    ObsMemContractError, ObsMemContractErrorCode, OBSMEM_CONTRACT_VERSION,
};
use crate::signing;

const TRANSITION_MEMORY_HANDOFF_SCHEMA_VERSION: u32 = 1;
const TRANSITION_OUTCOME_TRUTH_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionMemoryFollowOn {
    pub issue_number: u64,
    pub title: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionMemoryHandoff {
    pub schema_version: u32,
    pub handoff_id: String,
    pub workflow_id: String,
    pub outcome_truth_path: String,
    pub evidence_bundle_path: String,
    pub review_synthesis_path: String,
    pub signed_trace_path: String,
    pub signed_trace_public_key_path: String,
    #[serde(default)]
    pub follow_ons: Vec<TransitionMemoryFollowOn>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TransitionOutcomeTruth {
    schema_version: u32,
    issue_number: u64,
    pr_number: u64,
    branch: String,
    lifecycle_state: String,
    outcome_summary: String,
    #[serde(default)]
    outcome_facts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReviewSynthesis {
    synthesis_id: String,
    source_issue_number: u64,
    source_pr_number: u64,
    summary: String,
    findings: Vec<ReviewSynthesisFinding>,
    #[serde(default)]
    residual_risks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReviewSynthesisFinding {
    severity: String,
    id: String,
    summary: String,
    disposition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EvidenceBundle {
    bundle_id: String,
    version: String,
    issue_number: u64,
    evidence_inputs: Vec<EvidenceInput>,
    signed_trace: SignedTraceMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EvidenceInput {
    kind: String,
    path: String,
    sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SignedTraceMetadata {
    unsigned_path: String,
    signed_path: String,
    public_key_path: String,
    verification_mode: String,
}

pub fn build_write_request_from_transition_handoff(
    repo_root: &Path,
    handoff_path: &Path,
) -> Result<MemoryWriteRequest, ObsMemContractError> {
    let handoff: TransitionMemoryHandoff = read_json(handoff_path)?;
    if handoff.schema_version != TRANSITION_MEMORY_HANDOFF_SCHEMA_VERSION {
        return Err(ObsMemContractError::new(
            ObsMemContractErrorCode::ContractVersionMismatch,
            format!(
                "unsupported transition memory handoff schema version {} (expected {})",
                handoff.schema_version, TRANSITION_MEMORY_HANDOFF_SCHEMA_VERSION
            ),
        ));
    }
    if handoff.handoff_id.trim().is_empty() || handoff.workflow_id.trim().is_empty() {
        return Err(ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            "handoff_id and workflow_id must be non-empty",
        ));
    }

    let outcome_truth_path = resolve_repo_relative(repo_root, &handoff.outcome_truth_path)?;
    let evidence_bundle_path = resolve_repo_relative(repo_root, &handoff.evidence_bundle_path)?;
    let review_synthesis_path = resolve_repo_relative(repo_root, &handoff.review_synthesis_path)?;
    let signed_trace_path = resolve_repo_relative(repo_root, &handoff.signed_trace_path)?;
    let signed_trace_public_key_path =
        resolve_repo_relative(repo_root, &handoff.signed_trace_public_key_path)?;

    let outcome_truth: TransitionOutcomeTruth = read_json(&outcome_truth_path)?;
    if outcome_truth.schema_version != TRANSITION_OUTCOME_TRUTH_SCHEMA_VERSION {
        return Err(ObsMemContractError::new(
            ObsMemContractErrorCode::ContractVersionMismatch,
            format!(
                "unsupported transition outcome truth schema version {} (expected {})",
                outcome_truth.schema_version, TRANSITION_OUTCOME_TRUTH_SCHEMA_VERSION
            ),
        ));
    }
    let review_synthesis: ReviewSynthesis = read_json(&review_synthesis_path)?;
    let evidence_bundle: EvidenceBundle = read_json(&evidence_bundle_path)?;

    if review_synthesis.source_issue_number != outcome_truth.issue_number
        || review_synthesis.source_pr_number != outcome_truth.pr_number
    {
        return Err(ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            "review synthesis source issue/PR must match transition outcome truth",
        ));
    }
    if evidence_bundle.issue_number != outcome_truth.issue_number {
        return Err(ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            "evidence bundle issue_number must match transition outcome truth issue_number",
        ));
    }
    verify_signed_trace_binding(
        repo_root,
        &evidence_bundle.signed_trace,
        &handoff.signed_trace_path,
        &handoff.signed_trace_public_key_path,
    )?;
    verify_signed_trace_signature(&signed_trace_path, &signed_trace_public_key_path)?;

    let mut citations = vec![
        citation_for_path(&outcome_truth_path, &handoff.outcome_truth_path)?,
        citation_for_path(&evidence_bundle_path, &handoff.evidence_bundle_path)?,
        citation_for_path(&review_synthesis_path, &handoff.review_synthesis_path)?,
        citation_for_path(&signed_trace_path, &handoff.signed_trace_path)?,
        citation_for_path(
            &signed_trace_public_key_path,
            &handoff.signed_trace_public_key_path,
        )?,
    ];
    for input in evidence_bundle.evidence_inputs {
        let path = resolve_repo_relative(repo_root, &input.path)?;
        verify_evidence_input_digest(&path, &input.path, &input.sha256)?;
        citations.push(citation_for_path(&path, &input.path)?);
    }

    let mut summary_parts = vec![
        format!(
            "transition_issue=#{} pr=#{} lifecycle_state={}",
            outcome_truth.issue_number, outcome_truth.pr_number, outcome_truth.lifecycle_state
        ),
        outcome_truth.outcome_summary,
        review_synthesis.summary,
    ];
    summary_parts.extend(outcome_truth.outcome_facts);
    let summary = summary_parts.join(" | ");

    let mut request = MemoryWriteRequest {
        contract_version: OBSMEM_CONTRACT_VERSION,
        run_id: handoff.handoff_id.clone(),
        workflow_id: handoff.workflow_id,
        trace_bundle_rel_path: handoff.signed_trace_path,
        activation_log_rel_path: handoff.outcome_truth_path,
        failure_code: None,
        summary,
        tags: vec![
            "memory:csdlc-transition".to_string(),
            format!("issue:{}", outcome_truth.issue_number),
            format!("pr:{}", outcome_truth.pr_number),
            format!("lifecycle:{}", outcome_truth.lifecycle_state),
            format!("review_synthesis:{}", review_synthesis.synthesis_id),
            format!("evidence_bundle:{}", evidence_bundle.bundle_id),
        ],
        citations,
        trace_event_refs: vec![MemoryTraceRef {
            event_sequence: 0,
            event_kind: "signed_trace_verified".to_string(),
            step_id: Some("csdlc_transition_memory".to_string()),
            delegation_id: None,
        }],
        review_findings: review_synthesis
            .findings
            .into_iter()
            .map(|finding| MemoryReviewFinding {
                id: finding.id,
                severity: finding.severity,
                summary: finding.summary,
                disposition: finding.disposition,
            })
            .collect(),
        residual_risks: review_synthesis.residual_risks,
        follow_on_refs: handoff
            .follow_ons
            .into_iter()
            .map(|follow_on| MemoryFollowOnRef {
                issue_number: follow_on.issue_number,
                title: follow_on.title,
                status: follow_on.status,
            })
            .collect(),
    };
    request.normalize();
    request.validate()?;
    Ok(request)
}

fn resolve_repo_relative(repo_root: &Path, rel: &str) -> Result<PathBuf, ObsMemContractError> {
    validate_repo_relative(rel)?;
    let full = repo_root.join(rel);
    if !full.exists() {
        return Err(ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            format!("tracked handoff path does not exist: {rel}"),
        ));
    }
    Ok(full)
}

fn verify_signed_trace_binding(
    repo_root: &Path,
    signed_trace: &SignedTraceMetadata,
    handoff_signed_trace_path: &str,
    handoff_public_key_path: &str,
) -> Result<(), ObsMemContractError> {
    validate_repo_relative(&signed_trace.signed_path)?;
    validate_repo_relative(&signed_trace.public_key_path)?;
    resolve_repo_relative(repo_root, &signed_trace.signed_path)?;
    resolve_repo_relative(repo_root, &signed_trace.public_key_path)?;
    if signed_trace.signed_path != handoff_signed_trace_path {
        return Err(ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            "handoff signed_trace_path must match evidence bundle signed_trace.signed_path",
        ));
    }
    if signed_trace.public_key_path != handoff_public_key_path {
        return Err(ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            "handoff signed_trace_public_key_path must match evidence bundle signed_trace.public_key_path",
        ));
    }
    if signed_trace.verification_mode != "explicit_key" {
        return Err(ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            "evidence bundle signed trace verification_mode must be explicit_key",
        ));
    }
    Ok(())
}

fn verify_signed_trace_signature(
    signed_trace_path: &Path,
    signed_trace_public_key_path: &Path,
) -> Result<(), ObsMemContractError> {
    signing::verify_file(signed_trace_path, Some(signed_trace_public_key_path)).map_err(|err| {
        ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            format!(
                "signed trace verification failed for '{}' with key '{}': {err:#}",
                signed_trace_path.display(),
                signed_trace_public_key_path.display()
            ),
        )
    })
}

fn verify_evidence_input_digest(
    path: &Path,
    rel_path: &str,
    expected_sha256: &str,
) -> Result<(), ObsMemContractError> {
    if expected_sha256.len() != 64 || !expected_sha256.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            format!("evidence input sha256 must be a 64-character hexadecimal digest: {rel_path}"),
        ));
    }
    let bytes = fs::read(path).map_err(|err| {
        ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            format!("failed reading evidence input '{}': {err}", path.display()),
        )
    })?;
    let actual = format!("{:x}", Sha256::digest(&bytes));
    if actual != expected_sha256 {
        return Err(ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            format!("evidence input sha256 mismatch for {rel_path}"),
        ));
    }
    Ok(())
}

fn validate_repo_relative(rel: &str) -> Result<(), ObsMemContractError> {
    let path = Path::new(rel);
    if rel.trim().is_empty() || path.is_absolute() {
        return Err(ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            format!("path must be non-empty and repo-relative: {rel}"),
        ));
    }
    if rel.starts_with(".adl/") {
        return Err(ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            "tracked handoff must not depend on local-only .adl state",
        ));
    }
    for component in path.components() {
        if matches!(component, Component::ParentDir) {
            return Err(ObsMemContractError::new(
                ObsMemContractErrorCode::InvalidRequest,
                format!("repo-relative path must not traverse parents: {rel}"),
            ));
        }
    }
    Ok(())
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, ObsMemContractError> {
    let bytes = fs::read(path).map_err(|err| {
        ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            format!("failed reading '{}': {err}", path.display()),
        )
    })?;
    serde_json::from_slice(&bytes).map_err(|err| {
        ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            format!("failed parsing '{}': {err}", path.display()),
        )
    })
}

fn citation_for_path(path: &Path, rel_path: &str) -> Result<MemoryCitation, ObsMemContractError> {
    let bytes = fs::read(path).map_err(|err| {
        ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            format!("failed reading citation source '{}': {err}", path.display()),
        )
    })?;
    Ok(MemoryCitation {
        path: rel_path.to_string(),
        hash: stable_fingerprint_hex(&bytes),
    })
}

fn stable_fingerprint_hex(bytes: &[u8]) -> String {
    let mut acc: u64 = 0xcbf2_9ce4_8422_2325;
    for (idx, b) in bytes.iter().enumerate() {
        acc ^= (*b as u64).wrapping_add(idx as u64);
        acc = acc.wrapping_mul(0x100_0000_01b3);
    }
    format!("det64:{acc:016x}")
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    fn unique_temp_dir(label: &str) -> PathBuf {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "adl-transition-memory-{label}-pid{}-{n}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    fn write(path: &Path, body: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("mkdir parent");
        }
        fs::write(path, body).expect("write fixture");
    }

    fn copy_tracked_signed_trace_fixture(root: &Path) {
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("repo root");
        let tracked_signed = repo_root.join(
            "docs/milestones/v0.91.4/review/evidence/csdlc/fixtures/minimal_transition_trace_signed.adl.yaml",
        );
        let tracked_unsigned = repo_root.join(
            "docs/milestones/v0.91.4/review/evidence/csdlc/fixtures/minimal_transition_trace_unsigned.adl.yaml",
        );
        let tracked_public = repo_root.join(
            "docs/milestones/v0.91.4/review/evidence/csdlc/fixtures/minimal_transition_trace_public_key.b64",
        );
        fs::copy(
            &tracked_signed,
            root.join("docs/review/trace_signed.adl.yaml"),
        )
        .expect("copy signed trace");
        fs::copy(
            &tracked_unsigned,
            root.join("docs/review/trace_unsigned.adl.yaml"),
        )
        .expect("copy unsigned trace");
        fs::copy(&tracked_public, root.join("docs/review/trace_key.b64")).expect("copy trace key");
    }

    fn write_fixture_repo(root: &Path) -> PathBuf {
        write(
            &root.join("docs/review/source_packet.md"),
            "# source packet\n",
        );
        let source_packet_sha256 = format!(
            "{:x}",
            Sha256::digest(
                fs::read(root.join("docs/review/source_packet.md")).expect("read source packet")
            )
        );
        write(
            &root.join("docs/review/evidence.json"),
            &format!(
                r#"{{
  "bundle_id": "bundle-1",
  "version": "v0.91.4",
  "issue_number": 3354,
  "evidence_inputs": [
    {{
      "kind": "proof_packet",
      "path": "docs/review/source_packet.md",
      "sha256": "{source_packet_sha256}"
    }}
  ],
  "signed_trace": {{
    "unsigned_path": "docs/review/trace_unsigned.adl.yaml",
    "signed_path": "docs/review/trace_signed.adl.yaml",
    "public_key_path": "docs/review/trace_key.b64",
    "verification_mode": "explicit_key"
  }}
}}"#
            ),
        );
        write(
            &root.join("docs/review/review_synthesis.json"),
            r#"{
  "synthesis_id": "synth-1",
  "source_issue_number": 3354,
  "source_pr_number": 3388,
  "summary": "bounded review summary",
  "findings": [
    {
      "severity": "P2",
      "id": "finding-1",
      "summary": "proof mismatch fixed",
      "disposition": "fixed"
    }
  ],
  "residual_risks": ["later release work remains"]
}"#,
        );
        write(
            &root.join("docs/review/outcome_truth.json"),
            r#"{
  "schema_version": 1,
  "issue_number": 3354,
  "pr_number": 3388,
  "branch": "codex/3354-example",
  "lifecycle_state": "merged",
  "outcome_summary": "WP-06 converged tracked evidence",
  "outcome_facts": [
    "evidence bundle is tracked",
    "signed trace verifies"
  ]
}"#,
        );
        copy_tracked_signed_trace_fixture(root);
        let handoff = root.join("docs/review/handoff.json");
        write(
            &handoff,
            r#"{
  "schema_version": 1,
  "handoff_id": "handoff-1",
  "workflow_id": "csdlc_transition_memory",
  "outcome_truth_path": "docs/review/outcome_truth.json",
  "evidence_bundle_path": "docs/review/evidence.json",
  "review_synthesis_path": "docs/review/review_synthesis.json",
  "signed_trace_path": "docs/review/trace_signed.adl.yaml",
  "signed_trace_public_key_path": "docs/review/trace_key.b64",
  "follow_ons": [
    {
      "issue_number": 3355,
      "title": "Merge gate",
      "status": "merged"
    },
    {
      "issue_number": 3356,
      "title": "ObsMem transition memory",
      "status": "active"
    }
  ],
  "notes": ["tracked only"]
}"#,
        );
        handoff
    }

    #[test]
    fn transition_handoff_build_is_deterministic_and_structured() {
        let root = unique_temp_dir("deterministic");
        let handoff = write_fixture_repo(&root);

        let left = build_write_request_from_transition_handoff(&root, &handoff).expect("left");
        let right = build_write_request_from_transition_handoff(&root, &handoff).expect("right");

        assert_eq!(left, right);
        assert_eq!(left.review_findings.len(), 1);
        assert_eq!(left.residual_risks, vec!["later release work remains"]);
        assert_eq!(left.follow_on_refs.len(), 2);
        assert_eq!(left.run_id, "handoff-1");
        assert!(left
            .citations
            .iter()
            .any(|citation| citation.path == "docs/review/outcome_truth.json"));
    }

    #[test]
    fn transition_handoff_rejects_local_only_adl_inputs() {
        let root = unique_temp_dir("local-only");
        let handoff = write_fixture_repo(&root);
        let body = fs::read_to_string(&handoff).expect("read handoff");
        fs::write(
            &handoff,
            body.replace(
                "\"docs/review/outcome_truth.json\"",
                "\".adl/v0.91.4/tasks/local-only.json\"",
            ),
        )
        .expect("rewrite handoff");

        let err = build_write_request_from_transition_handoff(&root, &handoff)
            .expect_err(".adl path must fail");
        assert_eq!(err.code.as_str(), "OBSMEM_INVALID_REQUEST");
        assert!(err.message.contains("local-only .adl"));
    }

    #[test]
    fn tracked_transition_handoff_packet_builds_successfully() {
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("repo root");
        let handoff = repo_root.join(
            "docs/milestones/v0.91.4/review/obsmem_transition_memory/ct_demo_001_obsmem_transition_memory_handoff.json",
        );

        let request =
            build_write_request_from_transition_handoff(repo_root, &handoff).expect("request");
        assert_eq!(
            request.run_id,
            "ct_demo_001_obsmem_transition_memory_handoff"
        );
        assert_eq!(request.workflow_id, "v0914_csdlc_transition_memory");
        assert!(request
            .review_findings
            .iter()
            .any(|finding| finding.id == "wp05-review-001"));
        assert!(request.citations.iter().any(|citation| {
            citation.path
                == "docs/milestones/v0.91.4/review/obsmem_transition_memory/ct_demo_001_transition_outcome_truth.json"
        }));
    }

    #[test]
    fn transition_handoff_rejects_evidence_digest_mismatch() {
        let root = unique_temp_dir("digest-mismatch");
        let handoff = write_fixture_repo(&root);
        let evidence_path = root.join("docs/review/evidence.json");
        let body = fs::read_to_string(&evidence_path).expect("read evidence");
        fs::write(
            &evidence_path,
            body.replace("\"sha256\": \"", "\"sha256\": \"deadbeef"),
        )
        .expect("rewrite evidence");

        let err = build_write_request_from_transition_handoff(&root, &handoff)
            .expect_err("digest mismatch must fail");
        assert_eq!(err.code.as_str(), "OBSMEM_INVALID_REQUEST");
        assert!(err.message.contains("sha256"));
    }

    #[test]
    fn transition_handoff_rejects_signed_trace_binding_mismatch() {
        let root = unique_temp_dir("trace-binding");
        let handoff = write_fixture_repo(&root);
        let body = fs::read_to_string(&handoff).expect("read handoff");
        fs::write(
            &handoff,
            body.replace(
                "\"docs/review/trace_signed.adl.yaml\"",
                "\"docs/review/other_trace_signed.adl.yaml\"",
            ),
        )
        .expect("rewrite handoff");
        fs::copy(
            root.join("docs/review/trace_signed.adl.yaml"),
            root.join("docs/review/other_trace_signed.adl.yaml"),
        )
        .expect("copy alternate trace");

        let err = build_write_request_from_transition_handoff(&root, &handoff)
            .expect_err("trace binding mismatch must fail");
        assert_eq!(err.code.as_str(), "OBSMEM_INVALID_REQUEST");
        assert!(err.message.contains("signed_trace_path"));
    }
}
