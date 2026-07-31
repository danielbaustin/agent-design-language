use adl_adapters::HttpRequest;
use secrecy::SecretString;

#[test]
fn bearer_secret_debug_is_redacted() {
    let request = HttpRequest {
        method: "POST".into(),
        endpoint: "https://example.com".into(),
        body: Vec::new(),
        bearer: Some(SecretString::from("canary-secret")),
    };
    assert!(!format!("{request:?}").contains("canary-secret"));
}

#[test]
fn authorization_is_not_locally_mintable_as_verified() {
    let name = std::any::type_name::<adl_adapters::AuthorizationEnvelope>();
    assert!(!name.contains("VerifiedAuthorization"));
}

#[test]
fn errors_contain_no_request_or_secret_fields() {
    assert!(!format!("{:?}", adl_adapters::HttpAdapterError::Authentication).contains("canary"));
}

#[test]
fn compatibility_errors_contain_no_payload() {
    assert_eq!(
        format!("{:?}", adl_adapters::CompatibilityError::LossyPayload),
        "LossyPayload"
    );
}
