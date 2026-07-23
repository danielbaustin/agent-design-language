use adl_records::{Record, SignedEnvelope};

fn main() {
    let bundle = serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "contract": "adl.records.schema-bundle.v1",
        "record": schemars::schema_for!(Record),
        "signed_envelope": schemars::schema_for!(SignedEnvelope),
        "semantic_validation": "Record::validate and verify_envelope are mandatory stage-two validation"
    });
    println!("{}", serde_json::to_string_pretty(&bundle).unwrap());
}
