#![allow(dead_code)]

use adl_engine::{ProviderRequest, ToolRequest};
use serde_json::json;

pub fn provider_request() -> ProviderRequest {
    serde_json::from_value(json!({
        "request_id": "request-1",
        "idempotency_key": "key-1",
        "sequence": 1,
        "node_id": "node-1",
        "attempt": 1,
        "provider_ref": "provider-1",
        "model": "model-1",
        "prompt": {"system": "system", "user": "hello"},
        "inputs": {"x": 1},
        "timeout_ticks": 10
    }))
    .unwrap()
}

pub fn tool_request() -> ToolRequest {
    serde_json::from_value(json!({
        "request_id": "request-2",
        "idempotency_key": "key-2",
        "sequence": 2,
        "node_id": "node-2",
        "attempt": 1,
        "tool": "read",
        "run": {"identity": "run", "name": "test", "inputs": {}, "placement_target": null},
        "inputs": {"path": "docs/readme.md"},
        "timeout_ticks": 10
    }))
    .unwrap()
}
