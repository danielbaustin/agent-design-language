use std::collections::BTreeMap;

use adl_engine::{
    CancelCompletion, CompletionOutcome, EngineEffect, FailureClass, PortCompletion, PortFailure,
    PortOutput, ProviderCompletion, ToolCompletion,
};
use adl_records::{ErrorRecord, ExecutionResult, Record, RecordHeader};
use adl_runtime_kernel::{DomainResult, IngressError};
use sha2::{Digest, Sha256};

use crate::effect_identity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterOutcome {
    pub completion: PortCompletion,
    pub record: Record,
}

pub(crate) fn map_outcome(
    effect: EngineEffect,
    header: RecordHeader,
    result: Result<DomainResult, IngressError>,
) -> AdapterOutcome {
    let (outcome, record) = match result {
        Ok(result) if result.work_id == effect_identity(&effect).1 => {
            let output = PortOutput::new(
                "application/vnd.adl.runtime-result-hash",
                result.result_hash.as_bytes().to_vec(),
            );
            (
                CompletionOutcome::Success(output),
                success_record(header, &result),
            )
        }
        Ok(_) => failure_pair(
            header,
            FailureClass::Protocol,
            "runtime work identity mismatch",
        ),
        Err(error) => failure_pair(header, failure_class(&error), &error.to_string()),
    };
    let completion = match effect {
        EngineEffect::Provider(request) => PortCompletion::Provider(Box::new(ProviderCompletion {
            request_id: request.request_id,
            node_id: request.node_id,
            attempt: request.attempt,
            outcome,
        })),
        EngineEffect::Tool(request) => PortCompletion::Tool(Box::new(ToolCompletion {
            request_id: request.request_id,
            node_id: request.node_id,
            attempt: request.attempt,
            outcome,
        })),
        EngineEffect::Cancel(request) => PortCompletion::Cancel(CancelCompletion {
            request_id: request.request_id,
            node_id: request.node_id,
            attempt: request.attempt,
            acknowledged: matches!(outcome, CompletionOutcome::Success(_)),
        }),
    };
    AdapterOutcome { completion, record }
}

fn success_record(mut header: RecordHeader, result: &DomainResult) -> Record {
    header.record_id = stable_record_id(&header.record_id, &result.result_hash);
    header
        .metadata
        .insert("runtime_work_id".into(), result.work_id.clone());
    header
        .metadata
        .insert("runtime_result_hash".into(), result.result_hash.clone());
    Record::ExecutionResult(ExecutionResult {
        header,
        status: "succeeded".into(),
        output_digest: Some(result.result_hash.clone()),
        diagnostic: None,
    })
}

fn failure_pair(
    header: RecordHeader,
    class: FailureClass,
    message: &str,
) -> (CompletionOutcome, Record) {
    let code = format!("{:?}", class).to_ascii_lowercase();
    let retryable = matches!(class, FailureClass::Retryable | FailureClass::Saturation);
    let failure = PortFailure::new(class, message);
    let mut error_header = header;
    error_header.record_id = stable_record_id(&error_header.record_id, message);
    let record = Record::Error(ErrorRecord {
        header: error_header,
        code,
        message: message.to_owned(),
        retryable,
    });
    (CompletionOutcome::Failure(failure), record)
}

fn failure_class(error: &IngressError) -> FailureClass {
    match error {
        IngressError::Invalid | IngressError::UnsupportedKind => FailureClass::InvalidRequest,
        IngressError::Conflict => FailureClass::Protocol,
        IngressError::Saturated => FailureClass::Saturation,
        IngressError::Closed => FailureClass::Resource,
        IngressError::ExecutionFailed => FailureClass::Permanent,
        IngressError::DrainTimeout => FailureClass::Timeout,
    }
}

fn stable_record_id(source: &str, result: &str) -> String {
    let fields = BTreeMap::from([("source", source), ("result", result)]);
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&fields).unwrap_or_default())
    )
}
