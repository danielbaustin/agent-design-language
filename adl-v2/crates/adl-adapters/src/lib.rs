//! Small, opt-in adapters for the inert ADL v2 engine ports.

mod compatibility;
mod governed_tool;
mod https;
mod mock;

pub use compatibility::{CompatibilityAdapter, CompatibilityError, CompatibilityInput};
pub use governed_tool::{
    request_digest, AuthorizationEnvelope, AuthorizationVerifier, GovernedToolAdapter,
    GovernedToolError, ToolPort,
};
pub use https::{
    cancellation_pair, Cancellation, CancellationHandle, EndpointAuthorizer, EndpointPermit,
    HttpAdapter, HttpAdapterError, HttpRequest, HttpResponse,
};
pub use mock::{MockAdapter, MockError, ProviderStep, ToolStep};
