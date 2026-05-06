//! Runtime v2 public surface for core operational APIs and proof artifacts.
//!
//! This module groups the prototype runtime domain into smaller documented surfaces:
//! manifold state, kernel control, citizen lifecycle, private-state pathways,
//! and proof-bearing contracts used by downstream review tooling.

mod access_control;
mod bid_schema;
mod boot_admission;
mod challenge;
mod citizen;
mod contract_lifecycle_state;
mod contract_market_demo;
mod contract_schema;
mod contracts;
mod csm_run;
mod delegation_subcontract;
mod evaluation_selection;
mod external_counterparty;
mod feature_proof_coverage;
mod foundation;
mod governed_episode;
mod governed_tools_flagship_demo;
mod hardening;
mod integrated_csm_run;
mod invariant;
mod invariant_contract;
mod kernel_loop;
mod manifold;
mod moral_event_validation;
mod moral_metrics;
mod moral_trace_schema;
mod observatory;
mod observatory_flagship;
mod operator;
mod outcome_linkage_attribution;
mod private_state;
mod private_state_envelope;
mod private_state_equivocation;
mod private_state_lineage;
mod private_state_observatory;
mod private_state_sanctuary;
mod private_state_sealing;
mod private_state_witness;
mod quarantine;
mod recovery;
mod resource_stewardship_bridge;
mod security;
mod snapshot;
mod standing;
mod transition_authority;
mod types;
mod validators;

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[allow(unused_imports)]
pub use access_control::*;
#[allow(unused_imports)]
pub use bid_schema::*;
#[allow(unused_imports)]
pub use boot_admission::*;
#[allow(unused_imports)]
pub use challenge::*;
#[allow(unused_imports)]
pub use citizen::*;
#[allow(unused_imports)]
pub use contract_lifecycle_state::*;
#[allow(unused_imports)]
pub use contract_market_demo::*;
#[allow(unused_imports)]
pub use contract_schema::*;
#[allow(unused_imports)]
pub use contracts::*;
#[allow(unused_imports)]
pub use csm_run::*;
#[allow(unused_imports)]
pub use delegation_subcontract::*;
#[allow(unused_imports)]
pub use evaluation_selection::*;
#[allow(unused_imports)]
pub use external_counterparty::*;
#[allow(unused_imports)]
pub use feature_proof_coverage::*;
#[allow(unused_imports)]
pub use foundation::*;
#[allow(unused_imports)]
pub use governed_episode::*;
#[allow(unused_imports)]
pub use governed_tools_flagship_demo::*;
#[allow(unused_imports)]
pub use hardening::*;
#[allow(unused_imports)]
pub use integrated_csm_run::*;
#[allow(unused_imports)]
pub use invariant::*;
#[allow(unused_imports)]
pub use invariant_contract::*;
#[allow(unused_imports)]
pub use kernel_loop::*;
#[allow(unused_imports)]
pub use manifold::*;
#[allow(unused_imports)]
pub use moral_event_validation::*;
#[allow(unused_imports)]
pub use moral_metrics::*;
#[allow(unused_imports)]
pub use moral_trace_schema::*;
#[allow(unused_imports)]
pub use observatory::*;
#[allow(unused_imports)]
pub use observatory_flagship::*;
#[allow(unused_imports)]
pub use operator::*;
#[allow(unused_imports)]
pub use outcome_linkage_attribution::*;
#[allow(unused_imports)]
pub use private_state::*;
#[allow(unused_imports)]
pub use private_state_envelope::*;
#[allow(unused_imports)]
pub use private_state_equivocation::*;
#[allow(unused_imports)]
pub use private_state_lineage::*;
#[allow(unused_imports)]
pub use private_state_observatory::*;
#[allow(unused_imports)]
pub use private_state_sanctuary::*;
#[allow(unused_imports)]
pub use private_state_sealing::*;
#[allow(unused_imports)]
pub use private_state_witness::*;
#[allow(unused_imports)]
pub use quarantine::*;
#[allow(unused_imports)]
pub use recovery::*;
#[allow(unused_imports)]
pub use resource_stewardship_bridge::*;
#[allow(unused_imports)]
pub use security::*;
#[allow(unused_imports)]
pub use snapshot::*;
#[allow(unused_imports)]
pub use standing::*;
#[allow(unused_imports)]
pub use transition_authority::*;
#[allow(unused_imports)]
pub use types::*;
#[allow(unused_imports)]
pub(crate) use validators::*;

#[cfg(test)]
mod tests;
