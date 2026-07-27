use std::{
    collections::{BTreeMap, BTreeSet},
    future::Future,
    path::PathBuf,
    pin::Pin,
    process::ExitCode,
    sync::Arc,
};

#[path = "../observability.rs"]
mod observability;

use adl_runtime_kernel::{
    bootstrap_reasoning_services, build_live_assembly,
    build_production_operation_executors_with_recorder, execute_loop, generate_runtime_instance_id,
    load_control_tls, monitor_until_stop,
    proof::{load_capsule, run_proof},
    serve_control_listener_until_ready, validate_production_operation_executors,
    verifying_key_from_hex, AdaptationState, CheckpointShutdownRequest, CheckpointingControl,
    ControlAuthority, ControlCapability, ControlService, Kernel, KernelExit, LiveBindings,
    LiveContinuity, LiveKernelSnapshot, LoopDefinition, LoopStatus, ReasoningEdge,
    ReasoningGraphDefinition, ReasoningNode, RecordedObservation, RsntpTimeSampleSource,
    RuntimeInitConfig, RuntimeRecorder, SysinfoWeatherObserver, TimeQualificationBounds,
    TrustedControlKey, ValidatedReasoningGraph, MAX_SHADOW_FIXTURE_BYTES, REASONING_GRAPH_SCHEMA,
};
use observability::{RuntimeVectorConfig, RuntimeVectorPipeline};
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
            let continuity_secret = match std::env::var("ADL_RUNTIME_CONTINUITY_SIGNING_KEY_HEX")
                .map_err(|_| ())
                .and_then(|value| LiveContinuity::signing_key_from_hex(&value).map_err(|_| ()))
            {
                Ok(secret) => secret,
                Err(()) => {
                    eprintln!("runtime continuity signing key is missing or invalid");
                    return ExitCode::from(78);
                }
            };
            let continuity_key_id = std::env::var("ADL_RUNTIME_CONTINUITY_KEY_ID")
                .unwrap_or_else(|_| "runtime-continuity".to_owned());
            if continuity_key_id.trim().is_empty() {
                eprintln!("runtime continuity key id is empty");
                return ExitCode::from(78);
            }
            let operation_state_dir = match std::env::var("ADL_RUNTIME_V3_LOCAL_STATE_DIR")
                .ok()
                .filter(|value| !value.trim().is_empty())
            {
                Some(value) => std::path::PathBuf::from(value),
                None => {
                    eprintln!("runtime local adapter state root is missing");
                    return ExitCode::from(78);
                }
            };
            if !operation_state_dir.is_absolute() {
                eprintln!("runtime local adapter state root must be absolute");
                return ExitCode::from(78);
            }
            if let Err(error) = std::fs::create_dir_all(&operation_state_dir) {
                eprintln!("runtime local adapter state root is invalid: {error}");
                return ExitCode::from(78);
            }
            let operation_state_identity = match operation_state_dir.canonicalize() {
                Ok(path) if path.is_dir() => path,
                Ok(_) => {
                    eprintln!("runtime local adapter state root is not a directory");
                    return ExitCode::from(78);
                }
                Err(error) => {
                    eprintln!("runtime local adapter state root is invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let instance_id = generate_runtime_instance_id();
            let recorder = RuntimeRecorder::new(1_024);
            let reasoning = match bootstrap_reasoning_services(recorder.clone()) {
                Ok(reasoning) => reasoning,
                Err(error) => {
                    eprintln!("runtime reasoning bootstrap invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let operation_executors = match build_production_operation_executors_with_recorder(
                operation_state_identity.clone(),
                recorder.clone(),
            ) {
                Ok(executors) => executors,
                Err(error) => {
                    eprintln!("runtime local adapter state root is invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            if let Err(error) = validate_production_operation_executors(&operation_executors) {
                eprintln!("runtime live operation adapters unavailable: {error}");
                return ExitCode::from(78);
            }
            let operation_key = match std::env::var("ADL_RUNTIME_OPERATION_PUBLIC_KEY_HEX")
                .map_err(|_| ())
                .and_then(|value| verifying_key_from_hex(&value).map_err(|_| ()))
            {
                Ok(key) => key,
                Err(()) => {
                    eprintln!("runtime operation permit key is missing or invalid");
                    return ExitCode::from(78);
                }
            };
            if ed25519_dalek::SigningKey::from_bytes(&continuity_secret).verifying_key()
                == operation_key
            {
                eprintln!("runtime continuity and operation keys must be distinct");
                return ExitCode::from(78);
            }
            let operation_key_id = std::env::var("ADL_RUNTIME_OPERATION_KEY_ID")
                .unwrap_or_else(|_| "runtime-operations".to_owned());
            if operation_key_id.trim().is_empty() {
                eprintln!("runtime operation key id is empty");
                return ExitCode::from(78);
            }
            let sntp_server = std::env::var("ADL_RUNTIME_SNTP_SERVER")
                .unwrap_or_else(|_| "pool.ntp.org".to_owned());
            let assembly = match build_live_assembly(LiveBindings {
                recorder: recorder.clone(),
                operation_executors,
                permit_keys: BTreeMap::from([(operation_key_id.clone(), operation_key)]),
                reasoning,
                time_source: Arc::new(RsntpTimeSampleSource::new(sntp_server.clone())),
                time_bounds: TimeQualificationBounds {
                    timeout: std::time::Duration::from_secs(3),
                    max_offset: std::time::Duration::from_secs(5),
                    max_round_trip: std::time::Duration::from_secs(2),
                },
            }) {
                Ok(assembly) => assembly,
                Err(error) => {
                    eprintln!("runtime live topology invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let minimum_generation = match std::env::var("ADL_RUNTIME_CONTINUITY_MIN_GENERATION") {
                Ok(value) => match value.parse::<u64>() {
                    Ok(value) => value,
                    Err(_) => {
                        eprintln!("runtime continuity minimum generation is invalid");
                        return ExitCode::from(78);
                    }
                },
                Err(_) => {
                    eprintln!("runtime continuity minimum generation is missing");
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
            let continuity_public_key =
                ed25519_dalek::SigningKey::from_bytes(&continuity_secret).verifying_key();
            if public_key == operation_key || public_key == continuity_public_key {
                eprintln!("runtime control, operation, and continuity keys must be distinct");
                return ExitCode::from(78);
            }
            let key_id = std::env::var("ADL_RUNTIME_CONTROL_KEY_ID")
                .unwrap_or_else(|_| "operator".to_owned());
            let principal = std::env::var("ADL_RUNTIME_CONTROL_PRINCIPAL")
                .unwrap_or_else(|_| "operator".to_owned());
            if key_id.trim().is_empty() || key_id != key_id.trim() {
                eprintln!("runtime control key id is empty or has surrounding whitespace");
                return ExitCode::from(78);
            }
            if principal.trim().is_empty() || principal != principal.trim() {
                eprintln!("runtime control principal is empty or has surrounding whitespace");
                return ExitCode::from(78);
            }
            let service_schemas = assembly
                .contracts
                .contracts()
                .map(|contract| (contract.service.clone(), contract.config_schema.clone()))
                .collect::<BTreeMap<_, _>>();
            let tls_certificate_hash = match file_hash(&init.api.tls.certificate_chain_path).await {
                Ok(hash) => hash,
                Err(error) => {
                    eprintln!("runtime TLS certificate identity could not be hashed: {error}");
                    return ExitCode::from(78);
                }
            };
            let tls_private_key_hash = match file_hash(&init.api.tls.private_key_path).await {
                Ok(hash) => hash,
                Err(error) => {
                    eprintln!("runtime TLS private key identity could not be hashed: {error}");
                    return ExitCode::from(78);
                }
            };
            let binding_projection = serde_json::json!({
                "assembly_config_hash": assembly.config_hash,
                "runtime_init": &init,
                "sntp_server": &sntp_server,
                "operation_key_id": &operation_key_id,
                "operation_key": hex::encode(operation_key.as_bytes()),
                "control_key_id": &key_id,
                "control_principal": &principal,
                "control_key": hex::encode(public_key.as_bytes()),
                "continuity_key_id": &continuity_key_id,
                "continuity_key": hex::encode(continuity_public_key.as_bytes()),
                "continuity_minimum_generation": minimum_generation,
                "operation_state_root": operation_state_identity,
                "tls_certificate_hash": tls_certificate_hash,
                "tls_private_key_hash": tls_private_key_hash,
            });
            let config_hash = blake3::hash(
                &serde_json::to_vec(&binding_projection)
                    .expect("runtime binding JSON is encodable"),
            )
            .to_hex()
            .to_string();
            let snapshot = LiveKernelSnapshot::new(
                assembly.topology_hash.clone(),
                config_hash,
                service_schemas,
            );
            let continuity_root = serve_args.continuity_root;
            let mut continuity = LiveContinuity::new(
                &continuity_root,
                continuity_key_id,
                &continuity_secret,
                snapshot,
                minimum_generation,
            )
            .with_canonical_ingress(assembly.canonical_ingress.clone());
            if let Err(error) = continuity.restore_latest(&recorder).await {
                eprintln!("runtime continuity restore refused: {error}");
                return ExitCode::from(78);
            }
            let authority = ControlAuthority::new(BTreeMap::from([(
                key_id,
                TrustedControlKey {
                    principal,
                    verifying_key: public_key,
                    capabilities: BTreeSet::from([
                        ControlCapability::Read,
                        ControlCapability::Execute,
                        ControlCapability::Stop,
                    ]),
                },
            )]));
            let (lifecycle, mut shutdown_requests) = CheckpointingControl::channel(4);
            let service = Arc::new(
                ControlService::new_with_observatory_config_and_agents(
                    instance_id.clone(),
                    recorder.clone(),
                    lifecycle,
                    authority,
                    1024,
                    init.observatory_allowed_origins(),
                    init.agent_population(),
                )
                .with_canonical_ingress(assembly.canonical_ingress.clone()),
            );
            let observatory_token = match std::env::var("ADL_RUNTIME_OBSERVATORY_TOKEN") {
                Ok(token) => token,
                Err(_) => {
                    eprintln!("runtime Observatory read token is missing");
                    return ExitCode::from(78);
                }
            };
            if service
                .set_observatory_bearer_token(&observatory_token)
                .is_err()
            {
                eprintln!("runtime Observatory read token is invalid");
                return ExitCode::from(78);
            }
            if service
                .set_public_base_url(&init.api.public_base_url)
                .is_err()
            {
                eprintln!("runtime public HTTPS base is invalid");
                return ExitCode::from(78);
            }
            service.set_weather_stale_after(std::time::Duration::from_millis(
                init.weather.sample_millis.saturating_mul(2),
            ));
            let pressure_checkpoint_deadline =
                std::time::Duration::from_millis(init.weather.checkpoint_deadline_millis);
            let api_shutdown = tokio_util::sync::CancellationToken::new();
            let vector_config = match RuntimeVectorConfig::from_runtime_environment(
                operation_state_identity.clone(),
                instance_id.clone(),
                runtime_revision(),
            ) {
                Ok(config) => config,
                Err(error) => {
                    eprintln!("runtime observability configuration invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let mut observability = match RuntimeVectorPipeline::start(vector_config) {
                Ok(pipeline) => pipeline,
                Err(error) => {
                    eprintln!("runtime observability pipeline unavailable: {error}");
                    return ExitCode::from(78);
                }
            };
            recorder.set_observability_pipeline(observability.snapshot());
            let mut shutdown_signal = match ShutdownSignalReceiver::register() {
                Ok(signal) => signal,
                Err(error) => {
                    eprintln!("runtime signal handler registration failed: {error}");
                    return ExitCode::from(78);
                }
            };
            let listener = match tokio::net::TcpListener::bind(socket_addrs.as_slice()).await {
                Ok(listener) => listener,
                Err(error) => {
                    eprintln!("runtime control API bind failed: {error}");
                    return ExitCode::from(70);
                }
            };
            let mut handle = match Kernel::new(assembly.topology, recorder.clone())
                .start()
                .await
            {
                Ok(handle) => handle,
                Err(error) => {
                    eprintln!("runtime kernel failed to start: {error}");
                    return ExitCode::from(70);
                }
            };
            let (ready_sender, ready_receiver) = tokio::sync::oneshot::channel();
            let mut api = tokio::spawn(serve_control_listener_until_ready(
                service.clone(),
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
                adl_runtime_kernel::control_ready_event(
                    &instance_id,
                    bound_address,
                    &init.api.public_base_url,
                )
            );
            let mut pressure_retry_at = None;
            let mut observability_tick = tokio::time::interval(std::time::Duration::from_secs(1));
            observability_tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
            'serve: loop {
                if let Err(error) = observability.poll_health() {
                    recorder.set_observability_pipeline(observability.snapshot());
                    eprintln!("runtime observability pipeline failed: {error}");
                    api_shutdown.cancel();
                    let _ = handle.shutdown(std::time::Duration::from_secs(10)).await;
                    drain_control_api(&mut api).await;
                    break 'serve ExitCode::from(70);
                }
                recorder.set_observability_pipeline(observability.snapshot());
                let weather_service = service.clone();
                let pressure_delay = pressure_retry_at.take();
                let pressure_monitor = async {
                    if let Some(deadline) = pressure_delay {
                        tokio::time::sleep_until(deadline).await;
                    }
                    monitor_until_stop(
                        init.weather.clone(),
                        SysinfoWeatherObserver::for_path(&continuity_root),
                        move |report| weather_service.set_weather_report(report),
                    )
                    .await
                };
                tokio::pin!(pressure_monitor);
                let trigger = 'wait: loop {
                    tokio::select! {
                    pressure = &mut pressure_monitor => {
                        eprintln!("event=resource_pressure_stop state={:?} decision={:?}",
                            pressure.resource_state, pressure.shutdown_decision);
                        break 'wait TerminalTrigger::Pressure;
                    },
                    _ = observability_tick.tick() => {
                        if let Err(error) = observability.poll_health() {
                            recorder.set_observability_pipeline(observability.snapshot());
                            eprintln!("runtime observability pipeline failed: {error}");
                            api_shutdown.cancel();
                            let _ = handle.shutdown(std::time::Duration::from_secs(10)).await;
                            drain_control_api(&mut api).await;
                            break 'serve ExitCode::from(70);
                        }
                        recorder.set_observability_pipeline(observability.snapshot());
                    },
                    signal = shutdown_signal.recv() => {
                        if let Err(error) = signal {
                            eprintln!("runtime signal handler failed: {error}");
                            api_shutdown.cancel();
                            let _ = handle.shutdown(std::time::Duration::from_secs(10)).await;
                            drain_control_api(&mut api).await;
                            break 'serve ExitCode::from(70);
                        }
                        break 'wait TerminalTrigger::Signal;
                    },
                    request = shutdown_requests.recv() => {
                        let Some(request) = request else {
                            eprintln!("runtime checkpoint shutdown channel closed");
                            api_shutdown.cancel();
                            let _ = handle.shutdown(std::time::Duration::from_secs(10)).await;
                            drain_control_api(&mut api).await;
                            break 'serve ExitCode::from(70);
                        };
                        break 'wait TerminalTrigger::Signed(request);
                    },
                    exit = handle.wait_for_exit() => match exit {
                        Ok(exit) => {
                            api_shutdown.cancel();
                            drain_control_api(&mut api).await;
                            break 'serve process_exit(exit);
                        },
                        Err(error) => {
                            eprintln!("runtime kernel task failed: {error}");
                            api_shutdown.cancel();
                            let _ = handle.shutdown(std::time::Duration::from_secs(10)).await;
                            drain_control_api(&mut api).await;
                            break 'serve ExitCode::from(70);
                        }
                    },
                    result = &mut api => {
                        match result {
                            Ok(Ok(())) => eprintln!("runtime control API stopped unexpectedly"),
                            Ok(Err(error)) => eprintln!("runtime control API failed: {error}"),
                            Err(error) => eprintln!("runtime control API task failed: {error}"),
                        }
                        let _ = handle.shutdown(std::time::Duration::from_secs(10)).await;
                        break 'serve ExitCode::from(70);
                    },
                    };
                };

                let standard_deadline = std::time::Duration::from_secs(5);
                let shutdown_grace = std::time::Duration::from_secs(10);
                let (label, deadline, grace, retry_pressure, mut request) = match trigger {
                    TerminalTrigger::Pressure => (
                        "pressure",
                        pressure_checkpoint_deadline,
                        shutdown_grace,
                        true,
                        None,
                    ),
                    TerminalTrigger::Signal => {
                        ("signal", standard_deadline, shutdown_grace, false, None)
                    }
                    TerminalTrigger::Signed(request) => (
                        "signed",
                        standard_deadline,
                        request.grace,
                        false,
                        Some(request),
                    ),
                };
                let terminal_result = service
                    .serialize_terminal_checkpoint(&mut continuity, deadline)
                    .await;
                if let Err(error) = terminal_result {
                    if retry_pressure {
                        eprintln!("runtime pressure continuity checkpoint failed: {error}");
                        if !service.reopen_admission_if_no_terminal() {
                            eprintln!(
                                "event=resource_pressure_wait reason=terminal_request_pending"
                            );
                        }
                        pressure_retry_at = Some(
                            tokio::time::Instant::now()
                                + std::time::Duration::from_millis(init.weather.sample_millis),
                        );
                        continue 'serve;
                    }
                    eprintln!("runtime {label} terminal serialization failed: {error}");
                    if let Some(request) = request.take() {
                        request.respond(Err(()));
                    }
                    api_shutdown.cancel();
                    let _ = handle.shutdown(std::time::Duration::from_secs(10)).await;
                    drain_control_api(&mut api).await;
                    break 'serve ExitCode::from(74);
                }

                api_shutdown.cancel();
                let shutdown = handle.shutdown(grace).await;
                let terminal = match shutdown {
                    Ok(exit) => {
                        if let Some(request) = request.take() {
                            request.respond(Ok(exit.clone()));
                        }
                        process_exit(exit)
                    }
                    Err(error) => {
                        eprintln!("runtime {label} shutdown failed: {error}");
                        if let Some(request) = request.take() {
                            request.respond(Err(()));
                        }
                        ExitCode::from(70)
                    }
                };
                drain_control_api(&mut api).await;
                break 'serve terminal;
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
                "usage: adl-runtime-kernel [serve [--init path] [--continuity-root path]|demo [capsule-path]|shadow-loop|fatal-once [capsule-path]]"
            );
            ExitCode::from(64)
        }
    }
}

enum TerminalTrigger {
    Pressure,
    Signal,
    Signed(CheckpointShutdownRequest),
}

struct ServeArgs {
    continuity_root: PathBuf,
    init_path: Option<PathBuf>,
}

impl ServeArgs {
    fn parse(args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut init_path = None;
        let mut continuity_root = None;
        let mut args = args.peekable();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--init" => {
                    let Some(path) = args.next() else {
                        return Err("--init requires a runtime init file path".to_owned());
                    };
                    init_path = Some(PathBuf::from(path));
                }
                "--continuity-root" | "--capsule" => {
                    let Some(path) = args.next() else {
                        return Err("--continuity-root requires a checkpoint directory".to_owned());
                    };
                    continuity_root = Some(PathBuf::from(path));
                }
                other if other.starts_with('-') => {
                    return Err(format!("unknown serve option: {other}"));
                }
                path => {
                    if continuity_root.is_some() {
                        return Err("serve accepts only one continuity root".to_owned());
                    }
                    continuity_root = Some(PathBuf::from(path));
                }
            }
        }
        Ok(Self {
            continuity_root: continuity_root
                .unwrap_or_else(|| PathBuf::from(".adl/runtime-v3/continuity")),
            init_path,
        })
    }
}

fn capsule_arg(mut args: impl Iterator<Item = String>) -> PathBuf {
    args.next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("adl-runtime-kernel-continuity.json"))
}

struct ShutdownSignalReceiver {
    ctrl_c: Pin<Box<dyn Future<Output = std::io::Result<()>>>>,
    #[cfg(unix)]
    terminate: tokio::signal::unix::Signal,
}

impl ShutdownSignalReceiver {
    fn register() -> std::io::Result<Self> {
        Ok(Self {
            ctrl_c: Box::pin(tokio::signal::ctrl_c()),
            #[cfg(unix)]
            terminate: tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?,
        })
    }

    async fn recv(&mut self) -> std::io::Result<()> {
        #[cfg(unix)]
        {
            tokio::select! {
                result = &mut self.ctrl_c => result,
                _ = self.terminate.recv() => Ok(()),
            }
        }
        #[cfg(not(unix))]
        {
            (&mut self.ctrl_c).await
        }
    }
}

async fn file_hash(path: &std::path::Path) -> std::io::Result<String> {
    Ok(blake3::hash(&tokio::fs::read(path).await?)
        .to_hex()
        .to_string())
}

fn runtime_revision() -> String {
    std::env::var("ADL_RUNTIME_REVISION")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_owned())
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
