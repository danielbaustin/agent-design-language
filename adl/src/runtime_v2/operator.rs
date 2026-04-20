use super::*;
impl RuntimeV2OperatorControlReport {
    pub fn prototype(
        manifold: &RuntimeV2ManifoldRoot,
        kernel: &RuntimeV2KernelLoopArtifacts,
        citizens: &RuntimeV2CitizenLifecycleArtifacts,
        snapshot: &RuntimeV2SnapshotAndRehydrationArtifacts,
        violation: &RuntimeV2InvariantViolationArtifact,
    ) -> Result<Self> {
        manifold.validate()?;
        kernel.validate()?;
        citizens.validate()?;
        snapshot.validate()?;
        violation.validate()?;
        if kernel.state.manifold_id != manifold.manifold_id
            || citizens.active_index.manifold_id != manifold.manifold_id
            || snapshot.snapshot.manifold_id != manifold.manifold_id
            || violation.manifold_id != manifold.manifold_id
        {
            return Err(anyhow!(
                "operator control inputs must share the same manifold id"
            ));
        }
        let active_state = RuntimeV2OperatorControlState::from_parts(
            "active",
            &kernel.state,
            citizens,
            manifold.snapshot_root.latest_snapshot_id.clone(),
        );
        let paused_state = RuntimeV2OperatorControlState {
            manifold_lifecycle_state: "paused".to_string(),
            kernel_loop_status: "operator_paused".to_string(),
            ..active_state.clone()
        };
        let _snapshotting_state = RuntimeV2OperatorControlState::from_parts(
            "snapshotting",
            &kernel.state,
            citizens,
            Some(snapshot.snapshot.snapshot_id.clone()),
        );
        let _terminated_state = RuntimeV2OperatorControlState {
            manifold_lifecycle_state: "terminated".to_string(),
            kernel_loop_status: "operator_terminated".to_string(),
            active_citizen_count: 0,
            pending_citizen_count: citizens.pending_index.citizens.len(),
            latest_snapshot_id: Some(snapshot.snapshot.snapshot_id.clone()),
            completed_through_event_sequence: kernel.state.completed_through_event_sequence + 6,
        };
        let report = Self {
            schema_version: RUNTIME_V2_OPERATOR_CONTROL_REPORT_SCHEMA.to_string(),
            report_id: "operator-report-0001".to_string(),
            manifold_id: manifold.manifold_id.clone(),
            artifact_path: "runtime_v2/operator/control_report.json".to_string(),
            generated_at_utc: "not_started".to_string(),
            control_interface_service_id: "operator_control_interface".to_string(),
            commands: vec![
                RuntimeV2OperatorCommandReport {
                    command: "inspect_manifold".to_string(),
                    requested_by: "operator.cli".to_string(),
                    affected_service: "operator_control_interface".to_string(),
                    pre_state: active_state.clone(),
                    post_state: active_state.clone(),
                    outcome: "allowed".to_string(),
                    trace_event_ref: "runtime_v2/traces/operator/inspect-manifold.json".to_string(),
                    reason: "reported bounded manifold lifecycle and kernel status".to_string(),
                },
                RuntimeV2OperatorCommandReport {
                    command: "inspect_citizens".to_string(),
                    requested_by: "operator.cli".to_string(),
                    affected_service: "operator_control_interface".to_string(),
                    pre_state: active_state.clone(),
                    post_state: active_state.clone(),
                    outcome: "allowed".to_string(),
                    trace_event_ref: "runtime_v2/traces/operator/inspect-citizens.json".to_string(),
                    reason: "reported active and pending provisional citizen counts".to_string(),
                },
                RuntimeV2OperatorCommandReport {
                    command: "pause_manifold".to_string(),
                    requested_by: "operator.cli".to_string(),
                    affected_service: "scheduler".to_string(),
                    pre_state: active_state.clone(),
                    post_state: active_state.clone(),
                    outcome: "deferred".to_string(),
                    trace_event_ref: "runtime_v2/traces/operator/pause-manifold.json".to_string(),
                    reason: "pause command is deferred while kernel lifecycle gating is explicit"
                        .to_string(),
                },
                RuntimeV2OperatorCommandReport {
                    command: "resume_manifold".to_string(),
                    requested_by: "operator.cli".to_string(),
                    affected_service: "scheduler".to_string(),
                    pre_state: paused_state.clone(),
                    post_state: paused_state,
                    outcome: "refused".to_string(),
                    trace_event_ref: "runtime_v2/traces/operator/resume-manifold.json".to_string(),
                    reason: "invalid resume attempt is deferred until fresh invariant evidence exists"
                        .to_string(),
                },
                RuntimeV2OperatorCommandReport {
                    command: "request_snapshot".to_string(),
                    requested_by: "operator.cli".to_string(),
                    affected_service: "snapshot_manager".to_string(),
                    pre_state: active_state.clone(),
                    post_state: active_state.clone(),
                    outcome: "deferred".to_string(),
                    trace_event_ref: "runtime_v2/traces/operator/request-snapshot.json".to_string(),
                    reason: "snapshot command is deferred while live snapshot re-hydration policy is under v0.90.1 guard"
                        .to_string(),
                },
                RuntimeV2OperatorCommandReport {
                    command: "inspect_last_failures".to_string(),
                    requested_by: "operator.cli".to_string(),
                    affected_service: "invariant_checker".to_string(),
                    pre_state: active_state.clone(),
                    post_state: active_state.clone(),
                    outcome: "allowed".to_string(),
                    trace_event_ref: violation.result.trace_ref.clone(),
                    reason: format!(
                        "latest blocking invariant failure is {}",
                        violation.violation_id
                    ),
                },
                RuntimeV2OperatorCommandReport {
                    command: "terminate_manifold".to_string(),
                    requested_by: "operator.cli".to_string(),
                    affected_service: "resource_ledger".to_string(),
                    pre_state: active_state.clone(),
                    post_state: active_state,
                    outcome: "deferred".to_string(),
                    trace_event_ref: "runtime_v2/traces/operator/terminate-manifold.json"
                        .to_string(),
                    reason: "termination is deferred until live deactivation policy is explicit"
                        .to_string(),
                },
            ],
        };
        report.validate()?;
        Ok(report)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_OPERATOR_CONTROL_REPORT_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 operator control report schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.report_id.clone(), "operator_control.report_id")?;
        normalize_id(self.manifold_id.clone(), "operator_control.manifold_id")?;
        validate_relative_path(&self.artifact_path, "operator_control.artifact_path")?;
        validate_timestamp_marker(&self.generated_at_utc, "operator_control.generated_at_utc")?;
        normalize_id(
            self.control_interface_service_id.clone(),
            "operator_control.control_interface_service_id",
        )?;
        if self.control_interface_service_id != "operator_control_interface" {
            return Err(anyhow!(
                "operator control report must be owned by operator_control_interface"
            ));
        }
        validate_operator_commands(&self.commands)?;
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 operator control report")
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        write_relative(
            root.as_ref(),
            &self.artifact_path,
            self.to_pretty_json_bytes()?,
        )
    }
}

impl RuntimeV2OperatorControlState {
    fn from_parts(
        manifold_lifecycle_state: &str,
        kernel_state: &RuntimeV2KernelServiceState,
        citizens: &RuntimeV2CitizenLifecycleArtifacts,
        latest_snapshot_id: Option<String>,
    ) -> Self {
        Self {
            manifold_lifecycle_state: manifold_lifecycle_state.to_string(),
            kernel_loop_status: kernel_state.loop_status.clone(),
            active_citizen_count: citizens.active_index.citizens.len(),
            pending_citizen_count: citizens.pending_index.citizens.len(),
            latest_snapshot_id,
            completed_through_event_sequence: kernel_state.completed_through_event_sequence,
        }
    }

    pub fn validate(&self) -> Result<()> {
        validate_lifecycle_state(&self.manifold_lifecycle_state)?;
        normalize_id(
            self.kernel_loop_status.clone(),
            "operator_control.kernel_loop_status",
        )?;
        if let Some(snapshot_id) = &self.latest_snapshot_id {
            normalize_id(snapshot_id.clone(), "operator_control.latest_snapshot_id")?;
        }
        if self.completed_through_event_sequence == 0 {
            return Err(anyhow!(
                "operator_control.completed_through_event_sequence must be positive"
            ));
        }
        if self.manifold_lifecycle_state == "terminated" && self.active_citizen_count != 0 {
            return Err(anyhow!(
                "operator_control terminated state must not retain active citizens"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2OperatorCommandReport {
    pub fn validate(&self) -> Result<()> {
        validate_operator_command(&self.command)?;
        normalize_id(self.requested_by.clone(), "operator_control.requested_by")?;
        normalize_id(
            self.affected_service.clone(),
            "operator_control.affected_service",
        )?;
        self.pre_state.validate()?;
        self.post_state.validate()?;
        validate_operator_outcome(&self.outcome)?;
        validate_relative_path(&self.trace_event_ref, "operator_control.trace_event_ref")?;
        validate_nonempty_text(&self.reason, "operator_control.reason")?;
        if self.outcome == "allowed" && self.pre_state == self.post_state {
            match self.command.as_str() {
                "inspect_manifold" | "inspect_citizens" | "inspect_last_failures" => {}
                _ => {
                    return Err(anyhow!(
                        "operator mutating control commands must change post_state when allowed"
                    ))
                }
            }
        }
        Ok(())
    }
}
