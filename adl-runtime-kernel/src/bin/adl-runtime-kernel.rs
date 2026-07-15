use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    process::ExitCode,
    sync::Arc,
};

use adl_runtime_kernel::{
    execute_loop, generate_runtime_instance_id, load_control_tls,
    proof::{build_proof_runtime, load_capsule, run_proof},
    serve_control_listener_until_ready, verifying_key_from_hex, AdaptationState, ControlAuthority,
    ControlCapability, ControlService, KernelExit, LoopDefinition, LoopStatus, ReasoningEdge,
    ReasoningGraphDefinition, ReasoningNode, RecordedObservation, ResourceState, RuntimeInitConfig,
    SysinfoWeatherObserver, TrustedControlKey, ValidatedReasoningGraph, WeatherConfig,
    WeatherHealthReport, WeatherObserver, MAX_SHADOW_FIXTURE_BYTES, REASONING_GRAPH_SCHEMA,
};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "demo".to_owned());

    match command.as_str() {
        "serve" => {
            let serve_args = match ServeArgs::parse(args) {
                Ok(args) => args,
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::from(64);
                }
            };
            let init = match RuntimeInitConfig::load(serve_args.init_path.clone()) {
                Ok(config) => config,
                Err(error) => {
                    eprintln!("runtime init invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let tls = match load_control_tls(&init.api.tls).await {
                Ok(config) => config,
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::from(78);
                }
            };
            let socket_addrs = match init.socket_addrs() {
                Ok(addrs) => addrs,
                Err(error) => {
                    eprintln!("runtime init invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let proof = match build_proof_runtime(&serve_args.capsule, 3) {
                Ok(proof) => proof,
                Err(error) => {
                    eprintln!("runtime topology invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let public_key = match std::env::var("ADL_RUNTIME_CONTROL_PUBLIC_KEY_HEX")
                .map_err(|_| ())
                .and_then(|value| verifying_key_from_hex(&value).map_err(|_| ()))
            {
                Ok(key) => key,
                Err(()) => {
                    eprintln!("runtime control key is missing or invalid");
                    return ExitCode::from(78);
                }
            };
            let key_id = std::env::var("ADL_RUNTIME_CONTROL_KEY_ID")
                .unwrap_or_else(|_| "operator".to_owned());
            let principal = std::env::var("ADL_RUNTIME_CONTROL_PRINCIPAL")
                .unwrap_or_else(|_| "operator".to_owned());
            let authority = ControlAuthority::new(BTreeMap::from([(
                key_id,
                TrustedControlKey {
                    principal,
                    verifying_key: public_key,
                    capabilities: BTreeSet::from([
                        ControlCapability::Read,
                        ControlCapability::Stop,
                    ]),
                },
            )]));
            let listener = match tokio::net::TcpListener::bind(socket_addrs.as_slice()).await {
                Ok(listener) => listener,
                Err(error) => {
                    eprintln!("runtime control API bind failed: {error}");
                    return ExitCode::from(70);
                }
            };
            let mut handle = match proof.kernel.start().await {
                Ok(handle) => handle,
                Err(error) => {
                    eprintln!("runtime kernel failed to start: {error}");
                    return ExitCode::from(70);
                }
            };
            let instance_id = generate_runtime_instance_id();
            let service = Arc::new(ControlService::new_with_observatory_config_and_agents(
                instance_id.clone(),
                proof.recorder,
                handle.control(),
                authority,
                1024,
                init.observatory_allowed_origins(),
                init.agent_population(),
            ));
            let mut weather_observer = SysinfoWeatherObserver::default();
            service.set_weather_report(WeatherHealthReport::from_sample(
                &WeatherConfig::default(),
                weather_observer.sample(),
                ResourceState::Healthy,
            ));
            let api_shutdown = tokio_util::sync::CancellationToken::new();
            let (ready_sender, ready_receiver) = tokio::sync::oneshot::channel();
            let mut api = tokio::spawn(serve_control_listener_until_ready(
                service,
                listener,
                tls,
                ready_sender,
                api_shutdown.clone().cancelled_owned(),
            ));
            let bound_address = match ready_receiver.await {
                Ok(address) => address,
                Err(_) => {
                    eprintln!("runtime control API failed before readiness");
                    let _ = handle.shutdown(std::time::Duration::from_secs(10)).await;
                    drain_control_api(&mut api).await;
                    return ExitCode::from(70);
                }
            };
            eprintln!(
                "{}",
                adl_runtime_kernel::control_ready_event(&instance_id, bound_address)
            );
            tokio::select! {
                signal = shutdown_signal() => {
                    if let Err(error) = signal {
                        eprintln!("runtime signal handler failed: {error}");
                        api_shutdown.cancel();
                        let _ = handle.shutdown(std::time::Duration::from_secs(10)).await;
                        drain_control_api(&mut api).await;
                        return ExitCode::from(70);
                    }
                    api_shutdown.cancel();
                    let shutdown = handle.shutdown(std::time::Duration::from_secs(10)).await;
                    drain_control_api(&mut api).await;
                    match shutdown {
                        Ok(exit) => {
                            process_exit(exit)
                        },
                        Err(error) => {
                            eprintln!("runtime shutdown failed: {error}");
                            ExitCode::from(70)
                        }
                    }
                }
                exit = handle.wait_for_exit() => match exit {
                    Ok(exit) => {
                        api_shutdown.cancel();
                        drain_control_api(&mut api).await;
                        process_exit(exit)
                    },
                    Err(error) => {
                        eprintln!("runtime kernel task failed: {error}");
                        api_shutdown.cancel();
                        let _ = handle.shutdown(std::time::Duration::from_secs(10)).await;
                        drain_control_api(&mut api).await;
                        ExitCode::from(70)
                    }
                },
                result = &mut api => {
                    match result {
                        Ok(Ok(())) => eprintln!("runtime control API stopped unexpectedly"),
                        Ok(Err(error)) => eprintln!("runtime control API failed: {error}"),
                        Err(error) => eprintln!("runtime control API task failed: {error}"),
                    }
                    let _ = handle.shutdown(std::time::Duration::from_secs(10)).await;
                    ExitCode::from(70)
                },
            }
        }
        "demo" => {
            let capsule = capsule_arg(args);
            match run_proof(&capsule, 3).await {
                Ok((exit, continuity)) => {
                    println!(
                        "{}",
                        serde_json::json!({
                            "schema": "adl.runtime_kernel.proof.v1",
                            "exit": format!("{exit:?}"),
                            "capsule": continuity,
                        })
                    );
                    ExitCode::SUCCESS
                }
                Err(error) => {
                    eprintln!("runtime kernel proof failed: {error}");
                    ExitCode::from(70)
                }
            }
        }
        "shadow-loop" => match run_shadow_loop().await {
            Ok(value) => {
                println!("{value}");
                ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("runtime shadow loop failed: {error}");
                ExitCode::from(70)
            }
        },
        "fatal-once" => {
            let capsule = capsule_arg(args);
            let marker = capsule.with_extension("fatal-once");
            if !marker.exists() {
                if let Err(error) = run_proof(&capsule, 3).await {
                    eprintln!("failed to checkpoint before fatal exit: {error}");
                    return ExitCode::from(74);
                }
                if let Err(error) = tokio::fs::write(&marker, b"fatal child exit injected").await {
                    eprintln!("failed to write fatal-once marker: {error}");
                    return ExitCode::from(74);
                }
                eprintln!("classified_fatal_exit:first_generation");
                ExitCode::from(70)
            } else {
                match run_proof(&capsule, 3).await {
                    Ok(_) => match load_capsule(&capsule).await {
                        Ok(capsule) if capsule.generation >= 2 => ExitCode::SUCCESS,
                        Ok(_) => {
                            eprintln!("runtime recovery did not advance continuity generation");
                            ExitCode::from(70)
                        }
                        Err(error) => {
                            eprintln!("runtime recovery capsule invalid: {error}");
                            ExitCode::from(70)
                        }
                    },
                    Err(error) => {
                        eprintln!("runtime recovery proof failed: {error}");
                        ExitCode::from(70)
                    }
                }
            }
        }
        _ => {
            eprintln!(
                "usage: adl-runtime-kernel [serve [--init path] [--capsule path]|demo [capsule-path]|shadow-loop|fatal-once [capsule-path]]"
            );
            ExitCode::from(64)
        }
    }
}

struct ServeArgs {
    capsule: PathBuf,
    init_path: Option<PathBuf>,
}

impl ServeArgs {
    fn parse(args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut init_path = None;
        let mut capsule = None;
        let mut args = args.peekable();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--init" => {
                    let Some(path) = args.next() else {
                        return Err("--init requires a runtime init file path".to_owned());
                    };
                    init_path = Some(PathBuf::from(path));
                }
                "--capsule" => {
                    let Some(path) = args.next() else {
                        return Err("--capsule requires a continuity capsule path".to_owned());
                    };
                    capsule = Some(PathBuf::from(path));
                }
                other if other.starts_with('-') => {
                    return Err(format!("unknown serve option: {other}"));
                }
                path => {
                    if capsule.is_some() {
                        return Err("serve accepts only one continuity capsule path".to_owned());
                    }
                    capsule = Some(PathBuf::from(path));
                }
            }
        }
        Ok(Self {
            capsule: capsule.unwrap_or_else(|| PathBuf::from("adl-runtime-kernel-continuity.json")),
            init_path,
        })
    }
}

fn capsule_arg(mut args: impl Iterator<Item = String>) -> PathBuf {
    args.next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("adl-runtime-kernel-continuity.json"))
}

async fn shutdown_signal() -> std::io::Result<()> {
    #[cfg(unix)]
    {
        let mut terminate =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
        tokio::select! {
            result = tokio::signal::ctrl_c() => result,
            _ = terminate.recv() => Ok(()),
        }
    }
    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c().await
    }
}

async fn run_shadow_loop() -> Result<serde_json::Value, String> {
    let fixture = tokio::task::spawn_blocking(|| {
        let mut fixture = String::new();
        std::io::Read::read_to_string(
            &mut std::io::Read::take(std::io::stdin(), (MAX_SHADOW_FIXTURE_BYTES + 1) as u64),
            &mut fixture,
        )?;
        Ok::<_, std::io::Error>(fixture)
    })
    .await
    .map_err(|error| error.to_string())?
    .map_err(|error| error.to_string())?;
    if fixture.len() > MAX_SHADOW_FIXTURE_BYTES {
        return Err("shadow fixture JSON exceeds 1 MiB".to_owned());
    }
    if fixture.trim().is_empty() {
        return Err("shadow fixture JSON is required on stdin".to_owned());
    }
    let fixture: serde_json::Value =
        serde_json::from_str(&fixture).map_err(|error| error.to_string())?;
    let max_iterations = fixture["max_iterations"]
        .as_u64()
        .and_then(|value| u32::try_from(value).ok())
        .ok_or_else(|| "max_iterations must be a u32".to_owned())?;
    let graph = ValidatedReasoningGraph::validate(ReasoningGraphDefinition {
        schema: REASONING_GRAPH_SCHEMA.to_owned(),
        version: 1,
        entry: "observe".to_owned(),
        exits: BTreeSet::from(["decide".to_owned()]),
        nodes: vec![
            ReasoningNode {
                id: "observe".to_owned(),
                score_delta: 1,
            },
            ReasoningNode {
                id: "evaluate".to_owned(),
                score_delta: 1,
            },
            ReasoningNode {
                id: "decide".to_owned(),
                score_delta: 1,
            },
        ],
        edges: vec![
            ReasoningEdge {
                from: "observe".to_owned(),
                to: "evaluate".to_owned(),
            },
            ReasoningEdge {
                from: "evaluate".to_owned(),
                to: "decide".to_owned(),
            },
        ],
    })
    .map_err(|error| error.to_string())?;
    let policy_hash = blake3::hash(b"shadow-parity-policy").to_hex().to_string();
    let outcome = execute_loop(
        &graph,
        &LoopDefinition {
            target_score: 7,
            max_iterations,
            deadline_millis: 1_000,
        },
        &RecordedObservation {
            observation_id: "shadow-observation".to_owned(),
            score: 0,
            evidence_hash: blake3::hash(b"shadow-fixture").to_hex().to_string(),
        },
        AdaptationState::new(0, graph.hash(), policy_hash),
        CancellationToken::new(),
    )
    .await
    .map_err(|error| error.to_string())?;
    let terminal_node_id = if outcome.status == LoopStatus::Converged {
        graph.definition().exits.iter().next().cloned()
    } else {
        None
    };
    let state_hash = outcome.state.hash().map_err(|error| error.to_string())?;
    Ok(serde_json::json!({
        "schema": "adl.runtime.shadow_loop.v1",
        "status": outcome.status,
        "iterations": outcome.iterations,
        "terminal_node_id": terminal_node_id,
        "exit_node_ids": graph.definition().exits,
        "replay": outcome.replay.iter().map(|event| event.sequence).collect::<Vec<_>>(),
        "state_hash": state_hash,
        "state": outcome.state,
        "evidence": ["bounded_loop", "deterministic_replay"]
    }))
}

fn process_exit(exit: KernelExit) -> ExitCode {
    match exit {
        KernelExit::Clean => ExitCode::SUCCESS,
        KernelExit::Fatal { component } => {
            eprintln!("classified_fatal_exit:{component}");
            ExitCode::from(70)
        }
        KernelExit::ShutdownFailed { components } => {
            eprintln!("classified_shutdown_failure:{components:?}");
            ExitCode::from(74)
        }
        KernelExit::ShutdownDeadlineExceeded { aborted } => {
            eprintln!("classified_shutdown_deadline:{aborted:?}");
            ExitCode::from(70)
        }
    }
}

async fn drain_control_api(
    api: &mut tokio::task::JoinHandle<Result<(), adl_runtime_kernel::ControlApiError>>,
) {
    if tokio::time::timeout(std::time::Duration::from_secs(3), &mut *api)
        .await
        .is_err()
    {
        api.abort();
    }
}
