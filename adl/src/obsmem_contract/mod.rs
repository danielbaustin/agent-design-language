mod client;
mod error;
mod models;
mod validation;

#[cfg(test)]
mod tests;

pub use client::ObsMemClient;
pub use error::{ObsMemContractError, ObsMemContractErrorCode};
pub use models::{
    MemoryCitation, MemoryFollowOnRef, MemoryQuery, MemoryQueryResult, MemoryRecord,
    MemoryReviewFinding, MemoryTraceRef, MemoryWriteAck, MemoryWriteRequest,
    OBSMEM_CONTRACT_VERSION,
};
