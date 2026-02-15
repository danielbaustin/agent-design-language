use anyhow::{anyhow, Result};
use std::collections::HashMap;

use crate::adl;
use crate::plan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AdlVersion {
    V0_1,
    V0_2,
    V0_3,
}

fn parse_version(version: &str) -> Result<AdlVersion> {
    let v = version.trim();
    match v {
        "0.1" => Ok(AdlVersion::V0_1),
        "0.2" => Ok(AdlVersion::V0_2),
        "0.3" => Ok(AdlVersion::V0_3),
        "" => Err(anyhow!("ADL document is missing required field: version")),
        _ => Err(anyhow!(
            "unsupported ADL version '{v}' (supported: 0.1, 0.2, 0.3)"
        )),
    }
}

/// A resolved view of the `run` section that is convenient for printing and prompt assembly.
#[derive(Debug, Clone)]
pub struct AdlResolved {
    pub run_id: String,
    pub workflow_id: String,
    pub steps: Vec<ResolvedStep>,
    /// Copy of the document for lookups (tasks, agents, defaults, etc.).
    pub doc: adl::AdlDoc,
}

#[derive(Debug, Clone)]
pub struct ResolvedStep {
    pub id: String,
    pub agent: Option<String>,
    pub provider: Option<String>,
    pub task: Option<String>,
    pub prompt: Option<adl::PromptSpec>,
    pub inputs: HashMap<String, String>,
    pub save_as: Option<String>,
    pub write_to: Option<String>,
}

impl ResolvedStep {
    /// Returns the prompt to use for this step in priority order:
    /// 1) step.prompt
    /// 2) task.prompt (if task is set)
    /// 3) agent.prompt (if agent is set)
    pub fn effective_prompt<'a>(
        &'a self,
        resolved: &'a AdlResolved,
    ) -> Option<&'a adl::PromptSpec> {
        if let Some(p) = self.prompt.as_ref() {
            return Some(p);
        }

        if let Some(task_key) = self.task.as_ref() {
            if let Some(task) = resolved.doc.tasks.get(task_key) {
                return Some(&task.prompt);
            }
        }

        if let Some(agent_key) = self.agent.as_ref() {
            if let Some(agent) = resolved.doc.agents.get(agent_key) {
                if let Some(p) = agent.prompt.as_ref() {
                    return Some(p);
                }
            }
        }

        None
    }

    /// Returns the effective prompt with run.defaults.system applied
    /// if the prompt has no system message.
    pub fn effective_prompt_with_defaults(
        &self,
        resolved: &AdlResolved,
    ) -> Option<adl::PromptSpec> {
        let mut p = self.effective_prompt(resolved)?.clone();
        if p.system.is_none() {
            if let Some(default_system) = resolved.doc.run.defaults.system.as_ref() {
                p.system = Some(default_system.clone());
            }
        }
        Some(p)
    }
}

/// Resolve a provider id for a step using these rules:
/// 1) If the step has an agent and that agent has `provider`, use it.
/// 2) Else, if the doc defines exactly one provider, use that.
/// 3) Else, return None (unresolved).
fn resolve_provider_for_step(step: &adl::StepSpec, doc: &adl::AdlDoc) -> Option<String> {
    // Agent-level provider wins.
    if let Some(agent_id) = step.agent.as_ref() {
        if let Some(agent) = doc.agents.get(agent_id) {
            if !agent.provider.trim().is_empty() {
                return Some(agent.provider.clone());
            }
        }
    }

    // Fallback: if there is exactly one provider in the doc, use it.
    if doc.providers.len() == 1 {
        return doc.providers.keys().next().cloned();
    }

    None
}

/// Resolve the run section into a deterministic, convenient form.
pub fn resolve_run(doc: &adl::AdlDoc) -> Result<AdlResolved> {
    let _version = parse_version(&doc.version)?;

    let run_id = doc.run.name.clone().unwrap_or_else(|| "run".to_string());
    // WorkflowSpec currently has no explicit `id` field; keep a stable label for printing/tracing.
    let workflow_id = "workflow".to_string();

    let mut steps = Vec::new();
    for (idx, s) in doc.run.workflow.steps.iter().enumerate() {
        // Preserve explicit step ids; otherwise derive a deterministic fallback.
        let id =
            s.id.clone()
                .unwrap_or_else(|| s.task.clone().unwrap_or_else(|| format!("step-{idx}")));

        let provider = resolve_provider_for_step(s, doc);

        steps.push(ResolvedStep {
            id,
            agent: s.agent.clone(),
            provider,
            task: s.task.clone(),
            prompt: s.prompt.clone(),
            inputs: s.inputs.clone(),
            save_as: s.save_as.clone(),
            write_to: s.write_to.clone(),
        });
    }

    Ok(AdlResolved {
        run_id,
        workflow_id,
        steps,
        doc: doc.clone(),
    })
}

/// Used by the CLI `--print-plan` path; kept separate from execution for clarity.
pub fn print_resolved_plan(resolved: &AdlResolved) {
    plan::print_plan(
        plan::PlanHeaders {
            run: "Resolved run:",
            workflow: "Workflow:    ",
            steps: "Steps:       ",
        },
        &resolved.run_id,
        &resolved.workflow_id,
        resolved.steps.len(),
        resolved.steps.iter(),
        |step| {
            let agent = step.agent.as_deref().unwrap_or("<unresolved-agent>");
            let provider = step.provider.as_deref().unwrap_or("<unresolved-provider>");
            let task = step.task.as_deref().unwrap_or("<unresolved-task>");
            format!("{}  agent={agent} provider={provider} task={task}", step.id)
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_doc() -> adl::AdlDoc {
        let mut providers = std::collections::HashMap::new();
        providers.insert(
            "local".to_string(),
            adl::ProviderSpec {
                kind: "ollama".to_string(),
                base_url: None,
                default_model: None,
                config: std::collections::HashMap::new(),
            },
        );

        let mut agents = std::collections::HashMap::new();
        agents.insert(
            "a1".to_string(),
            adl::AgentSpec {
                provider: "local".to_string(),
                model: "phi4-mini".to_string(),
                temperature: None,
                top_k: None,
                description: None,
                prompt: Some(adl::PromptSpec {
                    system: None,
                    developer: None,
                    user: Some("agent user".to_string()),
                    context: None,
                    output: None,
                }),
                tools: vec![],
            },
        );

        let mut tasks = std::collections::HashMap::new();
        tasks.insert(
            "t1".to_string(),
            adl::TaskSpec {
                description: None,
                prompt: adl::PromptSpec {
                    system: None,
                    developer: None,
                    user: Some("task user".to_string()),
                    context: None,
                    output: None,
                },
            },
        );

        adl::AdlDoc {
            version: "0.1".to_string(),
            providers,
            tools: std::collections::HashMap::new(),
            agents,
            tasks,
            run: adl::RunSpec {
                name: Some("r".to_string()),
                created_at: None,
                defaults: adl::RunDefaults::default(),
                workflow: adl::WorkflowSpec {
                    kind: adl::WorkflowKind::Sequential,
                    steps: vec![],
                },
            },
        }
    }

    #[test]
    fn resolve_run_requires_nonempty_version() {
        let mut doc = minimal_doc();
        doc.version = "".to_string();
        let err = resolve_run(&doc).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("missing required field") || msg.contains("version"),
            "{msg}"
        );
    }

    #[test]
    fn resolve_run_rejects_unsupported_version() {
        let mut doc = minimal_doc();
        doc.version = "9.9".to_string();
        let err = resolve_run(&doc).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("unsupported ADL version") && msg.contains("9.9"),
            "{msg}"
        );
    }

    #[test]
    fn resolve_run_accepts_v0_3() {
        let mut doc = minimal_doc();
        doc.version = "0.3".to_string();
        let resolved = resolve_run(&doc).expect("v0.3 should resolve for parse/plan");
        assert_eq!(resolved.doc.version, "0.3");
    }

    #[test]
    fn resolve_provider_prefers_agent_provider() {
        let mut doc = minimal_doc();
        // Add a second provider so we prove agent-level selection wins.
        doc.providers.insert(
            "other".to_string(),
            adl::ProviderSpec {
                kind: "ollama".to_string(),
                base_url: None,
                default_model: None,
                config: std::collections::HashMap::new(),
            },
        );

        let step = adl::StepSpec {
            id: None,
            save_as: None,
            write_to: None,
            agent: Some("a1".to_string()),
            task: Some("t1".to_string()),
            prompt: None,
            inputs: std::collections::HashMap::new(),
            guards: vec![],
        };

        let p = super::resolve_provider_for_step(&step, &doc);
        assert_eq!(p.as_deref(), Some("local"));
    }

    #[test]
    fn resolve_provider_falls_back_to_single_provider() {
        let doc = minimal_doc();
        let step = adl::StepSpec {
            id: None,
            save_as: None,
            write_to: None,
            agent: None,
            task: Some("t1".to_string()),
            prompt: None,
            inputs: std::collections::HashMap::new(),
            guards: vec![],
        };

        let p = super::resolve_provider_for_step(&step, &doc);
        assert_eq!(p.as_deref(), Some("local"));
    }

    #[test]
    fn effective_prompt_priority_step_then_task_then_agent() {
        let mut doc = minimal_doc();

        // Step that references both task + agent but has no inline prompt => task wins.
        doc.run.workflow.steps.push(adl::StepSpec {
            id: None,
            save_as: None,
            write_to: None,
            agent: Some("a1".to_string()),
            task: Some("t1".to_string()),
            prompt: None,
            inputs: std::collections::HashMap::new(),
            guards: vec![],
        });
        let resolved = resolve_run(&doc).expect("resolve");
        let step0 = &resolved.steps[0];
        assert_eq!(
            step0
                .effective_prompt(&resolved)
                .and_then(|p| p.user.as_deref()),
            Some("task user")
        );

        // Now override with inline prompt => step wins.
        let mut doc2 = minimal_doc();
        doc2.run.workflow.steps.push(adl::StepSpec {
            id: None,
            save_as: None,
            write_to: None,
            agent: Some("a1".to_string()),
            task: Some("t1".to_string()),
            prompt: Some(adl::PromptSpec {
                system: None,
                developer: None,
                user: Some("step user".to_string()),
                context: None,
                output: None,
            }),
            inputs: std::collections::HashMap::new(),
            guards: vec![],
        });
        let resolved2 = resolve_run(&doc2).expect("resolve");
        let step1 = &resolved2.steps[0];
        assert_eq!(
            step1
                .effective_prompt(&resolved2)
                .and_then(|p| p.user.as_deref()),
            Some("step user")
        );

        // Task missing => agent prompt used.
        let mut doc3 = minimal_doc();
        doc3.run.workflow.steps.push(adl::StepSpec {
            id: None,
            save_as: None,
            write_to: None,
            agent: Some("a1".to_string()),
            task: Some("nope".to_string()),
            prompt: None,
            inputs: std::collections::HashMap::new(),
            guards: vec![],
        });
        let resolved3 = resolve_run(&doc3).expect("resolve");
        let step2 = &resolved3.steps[0];
        assert_eq!(
            step2
                .effective_prompt(&resolved3)
                .and_then(|p| p.user.as_deref()),
            Some("agent user")
        );
    }

    #[test]
    fn defaults_system_applies_when_prompt_missing_system() {
        let mut doc = minimal_doc();
        doc.run.defaults.system = Some("default sys".to_string());
        doc.run.workflow.steps.push(adl::StepSpec {
            id: None,
            save_as: None,
            write_to: None,
            agent: Some("a1".to_string()),
            task: Some("t1".to_string()),
            prompt: None,
            inputs: std::collections::HashMap::new(),
            guards: vec![],
        });

        let resolved = resolve_run(&doc).expect("resolve");
        let step = &resolved.steps[0];
        let p = step
            .effective_prompt_with_defaults(&resolved)
            .expect("prompt");
        assert_eq!(p.system.as_deref(), Some("default sys"));
    }

    #[test]
    fn defaults_system_does_not_override_existing_system() {
        let mut doc = minimal_doc();
        doc.run.defaults.system = Some("default sys".to_string());
        doc.run.workflow.steps.push(adl::StepSpec {
            id: None,
            save_as: None,
            write_to: None,
            agent: Some("a1".to_string()),
            task: Some("t1".to_string()),
            prompt: Some(adl::PromptSpec {
                system: Some("step sys".to_string()),
                developer: None,
                user: Some("step user".to_string()),
                context: None,
                output: None,
            }),
            inputs: std::collections::HashMap::new(),
            guards: vec![],
        });

        let resolved = resolve_run(&doc).expect("resolve");
        let step = &resolved.steps[0];
        let p = step
            .effective_prompt_with_defaults(&resolved)
            .expect("prompt");
        assert_eq!(p.system.as_deref(), Some("step sys"));
    }

    #[test]
    fn resolve_run_preserves_explicit_step_ids() {
        let mut doc = minimal_doc();
        doc.version = "0.2".to_string();
        doc.run.workflow.steps.push(adl::StepSpec {
            id: Some("step-1".to_string()),
            save_as: None,
            write_to: None,
            agent: Some("a1".to_string()),
            task: Some("t1".to_string()),
            prompt: None,
            inputs: std::collections::HashMap::new(),
            guards: vec![],
        });
        doc.run.workflow.steps.push(adl::StepSpec {
            id: Some("step-2".to_string()),
            save_as: None,
            write_to: None,
            agent: Some("a1".to_string()),
            task: Some("t1".to_string()),
            prompt: None,
            inputs: std::collections::HashMap::new(),
            guards: vec![],
        });

        let resolved = resolve_run(&doc).expect("resolve");
        assert_eq!(resolved.steps[0].id, "step-1");
        assert_eq!(resolved.steps[1].id, "step-2");
    }
}
