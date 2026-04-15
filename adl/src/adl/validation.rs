use anyhow::{anyhow, Result};
use std::collections::HashMap;

use super::*;

impl AdlDoc {
    /// Lightweight validation so we can fail fast with good errors.
    ///
    /// Invariants enforced here include:
    /// - id/reference consistency across providers/tools/agents/tasks/workflows
    /// - safe `write_to` path policy
    /// - retry and concurrency bounds (`>= 1` where specified)
    /// - mutual exclusion constraints for pattern/workflow run shapes
    pub fn validate(&self) -> Result<()> {
        if matches!(self.run.defaults.max_concurrency, Some(0)) {
            return Err(anyhow!(
                "run.defaults.max_concurrency must be >= 1 when provided"
            ));
        }
        if matches!(
            self.run.workflow.as_ref().and_then(|wf| wf.max_concurrency),
            Some(0)
        ) {
            return Err(anyhow!(
                "run.workflow.max_concurrency must be >= 1 when provided"
            ));
        }
        for workflow_id in sorted_keys(&self.workflows) {
            let workflow = &self.workflows[workflow_id];
            if matches!(workflow.max_concurrency, Some(0)) {
                return Err(anyhow!(
                    "workflows.{workflow_id}.max_concurrency must be >= 1 when provided"
                ));
            }
        }

        validate_id_fields("providers", &self.providers, |spec| spec.id.as_deref())?;
        validate_id_fields("tools", &self.tools, |spec| spec.id.as_deref())?;
        validate_id_fields("agents", &self.agents, |spec| spec.id.as_deref())?;
        validate_id_fields("tasks", &self.tasks, |spec| spec.id.as_deref())?;
        validate_id_fields("workflows", &self.workflows, |spec| spec.id.as_deref())?;

        for provider_id in sorted_keys(&self.providers) {
            let provider = &self.providers[provider_id];
            validate_provider(provider_id, provider)?;
        }

        for tool_id in sorted_keys(&self.tools) {
            let tool = &self.tools[tool_id];
            validate_tool(tool_id, tool)?;
        }

        for agent_id in sorted_keys(&self.agents) {
            let agent = &self.agents[agent_id];
            if !self.providers.is_empty() && !self.providers.contains_key(&agent.provider) {
                return Err(anyhow!(
                    "agents.{agent_id}.provider references unknown provider '{}'",
                    agent.provider
                ));
            }
            for tool_ref in &agent.tools {
                if !self.tools.contains_key(tool_ref) {
                    return Err(anyhow!(
                        "agents.{agent_id}.tools references unknown tool '{tool_ref}'"
                    ));
                }
            }
        }

        for task_id in sorted_keys(&self.tasks) {
            let task = &self.tasks[task_id];
            if let Some(agent_ref) = task.agent_ref.as_deref() {
                if !self.agents.contains_key(agent_ref) {
                    return Err(anyhow!(
                        "tasks.{task_id}.agent_ref references unknown agent '{agent_ref}'"
                    ));
                }
            }
            for tool_ref in &task.tool_allowlist {
                if !self.tools.contains_key(tool_ref) {
                    return Err(anyhow!(
                        "tasks.{task_id}.tool_allowlist references unknown tool '{tool_ref}'"
                    ));
                }
            }
        }

        let mut seen_patterns = std::collections::HashSet::new();
        for pattern in &self.patterns {
            if pattern.id.trim().is_empty() {
                return Err(anyhow!("pattern id must not be empty"));
            }
            if !seen_patterns.insert(pattern.id.clone()) {
                return Err(anyhow!("duplicate pattern id '{}'", pattern.id));
            }
            pattern.validate()?;
        }

        if let Some(remote) = self.run.remote.as_ref() {
            if remote.endpoint.trim().is_empty() {
                return Err(anyhow!("run.remote.endpoint must not be empty"));
            }
            if remote.require_key_id && !remote.require_signed_requests {
                return Err(anyhow!(
                            "run.remote.require_key_id=true requires run.remote.require_signed_requests=true"
                        ));
            }
            for source in &remote.verify_allowed_key_sources {
                if crate::signing::VerificationKeySource::parse(source).is_none() {
                    return Err(anyhow!(
                                "run.remote.verify_allowed_key_sources contains unsupported source '{}' (allowed: embedded, explicit_key)",
                                source
                            ));
                }
            }
        }
        if let Some(policy) = self.run.delegation_policy.as_ref() {
            let mut seen_rule_ids = std::collections::BTreeSet::new();
            for (idx, rule) in policy.rules.iter().enumerate() {
                if rule.id.trim().is_empty() {
                    return Err(anyhow!(
                        "run.delegation_policy.rules[{idx}].id must not be empty"
                    ));
                }
                if !seen_rule_ids.insert(rule.id.clone()) {
                    return Err(anyhow!(
                        "run.delegation_policy.rules contains duplicate id '{}'",
                        rule.id
                    ));
                }
                if let Some(target_id) = rule.target_id.as_ref() {
                    if target_id.trim().is_empty() {
                        return Err(anyhow!(
                                    "run.delegation_policy.rules[{idx}].target_id must not be empty when provided"
                                ));
                    }
                }
            }
        }

        if let Some(pattern_ref) = self.run.pattern_ref.as_ref() {
            if !self.patterns.iter().any(|p| p.id == *pattern_ref) {
                return Err(anyhow!(
                    "run.pattern_ref references unknown pattern '{}'",
                    pattern_ref
                ));
            }
            if self.run.workflow_ref.is_some() || self.run.workflow.is_some() {
                return Err(anyhow!(
                            "run.pattern_ref cannot be combined with run.workflow_ref or inline run.workflow"
                        ));
            }
            return Ok(());
        }

        let workflow = self.run.resolve_workflow(self)?;
        let mut conversation_turn_ids = std::collections::HashSet::new();
        for (idx, step) in workflow.steps.iter().enumerate() {
            let step_id = step
                .id
                .as_deref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("step-{idx}"));

            if step_id.starts_with("p::") {
                return Err(anyhow!(
                    "step id '{}' uses reserved compiler prefix 'p::'",
                    step_id
                ));
            }

            if let Some(agent) = step.agent.as_ref() {
                if !self.agents.is_empty() && !self.agents.contains_key(agent) {
                    return Err(anyhow!(
                        "run.workflow.steps[{idx}] references unknown agent '{agent}'"
                    ));
                }
            }

            if let Some(task_ref) = step.task.as_ref() {
                if !self.tasks.is_empty() && !self.tasks.contains_key(task_ref) {
                    return Err(anyhow!(
                        "run.workflow.steps[{idx}] references unknown task '{task_ref}'"
                    ));
                }
            }

            if let Some(prompt) = step.prompt.as_ref() {
                // In MVP, `prompt` is an inline PromptSpec, so nothing to resolve.
                // Keep a placeholder for future prompt registries.
                let _ = prompt;
            }

            if step.write_to.is_some() && step.save_as.is_none() {
                return Err(anyhow!(
                    "step '{}' uses write_to but is missing save_as",
                    step_id
                ));
            }

            if let Some(write_to) = step.write_to.as_deref() {
                if write_to.trim().is_empty() {
                    return Err(anyhow!("step '{}' has empty write_to path", step_id));
                }
                let path = std::path::Path::new(write_to);
                if path.is_absolute()
                    || path
                        .components()
                        .any(|c| matches!(c, std::path::Component::ParentDir))
                {
                    return Err(anyhow!(
                        "step '{}' write_to must be a relative path without '..'",
                        step_id
                    ));
                }
            }

            if let Some(retry) = step.retry.as_ref() {
                if retry.max_attempts == 0 {
                    return Err(anyhow!(
                        "step '{}' has invalid retry.max_attempts=0 (must be >= 1)",
                        step_id
                    ));
                }
            }

            if let Some(turn) = step.conversation.as_ref() {
                if turn.id.trim().is_empty() {
                    return Err(anyhow!(
                        "step '{}' conversation.id must not be empty",
                        step_id
                    ));
                }
                if turn.speaker.trim().is_empty() {
                    return Err(anyhow!(
                        "step '{}' conversation.speaker must not be empty",
                        step_id
                    ));
                }
                if matches!(turn.sequence, Some(0)) {
                    return Err(anyhow!(
                        "step '{}' conversation.sequence must be >= 1",
                        step_id
                    ));
                }
                if turn.responds_to.as_deref() == Some(turn.id.as_str()) {
                    return Err(anyhow!(
                        "step '{}' conversation.responds_to must not reference the same turn id",
                        step_id
                    ));
                }
                if !conversation_turn_ids.insert(turn.id.clone()) {
                    return Err(anyhow!(
                        "duplicate conversation turn id '{}' (must be unique per workflow)",
                        turn.id
                    ));
                }
            }
        }

        Ok(())
    }
}

fn sorted_keys<T>(m: &HashMap<String, T>) -> Vec<&String> {
    let mut keys: Vec<&String> = m.keys().collect();
    keys.sort();
    keys
}

fn validate_id_fields<T>(
    section: &str,
    items: &HashMap<String, T>,
    get_id: impl Fn(&T) -> Option<&str>,
) -> Result<()> {
    for key in sorted_keys(items) {
        if let Some(explicit_id) = get_id(&items[key]) {
            if explicit_id != key {
                return Err(anyhow!(
                    "{section}.{key}.id must match key '{key}' when provided (found '{explicit_id}')"
                ));
            }
        }
    }
    Ok(())
}

pub(super) fn validate_provider(provider_id: &str, provider: &ProviderSpec) -> Result<()> {
    if let Some(profile) = provider.profile.as_deref() {
        if profile.trim().is_empty() {
            return Err(anyhow!("providers.{provider_id}.profile must not be empty"));
        }
        if !provider.kind.trim().is_empty()
            || provider.base_url.is_some()
            || provider.default_model.is_some()
        {
            return Err(anyhow!(
                "providers.{provider_id} uses profile and explicit provider identity fields together (remove type/base_url/default_model when profile is set; config remains available for bounded compatibility overrides)"
            ));
        }
        return Ok(());
    }

    match provider.kind.as_str() {
        "ollama" => {
            let endpoint = provider
                .base_url
                .as_deref()
                .or_else(|| provider.config.get("endpoint").and_then(|v| v.as_str()));
            if let Some(endpoint) = endpoint {
                if !crate::provider::is_allowed_ollama_endpoint(endpoint) {
                    return Err(anyhow!(
                        "providers.{provider_id} kind 'ollama' requires an http:// or https:// base_url/config.endpoint when remote transport is configured"
                    ));
                }
            }
            Ok(())
        }
        "local_ollama" => {
            if provider.base_url.is_some()
                || provider
                    .config
                    .get("endpoint")
                    .and_then(|v| v.as_str())
                    .is_some()
            {
                return Err(anyhow!(
                    "providers.{provider_id} kind 'local_ollama' is CLI-backed and must not set base_url or config.endpoint; use kind 'ollama' for remote HTTP transport"
                ));
            }
            Ok(())
        }
        "mock" | "openai" | "anthropic" => Ok(()),
        "http" | "http_remote" => {
            let endpoint = provider
                .base_url
                .as_deref()
                .or_else(|| provider.config.get("endpoint").and_then(|v| v.as_str()));
            let Some(endpoint) = endpoint else {
                return Err(anyhow!(
                    "providers.{provider_id} kind '{}' requires base_url or config.endpoint",
                    provider.kind
                ));
            };
            if !crate::provider::is_allowed_remote_endpoint(endpoint) {
                return Err(anyhow!(
                    "providers.{provider_id} kind '{}' requires an https:// base_url or config.endpoint; plaintext http:// is only allowed for localhost/loopback test endpoints",
                    provider.kind
                ));
            }
            Ok(())
        }
        other => Err(anyhow!(
            "providers.{provider_id} has unsupported kind '{other}' (supported: ollama, local_ollama, mock, http, http_remote, openai, anthropic)"
        )),
    }
}

pub(super) fn validate_tool(tool_id: &str, tool: &ToolSpec) -> Result<()> {
    match tool.kind.as_str() {
        "mcp" | "local" | "http" | "builtin" => Ok(()),
        other => Err(anyhow!(
            "tools.{tool_id} has unsupported kind '{other}' (supported: mcp, local, http, builtin)"
        )),
    }
}
