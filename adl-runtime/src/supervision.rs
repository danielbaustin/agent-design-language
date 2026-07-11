//! CSM component supervision contracts.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentRestartPolicy {
    RestartWithBackoff,
    DegradeAndContinue,
    EscalateToGovernedShutdown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ComponentSupervisionPolicy {
    pub component: &'static str,
    pub restart_policy: ComponentRestartPolicy,
    pub critical_for_continuity: bool,
    pub telemetry_can_degrade: bool,
}

pub const SUPERVISION_SCHEMA: &str = "adl.csm.supervision_policy.v1";

pub fn default_component_supervision() -> Vec<ComponentSupervisionPolicy> {
    vec![
        ComponentSupervisionPolicy {
            component: "runtime_api",
            restart_policy: ComponentRestartPolicy::RestartWithBackoff,
            critical_for_continuity: false,
            telemetry_can_degrade: false,
        },
        ComponentSupervisionPolicy {
            component: "chronosense",
            restart_policy: ComponentRestartPolicy::RestartWithBackoff,
            critical_for_continuity: true,
            telemetry_can_degrade: false,
        },
        ComponentSupervisionPolicy {
            component: "scheduler",
            restart_policy: ComponentRestartPolicy::RestartWithBackoff,
            critical_for_continuity: true,
            telemetry_can_degrade: false,
        },
        ComponentSupervisionPolicy {
            component: "freedom_gate",
            restart_policy: ComponentRestartPolicy::RestartWithBackoff,
            critical_for_continuity: true,
            telemetry_can_degrade: false,
        },
        ComponentSupervisionPolicy {
            component: "checkpoint",
            restart_policy: ComponentRestartPolicy::EscalateToGovernedShutdown,
            critical_for_continuity: true,
            telemetry_can_degrade: false,
        },
        ComponentSupervisionPolicy {
            component: "observability",
            restart_policy: ComponentRestartPolicy::DegradeAndContinue,
            critical_for_continuity: false,
            telemetry_can_degrade: true,
        },
        ComponentSupervisionPolicy {
            component: "cloud_bridge",
            restart_policy: ComponentRestartPolicy::DegradeAndContinue,
            critical_for_continuity: false,
            telemetry_can_degrade: true,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkpoint_policy_is_continuity_critical() {
        let checkpoint = default_component_supervision()
            .into_iter()
            .find(|policy| policy.component == "checkpoint")
            .expect("checkpoint policy");
        assert!(checkpoint.critical_for_continuity);
        assert_eq!(
            checkpoint.restart_policy,
            ComponentRestartPolicy::EscalateToGovernedShutdown
        );
    }

    #[test]
    fn freedom_gate_policy_is_runtime_critical_and_fail_closed() {
        let freedom_gate = default_component_supervision()
            .into_iter()
            .find(|policy| policy.component == "freedom_gate")
            .expect("freedom gate policy");
        assert!(freedom_gate.critical_for_continuity);
        assert!(!freedom_gate.telemetry_can_degrade);
        assert_eq!(
            freedom_gate.restart_policy,
            ComponentRestartPolicy::RestartWithBackoff
        );
    }
}
