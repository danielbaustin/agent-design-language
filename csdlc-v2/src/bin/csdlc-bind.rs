use clap::{Parser, Subcommand};
use csdlc_v2::{
    amend_claim_scope, heartbeat_claim, recover_claim, rehome_claim_authority,
    release_closed_claim, release_derived_bind, revoke_active_claim, run_derived_bind,
    transition_active_claim, AmendClaimScopeRequest, BindReleaseRequest, DerivedBindRequest,
    HeartbeatRequest, RecoverClaimRequest, RehomeClaimAuthorityRequest, ReleaseClosedClaimRequest,
    RevokeActiveClaimRequest, Store, TransitionActiveClaimRequest,
};
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(args_conflicts_with_subcommands = true)]
struct Cli {
    #[arg(long)]
    root: PathBuf,
    #[arg(long)]
    heartbeat_request: Option<PathBuf>,
    #[arg(long)]
    recover_request: Option<PathBuf>,
    #[arg(long)]
    rehome_request: Option<PathBuf>,
    #[arg(long)]
    amend_request: Option<PathBuf>,
    #[arg(long)]
    transition_request: Option<PathBuf>,
    #[arg(long)]
    release_request: Option<PathBuf>,
    #[arg(long)]
    revoke_request: Option<PathBuf>,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Run {
        #[arg(long)]
        request: PathBuf,
    },
    Release {
        #[arg(long)]
        request: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    let result = if let Some(command) = cli.command {
        match command {
            Command::Run { request } => fs::read(request)
                .map_err(csdlc_v2::V2Error::from)
                .and_then(|bytes| {
                    serde_json::from_slice::<DerivedBindRequest>(&bytes)
                        .map_err(csdlc_v2::V2Error::from)
                })
                .and_then(|request| run_derived_bind(&Store::new(cli.root.clone()), request))
                .map(|value| serde_json::to_value(value).expect("JSON")),
            Command::Release { request } => fs::read(request)
                .map_err(csdlc_v2::V2Error::from)
                .and_then(|bytes| {
                    serde_json::from_slice::<BindReleaseRequest>(&bytes)
                        .map_err(csdlc_v2::V2Error::from)
                })
                .and_then(|request| release_derived_bind(&Store::new(cli.root.clone()), request))
                .map(|value| serde_json::to_value(value).expect("JSON")),
        }
    } else if let Some(path) = cli.rehome_request {
        fs::read(path)
            .map_err(csdlc_v2::V2Error::from)
            .and_then(|bytes| {
                serde_json::from_slice::<RehomeClaimAuthorityRequest>(&bytes)
                    .map_err(csdlc_v2::V2Error::from)
            })
            .and_then(|request| {
                rehome_claim_authority(&Store::new(cli.root.clone()), request)
                    .map(|value| serde_json::to_value(value).expect("JSON"))
            })
    } else if let Some(path) = cli.revoke_request {
        fs::read(path)
            .map_err(csdlc_v2::V2Error::from)
            .and_then(|bytes| {
                serde_json::from_slice::<RevokeActiveClaimRequest>(&bytes)
                    .map_err(csdlc_v2::V2Error::from)
            })
            .and_then(|request| {
                revoke_active_claim(&Store::new(cli.root.clone()), request)
                    .map(|value| serde_json::to_value(value).expect("JSON"))
            })
    } else if let Some(path) = cli.transition_request {
        fs::read(path)
            .map_err(csdlc_v2::V2Error::from)
            .and_then(|bytes| {
                serde_json::from_slice::<TransitionActiveClaimRequest>(&bytes)
                    .map_err(csdlc_v2::V2Error::from)
            })
            .and_then(|request| {
                transition_active_claim(&Store::new(cli.root.clone()), request)
                    .map(|value| serde_json::to_value(value).expect("JSON"))
            })
    } else if let Some(path) = cli.release_request {
        fs::read(path)
            .map_err(csdlc_v2::V2Error::from)
            .and_then(|bytes| {
                serde_json::from_slice::<ReleaseClosedClaimRequest>(&bytes)
                    .map_err(csdlc_v2::V2Error::from)
            })
            .and_then(|request| {
                release_closed_claim(&Store::new(cli.root.clone()), request)
                    .map(|value| serde_json::to_value(value).expect("JSON"))
            })
    } else if let Some(path) = cli.heartbeat_request {
        fs::read(path).map_err(csdlc_v2::V2Error::from).and_then(|bytes| serde_json::from_slice::<HeartbeatRequest>(&bytes).map_err(csdlc_v2::V2Error::from)).and_then(|request| heartbeat_claim(&Store::new(cli.root.clone()), request.issue, &request.claim_id, request.expected_generation, request.now_unix_seconds, request.extend_seconds).map(|_| serde_json::json!({"schema":"csdlc.heartbeat_result.v1","issue":request.issue})))
    } else if let Some(path) = cli.recover_request {
        fs::read(path)
            .map_err(csdlc_v2::V2Error::from)
            .and_then(|bytes| {
                serde_json::from_slice::<RecoverClaimRequest>(&bytes)
                    .map_err(csdlc_v2::V2Error::from)
            })
            .and_then(|request| {
                recover_claim(&Store::new(cli.root.clone()), request)
                    .map(|value| serde_json::to_value(value).expect("JSON"))
            })
    } else if let Some(path) = cli.amend_request {
        fs::read(path)
            .map_err(csdlc_v2::V2Error::from)
            .and_then(|bytes| {
                serde_json::from_slice::<AmendClaimScopeRequest>(&bytes)
                    .map_err(csdlc_v2::V2Error::from)
            })
            .and_then(|request| {
                amend_claim_scope(&Store::new(cli.root.clone()), request)
                    .map(|value| serde_json::to_value(value).expect("JSON"))
            })
    } else {
        Err(csdlc_v2::V2Error::new(
            csdlc_v2::ErrorCode::InvalidInput,
            "a run/release subcommand or typed claim-maintenance request is required",
        ))
    };
    match result {
        Ok(value) => println!("{}", serde_json::to_string(&value).expect("JSON")),
        Err(error) => {
            eprintln!("csdlc-bind: {}", error);
            println!(
                "{}",
                serde_json::json!({"schema":"csdlc.error.v1","code":error.code,"message":error.message})
            );
            std::process::exit(error.code.exit_code());
        }
    }
}
