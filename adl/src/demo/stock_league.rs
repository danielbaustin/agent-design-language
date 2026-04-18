#[path = "stock_league/files.rs"]
mod files;
#[path = "stock_league/model.rs"]
mod model;
#[path = "stock_league/reports.rs"]
mod reports;
#[path = "stock_league/shared.rs"]
mod shared;
#[path = "stock_league/steps.rs"]
mod steps;

use std::path::{Path, PathBuf};

use anyhow::Result;

pub(super) const DEMO_NAME: &str = shared::DEMO_NAME;
pub(super) const INTEGRATION_DEMO_NAME: &str = shared::INTEGRATION_DEMO_NAME;
pub(super) const EXTENSION_DEMO_NAME: &str = shared::EXTENSION_DEMO_NAME;

pub(super) fn write_stock_league_scaffold_step(
    out_dir: &Path,
    step_id: &str,
) -> Result<Vec<PathBuf>> {
    steps::write_stock_league_scaffold_step(out_dir, step_id)
}

pub(super) fn write_stock_league_integration_step(
    out_dir: &Path,
    step_id: &str,
) -> Result<Vec<PathBuf>> {
    steps::write_stock_league_integration_step(out_dir, step_id)
}

pub(super) fn write_stock_league_extension_step(
    out_dir: &Path,
    step_id: &str,
) -> Result<Vec<PathBuf>> {
    steps::write_stock_league_extension_step(out_dir, step_id)
}
