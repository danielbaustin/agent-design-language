use anyhow::{bail, Context, Result};
use serde::Deserialize;

const EMBEDDED_V08_WORKFLOW_TEMPLATE: &str =
    include_str!("../../../adl-spec/examples/v0.8/godel_experiment_workflow.template.v1.json");

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GodelWorkflowTemplate {
    pub template_name: String,
    pub template_version: u32,
    pub stage_order: Vec<String>,
    pub stages: Vec<GodelWorkflowStage>,
    pub determinism: TemplateDeterminism,
    pub security_privacy: TemplateSecurityPrivacy,
    pub replay_audit: TemplateReplayAudit,
    pub downstream: TemplateDownstream,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GodelWorkflowStage {
    pub stage_id: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub artifact_contracts: Vec<ArtifactContract>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactContract {
    pub schema_name: String,
    pub schema_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemplateDeterminism {
    pub stage_order_fixed: bool,
    pub input_order_required: bool,
    pub hidden_state_allowed: bool,
    pub tie_break_policy: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemplateSecurityPrivacy {
    pub allow_secrets: bool,
    pub allow_raw_prompts: bool,
    pub allow_tool_args: bool,
    pub allow_absolute_host_paths: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemplateReplayAudit {
    pub replay_compatible: bool,
    pub artifact_references_required: bool,
    pub traceability_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemplateDownstream {
    pub obsmem_indexing_ready: bool,
    pub demo_path_ready: bool,
    pub related_issues: Vec<u64>,
}

pub fn embedded_v08_workflow_template() -> Result<GodelWorkflowTemplate> {
    parse_workflow_template(EMBEDDED_V08_WORKFLOW_TEMPLATE)
        .context("parse embedded v0.8 Gödel workflow template")
}

pub fn parse_workflow_template(raw: &str) -> Result<GodelWorkflowTemplate> {
    let template: GodelWorkflowTemplate =
        serde_json::from_str(raw).context("deserialize Gödel workflow template JSON")?;
    validate_template_contract(&template)?;
    Ok(template)
}

fn validate_template_contract(template: &GodelWorkflowTemplate) -> Result<()> {
    if template.template_name != "godel_experiment_workflow" {
        bail!("workflow template has unexpected template_name");
    }
    if template.template_version != 1 {
        bail!("workflow template has unexpected template_version");
    }
    if !template.determinism.stage_order_fixed {
        bail!("workflow template must declare stage_order_fixed=true");
    }
    if !template.determinism.input_order_required {
        bail!("workflow template must declare input_order_required=true");
    }
    if template.determinism.hidden_state_allowed {
        bail!("workflow template must declare hidden_state_allowed=false");
    }
    if template.determinism.tie_break_policy != "lexicographic_ids" {
        bail!("workflow template must declare lexicographic_ids tie-break policy");
    }
    if template.security_privacy.allow_secrets
        || template.security_privacy.allow_raw_prompts
        || template.security_privacy.allow_tool_args
        || template.security_privacy.allow_absolute_host_paths
    {
        bail!("workflow template must keep security/privacy allowances disabled");
    }
    if !template.replay_audit.replay_compatible
        || !template.replay_audit.artifact_references_required
        || !template.replay_audit.traceability_required
    {
        bail!("workflow template must require replay-compatible traceable artifacts");
    }
    if !template.downstream.obsmem_indexing_ready || !template.downstream.demo_path_ready {
        bail!("workflow template must remain downstream-ready for indexing and demos");
    }
    if template.stage_order.len() != template.stages.len() {
        bail!("workflow template stage_order must align with stage definitions");
    }
    for (expected_stage, stage) in template.stage_order.iter().zip(&template.stages) {
        if expected_stage != &stage.stage_id {
            bail!("workflow template stages must match stage_order exactly");
        }
    }

    let Some(record_stage) = template
        .stages
        .iter()
        .find(|stage| stage.stage_id == "record")
    else {
        bail!("workflow template must define a record stage");
    };
    if !record_stage
        .outputs
        .iter()
        .any(|output| output == "experiment_record_ref")
    {
        bail!("record stage must output experiment_record_ref");
    }
    if !record_stage
        .artifact_contracts
        .iter()
        .any(|contract| contract.schema_name == "experiment_record" && contract.schema_version == 1)
    {
        bail!("record stage must declare experiment_record schema contract");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_template_is_valid_and_record_stage_is_contractual() {
        let template = embedded_v08_workflow_template().expect("embedded template");
        assert_eq!(
            template.stage_order,
            vec![
                "failure".to_string(),
                "hypothesis".to_string(),
                "mutation".to_string(),
                "experiment".to_string(),
                "evaluation".to_string(),
                "record".to_string(),
            ]
        );
        let record_stage = template
            .stages
            .iter()
            .find(|stage| stage.stage_id == "record")
            .expect("record stage");
        assert!(record_stage
            .outputs
            .iter()
            .any(|output| output == "experiment_record_ref"));
    }

    #[test]
    fn parse_rejects_record_stage_without_experiment_record_contract() {
        let err = parse_workflow_template(
            r#"{
              "template_name": "godel_experiment_workflow",
              "template_version": 1,
              "stage_order": ["record"],
              "stages": [{
                "stage_id": "record",
                "inputs": [],
                "outputs": ["experiment_record_ref"],
                "artifact_contracts": []
              }],
              "determinism": {
                "stage_order_fixed": true,
                "input_order_required": true,
                "hidden_state_allowed": false,
                "tie_break_policy": "lexicographic_ids"
              },
              "security_privacy": {
                "allow_secrets": false,
                "allow_raw_prompts": false,
                "allow_tool_args": false,
                "allow_absolute_host_paths": false
              },
              "replay_audit": {
                "replay_compatible": true,
                "artifact_references_required": true,
                "traceability_required": true
              },
              "downstream": {
                "obsmem_indexing_ready": true,
                "demo_path_ready": true,
                "related_issues": [609]
              }
            }"#,
        )
        .expect_err("record stage without schema contract must fail");
        assert!(err
            .to_string()
            .contains("record stage must declare experiment_record schema contract"));
    }
}
