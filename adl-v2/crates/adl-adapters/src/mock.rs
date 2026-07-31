use adl_engine::{
    CompletionOutcome, ProviderCompletion, ProviderRequest, ToolCompletion, ToolRequest,
};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MockError {
    Exhausted,
    UnexpectedRequest,
}

#[derive(Debug, Clone)]
pub struct ProviderStep {
    expected: Vec<u8>,
    outcome: CompletionOutcome,
}

impl ProviderStep {
    pub fn new(request: &ProviderRequest, outcome: CompletionOutcome) -> Result<Self, MockError> {
        Ok(Self {
            expected: canonical(request)?,
            outcome,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ToolStep {
    expected: Vec<u8>,
    outcome: CompletionOutcome,
}

impl ToolStep {
    pub fn new(request: &ToolRequest, outcome: CompletionOutcome) -> Result<Self, MockError> {
        Ok(Self {
            expected: canonical(request)?,
            outcome,
        })
    }
}

#[derive(Debug, Default)]
pub struct MockAdapter {
    providers: VecDeque<ProviderStep>,
    tools: VecDeque<ToolStep>,
}

impl MockAdapter {
    pub fn scripted(providers: Vec<ProviderStep>, tools: Vec<ToolStep>) -> Self {
        Self {
            providers: providers.into(),
            tools: tools.into(),
        }
    }

    pub fn provider(&mut self, request: &ProviderRequest) -> Result<ProviderCompletion, MockError> {
        let step = self.providers.pop_front().ok_or(MockError::Exhausted)?;
        if step.expected != canonical(request)? {
            return Err(MockError::UnexpectedRequest);
        }
        Ok(ProviderCompletion {
            request_id: request.request_id.clone(),
            node_id: request.node_id.clone(),
            attempt: request.attempt,
            outcome: step.outcome,
        })
    }

    pub fn tool(&mut self, request: &ToolRequest) -> Result<ToolCompletion, MockError> {
        let step = self.tools.pop_front().ok_or(MockError::Exhausted)?;
        if step.expected != canonical(request)? {
            return Err(MockError::UnexpectedRequest);
        }
        Ok(ToolCompletion {
            request_id: request.request_id.clone(),
            node_id: request.node_id.clone(),
            attempt: request.attempt,
            outcome: step.outcome,
        })
    }

    pub fn is_exhausted(&self) -> bool {
        self.providers.is_empty() && self.tools.is_empty()
    }
}

fn canonical<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, MockError> {
    serde_json::to_vec(value).map_err(|_| MockError::UnexpectedRequest)
}
