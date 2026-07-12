use std::{
    io::{Read, Write},
    process::ExitCode,
    thread,
    time::Duration,
};

use adl_runtime_kernel::AdaptationState;

fn main() -> ExitCode {
    match std::env::args().nth(1).as_deref() {
        Some("hang") => {
            thread::sleep(Duration::from_secs(30));
            ExitCode::SUCCESS
        }
        #[cfg(unix)]
        #[allow(clippy::zombie_processes)]
        // Deliberate orphan used to prove harness group cleanup.
        Some("fork-and-exit") => {
            let child = std::process::Command::new("/bin/sleep")
                .arg("30")
                .spawn()
                .expect("spawn fixture descendant");
            println!("{}", child.id());
            ExitCode::SUCCESS
        }
        #[cfg(unix)]
        #[allow(clippy::zombie_processes)]
        Some("detached-stream-descendant") => {
            let marker = std::env::args().nth(2).expect("PID marker path");
            let child = std::process::Command::new("/bin/sleep")
                .arg("30")
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("spawn detached-stream fixture descendant");
            std::fs::write(marker, child.id().to_string()).expect("write descendant PID marker");
            println!(r#"{{"status":"ok"}}"#);
            ExitCode::SUCCESS
        }
        Some("oversized-file") => {
            let output = std::env::args().nth(2).expect("output path");
            std::fs::write(output, vec![b'x'; 1_048_576]).expect("write oversized fixture");
            ExitCode::SUCCESS
        }
        Some("json") => {
            println!(r#"{{"status":"ok"}}"#);
            ExitCode::SUCCESS
        }
        Some("duplex-pressure") => {
            let mut stdout = std::io::stdout().lock();
            stdout.write_all(br#"{"status":"ok"}"#).unwrap();
            stdout.write_all(&vec![b' '; 256 * 1024]).unwrap();
            stdout.flush().unwrap();
            std::io::stdin().read_to_end(&mut Vec::new()).unwrap();
            ExitCode::SUCCESS
        }
        Some("divergent-loop") => {
            let state = AdaptationState::new(7, "fixture-graph", "fixture-policy");
            let state_hash = state.hash().expect("fixture state hash");
            println!(
                "{}",
                serde_json::json!({
                    "schema": "adl.runtime.shadow_loop.v1",
                    "status": "converged",
                    "iterations": 3,
                    "terminal_node_id": "deny",
                    "exit_node_ids": ["decide"],
                    "replay": [1, 2, 3],
                    "state_hash": state_hash,
                    "state": state,
                    "evidence": ["bounded_loop", "deterministic_replay"]
                })
            );
            ExitCode::SUCCESS
        }
        _ => ExitCode::from(64),
    }
}
