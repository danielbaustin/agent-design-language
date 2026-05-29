use adl::provider_adapter_cli::{run_provider_adapter_cli, ProviderAdapterCliArgs};
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    run_provider_adapter_cli(ProviderAdapterCliArgs::parse())
}
