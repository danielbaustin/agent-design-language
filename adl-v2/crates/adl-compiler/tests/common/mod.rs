use adl_language::{
    AdlDocument, Agent, Prompt, Provider, Run, Task, Tool, Workflow, WorkflowKind, WorkflowStep,
};
use serde_json::json;
use std::collections::BTreeMap;

pub fn document(kind: WorkflowKind) -> AdlDocument {
    AdlDocument {
        version: "0.5".into(),
        providers: BTreeMap::from([(
            "local".into(),
            Provider {
                id: None,
                kind: Some("test".into()),
                profile: None,
                default_model: Some("model-a".into()),
                config: BTreeMap::new(),
            },
        )]),
        tools: BTreeMap::from([
            (
                "alpha".into(),
                Tool {
                    id: None,
                    kind: "test".into(),
                    config: BTreeMap::new(),
                },
            ),
            (
                "zeta".into(),
                Tool {
                    id: None,
                    kind: "test".into(),
                    config: BTreeMap::new(),
                },
            ),
        ]),
        agents: BTreeMap::from([(
            "worker".into(),
            Agent {
                id: None,
                provider: "local".into(),
                model: None,
                tools: vec!["zeta".into(), "alpha".into()],
            },
        )]),
        tasks: BTreeMap::from([
            (
                "produce".into(),
                Task {
                    id: None,
                    agent_ref: Some("worker".into()),
                    inputs: vec![],
                    tool_allowlist: vec![],
                    prompt: Prompt {
                        system: None,
                        user: "produce".into(),
                    },
                },
            ),
            (
                "consume".into(),
                Task {
                    id: None,
                    agent_ref: Some("worker".into()),
                    inputs: vec!["source".into()],
                    tool_allowlist: vec!["alpha".into()],
                    prompt: Prompt {
                        system: None,
                        user: "consume".into(),
                    },
                },
            ),
        ]),
        workflows: BTreeMap::from([(
            "flow".into(),
            Workflow {
                id: None,
                kind,
                steps: vec![
                    WorkflowStep {
                        id: "first".into(),
                        agent: None,
                        task: "produce".into(),
                        inputs: BTreeMap::from([("value".into(), json!({"b": 2, "a": 1}))]),
                        save_as: Some("result".into()),
                    },
                    WorkflowStep {
                        id: "second".into(),
                        agent: None,
                        task: "consume".into(),
                        inputs: BTreeMap::from([("source".into(), json!("@state:result"))]),
                        save_as: None,
                    },
                ],
            },
        )]),
        run: Run {
            id: Some("run-1".into()),
            name: "example".into(),
            workflow_ref: Some("flow".into()),
            workflow: None,
            inputs: BTreeMap::from([("request".into(), json!("hello"))]),
            placement: None,
        },
    }
}
