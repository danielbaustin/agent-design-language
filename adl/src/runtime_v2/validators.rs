use super::*;
pub(crate) fn normalize_id(value: String, field: &str) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    if trimmed.contains('/') || trimmed.contains('\\') || trimmed.contains(':') {
        return Err(anyhow!("{field} must be a stable identifier, not a path"));
    }
    Ok(trimmed.to_string())
}

pub(crate) fn validate_lifecycle_state(value: &str) -> Result<()> {
    match value {
        "initialized" | "active" | "paused" | "snapshotting" | "rehydrating" | "terminated" => {
            Ok(())
        }
        other => Err(anyhow!("unsupported manifold lifecycle_state '{other}'")),
    }
}

pub(crate) fn validate_clock_anchor(anchor: &ManifoldClockAnchor) -> Result<()> {
    normalize_id(anchor.anchor_id.clone(), "clock_anchor.anchor_id")?;
    match anchor.clock_kind.as_str() {
        "monotonic_logical" | "wall_clock_bound" => {}
        other => return Err(anyhow!("unsupported clock_anchor.clock_kind '{other}'")),
    }
    let observed = anchor.observed_at_utc.trim();
    if observed.is_empty() {
        return Err(anyhow!("clock_anchor.observed_at_utc must not be empty"));
    }
    Ok(())
}

pub(crate) fn validate_registry_refs(refs: &CitizenRegistryRefs) -> Result<()> {
    validate_relative_path(&refs.registry_root, "citizen_registry_refs.registry_root")?;
    validate_relative_path(&refs.active_index, "citizen_registry_refs.active_index")?;
    validate_relative_path(&refs.pending_index, "citizen_registry_refs.pending_index")
}

pub(crate) fn validate_kernel_refs(refs: &KernelServiceRefs) -> Result<()> {
    validate_relative_path(&refs.registry_path, "kernel_service_refs.registry_path")?;
    validate_relative_path(
        &refs.service_loop_path,
        "kernel_service_refs.service_loop_path",
    )?;
    validate_relative_path(
        &refs.service_state_path,
        "kernel_service_refs.service_state_path",
    )
}

pub(crate) fn validate_trace_root(trace_root: &TraceRootRef) -> Result<()> {
    validate_relative_path(&trace_root.trace_root, "trace_root.trace_root")?;
    validate_relative_path(&trace_root.event_log_path, "trace_root.event_log_path")?;
    if trace_root.next_event_sequence == 0 {
        return Err(anyhow!("trace_root.next_event_sequence must be positive"));
    }
    Ok(())
}

pub(crate) fn validate_snapshot_root(snapshot_root: &SnapshotRootRef) -> Result<()> {
    validate_relative_path(&snapshot_root.snapshot_root, "snapshot_root.snapshot_root")?;
    if let Some(id) = &snapshot_root.latest_snapshot_id {
        normalize_id(id.clone(), "snapshot_root.latest_snapshot_id")?;
    }
    validate_relative_path(
        &snapshot_root.rehydration_report_path,
        "snapshot_root.rehydration_report_path",
    )
}

pub(crate) fn validate_invariant_policy_refs(refs: &InvariantPolicyRefs) -> Result<()> {
    validate_relative_path(&refs.policy_path, "invariant_policy_refs.policy_path")?;
    match refs.enforcement_mode.as_str() {
        "fail_closed_before_activation" | "report_only" => {}
        other => {
            return Err(anyhow!(
                "unsupported invariant_policy_refs.enforcement_mode '{other}'"
            ))
        }
    }
    if refs.blocking_invariants.is_empty() {
        return Err(anyhow!(
            "invariant_policy_refs.blocking_invariants must not be empty"
        ));
    }
    for invariant in &refs.blocking_invariants {
        normalize_id(
            invariant.clone(),
            "invariant_policy_refs.blocking_invariants",
        )?;
    }
    Ok(())
}

pub(crate) fn validate_review_surface(surface: &RuntimeV2ManifoldReviewSurface) -> Result<()> {
    if surface.required_artifacts.is_empty() {
        return Err(anyhow!(
            "review_surface.required_artifacts must not be empty"
        ));
    }
    for path in &surface.required_artifacts {
        validate_relative_path(path, "review_surface.required_artifacts")?;
    }
    if surface.proof_hook_command.trim().is_empty() {
        return Err(anyhow!(
            "review_surface.proof_hook_command must not be empty"
        ));
    }
    validate_relative_path(
        &surface.proof_hook_output_path,
        "review_surface.proof_hook_output_path",
    )?;
    if surface.downstream_boundaries.is_empty() {
        return Err(anyhow!(
            "review_surface.downstream_boundaries must name later WP boundaries"
        ));
    }
    if surface.non_goals.is_empty() {
        return Err(anyhow!("review_surface.non_goals must not be empty"));
    }
    Ok(())
}

pub(crate) fn validate_relative_path(value: &str, field: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    if trimmed.starts_with('/')
        || trimmed.starts_with('\\')
        || trimmed.contains('\\')
        || trimmed.contains(':')
    {
        return Err(anyhow!("{field} must be a repository-relative path"));
    }
    for component in Path::new(trimmed).components() {
        use std::path::Component;
        match component {
            Component::Normal(_) => {}
            Component::CurDir => {}
            _ => return Err(anyhow!("{field} must not traverse outside the repo")),
        }
    }
    Ok(())
}

pub(crate) fn validate_service_lifecycle_state(value: &str, field: &str) -> Result<()> {
    match value {
        "registered" | "ready" | "paused" | "blocked" | "terminated" => Ok(()),
        other => Err(anyhow!("unsupported {field} '{other}'")),
    }
}

pub(crate) fn validate_provisional_status(value: &str) -> Result<()> {
    match value {
        "provisional" | "admitted" | "rejected" => Ok(()),
        other => Err(anyhow!("unsupported citizen.provisional_status '{other}'")),
    }
}

pub(crate) fn validate_citizen_lifecycle_state(value: &str) -> Result<()> {
    match value {
        "proposed" | "admitted" | "active" | "paused" | "sleeping" | "waking" | "terminated"
        | "rejected" => Ok(()),
        other => Err(anyhow!("unsupported citizen.lifecycle_state '{other}'")),
    }
}

pub(crate) fn validate_citizen_index_kind(value: &str) -> Result<()> {
    match value {
        "active" | "pending" => Ok(()),
        other => Err(anyhow!("unsupported citizen_index.index_kind '{other}'")),
    }
}

pub(crate) fn validate_snapshot_invariant_statuses(
    statuses: &[RuntimeV2SnapshotInvariantStatus],
) -> Result<()> {
    if statuses.is_empty() {
        return Err(anyhow!("snapshot invariant_status must not be empty"));
    }
    let mut seen = std::collections::BTreeSet::new();
    for status in statuses {
        normalize_id(status.invariant_id.clone(), "snapshot.invariant_id")?;
        match status.status.as_str() {
            "passed" | "failed" | "not_checked" => {}
            other => return Err(anyhow!("unsupported snapshot invariant status '{other}'")),
        }
        if !seen.insert(status.invariant_id.clone()) {
            return Err(anyhow!(
                "snapshot invariant_status contains duplicate invariant '{}'",
                status.invariant_id
            ));
        }
    }
    Ok(())
}

pub(crate) fn validate_invariant_violation_severity(value: &str) -> Result<()> {
    match value {
        "blocking" | "warning" | "audit" => Ok(()),
        other => Err(anyhow!(
            "unsupported invariant_violation.severity '{other}'"
        )),
    }
}

pub(crate) fn validate_invariant_violation_evaluated_refs(
    refs: &[RuntimeV2InvariantViolationEvaluatedRef],
) -> Result<()> {
    if refs.is_empty() {
        return Err(anyhow!(
            "invariant_violation.evaluated_refs must not be empty"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for evaluated_ref in refs {
        evaluated_ref.validate()?;
        let key = format!("{}:{}", evaluated_ref.ref_kind, evaluated_ref.artifact_ref);
        if !seen.insert(key) {
            return Err(anyhow!(
                "invariant_violation.evaluated_refs contains duplicate ref"
            ));
        }
    }
    Ok(())
}

pub(crate) fn validate_operator_commands(commands: &[RuntimeV2OperatorCommandReport]) -> Result<()> {
    let required = [
        "inspect_manifold",
        "inspect_citizens",
        "pause_manifold",
        "resume_manifold",
        "request_snapshot",
        "inspect_last_failures",
        "terminate_manifold",
    ];
    if commands.len() != required.len() {
        return Err(anyhow!(
            "operator_control.commands must cover each bounded operator command exactly once"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for (expected, command) in required.iter().zip(commands.iter()) {
        command.validate()?;
        if command.command != *expected {
            return Err(anyhow!(
                "operator_control.commands must preserve deterministic command order"
            ));
        }
        if !seen.insert(command.command.clone()) {
            return Err(anyhow!(
                "operator_control.commands contains duplicate command '{}'",
                command.command
            ));
        }
    }
    Ok(())
}

pub(crate) fn validate_operator_command(value: &str) -> Result<()> {
    match value {
        "inspect_manifold"
        | "inspect_citizens"
        | "pause_manifold"
        | "resume_manifold"
        | "request_snapshot"
        | "inspect_last_failures"
        | "terminate_manifold" => Ok(()),
        other => Err(anyhow!("unsupported operator_control.command '{other}'")),
    }
}

pub(crate) fn validate_operator_outcome(value: &str) -> Result<()> {
    match value {
        "allowed" | "refused" | "deferred" => Ok(()),
        other => Err(anyhow!("unsupported operator_control.outcome '{other}'")),
    }
}

pub(crate) fn validate_security_boundary_rules(
    rules: &[RuntimeV2SecurityBoundaryEvaluatedRule],
) -> Result<()> {
    if rules.len() < 3 {
        return Err(anyhow!(
            "security_boundary.evaluated_rules must include operator, invariant, and kernel checks"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    let mut has_operator = false;
    let mut has_invariant = false;
    let mut has_kernel = false;
    for rule in rules {
        rule.validate()?;
        if !seen.insert(rule.rule_id.clone()) {
            return Err(anyhow!(
                "security_boundary.evaluated_rules contains duplicate rule '{}'",
                rule.rule_id
            ));
        }
        match rule.rule_kind.as_str() {
            "operator_policy" => has_operator = true,
            "blocking_invariant" => has_invariant = true,
            "kernel_service_policy" => has_kernel = true,
            _ => {}
        }
    }
    if !(has_operator && has_invariant && has_kernel) {
        return Err(anyhow!(
            "security_boundary.evaluated_rules missing required policy/invariant/kernel coverage"
        ));
    }
    Ok(())
}

pub(crate) fn validate_security_boundary_rule_kind(value: &str) -> Result<()> {
    match value {
        "operator_policy" | "blocking_invariant" | "kernel_service_policy" => Ok(()),
        other => Err(anyhow!("unsupported security_boundary.rule_kind '{other}'")),
    }
}

pub(crate) fn validate_security_boundary_decision(value: &str) -> Result<()> {
    match value {
        "refuse" | "blocking_failure_present" | "keep_paused" => Ok(()),
        other => Err(anyhow!("unsupported security_boundary.decision '{other}'")),
    }
}

pub(crate) fn validate_security_boundary_related_artifacts(artifacts: &[String]) -> Result<()> {
    if artifacts.is_empty() {
        return Err(anyhow!(
            "security_boundary.related_artifacts must not be empty"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for artifact in artifacts {
        validate_relative_path(artifact, "security_boundary.related_artifacts")?;
        if !seen.insert(artifact.clone()) {
            return Err(anyhow!(
                "security_boundary.related_artifacts contains duplicate artifact"
            ));
        }
    }
    if !artifacts
        .iter()
        .any(|artifact| artifact == "runtime_v2/invariants/violation-0001.json")
    {
        return Err(anyhow!(
            "security_boundary.related_artifacts must include invariant violation evidence"
        ));
    }
    if !artifacts
        .iter()
        .any(|artifact| artifact == "runtime_v2/operator/control_report.json")
    {
        return Err(anyhow!(
            "security_boundary.related_artifacts must include operator control evidence"
        ));
    }
    Ok(())
}

pub(crate) fn validate_nonempty_text(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

pub(crate) fn validate_display_name(value: &str, field: &str) -> Result<()> {
    validate_nonempty_text(value, field)
}

pub(crate) fn validate_timestamp_marker(value: &str, field: &str) -> Result<()> {
    validate_nonempty_text(value, field)
}

pub(crate) fn validate_required_kernel_services(
    services: &[RuntimeV2KernelServiceRegistration],
) -> Result<()> {
    let required = [
        "clock_service",
        "identity_admission_guard",
        "scheduler",
        "resource_ledger",
        "trace_writer",
        "snapshot_manager",
        "invariant_checker",
        "operator_control_interface",
    ];
    for required_service in required {
        if !services
            .iter()
            .any(|service| service.service_id == required_service)
        {
            return Err(anyhow!(
                "kernel_registry.services missing required service '{required_service}'"
            ));
        }
    }
    Ok(())
}

pub(crate) fn write_relative(root: &Path, rel_path: &str, bytes: Vec<u8>) -> Result<()> {
    validate_relative_path(rel_path, "runtime_v2.write_relative")?;
    let path = root.join(rel_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("failed to create '{}'", parent.display()))?;
    }
    std::fs::write(&path, bytes).with_context(|| format!("failed to write '{}'", path.display()))
}

pub(crate) fn checksum_for_serialize(value: &impl Serialize) -> Result<String> {
    let bytes = serde_json::to_vec(value).context("serialize Runtime v2 checksum input")?;
    let mut checksum = 0xcbf29ce484222325_u64;
    for byte in bytes {
        checksum ^= u64::from(byte);
        checksum = checksum.wrapping_mul(0x100000001b3);
    }
    Ok(format!("fnv1a64:{checksum:016x}"))
}

pub(crate) fn prototype_kernel_services() -> Vec<RuntimeV2KernelServiceRegistration> {
    [
        (
            "clock_service",
            "clock",
            "runtime_v2/kernel/clock_service.json",
        ),
        (
            "identity_admission_guard",
            "admission",
            "runtime_v2/kernel/admission_guard.json",
        ),
        ("scheduler", "scheduler", "runtime_v2/kernel/scheduler.json"),
        (
            "resource_ledger",
            "resource",
            "runtime_v2/resource_ledger.json",
        ),
        ("trace_writer", "trace", "runtime_v2/traces/events.jsonl"),
        ("snapshot_manager", "snapshot", "runtime_v2/snapshots"),
        (
            "invariant_checker",
            "invariant",
            "runtime_v2/invariants/policy.json",
        ),
        (
            "operator_control_interface",
            "operator",
            "runtime_v2/operator/control_report.json",
        ),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, (service_id, service_kind, owns_artifact_path))| {
        RuntimeV2KernelServiceRegistration {
            service_id: service_id.to_string(),
            service_kind: service_kind.to_string(),
            lifecycle_state: "registered".to_string(),
            activation_order: index as u64 + 1,
            owns_artifact_path: owns_artifact_path.to_string(),
        }
    })
    .collect()
}
