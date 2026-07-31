# Live provider probe disposition

- Kimi/Moonshot through `adl-provider-adapter`: HTTP 429, insufficient balance; typed as non-retryable billing failure.
- MiniMax through `adl-provider-adapter`: HTTP 402, insufficient balance (1008); typed as non-retryable billing failure.
- Both calls reached the provider through the Rust adapter. No successful completion is claimed because the approved accounts have no balance.
