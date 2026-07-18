use clap::Parser;
use csdlc_v2::{
    amend_claim_scope, bind_issue, heartbeat_claim, recover_claim, release_closed_claim,
    AmendClaimScopeRequest, BindRequest, HeartbeatRequest, RecoverClaimRequest,
    ReleaseClosedClaimRequest, Store,
};
use std::{fs, path::PathBuf};

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    root: PathBuf,
    #[arg(long)]
    request: Option<PathBuf>,
    #[arg(long, conflicts_with_all = ["request", "recover_request", "amend_request"])]
    heartbeat_request: Option<PathBuf>,
    #[arg(long, conflicts_with_all = ["request", "heartbeat_request", "amend_request"])]
    recover_request: Option<PathBuf>,
    #[arg(long, conflicts_with_all = ["request", "heartbeat_request", "recover_request"])]
    amend_request: Option<PathBuf>,
    #[arg(long, conflicts_with_all = ["request", "heartbeat_request", "recover_request", "amend_request"])]
    release_request: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let result = if let Some(path) = cli.release_request {
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
        let path = cli.request.ok_or_else(|| {
            csdlc_v2::V2Error::new(
                csdlc_v2::ErrorCode::InvalidInput,
                "one of --request, --heartbeat-request, --recover-request, --amend-request, or --release-request is required",
            )
        });
        path.and_then(|path| fs::read(path).map_err(csdlc_v2::V2Error::from))
            .and_then(|bytes| {
                serde_json::from_slice::<BindRequest>(&bytes).map_err(csdlc_v2::V2Error::from)
            })
            .and_then(|request| {
                bind_issue(&Store::new(cli.root.clone()), request)
                    .map(|value| serde_json::to_value(value).expect("JSON"))
            })
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
