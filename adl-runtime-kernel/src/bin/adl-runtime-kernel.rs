use std::{path::PathBuf, process::ExitCode};

use adl_runtime_kernel::{
    proof::{build_proof_runtime, load_capsule, run_proof},
    KernelExit,
};

#[tokio::main]
async fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "demo".to_owned());
    let capsule = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("adl-runtime-kernel-continuity.json"));

    match command.as_str() {
        "serve" => {
            let proof = match build_proof_runtime(&capsule, 3) {
                Ok(proof) => proof,
                Err(error) => {
                    eprintln!("runtime topology invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let mut handle = match proof.kernel.start().await {
                Ok(handle) => handle,
                Err(error) => {
                    eprintln!("runtime kernel failed to start: {error}");
                    return ExitCode::from(70);
                }
            };
            tokio::select! {
                signal = tokio::signal::ctrl_c() => {
                    if let Err(error) = signal {
                        eprintln!("runtime signal handler failed: {error}");
                        return ExitCode::from(70);
                    }
                    match handle.shutdown(std::time::Duration::from_secs(10)).await {
                        Ok(exit) => process_exit(exit),
                        Err(error) => {
                            eprintln!("runtime shutdown failed: {error}");
                            ExitCode::from(70)
                        }
                    }
                }
                exit = handle.wait_for_exit() => match exit {
                    Ok(exit) => process_exit(exit),
                    Err(error) => {
                        eprintln!("runtime kernel task failed: {error}");
                        ExitCode::from(70)
                    }
                }
            }
        }
        "demo" => match run_proof(&capsule, 3).await {
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
        },
        "fatal-once" => {
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
            eprintln!("usage: adl-runtime-kernel [serve|demo|fatal-once] [capsule-path]");
            ExitCode::from(64)
        }
    }
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
