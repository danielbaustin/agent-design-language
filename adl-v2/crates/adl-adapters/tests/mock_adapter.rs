mod common;

use adl_adapters::{MockAdapter, MockError, ProviderStep, ToolStep};
use adl_engine::{CompletionOutcome, FailureClass, PortFailure, PortOutput};

fn success(text: &str) -> CompletionOutcome {
    CompletionOutcome::Success(PortOutput::new("text/plain", text.as_bytes().to_vec()))
}

#[test]
fn provider_script_returns_bound_identity() {
    let request = common::provider_request();
    let mut mock = MockAdapter::scripted(
        vec![ProviderStep::new(&request, success("ok")).unwrap()],
        vec![],
    );
    let completion = mock.provider(&request).unwrap();
    assert_eq!(completion.request_id, request.request_id);
    assert_eq!(completion.node_id, request.node_id);
}

#[test]
fn provider_script_preserves_failure() {
    let request = common::provider_request();
    let outcome = CompletionOutcome::Failure(PortFailure::new(FailureClass::Timeout, "late"));
    let mut mock = MockAdapter::scripted(
        vec![ProviderStep::new(&request, outcome.clone()).unwrap()],
        vec![],
    );
    assert_eq!(mock.provider(&request).unwrap().outcome, outcome);
}

#[test]
fn provider_script_rejects_wrong_request() {
    let expected = common::provider_request();
    let mut actual = expected.clone();
    actual.model = Some("other".into());
    let mut mock = MockAdapter::scripted(
        vec![ProviderStep::new(&expected, success("ok")).unwrap()],
        vec![],
    );
    assert_eq!(mock.provider(&actual), Err(MockError::UnexpectedRequest));
}

#[test]
fn provider_script_rejects_exhaustion() {
    assert_eq!(
        MockAdapter::default().provider(&common::provider_request()),
        Err(MockError::Exhausted)
    );
}

#[test]
fn tool_script_returns_bound_identity() {
    let request = common::tool_request();
    let mut mock = MockAdapter::scripted(
        vec![],
        vec![ToolStep::new(&request, success("ok")).unwrap()],
    );
    assert_eq!(mock.tool(&request).unwrap().request_id, request.request_id);
}

#[test]
fn tool_script_rejects_wrong_input() {
    let expected = common::tool_request();
    let mut actual = expected.clone();
    actual.tool = "write".into();
    let mut mock = MockAdapter::scripted(
        vec![],
        vec![ToolStep::new(&expected, success("ok")).unwrap()],
    );
    assert_eq!(mock.tool(&actual), Err(MockError::UnexpectedRequest));
}

#[test]
fn script_reports_exhausted_after_consumption() {
    let request = common::provider_request();
    let mut mock = MockAdapter::scripted(
        vec![ProviderStep::new(&request, success("ok")).unwrap()],
        vec![],
    );
    mock.provider(&request).unwrap();
    assert!(mock.is_exhausted());
}

#[test]
fn identical_scripts_are_deterministic() {
    let request = common::provider_request();
    let step = ProviderStep::new(&request, success("same")).unwrap();
    let mut left = MockAdapter::scripted(vec![step.clone()], vec![]);
    let mut right = MockAdapter::scripted(vec![step], vec![]);
    assert_eq!(left.provider(&request), right.provider(&request));
}
