use super::*;

#[test]
fn runtime_v2_godel_constructability_boundary_validates() {
    let packet = runtime_v2_godel_constructability_boundary().expect("boundary packet");
    let godel = runtime_v2_godel_agent_runtime_contract().expect("Godel runtime");
    let constructability =
        runtime_v2_constructability_anchor_validator_contract().expect("constructability");

    validate_runtime_v2_godel_constructability_boundary(&packet, &godel, &constructability)
        .expect("boundary validates");

    assert_eq!(
        packet.schema_version,
        RUNTIME_V2_GODEL_CONSTRUCTABILITY_BOUNDARY_SCHEMA
    );
    assert_eq!(packet.godel_runtime.agent_count, 10);
    assert_eq!(
        packet.godel_runtime.launch_plan_status,
        "csm_supervised_provider_request_admission_ready"
    );
    assert_eq!(packet.godel_runtime.provider_request_count, 10);
    assert!(packet.constructability_validator.promotion_requires_anchor);
    assert!(packet
        .v092_prohibited_claims
        .contains(&"live_hosted_provider_invocation".to_string()));
}

#[test]
fn runtime_v2_godel_constructability_boundary_rejects_launch_plan_drift() {
    let mut packet = runtime_v2_godel_constructability_boundary().expect("boundary packet");
    packet.godel_runtime.provider_request_count = 9;
    let godel = runtime_v2_godel_agent_runtime_contract().expect("Godel runtime");
    let constructability =
        runtime_v2_constructability_anchor_validator_contract().expect("constructability");

    let err =
        validate_runtime_v2_godel_constructability_boundary(&packet, &godel, &constructability)
            .expect_err("launch-plan drift should fail")
            .to_string();

    assert!(err.contains("launch-plan provider requests"));
}

#[test]
fn runtime_v2_godel_constructability_boundary_json_is_stable() {
    let mut packet = runtime_v2_godel_constructability_boundary().expect("boundary packet");
    let first = runtime_v2_godel_constructability_boundary_json_bytes(&packet).expect("first");

    packet.v092_allowed_claims.reverse();
    packet.v092_prohibited_claims.reverse();
    packet.promotion_requirements.reverse();
    packet.validation_commands.reverse();
    packet.non_claims.reverse();
    packet.godel_runtime.retained_non_claims.reverse();
    packet
        .constructability_validator
        .retained_non_claims
        .reverse();

    let second = runtime_v2_godel_constructability_boundary_json_bytes(&packet).expect("second");
    assert_eq!(first, second);
}

#[test]
fn runtime_v2_godel_constructability_boundary_rejects_missing_godel_non_claim() {
    let mut packet = runtime_v2_godel_constructability_boundary().expect("boundary packet");
    packet
        .godel_runtime
        .retained_non_claims
        .retain(|claim| claim != "not_live_hosted_provider_invocation");
    let godel = runtime_v2_godel_agent_runtime_contract().expect("Godel runtime");
    let constructability =
        runtime_v2_constructability_anchor_validator_contract().expect("constructability");

    let err =
        validate_runtime_v2_godel_constructability_boundary(&packet, &godel, &constructability)
            .expect_err("missing hosted invocation non-claim should fail")
            .to_string();

    assert!(err.contains("not_live_hosted_provider_invocation"));
}

#[test]
fn runtime_v2_godel_constructability_boundary_rejects_constructability_disabled() {
    let mut packet = runtime_v2_godel_constructability_boundary().expect("boundary packet");
    packet.constructability_validator.promotion_requires_anchor = false;
    let godel = runtime_v2_godel_agent_runtime_contract().expect("Godel runtime");
    let constructability =
        runtime_v2_constructability_anchor_validator_contract().expect("constructability");

    let err =
        validate_runtime_v2_godel_constructability_boundary(&packet, &godel, &constructability)
            .expect_err("disabled constructability anchor should fail")
            .to_string();

    assert!(err.contains("requires anchor"));
}

#[test]
fn runtime_v2_godel_constructability_boundary_rejects_unsafe_v092_claim() {
    let mut packet = runtime_v2_godel_constructability_boundary().expect("boundary packet");
    packet.v092_allowed_claims[0] =
        "v0.92 may describe a bounded Godel-agent birthday and also proves live hosted provider invocation."
            .to_string();
    let godel = runtime_v2_godel_agent_runtime_contract().expect("Godel runtime");
    let constructability =
        runtime_v2_constructability_anchor_validator_contract().expect("constructability");

    let err =
        validate_runtime_v2_godel_constructability_boundary(&packet, &godel, &constructability)
            .expect_err("unsafe claim should fail")
            .to_string();

    assert!(err.contains("prohibited claim"));
}

#[test]
fn runtime_v2_godel_constructability_boundary_rejects_punctuated_prohibited_claims() {
    for unsafe_claim in [
        "v0.92 proves live hosted-provider invocation.",
        "v0.92 proves autonomous self improvement.",
        "v0.92 permits source-code mutation without review.",
    ] {
        let mut packet = runtime_v2_godel_constructability_boundary().expect("boundary packet");
        packet.v092_allowed_claims.push(unsafe_claim.to_string());
        let godel = runtime_v2_godel_agent_runtime_contract().expect("Godel runtime");
        let constructability =
            runtime_v2_constructability_anchor_validator_contract().expect("constructability");

        let err =
            validate_runtime_v2_godel_constructability_boundary(&packet, &godel, &constructability)
                .expect_err("punctuated unsafe claim should fail")
                .to_string();

        assert!(err.contains("prohibited claim"));
    }
}

#[test]
fn runtime_v2_godel_constructability_boundary_rejects_agent_count_drift() {
    let mut packet = runtime_v2_godel_constructability_boundary().expect("boundary packet");
    packet.godel_runtime.agent_count = 9;
    let godel = runtime_v2_godel_agent_runtime_contract().expect("Godel runtime");
    let constructability =
        runtime_v2_constructability_anchor_validator_contract().expect("constructability");

    let err =
        validate_runtime_v2_godel_constructability_boundary(&packet, &godel, &constructability)
            .expect_err("agent count drift should fail")
            .to_string();

    assert!(err.contains("10+ Godel agents"));
}
