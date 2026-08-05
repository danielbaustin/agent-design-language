# Issue 5675 design

Add first-class Rust-native hosted adapter routes for Kimi/Moonshot and MiniMax.
Both routes use bearer credentials, bounded chat-completion requests, shared
redacted invocation logging, and provider-specific error handling. MiniMax
success-envelope errors are classified before response extraction so billing
failures remain typed and non-retryable.

The change is limited to the provider adapter and the MiniMax profile endpoint.
It does not add shell/Python lifecycle logic, credential provisioning, or new
provider transports.
