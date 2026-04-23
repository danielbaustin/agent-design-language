//! Runtime-v2 kernel loop contracts and service-loop state artifacts.
//!
//! Documents the public contract used by the service loop for service registry,
//! execution state, and event tracking.

use super::*;
impl RuntimeV2KernelLoopArtifacts {
    pub fn prototype(manifold: &RuntimeV2ManifoldRoot) -> Result<Self> {
        manifold.validate()?;
        let services = prototype_kernel_services();
        let events = services
            .iter()
            .enumerate()
            .map(|(index, service)| RuntimeV2KernelLoopEvent {
                schema_version: RUNTIME_V2_KERNEL_LOOP_EVENT_SCHEMA.to_string(),
                event_sequence: manifold.trace_root.next_event_sequence + index as u64,
                manifold_id: manifold.manifold_id.clone(),
                service_id: service.service_id.clone(),
                action: "service_tick".to_string(),
                outcome: "observed_ready".to_string(),
                artifact_ref: service.owns_artifact_path.clone(),
            })
            .collect::<Vec<_>>();
        let completed_through_event_sequence = events
            .last()
            .map(|event| event.event_sequence)
            .unwrap_or(manifold.trace_root.next_event_sequence);
        let state_services = events
            .iter()
            .map(|event| RuntimeV2KernelServiceStatus {
                service_id: event.service_id.clone(),
                lifecycle_state: "ready".to_string(),
                last_event_sequence: event.event_sequence,
                blocked_reason: None,
            })
            .collect::<Vec<_>>();
        let artifacts = Self {
            registry: RuntimeV2KernelServiceRegistry {
                schema_version: RUNTIME_V2_KERNEL_SERVICE_REGISTRY_SCHEMA.to_string(),
                manifold_id: manifold.manifold_id.clone(),
                registry_path: manifold.kernel_service_refs.registry_path.clone(),
                services,
            },
            state: RuntimeV2KernelServiceState {
                schema_version: RUNTIME_V2_KERNEL_SERVICE_STATE_SCHEMA.to_string(),
                manifold_id: manifold.manifold_id.clone(),
                service_state_path: manifold.kernel_service_refs.service_state_path.clone(),
                loop_status: "bounded_tick_complete".to_string(),
                completed_through_event_sequence,
                services: state_services,
            },
            events,
            service_loop_path: manifold.kernel_service_refs.service_loop_path.clone(),
        };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.registry.validate()?;
        self.state.validate()?;
        validate_relative_path(&self.service_loop_path, "kernel_loop.service_loop_path")?;
        if self.events.is_empty() {
            return Err(anyhow!("kernel_loop.events must not be empty"));
        }
        if self.registry.manifold_id != self.state.manifold_id {
            return Err(anyhow!(
                "kernel registry and service state manifold ids must match"
            ));
        }
        let mut seen_services = Vec::new();
        for (expected_sequence, event) in (self.events[0].event_sequence..).zip(self.events.iter())
        {
            event.validate()?;
            if event.manifold_id != self.registry.manifold_id {
                return Err(anyhow!("kernel loop event manifold id must match registry"));
            }
            if event.event_sequence != expected_sequence {
                return Err(anyhow!(
                    "kernel loop events must be contiguous and monotonically ordered"
                ));
            }
            if !self
                .registry
                .services
                .iter()
                .any(|service| service.service_id == event.service_id)
            {
                return Err(anyhow!(
                    "kernel loop event references unknown service '{}'",
                    event.service_id
                ));
            }
            seen_services.push(event.service_id.clone());
        }
        let registry_ids = self
            .registry
            .services
            .iter()
            .map(|service| service.service_id.clone())
            .collect::<Vec<_>>();
        if seen_services != registry_ids {
            return Err(anyhow!(
                "kernel loop event order must match service activation order"
            ));
        }
        if self.state.completed_through_event_sequence
            != self
                .events
                .last()
                .expect("events checked non-empty")
                .event_sequence
        {
            return Err(anyhow!(
                "kernel service state must record the last loop event sequence"
            ));
        }
        Ok(())
    }

    pub fn registry_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.registry).context("serialize kernel service registry")
    }

    pub fn state_pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(&self.state).context("serialize kernel service state")
    }

    pub fn service_loop_jsonl_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut out = Vec::new();
        for event in &self.events {
            serde_json::to_writer(&mut out, event).context("serialize kernel loop event")?;
            out.push(b'\n');
        }
        Ok(out)
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        write_relative(
            root,
            &self.registry.registry_path,
            self.registry_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            &self.state.service_state_path,
            self.state_pretty_json_bytes()?,
        )?;
        write_relative(
            root,
            &self.service_loop_path,
            self.service_loop_jsonl_bytes()?,
        )?;
        Ok(())
    }
}

impl RuntimeV2KernelServiceRegistry {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_KERNEL_SERVICE_REGISTRY_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 kernel service registry schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.manifold_id.clone(), "kernel_registry.manifold_id")?;
        validate_relative_path(&self.registry_path, "kernel_registry.registry_path")?;
        if self.services.is_empty() {
            return Err(anyhow!("kernel_registry.services must not be empty"));
        }
        let mut seen = std::collections::BTreeSet::new();
        for (index, service) in self.services.iter().enumerate() {
            service.validate()?;
            if !seen.insert(service.service_id.clone()) {
                return Err(anyhow!(
                    "kernel_registry.services contains duplicate service '{}'",
                    service.service_id
                ));
            }
            if service.activation_order != index as u64 + 1 {
                return Err(anyhow!(
                    "kernel_registry.services activation_order must be contiguous"
                ));
            }
        }
        validate_required_kernel_services(&self.services)
    }
}

impl RuntimeV2KernelServiceRegistration {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.service_id.clone(), "kernel_service.service_id")?;
        normalize_id(self.service_kind.clone(), "kernel_service.service_kind")?;
        validate_service_lifecycle_state(&self.lifecycle_state, "kernel_service.lifecycle_state")?;
        if self.activation_order == 0 {
            return Err(anyhow!("kernel_service.activation_order must be positive"));
        }
        validate_relative_path(
            &self.owns_artifact_path,
            "kernel_service.owns_artifact_path",
        )
    }
}

impl RuntimeV2KernelServiceState {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_KERNEL_SERVICE_STATE_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 kernel service state schema '{}'",
                self.schema_version
            ));
        }
        normalize_id(self.manifold_id.clone(), "kernel_state.manifold_id")?;
        validate_relative_path(&self.service_state_path, "kernel_state.service_state_path")?;
        normalize_id(self.loop_status.clone(), "kernel_state.loop_status")?;
        if self.completed_through_event_sequence == 0 {
            return Err(anyhow!(
                "kernel_state.completed_through_event_sequence must be positive"
            ));
        }
        if self.services.is_empty() {
            return Err(anyhow!("kernel_state.services must not be empty"));
        }
        for service in &self.services {
            service.validate()?;
        }
        Ok(())
    }
}

impl RuntimeV2KernelServiceStatus {
    pub fn validate(&self) -> Result<()> {
        normalize_id(self.service_id.clone(), "kernel_service_status.service_id")?;
        validate_service_lifecycle_state(
            &self.lifecycle_state,
            "kernel_service_status.lifecycle_state",
        )?;
        if self.last_event_sequence == 0 {
            return Err(anyhow!(
                "kernel_service_status.last_event_sequence must be positive"
            ));
        }
        if let Some(reason) = &self.blocked_reason {
            if reason.trim().is_empty() {
                return Err(anyhow!(
                    "kernel_service_status.blocked_reason must not be empty when present"
                ));
            }
        }
        Ok(())
    }
}

impl RuntimeV2KernelLoopEvent {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_KERNEL_LOOP_EVENT_SCHEMA {
            return Err(anyhow!(
                "unsupported Runtime v2 kernel loop event schema '{}'",
                self.schema_version
            ));
        }
        if self.event_sequence == 0 {
            return Err(anyhow!("kernel_loop_event.event_sequence must be positive"));
        }
        normalize_id(self.manifold_id.clone(), "kernel_loop_event.manifold_id")?;
        normalize_id(self.service_id.clone(), "kernel_loop_event.service_id")?;
        normalize_id(self.action.clone(), "kernel_loop_event.action")?;
        match self.outcome.as_str() {
            "observed_ready" | "deferred" | "refused" | "blocked" => {}
            other => return Err(anyhow!("unsupported kernel_loop_event.outcome '{other}'")),
        }
        validate_relative_path(&self.artifact_ref, "kernel_loop_event.artifact_ref")
    }
}
