mod common;

use adl_adapters::{
    request_digest, AuthorizationEnvelope, AuthorizationVerifier, GovernedToolAdapter,
    GovernedToolError, ToolPort,
};
use adl_engine::{CompletionOutcome, PortOutput, ToolRequest};

#[derive(Default)]
struct Port {
    calls: usize,
}
struct Verifier(bool);
impl ToolPort for Port {
    fn invoke(&mut self, _: &ToolRequest) -> Result<CompletionOutcome, GovernedToolError> {
        self.calls += 1;
        Ok(CompletionOutcome::Success(PortOutput::new(
            "text/plain",
            b"ok".to_vec(),
        )))
    }
}
impl AuthorizationVerifier for Verifier {
    fn verify(&mut self, _: &AuthorizationEnvelope) -> Result<(), GovernedToolError> {
        self.0
            .then_some(())
            .ok_or(GovernedToolError::VerificationFailed)
    }
}

fn authorization(r: &ToolRequest) -> AuthorizationEnvelope {
    AuthorizationEnvelope {
        subject: "agent".into(),
        action: "invoke".into(),
        resource: "tool:read".into(),
        scope: "repo:read".into(),
        expires_at_tick: 20,
        request_digest: request_digest(r).unwrap(),
        evidence_ref: "record:1".into(),
    }
}
#[allow(clippy::too_many_arguments)]
fn invoke(
    r: &ToolRequest,
    a: &AuthorizationEnvelope,
    verifier: bool,
    subject: &str,
    action: &str,
    resource: &str,
    scope: &str,
    now: u64,
) -> Result<adl_engine::ToolCompletion, GovernedToolError> {
    GovernedToolAdapter::new(Port::default(), Verifier(verifier))
        .invoke(r, a, subject, action, resource, scope, now)
}

#[test]
fn matching_authorization_invokes_once() {
    let r = common::tool_request();
    let mut a = GovernedToolAdapter::new(Port::default(), Verifier(true));
    a.invoke(
        &r,
        &authorization(&r),
        "agent",
        "invoke",
        "tool:read",
        "repo:read",
        10,
    )
    .unwrap();
    assert_eq!(a.into_inner().0.calls, 1);
}
#[test]
fn verifier_denial_prevents_invocation() {
    let r = common::tool_request();
    assert_eq!(
        invoke(
            &r,
            &authorization(&r),
            false,
            "agent",
            "invoke",
            "tool:read",
            "repo:read",
            10
        ),
        Err(GovernedToolError::VerificationFailed)
    );
}
#[test]
fn expired_authorization_is_denied() {
    let r = common::tool_request();
    assert_eq!(
        invoke(
            &r,
            &authorization(&r),
            true,
            "agent",
            "invoke",
            "tool:read",
            "repo:read",
            20
        ),
        Err(GovernedToolError::Expired)
    );
}
#[test]
fn subject_mismatch_is_denied() {
    let r = common::tool_request();
    assert_eq!(
        invoke(
            &r,
            &authorization(&r),
            true,
            "other",
            "invoke",
            "tool:read",
            "repo:read",
            10
        ),
        Err(GovernedToolError::SubjectMismatch)
    );
}
#[test]
fn action_mismatch_is_denied() {
    let r = common::tool_request();
    assert_eq!(
        invoke(
            &r,
            &authorization(&r),
            true,
            "agent",
            "delete",
            "tool:read",
            "repo:read",
            10
        ),
        Err(GovernedToolError::ActionMismatch)
    );
}
#[test]
fn resource_mismatch_is_denied() {
    let r = common::tool_request();
    assert_eq!(
        invoke(
            &r,
            &authorization(&r),
            true,
            "agent",
            "invoke",
            "tool:write",
            "repo:read",
            10
        ),
        Err(GovernedToolError::ResourceMismatch)
    );
}
#[test]
fn scope_mismatch_is_denied() {
    let r = common::tool_request();
    assert_eq!(
        invoke(
            &r,
            &authorization(&r),
            true,
            "agent",
            "invoke",
            "tool:read",
            "repo:write",
            10
        ),
        Err(GovernedToolError::ScopeMismatch)
    );
}
#[test]
fn request_mismatch_is_denied() {
    let r = common::tool_request();
    let mut c = r.clone();
    c.tool = "write".into();
    assert_eq!(
        invoke(
            &c,
            &authorization(&r),
            true,
            "agent",
            "invoke",
            "tool:read",
            "repo:read",
            10
        ),
        Err(GovernedToolError::RequestMismatch)
    );
}
#[test]
fn digest_is_stable() {
    let r = common::tool_request();
    assert_eq!(request_digest(&r), request_digest(&r));
}
#[test]
fn digest_changes_with_request() {
    let r = common::tool_request();
    let mut c = r.clone();
    c.sequence += 1;
    assert_ne!(request_digest(&r), request_digest(&c));
}
#[test]
fn completion_preserves_attempt() {
    let r = common::tool_request();
    assert_eq!(
        invoke(
            &r,
            &authorization(&r),
            true,
            "agent",
            "invoke",
            "tool:read",
            "repo:read",
            10
        )
        .unwrap()
        .attempt,
        r.attempt
    );
}
