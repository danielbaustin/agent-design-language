use anyhow::{anyhow, Result};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap, VecDeque};

use crate::adl;
use crate::resolve::ResolvedStep;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExecutionNode {
    pub step_id: String,
    pub depends_on: Vec<String>,
    pub save_as: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExecutionPlan {
    pub workflow_kind: adl::WorkflowKind,
    pub nodes: Vec<ExecutionNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledPatternStep {
    pub step_id: String,
    pub task_symbol: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledPattern {
    pub execution_plan: ExecutionPlan,
    pub compiled_steps: Vec<CompiledPatternStep>,
}

pub struct PatternRegistry<'a> {
    by_id: BTreeMap<&'a str, &'a adl::PatternSpec>,
}

impl<'a> PatternRegistry<'a> {
    /// Build a deterministic pattern registry keyed by pattern id.
    pub fn new(patterns: &'a [adl::PatternSpec]) -> Result<Self> {
        let mut by_id: BTreeMap<&'a str, &'a adl::PatternSpec> = BTreeMap::new();
        for pattern in patterns {
            if by_id.insert(pattern.id.as_str(), pattern).is_some() {
                return Err(anyhow!("duplicate pattern id '{}' in registry", pattern.id));
            }
        }
        Ok(Self { by_id })
    }

    pub fn get(&self, pattern_id: &str) -> Option<&'a adl::PatternSpec> {
        self.by_id.get(pattern_id).copied()
    }

    pub fn compile(&self, pattern_id: &str) -> Result<CompiledPattern> {
        let pattern = self.get(pattern_id).ok_or_else(|| {
            let available = self
                .by_id
                .keys()
                .map(|id| format!("'{}'", id))
                .collect::<Vec<_>>()
                .join(", ");
            anyhow!(
                "unknown pattern id '{}' in registry (available: [{}])",
                pattern_id,
                available
            )
        })?;
        compile_pattern(pattern)
    }

    /// Compile all registered patterns in deterministic id order.
    pub fn compile_all(&self) -> Result<Vec<(String, CompiledPattern)>> {
        let mut compiled = Vec::with_capacity(self.by_id.len());
        for (pattern_id, pattern) in &self.by_id {
            compiled.push(((*pattern_id).to_string(), compile_pattern(pattern)?));
        }
        Ok(compiled)
    }
}

fn parse_state_ref(value: &str) -> Option<&str> {
    let raw = value.strip_prefix("@state:")?;
    let key = raw.trim();
    if key.is_empty() {
        None
    } else {
        Some(key)
    }
}

pub fn build_execution_plan(
    workflow_kind: adl::WorkflowKind,
    steps: &[ResolvedStep],
) -> Result<ExecutionPlan> {
    let mut id_to_index: HashMap<&str, usize> = HashMap::new();
    for (idx, step) in steps.iter().enumerate() {
        if id_to_index.insert(step.id.as_str(), idx).is_some() {
            return Err(anyhow!("duplicate step id '{}' in workflow", step.id));
        }
    }

    let mut state_producer_by_key: HashMap<&str, &str> = HashMap::new();
    for step in steps {
        if let Some(key) = step.save_as.as_deref() {
            if key.trim().is_empty() {
                return Err(anyhow!("step '{}' has empty save_as key", step.id));
            }
            if state_producer_by_key
                .insert(key, step.id.as_str())
                .is_some()
            {
                return Err(anyhow!(
                    "duplicate save_as key '{}' (must be unique per workflow)",
                    key
                ));
            }
        }
    }

    let mut nodes = Vec::with_capacity(steps.len());
    for step in steps {
        let mut deps: Vec<String> = Vec::new();
        for value in step.inputs.values() {
            let Some(state_key) = parse_state_ref(value) else {
                continue;
            };
            // WP-03 call namespaces (e.g. @state:child.output) are produced
            // dynamically by call steps and cannot always be mapped statically.
            if state_key.contains('.') {
                continue;
            }
            let producer_step_id = state_producer_by_key.get(state_key).ok_or_else(|| {
                anyhow!(
                    "step '{}' references unknown saved state '{}' via @state:{}",
                    step.id,
                    state_key,
                    state_key
                )
            })?;
            if *producer_step_id == step.id.as_str() {
                return Err(anyhow!(
                    "step '{}' cannot depend on its own @state output '{}'",
                    step.id,
                    state_key
                ));
            }
            if !deps.iter().any(|d| d == *producer_step_id) {
                deps.push((*producer_step_id).to_string());
            }
        }
        deps.sort();

        nodes.push(ExecutionNode {
            step_id: step.id.clone(),
            depends_on: deps,
            save_as: step.save_as.clone(),
        });
    }

    if matches!(workflow_kind, adl::WorkflowKind::Concurrent) {
        apply_concurrent_fork_join_structure(&mut nodes);
    }

    validate_acyclic(&nodes)?;

    Ok(ExecutionPlan {
        workflow_kind,
        nodes,
    })
}

pub fn compile_pattern(pattern: &adl::PatternSpec) -> Result<CompiledPattern> {
    match pattern.kind {
        adl::PatternKind::Linear => compile_linear_pattern(pattern),
        adl::PatternKind::ForkJoin => compile_fork_join_pattern(pattern),
    }
}

fn compile_linear_pattern(pattern: &adl::PatternSpec) -> Result<CompiledPattern> {
    if pattern.steps.is_empty() {
        return Err(anyhow!(
            "pattern {} type=linear requires non-empty steps",
            pattern.id
        ));
    }

    let mut nodes = Vec::new();
    let mut compiled_steps = Vec::new();
    let mut prev: Option<String> = None;

    for sym in &pattern.steps {
        let step_id = format!("p::{}::{}", pattern.id, sym);
        let deps = prev.iter().cloned().collect::<Vec<_>>();
        nodes.push(ExecutionNode {
            step_id: step_id.clone(),
            depends_on: deps,
            save_as: Some(sym.clone()),
        });
        compiled_steps.push(CompiledPatternStep {
            step_id: step_id.clone(),
            task_symbol: sym.clone(),
        });
        prev = Some(step_id);
    }

    Ok(CompiledPattern {
        execution_plan: ExecutionPlan {
            workflow_kind: adl::WorkflowKind::Sequential,
            nodes,
        },
        compiled_steps,
    })
}

fn compile_fork_join_pattern(pattern: &adl::PatternSpec) -> Result<CompiledPattern> {
    let fork = pattern
        .fork
        .as_ref()
        .ok_or_else(|| anyhow!("pattern {} type=fork_join requires fork", pattern.id))?;
    let join = pattern
        .join
        .as_ref()
        .ok_or_else(|| anyhow!("pattern {} type=fork_join requires join", pattern.id))?;

    let mut nodes = Vec::new();
    let mut compiled_steps = Vec::new();
    let mut branch_last = Vec::new();

    let mut branches: Vec<&adl::PatternBranchSpec> = fork.branches.iter().collect();
    branches.sort_by(|a, b| a.id.cmp(&b.id));

    for br in branches {
        let mut prev: Option<String> = None;
        for sym in &br.steps {
            let step_id = format!("p::{}::{}::{}", pattern.id, br.id, sym);
            let deps = prev.iter().cloned().collect::<Vec<_>>();
            nodes.push(ExecutionNode {
                step_id: step_id.clone(),
                depends_on: deps,
                save_as: Some(format!("{}::{}", br.id, sym)),
            });
            compiled_steps.push(CompiledPatternStep {
                step_id: step_id.clone(),
                task_symbol: sym.clone(),
            });
            prev = Some(step_id);
        }
        if let Some(last) = prev {
            branch_last.push(last);
        }
    }

    if branch_last.is_empty() {
        return Err(anyhow!(
            "pattern {} type=fork_join requires at least one non-empty branch",
            pattern.id
        ));
    }

    branch_last.sort();
    let join_step_id = format!("p::{}::{}", pattern.id, join.step);
    nodes.push(ExecutionNode {
        step_id: join_step_id.clone(),
        depends_on: branch_last,
        save_as: Some(join.step.clone()),
    });
    compiled_steps.push(CompiledPatternStep {
        step_id: join_step_id,
        task_symbol: join.step.clone(),
    });

    Ok(CompiledPattern {
        execution_plan: ExecutionPlan {
            workflow_kind: adl::WorkflowKind::Concurrent,
            nodes,
        },
        compiled_steps,
    })
}

fn apply_concurrent_fork_join_structure(nodes: &mut [ExecutionNode]) {
    let plan_id = nodes
        .iter()
        .find(|n| n.step_id == "fork.plan")
        .map(|n| n.step_id.clone());
    let mut branch_ids: Vec<String> = nodes
        .iter()
        .filter(|n| n.step_id.starts_with("fork.branch."))
        .map(|n| n.step_id.clone())
        .collect();
    branch_ids.sort();

    for node in nodes.iter_mut() {
        if node.step_id.starts_with("fork.branch.") {
            if let Some(plan) = plan_id.as_ref() {
                if !node.depends_on.iter().any(|d| d == plan) {
                    node.depends_on.push(plan.clone());
                }
            }
            node.depends_on.sort();
        }

        if node.step_id == "fork.join" {
            for branch in &branch_ids {
                if !node.depends_on.iter().any(|d| d == branch) {
                    node.depends_on.push(branch.clone());
                }
            }
            node.depends_on.sort();
        }
    }
}

fn validate_acyclic(nodes: &[ExecutionNode]) -> Result<()> {
    let mut indegree: HashMap<&str, usize> = HashMap::new();
    let mut outgoing: HashMap<&str, Vec<&str>> = HashMap::new();

    for node in nodes {
        indegree.entry(node.step_id.as_str()).or_insert(0);
        outgoing.entry(node.step_id.as_str()).or_default();
    }

    for node in nodes {
        for dep in &node.depends_on {
            if !indegree.contains_key(dep.as_str()) {
                return Err(anyhow!(
                    "step '{}' depends on unknown step '{}'",
                    node.step_id,
                    dep
                ));
            }
            *indegree.entry(node.step_id.as_str()).or_insert(0) += 1;
            outgoing
                .entry(dep.as_str())
                .or_default()
                .push(node.step_id.as_str());
        }
    }

    let mut queue: VecDeque<&str> = indegree
        .iter()
        .filter_map(|(node, &deg)| if deg == 0 { Some(*node) } else { None })
        .collect();

    let mut seen = 0usize;
    while let Some(node) = queue.pop_front() {
        seen += 1;
        for next in outgoing.get(node).into_iter().flatten() {
            let deg = indegree
                .get_mut(next)
                .ok_or_else(|| anyhow!("internal DAG validation error for node '{}'", next))?;
            *deg -= 1;
            if *deg == 0 {
                queue.push_back(next);
            }
        }
    }

    if seen != nodes.len() {
        return Err(anyhow!(
            "workflow contains a dependency cycle ({} of {} nodes resolved)",
            seen,
            nodes.len()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolve::ResolvedStep;

    fn mk_step(id: &str, save_as: Option<&str>, input_refs: &[(&str, &str)]) -> ResolvedStep {
        let mut inputs = HashMap::new();
        for (k, v) in input_refs {
            inputs.insert((*k).to_string(), (*v).to_string());
        }
        ResolvedStep {
            id: id.to_string(),
            agent: None,
            provider: None,
            placement: None,
            task: None,
            call: None,
            with: HashMap::new(),
            as_ns: None,
            prompt: None,
            inputs,
            save_as: save_as.map(str::to_string),
            write_to: None,
            on_error: None,
            retry: None,
        }
    }

    #[test]
    fn build_plan_rejects_duplicate_step_ids() {
        let steps = vec![
            mk_step("fork.alpha", Some("alpha"), &[]),
            mk_step("fork.alpha", Some("alpha2"), &[]),
        ];
        let err = build_execution_plan(adl::WorkflowKind::Concurrent, &steps).unwrap_err();
        assert!(err.to_string().contains("duplicate step id"), "{err:#}");
    }

    #[test]
    fn build_plan_rejects_missing_state_producer() {
        let steps = vec![mk_step(
            "join",
            None,
            &[("alpha", "@state:alpha"), ("beta", "@state:beta")],
        )];
        let err = build_execution_plan(adl::WorkflowKind::Concurrent, &steps).unwrap_err();
        assert!(
            err.to_string().contains("references unknown saved state"),
            "{err:#}"
        );
    }

    #[test]
    fn build_plan_rejects_dependency_cycles() {
        let steps = vec![
            mk_step("a", Some("a_out"), &[("b", "@state:b_out")]),
            mk_step("b", Some("b_out"), &[("a", "@state:a_out")]),
        ];
        let err = build_execution_plan(adl::WorkflowKind::Concurrent, &steps).unwrap_err();
        assert!(err.to_string().contains("dependency cycle"), "{err:#}");
    }

    #[test]
    fn build_plan_allows_valid_linear_state_dependencies() {
        let steps = vec![
            mk_step("plan", Some("plan"), &[]),
            mk_step("branch.alpha", Some("alpha"), &[("plan", "@state:plan")]),
            mk_step(
                "join",
                Some("joined"),
                &[("alpha", "@state:alpha"), ("plan", "@state:plan")],
            ),
        ];
        let plan = build_execution_plan(adl::WorkflowKind::Concurrent, &steps).expect("valid");
        assert_eq!(plan.nodes.len(), 3);
        assert_eq!(plan.nodes[1].depends_on, vec!["plan".to_string()]);
        assert_eq!(
            plan.nodes[2].depends_on,
            vec!["branch.alpha".to_string(), "plan".to_string()]
        );
    }

    #[test]
    fn build_plan_adds_structural_fork_join_dependencies() {
        let steps = vec![
            mk_step("fork.branch.beta", Some("beta"), &[]),
            mk_step("fork.join", Some("joined"), &[("alpha", "@state:alpha")]),
            mk_step("fork.plan", None, &[]),
            mk_step("fork.branch.alpha", Some("alpha"), &[]),
        ];

        let plan = build_execution_plan(adl::WorkflowKind::Concurrent, &steps).expect("valid");
        assert_eq!(plan.nodes.len(), 4);

        let by_id: HashMap<&str, &ExecutionNode> =
            plan.nodes.iter().map(|n| (n.step_id.as_str(), n)).collect();

        assert_eq!(
            by_id["fork.branch.alpha"].depends_on,
            vec!["fork.plan".to_string()]
        );
        assert_eq!(
            by_id["fork.branch.beta"].depends_on,
            vec!["fork.plan".to_string()]
        );
        assert_eq!(
            by_id["fork.join"].depends_on,
            vec![
                "fork.branch.alpha".to_string(),
                "fork.branch.beta".to_string()
            ]
        );
    }

    #[test]
    fn compile_linear_pattern_is_byte_stable_and_uses_canonical_ids() {
        let pattern = adl::PatternSpec {
            id: "p_linear".to_string(),
            kind: adl::PatternKind::Linear,
            steps: vec!["A".to_string(), "B".to_string(), "C".to_string()],
            fork: None,
            join: None,
        };

        let c1 = compile_pattern(&pattern).expect("compile");
        let c2 = compile_pattern(&pattern).expect("compile");

        let b1 = serde_json::to_vec(&c1.execution_plan).expect("serialize");
        let b2 = serde_json::to_vec(&c2.execution_plan).expect("serialize");
        assert_eq!(b1, b2, "compiled plan bytes must be stable");

        assert_eq!(c1.execution_plan.nodes.len(), 3);
        assert_eq!(c1.execution_plan.nodes[0].step_id, "p::p_linear::A");
        assert_eq!(
            c1.execution_plan.nodes[1].depends_on,
            vec!["p::p_linear::A"]
        );
        assert_eq!(
            c1.execution_plan.nodes[2].depends_on,
            vec!["p::p_linear::B"]
        );
    }

    #[test]
    fn compile_fork_join_pattern_sets_join_dependencies_to_branch_tails() {
        let pattern = adl::PatternSpec {
            id: "p_fork".to_string(),
            kind: adl::PatternKind::ForkJoin,
            steps: vec![],
            fork: Some(adl::PatternForkSpec {
                branches: vec![
                    adl::PatternBranchSpec {
                        id: "left".to_string(),
                        steps: vec!["L1".to_string(), "L2".to_string()],
                    },
                    adl::PatternBranchSpec {
                        id: "right".to_string(),
                        steps: vec!["R1".to_string()],
                    },
                ],
            }),
            join: Some(adl::PatternJoinSpec {
                step: "J".to_string(),
            }),
        };

        let compiled = compile_pattern(&pattern).expect("compile");
        let by_id: HashMap<&str, &ExecutionNode> = compiled
            .execution_plan
            .nodes
            .iter()
            .map(|n| (n.step_id.as_str(), n))
            .collect();

        assert_eq!(
            by_id["p::p_fork::J"].depends_on,
            vec!["p::p_fork::left::L2", "p::p_fork::right::R1"]
        );
        assert_eq!(
            by_id["p::p_fork::left::L2"].depends_on,
            vec!["p::p_fork::left::L1"]
        );
    }

    #[test]
    fn compile_fork_join_pattern_is_byte_stable() {
        let pattern = adl::PatternSpec {
            id: "p_fork".to_string(),
            kind: adl::PatternKind::ForkJoin,
            steps: vec![],
            fork: Some(adl::PatternForkSpec {
                branches: vec![
                    adl::PatternBranchSpec {
                        id: "a".to_string(),
                        steps: vec!["A1".to_string()],
                    },
                    adl::PatternBranchSpec {
                        id: "b".to_string(),
                        steps: vec!["B1".to_string()],
                    },
                ],
            }),
            join: Some(adl::PatternJoinSpec {
                step: "J".to_string(),
            }),
        };

        let c1 = compile_pattern(&pattern).expect("compile");
        let c2 = compile_pattern(&pattern).expect("compile");
        assert_eq!(
            serde_json::to_vec(&c1.execution_plan).expect("serialize"),
            serde_json::to_vec(&c2.execution_plan).expect("serialize"),
            "compiled fork/join plan bytes must be stable"
        );
    }

    #[test]
    fn compile_fork_join_pattern_normalizes_branch_order_by_branch_id() {
        let pattern_declared = adl::PatternSpec {
            id: "p_fork".to_string(),
            kind: adl::PatternKind::ForkJoin,
            steps: vec![],
            fork: Some(adl::PatternForkSpec {
                branches: vec![
                    adl::PatternBranchSpec {
                        id: "right".to_string(),
                        steps: vec!["R1".to_string()],
                    },
                    adl::PatternBranchSpec {
                        id: "left".to_string(),
                        steps: vec!["L1".to_string()],
                    },
                ],
            }),
            join: Some(adl::PatternJoinSpec {
                step: "J".to_string(),
            }),
        };

        let pattern_sorted = adl::PatternSpec {
            id: "p_fork".to_string(),
            kind: adl::PatternKind::ForkJoin,
            steps: vec![],
            fork: Some(adl::PatternForkSpec {
                branches: vec![
                    adl::PatternBranchSpec {
                        id: "left".to_string(),
                        steps: vec!["L1".to_string()],
                    },
                    adl::PatternBranchSpec {
                        id: "right".to_string(),
                        steps: vec!["R1".to_string()],
                    },
                ],
            }),
            join: Some(adl::PatternJoinSpec {
                step: "J".to_string(),
            }),
        };

        let a = compile_pattern(&pattern_declared).expect("compile");
        let b = compile_pattern(&pattern_sorted).expect("compile");

        assert_eq!(
            serde_json::to_vec(&a.execution_plan).expect("serialize"),
            serde_json::to_vec(&b.execution_plan).expect("serialize"),
            "branch declaration order should not affect compiled plan"
        );
    }
    #[test]
    fn pattern_registry_compile_rejects_unknown_pattern_id() {
        let patterns = vec![
            adl::PatternSpec {
                id: "zeta".to_string(),
                kind: adl::PatternKind::Linear,
                steps: vec!["Z".to_string()],
                fork: None,
                join: None,
            },
            adl::PatternSpec {
                id: "alpha".to_string(),
                kind: adl::PatternKind::Linear,
                steps: vec!["A".to_string()],
                fork: None,
                join: None,
            },
        ];

        let registry = PatternRegistry::new(&patterns).expect("registry");
        let err = registry
            .compile("missing")
            .expect_err("unknown pattern id should fail deterministically");
        let msg = err.to_string();
        assert!(
            msg.contains("unknown pattern id 'missing' in registry"),
            "{err:#}"
        );
        assert!(
            msg.contains("available: ['alpha', 'zeta']"),
            "available ids should be canonical/sorted, got: {msg}"
        );
    }

    #[test]
    fn pattern_registry_new_rejects_duplicate_pattern_ids_deterministically() {
        let patterns = vec![
            adl::PatternSpec {
                id: "dup".to_string(),
                kind: adl::PatternKind::Linear,
                steps: vec!["A".to_string()],
                fork: None,
                join: None,
            },
            adl::PatternSpec {
                id: "dup".to_string(),
                kind: adl::PatternKind::Linear,
                steps: vec!["B".to_string()],
                fork: None,
                join: None,
            },
        ];

        let err = match PatternRegistry::new(&patterns) {
            Ok(_) => panic!("duplicate ids must fail deterministically"),
            Err(err) => err,
        };
        assert!(
            err.to_string()
                .contains("duplicate pattern id 'dup' in registry"),
            "{err:#}"
        );
    }

    #[test]
    fn pattern_registry_compile_all_is_deterministic_across_declaration_order() {
        let p_a = adl::PatternSpec {
            id: "p_a".to_string(),
            kind: adl::PatternKind::Linear,
            steps: vec!["A1".to_string(), "A2".to_string()],
            fork: None,
            join: None,
        };
        let p_b = adl::PatternSpec {
            id: "p_b".to_string(),
            kind: adl::PatternKind::ForkJoin,
            steps: vec![],
            fork: Some(adl::PatternForkSpec {
                branches: vec![
                    adl::PatternBranchSpec {
                        id: "right".to_string(),
                        steps: vec!["R1".to_string()],
                    },
                    adl::PatternBranchSpec {
                        id: "left".to_string(),
                        steps: vec!["L1".to_string()],
                    },
                ],
            }),
            join: Some(adl::PatternJoinSpec {
                step: "J".to_string(),
            }),
        };

        let declared_patterns = vec![p_b.clone(), p_a.clone()];
        let sorted_patterns = vec![p_a.clone(), p_b.clone()];

        let r_declared = PatternRegistry::new(&declared_patterns).expect("registry");
        let r_sorted = PatternRegistry::new(&sorted_patterns).expect("registry");

        let c1 = r_declared.compile_all().expect("compile all declared");
        let c2 = r_sorted.compile_all().expect("compile all sorted");

        let ids1: Vec<String> = c1.iter().map(|(id, _)| id.clone()).collect();
        let ids2: Vec<String> = c2.iter().map(|(id, _)| id.clone()).collect();
        assert_eq!(ids1, vec!["p_a".to_string(), "p_b".to_string()]);
        assert_eq!(ids1, ids2);

        let bytes1 = serde_json::to_vec(
            &c1.iter()
                .map(|(id, c)| (id, &c.execution_plan))
                .collect::<Vec<_>>(),
        )
        .expect("serialize");
        let bytes2 = serde_json::to_vec(
            &c2.iter()
                .map(|(id, c)| (id, &c.execution_plan))
                .collect::<Vec<_>>(),
        )
        .expect("serialize");
        assert_eq!(
            bytes1, bytes2,
            "compile_all order/bytes must be deterministic"
        );
    }
}
