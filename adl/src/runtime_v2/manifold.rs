use super::*;
impl RuntimeV2ManifoldRoot {
    pub fn prototype(manifold_id: impl Into<String>) -> Result<Self> {
        let manifold_id = normalize_id(manifold_id.into(), "manifold_id")?;
        Ok(Self {
            schema_version: RUNTIME_V2_MANIFOLD_SCHEMA.to_string(),
            artifact_path: DEFAULT_MANIFOLD_ARTIFACT_PATH.to_string(),
            lifecycle_state: "initialized".to_string(),
            clock_anchor: ManifoldClockAnchor {
                anchor_id: "clock_anchor_0000".to_string(),
                clock_kind: "monotonic_logical".to_string(),
                monotonic_tick: 0,
                observed_at_utc: "not_started".to_string(),
            },
            citizen_registry_refs: CitizenRegistryRefs {
                registry_root: "runtime_v2/citizens".to_string(),
                active_index: "runtime_v2/citizens/active_index.json".to_string(),
                pending_index: "runtime_v2/citizens/pending_index.json".to_string(),
            },
            kernel_service_refs: KernelServiceRefs {
                registry_path: "runtime_v2/kernel/service_registry.json".to_string(),
                service_loop_path: "runtime_v2/kernel/service_loop.jsonl".to_string(),
                service_state_path: "runtime_v2/kernel/service_state.json".to_string(),
            },
            trace_root: TraceRootRef {
                trace_root: "runtime_v2/traces".to_string(),
                event_log_path: "runtime_v2/traces/events.jsonl".to_string(),
                next_event_sequence: 1,
            },
            snapshot_root: SnapshotRootRef {
                snapshot_root: "runtime_v2/snapshots".to_string(),
                latest_snapshot_id: None,
                rehydration_report_path: "runtime_v2/rehydration_report.json".to_string(),
            },
            resource_ledger: ResourceLedgerRef {
                ledger_path: "runtime_v2/resource_ledger.json".to_string(),
                accounting_mode: "bounded_prototype".to_string(),
            },
            invariant_policy_refs: InvariantPolicyRefs {
                policy_path: "runtime_v2/invariants/policy.json".to_string(),
                enforcement_mode: "fail_closed_before_activation".to_string(),
                blocking_invariants: vec![
                    "single_active_manifold_instance".to_string(),
                    "no_duplicate_active_citizen_instance".to_string(),
                    "trace_sequence_must_advance_monotonically".to_string(),
                    "snapshot_restore_must_validate_before_active_state".to_string(),
                ],
            },
            review_surface: RuntimeV2ManifoldReviewSurface {
                required_artifacts: vec![
                    DEFAULT_MANIFOLD_ARTIFACT_PATH.to_string(),
                    "runtime_v2/citizens/active_index.json".to_string(),
                    "runtime_v2/kernel/service_registry.json".to_string(),
                    "runtime_v2/traces/events.jsonl".to_string(),
                    "runtime_v2/snapshots".to_string(),
                    "runtime_v2/invariants/policy.json".to_string(),
                ],
                proof_hook_command: "cargo test --manifest-path adl/Cargo.toml runtime_v2::tests::runtime_v2_manifold_root_contract_is_stable".to_string(),
                proof_hook_output_path: DEFAULT_MANIFOLD_ARTIFACT_PATH.to_string(),
                downstream_boundaries: vec![
                    "WP-06 owns the bounded kernel service loop behavior".to_string(),
                    "WP-07 owns provisional citizen record materialization".to_string(),
                    "WP-08 owns snapshot writing, sealing, and rehydration".to_string(),
                    "WP-09 owns invariant violation artifacts".to_string(),
                ],
                non_goals: vec![
                    "no true Godel-agent birthday or identity rebinding".to_string(),
                    "no full moral, emotional, or polis governance layer".to_string(),
                    "no cross-machine migration or cross-polis state transfer".to_string(),
                    "no live kernel scheduling behavior in WP-05".to_string(),
                ],
            },
            manifold_id,
        })
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_MANIFOLD_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 manifold schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.manifold_id.clone(), "manifold_id")?;
        validate_lifecycle_state(&self.lifecycle_state)?;
        validate_relative_path(&self.artifact_path, "artifact_path")?;
        validate_clock_anchor(&self.clock_anchor)?;
        validate_registry_refs(&self.citizen_registry_refs)?;
        validate_kernel_refs(&self.kernel_service_refs)?;
        validate_trace_root(&self.trace_root)?;
        validate_snapshot_root(&self.snapshot_root)?;
        validate_relative_path(
            &self.resource_ledger.ledger_path,
            "resource_ledger.ledger_path",
        )?;
        normalize_id(
            self.resource_ledger.accounting_mode.clone(),
            "resource_ledger.accounting_mode",
        )?;
        validate_invariant_policy_refs(&self.invariant_policy_refs)?;
        validate_review_surface(&self.review_surface)?;
        Ok(())
    }

    pub fn to_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize Runtime v2 manifold root")
    }

    pub fn write_to_path(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create Runtime v2 manifold parent '{}'",
                    parent.display()
                )
            })?;
        }
        std::fs::write(path, self.to_pretty_json_bytes()?).with_context(|| {
            format!(
                "failed to write Runtime v2 manifold root '{}'",
                path.display()
            )
        })
    }

    pub fn read_from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let bytes = std::fs::read(path).with_context(|| {
            format!(
                "failed to read Runtime v2 manifold root '{}'",
                path.display()
            )
        })?;
        let root: Self =
            serde_json::from_slice(&bytes).context("parse Runtime v2 manifold root")?;
        root.validate()?;
        Ok(root)
    }

    pub fn artifact_path_buf(&self) -> PathBuf {
        PathBuf::from(&self.artifact_path)
    }
}

