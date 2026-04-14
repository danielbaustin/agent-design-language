use super::{
    MemoryQuery, MemoryQueryResult, MemoryWriteAck, MemoryWriteRequest, ObsMemContractError,
};

/// ObsMem boundary trait. Runtime code depends on this abstraction only.
///
/// Implementations may wrap local files, an embedded index, or a remote service,
/// but runtime behavior must remain deterministic for identical requests.
pub trait ObsMemClient {
    fn write_entry(
        &self,
        request: &MemoryWriteRequest,
    ) -> Result<MemoryWriteAck, ObsMemContractError>;

    /// v0.75 retrieval boundary: structured deterministic query only.
    ///
    /// Backends may support semantic retrieval internally, but returned records
    /// must be normalized into deterministic ordering for identical query+index
    /// inputs before results are exposed to runtime callers.
    fn query(&self, query: &MemoryQuery) -> Result<MemoryQueryResult, ObsMemContractError>;
}
