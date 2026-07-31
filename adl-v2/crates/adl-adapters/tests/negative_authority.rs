mod common;

use adl_adapters::{
    CompatibilityAdapter, CompatibilityError, CompatibilityInput, EndpointAuthorizer,
    EndpointPermit,
};
use serde_json::json;
use url::Url;

#[test]
fn url_alone_does_not_create_http_authority() {
    struct Deny;
    impl EndpointAuthorizer for Deny {
        fn authorize(&mut self, _: &Url) -> bool {
            false
        }
    }
    assert!(EndpointPermit::admit("https://127.0.0.1:1", &mut Deny).is_err());
}

#[test]
fn provider_aliases_are_not_inferred() {
    let adapter = CompatibilityAdapter::new(vec!["v1".into()], vec!["openai".into()]);
    let result = adapter.translate(CompatibilityInput {
        version: "v1".into(),
        provider: "openai-compatible".into(),
        model: None,
        payload: json!({}),
    });
    assert_eq!(result, Err(CompatibilityError::UnknownProvider));
}

#[test]
fn mock_does_not_accept_unscripted_provider_work() {
    assert!(adl_adapters::MockAdapter::default()
        .provider(&common::provider_request())
        .is_err());
}

#[test]
fn production_sources_contain_no_shell_or_aws_authority() {
    let sources = [
        include_str!("../src/compatibility.rs"),
        include_str!("../src/governed_tool.rs"),
        include_str!("../src/https.rs"),
        include_str!("../src/mock.rs"),
    ]
    .join("\n");
    for forbidden in ["std::process", "Command::new", "aws_sdk", "Runtime v2"] {
        assert!(
            !sources.contains(forbidden),
            "forbidden authority: {forbidden}"
        );
    }
}
