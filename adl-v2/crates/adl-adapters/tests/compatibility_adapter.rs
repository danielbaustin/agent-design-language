use adl_adapters::{CompatibilityAdapter, CompatibilityError, CompatibilityInput};
use serde_json::json;

fn adapter() -> CompatibilityAdapter {
    CompatibilityAdapter::new(vec!["v1".into()], vec!["openai".into()])
}
fn input() -> CompatibilityInput {
    CompatibilityInput {
        version: "v1".into(),
        provider: "openai".into(),
        model: Some("m".into()),
        payload: json!({"prompt":"hi"}),
    }
}

#[test]
fn approved_input_is_lossless() {
    let value = input();
    assert_eq!(adapter().translate(value.clone()), Ok(value));
}
#[test]
fn unknown_version_is_rejected() {
    let mut v = input();
    v.version = "v2".into();
    assert_eq!(
        adapter().translate(v),
        Err(CompatibilityError::UnknownVersion)
    );
}
#[test]
fn unknown_provider_is_rejected() {
    let mut v = input();
    v.provider = "alias".into();
    assert_eq!(
        adapter().translate(v),
        Err(CompatibilityError::UnknownProvider)
    );
}
#[test]
fn scalar_payload_is_rejected() {
    let mut v = input();
    v.payload = json!(1);
    assert_eq!(
        adapter().translate(v),
        Err(CompatibilityError::LossyPayload)
    );
}
#[test]
fn array_payload_is_rejected() {
    let mut v = input();
    v.payload = json!([]);
    assert_eq!(
        adapter().translate(v),
        Err(CompatibilityError::LossyPayload)
    );
}
#[test]
fn null_payload_is_rejected() {
    let mut v = input();
    v.payload = json!(null);
    assert_eq!(
        adapter().translate(v),
        Err(CompatibilityError::LossyPayload)
    );
}
#[test]
fn explicit_none_model_is_preserved() {
    let mut v = input();
    v.model = None;
    assert_eq!(adapter().translate(v.clone()), Ok(v));
}
#[test]
fn empty_object_is_lossless() {
    let mut v = input();
    v.payload = json!({});
    assert_eq!(adapter().translate(v.clone()), Ok(v));
}
