use super::*;

use crate::cli::pr_cmd_cards::{validate_bootstrap_output_card, write_output_card};
use crate::cli::pr_cmd_prompt::{infer_wp_from_title, render_generated_issue_prompt};
use crate::cli::pr_cmd_validate::bootstrap_stub_reason;
use adl::control_plane::{
    card_input_path, card_output_path, card_plan_path, card_review_policy_path, card_stp_path,
};
use std::env;

mod support;
mod versioned_bootstrap;

pub(crate) use support::*;

mod basics;
mod finish;
mod lifecycle;
mod repo_helpers;
