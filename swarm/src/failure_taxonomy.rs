use crate::{bounded_executor, execute, instrumentation, provider, remote_exec, sandbox, signing};

/// Canonical stable failure codes for v0.75 deterministic substrate.
pub const RUNTIME_FAILURE: &str = "runtime_failure";
pub const TOOL_FAILURE: &str = "tool_failure";
pub const POLICY_DENIED: &str = "policy_denied";
pub const VERIFICATION_FAILED: &str = "verification_failed";
pub const REPLAY_INVARIANT_VIOLATION: &str = "replay_invariant_violation";
pub const USER_ABORT: &str = "user_abort";
pub const EXTERNAL_ABORT: &str = "external_abort";

/// Stable classification entry point shared by runtime and replay surfaces.
pub fn classify(err: &anyhow::Error) -> Option<&'static str> {
    execute::stable_failure_kind(err)
        .or_else(|| provider::stable_failure_kind(err))
        .or_else(|| remote_exec::stable_failure_kind(err))
        .or_else(|| bounded_executor::stable_failure_kind(err))
        .or_else(|| signing::stable_failure_kind(err))
        .or_else(|| instrumentation::stable_failure_kind(err))
        .or_else(|| {
            err.chain().find_map(|cause| {
                if cause.downcast_ref::<sandbox::SandboxPathError>().is_some() {
                    Some("sandbox_denied")
                } else if cause.downcast_ref::<std::io::Error>().is_some() {
                    Some("io_error")
                } else {
                    None
                }
            })
        })
}

/// Canonical taxonomy category for stable failure codes.
pub fn category_for_code(code: &str) -> &'static str {
    match code {
        "policy_denied" => POLICY_DENIED,
        "verification_failed" => VERIFICATION_FAILED,
        "replay_invariant_violation" => REPLAY_INVARIANT_VIOLATION,
        "provider_error" | "timeout" => TOOL_FAILURE,
        "panic" | "schema_error" | "sandbox_denied" | "io_error" => RUNTIME_FAILURE,
        "user_abort" => USER_ABORT,
        "external_abort" => EXTERNAL_ABORT,
        _ => RUNTIME_FAILURE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category_mapping_is_stable_for_core_codes() {
        assert_eq!(category_for_code("policy_denied"), POLICY_DENIED);
        assert_eq!(
            category_for_code("verification_failed"),
            VERIFICATION_FAILED
        );
        assert_eq!(
            category_for_code("replay_invariant_violation"),
            REPLAY_INVARIANT_VIOLATION
        );
        assert_eq!(category_for_code("provider_error"), TOOL_FAILURE);
        assert_eq!(category_for_code("timeout"), TOOL_FAILURE);
        assert_eq!(category_for_code("schema_error"), RUNTIME_FAILURE);
        assert_eq!(category_for_code("unknown_code"), RUNTIME_FAILURE);
    }
}
