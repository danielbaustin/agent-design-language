use std::{
    fs,
    io::Write,
    process::{Command, Stdio},
};

use adl_runtime_kernel::{
    AuthorityGrant, Commitment, FreedomGate, GovernanceKeys, GovernedActionRequest,
    MediationDecision, OperatorDecision, OperatorDisposition, TrustedGovernanceTime,
    AUTHORITY_GRANT_SCHEMA, COMMITMENT_SCHEMA, OPERATOR_DECISION_SCHEMA,
};
use ed25519_dalek::SigningKey;
use serde_json::{json, Value};
use tempfile::TempDir;

const BIN: &str = env!("CARGO_BIN_EXE_adl-runtime-governed-operations");
const POLICY: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

struct TestTime(u64);
impl TrustedGovernanceTime for TestTime {
    fn now_unix_millis(&self) -> u64 {
        self.0
    }
}

fn signed_command(id: &str, citizen: &str, now: u64) -> Value {
    let policy = SigningKey::from_bytes(&[1; 32]);
    let authority = SigningKey::from_bytes(&[2; 32]);
    let commitment = Commitment {
        schema: COMMITMENT_SCHEMA.to_owned(),
        commitment_id: format!("commit-{id}"),
        principal: citizen.to_owned(),
        action: "provider.invoke".to_owned(),
        resource: "provider".to_owned(),
        max_units: 8,
        policy_hash: POLICY.to_owned(),
        expires_unix_millis: now + 60_000,
        signing_key_id: "policy".to_owned(),
        signature: String::new(),
    }
    .sign(&policy)
    .unwrap();
    let grant = AuthorityGrant {
        schema: AUTHORITY_GRANT_SCHEMA.to_owned(),
        grant_id: format!("grant-{id}"),
        principal: citizen.to_owned(),
        action: "provider.invoke".to_owned(),
        resource: "provider".to_owned(),
        max_units: 8,
        max_delegation_depth: 2,
        parent_grant_hash: None,
        policy_hash: POLICY.to_owned(),
        expires_unix_millis: now + 60_000,
        signing_key_id: "authority".to_owned(),
        signature: String::new(),
    }
    .sign(&authority)
    .unwrap();
    json!({
        "request_id": id,
        "idempotency_key": format!("idem-{id}"),
        "citizen_id": citizen,
        "agent_id": format!("agent-{citizen}"),
        "action": "provider.invoke",
        "resource": "provider",
        "units": 1,
        "payload": format!("private-{id}"),
        "commitment": commitment,
        "authority_chain": [grant]
    })
}

fn resign(request: &mut Value, now: u64) {
    let policy = SigningKey::from_bytes(&[1; 32]);
    let authority = SigningKey::from_bytes(&[2; 32]);
    let citizen = request["citizen_id"].as_str().unwrap().to_owned();
    let action = request["action"].as_str().unwrap().to_owned();
    let resource = request["resource"].as_str().unwrap().to_owned();
    let id = request["request_id"].as_str().unwrap().to_owned();
    let units = request["units"].as_u64().unwrap();
    request["commitment"] = serde_json::to_value(
        Commitment {
            schema: COMMITMENT_SCHEMA.to_owned(),
            commitment_id: format!("commit-{id}"),
            principal: citizen.clone(),
            action: action.clone(),
            resource: resource.clone(),
            max_units: 8,
            policy_hash: POLICY.to_owned(),
            expires_unix_millis: now + 60_000,
            signing_key_id: "policy".to_owned(),
            signature: String::new(),
        }
        .sign(&policy)
        .unwrap(),
    )
    .unwrap();
    request["authority_chain"] = serde_json::to_value([AuthorityGrant {
        schema: AUTHORITY_GRANT_SCHEMA.to_owned(),
        grant_id: format!("grant-{id}"),
        principal: citizen,
        action,
        resource,
        max_units: units.max(1),
        max_delegation_depth: 2,
        parent_grant_hash: None,
        policy_hash: POLICY.to_owned(),
        expires_unix_millis: now + 60_000,
        signing_key_id: "authority".to_owned(),
        signature: String::new(),
    }
    .sign(&authority)
    .unwrap()])
    .unwrap();
}

fn run(root: &TempDir, request: Value, time: u64) -> Value {
    run_with(root, request, time, "healthy", "")
}

fn run_with(root: &TempDir, request: Value, time: u64, condition: &str, revoked: &str) -> Value {
    run_program(root, request, time, condition, revoked, "/bin/cat")
}

fn run_program(
    root: &TempDir,
    request: Value,
    time: u64,
    condition: &str,
    revoked: &str,
    program: impl AsRef<std::ffi::OsStr>,
) -> Value {
    let policy = SigningKey::from_bytes(&[1; 32]);
    let authority = SigningKey::from_bytes(&[2; 32]);
    let mut child = Command::new(BIN)
        .current_dir(root.path())
        .env("ADL_PARITY_C_STATE_DIR", root.path().join("state"))
        .env("ADL_PARITY_C_TOOL_ROOT", root.path())
        .env("ADL_PARITY_C_POLICY_HASH", POLICY)
        .env(
            "ADL_PARITY_C_POLICY_PUBLIC_KEY_HEX",
            hex::encode(policy.verifying_key().to_bytes()),
        )
        .env(
            "ADL_PARITY_C_AUTHORITY_PUBLIC_KEY_HEX",
            hex::encode(authority.verifying_key().to_bytes()),
        )
        .env(
            "ADL_PARITY_C_OPERATOR_PUBLIC_KEY_HEX",
            hex::encode(SigningKey::from_bytes(&[5; 32]).verifying_key().to_bytes()),
        )
        .env("ADL_PARITY_C_AUTHORITY_PRINCIPAL", "alice")
        .env("ADL_PARITY_C_PERMIT_KEY_HEX", hex::encode([3; 32]))
        .env("ADL_PARITY_C_CHECKPOINT_KEY_HEX", hex::encode([4; 32]))
        .env("ADL_PARITY_C_TRUSTED_TIME_MILLIS", time.to_string())
        .env("ADL_PARITY_C_PROVIDER_PROGRAM", program)
        .env("ADL_PARITY_C_PROVIDER_CONDITION", condition)
        .env("ADL_PARITY_C_REVOKED_COMMITMENTS", revoked)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(request.to_string().as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        matches!(output.status.code(), Some(0 | 77)),
        "{:?}: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

fn success(value: &Value) {
    assert_eq!(value["status"], "completed", "{value}");
    assert_eq!(value["gate_before_actuation"], true);
    assert_eq!(value["lifelog_authoritative"], false);
    assert_eq!(value["private_payload_retained"], false);
}

mod parity_c_live_governance {
    use super::*;

    #[test]
    fn signed_gate_precedes_provider_actuation() {
        let root = TempDir::new().unwrap();
        let value = run(&root, signed_command("signed", "alice", 1_000), 1_000);
        success(&value);
        assert_eq!(value["actuation_count"], 1);
        assert!(value["adapters"]
            .as_array()
            .unwrap()
            .contains(&json!("canonical_ingress")));
    }

    #[test]
    fn denial_revocation_and_quarantine_prevent_actuation() {
        let root = TempDir::new().unwrap();
        let value = run_with(
            &root,
            signed_command("revoked", "alice", 1_000),
            1_000,
            "healthy",
            "commit-revoked",
        );
        assert_eq!(value["classification"], "revoked");
        assert_eq!(value["actuation_count"], 0);
    }

    #[test]
    fn tampered_request_cannot_masquerade_as_an_appeal() {
        let root = TempDir::new().unwrap();
        let mut request = signed_command("appeal", "alice", 1_000);
        request["action"] = "system.shutdown".into();
        let value = run(&root, request, 1_000);
        assert_eq!(value["classification"], "invalid_commitment");
        assert_eq!(value["actuation_count"], 0);
    }

    #[test]
    fn signed_appeal_disposition_is_recorded_without_actuation() {
        use std::{
            collections::{BTreeMap, BTreeSet},
            sync::Arc,
        };
        let root = TempDir::new().unwrap();
        let mut command = signed_command("appeal-live", "alice", 1_000);
        command["action"] = "system.shutdown".into();
        let request = GovernedActionRequest {
            request_id: "appeal-live".to_owned(),
            principal: "alice".to_owned(),
            action: "system.shutdown".to_owned(),
            resource: "provider".to_owned(),
            units: 1,
            payload_hash: blake3::hash(b"private-appeal-live").to_hex().to_string(),
            policy_hash: POLICY.to_owned(),
            commitment: serde_json::from_value(command["commitment"].clone()).unwrap(),
            authority_chain: serde_json::from_value(command["authority_chain"].clone()).unwrap(),
        };
        let operator = SigningKey::from_bytes(&[5; 32]);
        let gate = FreedomGate::new(
            POLICY,
            GovernanceKeys {
                policy: BTreeMap::from([(
                    "policy".to_owned(),
                    SigningKey::from_bytes(&[1; 32]).verifying_key(),
                )]),
                authority: BTreeMap::from([(
                    "authority".to_owned(),
                    SigningKey::from_bytes(&[2; 32]).verifying_key(),
                )]),
                authority_principals: BTreeMap::from([(
                    "authority".to_owned(),
                    "alice".to_owned(),
                )]),
                root_authority_keys: BTreeSet::from(["authority".to_owned()]),
                operator: BTreeMap::from([("operator".to_owned(), operator.verifying_key())]),
            },
            "permit",
            SigningKey::from_bytes(&[3; 32]),
            Arc::new(TestTime(1_000)),
            BTreeMap::from([("provider".to_owned(), 8)]),
        )
        .unwrap();
        let refusal = match gate.mediate(&request) {
            MediationDecision::Refused(evidence) => evidence,
            MediationDecision::Allowed(_) => panic!("tampered commitment was allowed"),
        };
        let decision = OperatorDecision {
            schema: OPERATOR_DECISION_SCHEMA.to_owned(),
            decision_id: "review-appeal-live".to_owned(),
            request_id: "appeal-live".to_owned(),
            refusal_hash: refusal.evidence_hash,
            disposition: OperatorDisposition::Retry,
            expires_unix_millis: 2_000,
            signing_key_id: "operator".to_owned(),
            signature: String::new(),
        }
        .sign(&operator)
        .unwrap();
        command["appeal_id"] = "appeal-live-1".into();
        command["operator_decision"] = serde_json::to_value(decision).unwrap();
        let outcome = run(&root, command, 1_000);
        assert_eq!(outcome["classification"], "appeal_retry_recorded");
        assert_eq!(outcome["actuation_count"], 0);
    }

    #[test]
    fn expired_or_replayed_gate_receipt_fails_closed() {
        let root = TempDir::new().unwrap();
        let expired = signed_command("expired", "alice", 1_000);
        assert_eq!(run(&root, expired, 70_001)["status"], "refused");
        let fresh = signed_command("once", "alice", 80_000);
        success(&run(&root, fresh.clone(), 80_000));
        let mut replay = fresh;
        replay["idempotency_key"] = "different".into();
        assert_eq!(
            run(&root, replay, 80_001)["classification"],
            "request_replay"
        );
    }
}

mod parity_c_delegation_resources {
    use super::*;

    #[test]
    fn delegation_chain_only_attenuates() {
        let root = TempDir::new().unwrap();
        let mut request = signed_command("delegate", "alice", 1_000);
        let root_grant: AuthorityGrant =
            serde_json::from_value(request["authority_chain"][0].clone()).unwrap();
        let child = AuthorityGrant {
            grant_id: "delegate-child".to_owned(),
            max_units: 1,
            max_delegation_depth: 1,
            parent_grant_hash: Some(root_grant.hash().unwrap()),
            ..root_grant
        }
        .sign(&SigningKey::from_bytes(&[2; 32]))
        .unwrap();
        request["authority_chain"]
            .as_array_mut()
            .unwrap()
            .push(serde_json::to_value(child).unwrap());
        success(&run(&root, request, 1_000));
    }

    #[test]
    fn widened_expired_or_replayed_delegation_is_rejected() {
        let root = TempDir::new().unwrap();
        let mut request = signed_command("widen", "alice", 1_000);
        let mut grant: AuthorityGrant =
            serde_json::from_value(request["authority_chain"][0].clone()).unwrap();
        grant.parent_grant_hash = Some(grant.hash().unwrap());
        grant.max_units = 9;
        grant.max_delegation_depth = 3;
        request["authority_chain"].as_array_mut().unwrap().push(
            serde_json::to_value(grant.sign(&SigningKey::from_bytes(&[2; 32])).unwrap()).unwrap(),
        );
        assert_eq!(
            run(&root, request, 1_000)["classification"],
            "invalid_delegation"
        );
    }

    #[test]
    fn pre_dispatch_cancellation_releases_capacity() {
        let root = TempDir::new().unwrap();
        let mut cancelled = signed_command("cancelled", "alice", 1_000);
        cancelled["cancelled"] = true.into();
        let outcomes = run(
            &root,
            json!([cancelled, signed_command("after-cancel", "alice", 1_000)]),
            1_000,
        );
        assert_eq!(outcomes[0]["classification"], "scheduler_cancelled");
        success(&outcomes[1]);
    }

    #[test]
    fn in_flight_cancellation_kills_provider_and_releases_capacity() {
        use std::os::unix::fs::PermissionsExt;
        let root = TempDir::new().unwrap();
        let provider = root.path().join("slow-provider");
        let marker = root.path().join("descendant-side-effect");
        fs::write(
            &provider,
            format!("#!/bin/sh\nsleep 1\necho leaked > {}\n", marker.display()),
        )
        .unwrap();
        fs::set_permissions(&provider, fs::Permissions::from_mode(0o700)).unwrap();
        let mut cancelled = signed_command("cancel-race", "alice", 1_000);
        cancelled["cancel_after_millis"] = 50.into();
        let started = std::time::Instant::now();
        let value = run_program(&root, cancelled, 1_000, "healthy", "", &provider);
        assert_eq!(value["classification"], "scheduler_cancelled");
        assert!(started.elapsed() < std::time::Duration::from_secs(2));
        std::thread::sleep(std::time::Duration::from_millis(1_200));
        assert!(!marker.exists());
        success(&run(
            &root,
            signed_command("after-cancel-race", "alice", 2_000),
            2_000,
        ));
    }

    #[test]
    fn retry_and_idempotency_bounds_prevent_duplicate_work() {
        let root = TempDir::new().unwrap();
        let request = signed_command("idem", "alice", 1_000);
        success(&run(&root, request.clone(), 1_000));
        let replay = run(&root, request.clone(), 2_000);
        assert_eq!(replay["classification"], "idempotent_replay");
        assert_eq!(replay["actuation_count"], 1);
        let mut changed = request;
        changed["payload"] = "changed-after-completion".into();
        assert_eq!(
            run(&root, changed, 3_000)["classification"],
            "idempotency_conflict"
        );
    }
}

mod parity_c_provider_scheduler_tools {
    use super::*;

    #[test]
    fn two_agents_execute_governed_provider_and_tool_work() {
        let root = TempDir::new().unwrap();
        fs::write(root.path().join("tool-target"), b"real-file").unwrap();
        let mut tool = signed_command("agent-b", "bob", 1_000);
        tool["action"] = "tool.file_metadata".into();
        tool["resource"] = "tool-root".into();
        tool["payload"] = "tool-target".into();
        resign(&mut tool, 1_000);
        let outcomes = run(
            &root,
            json!([signed_command("agent-a", "alice", 1_000), tool]),
            1_000,
        );
        success(&outcomes[0]);
        success(&outcomes[1]);

        let outside = TempDir::new().unwrap();
        fs::write(outside.path().join("secret"), b"not allowlisted").unwrap();
        std::os::unix::fs::symlink(outside.path().join("secret"), root.path().join("escape"))
            .unwrap();
        let mut escaped = signed_command("agent-c", "alice", 3_000);
        escaped["action"] = "tool.file_metadata".into();
        escaped["resource"] = "tool-root".into();
        escaped["payload"] = "escape".into();
        resign(&mut escaped, 3_000);
        assert_eq!(
            run(&root, escaped, 3_000)["classification"],
            "tool_path_not_allowlisted"
        );
    }

    #[test]
    fn scheduler_dispatch_is_deterministic_and_bounded() {
        use std::os::unix::fs::PermissionsExt;
        let root = TempDir::new().unwrap();
        let provider = root.path().join("slow-provider");
        fs::write(&provider, "#!/bin/sh\nsleep 1\ncat\n").unwrap();
        fs::set_permissions(&provider, fs::Permissions::from_mode(0o700)).unwrap();
        let started = std::time::Instant::now();
        let outcomes = run_program(
            &root,
            json!([
                signed_command("scheduled-a", "alice", 1_000),
                signed_command("scheduled-b", "bob", 1_000),
                signed_command("scheduled-c", "carol", 1_000)
            ]),
            1_000,
            "healthy",
            "",
            &provider,
        );
        assert_eq!(
            outcomes
                .as_array()
                .unwrap()
                .iter()
                .filter(|outcome| outcome["status"] == "completed")
                .count(),
            2
        );
        assert!(started.elapsed() < std::time::Duration::from_secs(2));
        assert!(outcomes
            .as_array()
            .unwrap()
            .iter()
            .any(|outcome| outcome["classification"] == "scheduler_saturated"));
    }

    #[test]
    fn provider_timeout_auth_quota_and_malformed_output_are_classified() {
        for (index, condition) in ["timeout", "auth", "quota", "malformed"]
            .into_iter()
            .enumerate()
        {
            let root = TempDir::new().unwrap();
            let value = run_with(
                &root,
                signed_command(&format!("failure-{index}"), "alice", 1_000),
                1_000,
                condition,
                "",
            );
            let expected = if condition == "malformed" {
                "provider_malformed_output".to_owned()
            } else {
                format!("provider_{condition}")
            };
            assert_eq!(value["classification"], expected);
        }
        let root = TempDir::new().unwrap();
        let request = signed_command("retry-provider", "alice", 1_000);
        assert_eq!(
            run_with(&root, request.clone(), 1_000, "timeout", "")["classification"],
            "provider_timeout"
        );
        success(&run(&root, request, 2_000));

        use std::os::unix::fs::PermissionsExt;
        let root = TempDir::new().unwrap();
        let hung = root.path().join("hung-provider");
        fs::write(&hung, "#!/bin/sh\nsleep 5\n").unwrap();
        fs::set_permissions(&hung, fs::Permissions::from_mode(0o700)).unwrap();
        let started = std::time::Instant::now();
        assert_eq!(
            run_program(
                &root,
                signed_command("hung", "alice", 1_000),
                1_000,
                "healthy",
                "",
                &hung,
            )["classification"],
            "provider_timeout"
        );
        assert!(started.elapsed() < std::time::Duration::from_secs(4));
    }

    #[test]
    fn shepherd_cannot_grant_or_bypass_authority() {
        let root = TempDir::new().unwrap();
        let mut request = signed_command("shepherd", "alice", 1_000);
        request["citizen_id"] = "shepherd".into();
        assert_eq!(
            run(&root, request, 1_000)["classification"],
            "invalid_commitment"
        );
    }
}

mod parity_c_private_identity {
    use super::*;

    #[test]
    fn private_state_is_partitioned_by_authoritative_identity() {
        let root = TempDir::new().unwrap();
        success(&run(
            &root,
            signed_command("alice-write", "alice", 1_000),
            1_000,
        ));
        success(&run(
            &root,
            signed_command("bob-write", "bob", 2_000),
            2_000,
        ));
        let checkpoint = fs::read_to_string(root.path().join("state/checkpoint.json")).unwrap();
        assert!(checkpoint.contains("alice|provider.invoke|provider|commit-alice-write"));
        assert!(checkpoint.contains("bob|provider.invoke|provider|commit-bob-write"));
        assert!(!checkpoint.contains("private-alice-write"));
    }

    #[test]
    fn cross_identity_read_and_write_fail_closed() {
        let root = TempDir::new().unwrap();
        let mut request = signed_command("cross", "alice", 1_000);
        request["read_citizen_id"] = "bob".into();
        assert_eq!(
            run(&root, request, 1_000)["classification"],
            "cross_identity_denied"
        );
    }

    #[test]
    fn provider_or_display_identity_cannot_substitute_for_citizen_identity() {
        let root = TempDir::new().unwrap();
        let mut request = signed_command("display", "alice", 1_000);
        request["citizen_id"] = "agent-alice".into();
        assert_eq!(
            run(&root, request, 1_000)["classification"],
            "invalid_commitment"
        );
    }

    #[test]
    fn restart_preserves_redacted_identity_scoped_state() {
        let root = TempDir::new().unwrap();
        let request = signed_command("persist", "alice", 1_000);
        success(&run(&root, request.clone(), 1_000));
        assert_eq!(
            run(&root, request, 2_000)["classification"],
            "idempotent_replay"
        );
        let log = fs::read_to_string(root.path().join("state/lifelog.jsonl")).unwrap();
        assert!(!log.contains("private-persist"));
    }
}

mod parity_c_time_continuity {
    use super::*;

    #[test]
    fn unqualified_or_regressing_time_cannot_authorize_actuation() {
        let root = TempDir::new().unwrap();
        success(&run(&root, signed_command("time-a", "alice", 2_000), 2_000));
        assert_eq!(
            run(&root, signed_command("time-b", "alice", 2_000), 1_999)["classification"],
            "unqualified_or_regressing_time"
        );
    }

    #[test]
    fn authenticated_checkpoint_is_the_only_restore_authority() {
        let root = TempDir::new().unwrap();
        fs::create_dir_all(root.path().join("state")).unwrap();
        fs::write(root.path().join("state/checkpoint.lock"), b"stale-owner").unwrap();
        success(&run(
            &root,
            signed_command("checkpoint", "alice", 1_000),
            1_000,
        ));
        let path = root.path().join("state/checkpoint.json");
        let mut value: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        value["state"]["generation"] = 999.into();
        fs::write(path, serde_json::to_vec(&value).unwrap()).unwrap();
        assert_eq!(
            run(
                &root,
                signed_command("after-corrupt", "alice", 2_000),
                2_000
            )["classification"],
            "checkpoint_authentication_failed"
        );
    }

    #[test]
    fn lifelog_is_redacted_append_only_and_non_authoritative() {
        let root = TempDir::new().unwrap();
        success(&run(&root, signed_command("log-a", "alice", 1_000), 1_000));
        success(&run(&root, signed_command("log-b", "alice", 2_000), 2_000));
        let log = fs::read_to_string(root.path().join("state/lifelog.jsonl")).unwrap();
        assert_eq!(log.lines().count(), 2);
        assert!(!log.contains("private-log"));
        fs::write(root.path().join("state/lifelog.jsonl"), b"tampered\n").unwrap();
        success(&run(&root, signed_command("log-c", "alice", 3_000), 3_000));
    }

    #[test]
    fn restart_revalidates_revocation_without_duplicate_side_effects() {
        let root = TempDir::new().unwrap();
        let request = signed_command("restart", "alice", 1_000);
        success(&run(&root, request.clone(), 1_000));
        let replay = run_with(&root, request, 2_000, "healthy", "commit-restart");
        assert_eq!(replay["classification"], "revoked");
        assert_eq!(replay["actuation_count"], 1);
    }

    #[test]
    fn shutdown_commits_final_checkpoint_and_isolates_lifelog_failure() {
        let root = TempDir::new().unwrap();
        fs::create_dir_all(root.path().join("state/lifelog.jsonl")).unwrap();
        let mut shutdown = signed_command("shutdown", "alice", 1_000);
        shutdown["action"] = "system.shutdown".into();
        shutdown["resource"] = "kernel".into();
        resign(&mut shutdown, 1_000);
        success(&run(&root, shutdown, 1_000));
        assert!(root.path().join("state/checkpoint.json").exists());
        assert_eq!(
            run(
                &root,
                signed_command("after-shutdown", "alice", 2_000),
                2_000
            )["classification"],
            "admission_closed"
        );
    }

    #[test]
    fn shutdown_orders_batch_and_blocks_later_actuation() {
        let root = TempDir::new().unwrap();
        let mut shutdown = signed_command("ordered-shutdown", "alice", 1_000);
        shutdown["action"] = "system.shutdown".into();
        shutdown["resource"] = "kernel".into();
        resign(&mut shutdown, 1_000);
        let outcomes = run(
            &root,
            json!([shutdown, signed_command("too-late", "bob", 1_000)]),
            1_000,
        );
        success(&outcomes[0]);
        assert_eq!(outcomes[1]["classification"], "admission_closed");
        assert_eq!(outcomes[1]["actuation_count"], 1);
    }
}

mod parity_c_production_credit {
    use super::*;

    #[test]
    fn all_owned_components_use_production_or_cots_adapters() {
        let root = TempDir::new().unwrap();
        let value = run(&root, signed_command("inventory", "alice", 1_000), 1_000);
        success(&value);
        for adapter in [
            "canonical_ingress",
            "resident_agent",
            "resident_shepherd",
            "bounded_scheduler",
            "external_process_provider",
        ] {
            assert!(value["adapters"]
                .as_array()
                .unwrap()
                .contains(&json!(adapter)));
        }
    }

    #[test]
    fn degraded_fixture_mock_and_metadata_paths_receive_zero_credit() {
        let owned = [
            include_str!("../src/governed_operations.rs"),
            include_str!("../src/bin/adl-runtime-governed-operations.rs"),
        ]
        .join("\n")
        .to_ascii_lowercase();
        for forbidden in [
            "degradedoperationexecutor",
            "mockexecutor",
            "fixtureexecutor",
        ] {
            assert!(!owned.contains(forbidden));
        }
    }
}

mod parity_c_boundary_contract {
    #[test]
    fn runtime_v2_aws_and_cross_lane_paths_are_absent() {
        let owned = include_str!("../src/governed_operations.rs").to_ascii_lowercase();
        for forbidden in [
            "aws_",
            "adl-runtime/src",
            "parity_b",
            "observatory",
            "weather",
        ] {
            assert!(!owned.contains(forbidden));
        }
    }

    #[test]
    fn retained_evidence_excludes_credentials_private_state_and_machine_paths() {
        let owned = include_str!("../src/governed_operations.rs");
        assert!(!owned.contains("/Users/"));
        assert!(!owned.contains("/Volumes/"));
        assert!(!owned.contains("BEGIN PRIVATE KEY"));
    }
}
