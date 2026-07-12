use std::fs;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use adl_runtime::observability::{ObservabilityConfig, ObservabilityHealth, ObservabilityRuntime};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let epoch_ms = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_millis();
    let proof_root = PathBuf::from(format!(".adl/proofs/issue-5117/vector-local/{epoch_ms}"));
    fs::create_dir_all(&proof_root)?;

    let mut runtime =
        ObservabilityRuntime::start(ObservabilityConfig::from_runtime_environment(&proof_root));
    let startup = runtime.status();
    if startup.health != ObservabilityHealth::Ready {
        return Err(format!("Vector component did not become ready: {startup:?}").into());
    }

    #[cfg(unix)]
    let restart = {
        let first_pid = startup
            .vector_pid
            .ok_or("ready Vector component has no PID")?;
        unsafe {
            if libc::kill(first_pid as i32, libc::SIGKILL) != 0 {
                return Err("failed to inject Vector child exit".into());
            }
        }
        let exit_deadline = SystemTime::now() + Duration::from_secs(2);
        loop {
            let status = runtime.status();
            if status.health == ObservabilityHealth::Degraded && status.vector_pid.is_none() {
                break;
            }
            if SystemTime::now() >= exit_deadline {
                return Err(format!("Vector child exit was not observed: {status:?}").into());
            }
            sleep(Duration::from_millis(25));
        }
        runtime.append(
            "events",
            "audit",
            &json!({
                "proof": "issue-5117-replay-during-vector-outage",
                "authorization": "must-not-survive-redaction"
            }),
        )?;
        let restart_deadline = SystemTime::now() + Duration::from_secs(5);
        let restarted = loop {
            let status = runtime.status();
            if status.health == ObservabilityHealth::Ready
                && status.restart_count > 0
                && status.vector_pid.is_some_and(|pid| pid != first_pid)
            {
                break status;
            }
            if SystemTime::now() >= restart_deadline {
                return Err(format!("Vector child did not restart: {status:?}").into());
            }
            sleep(Duration::from_millis(100));
        };
        json!({
            "injected_exit": true,
            "first_pid": first_pid,
            "replacement_pid": restarted.vector_pid,
            "restart_count": restarted.restart_count
        })
    };

    #[cfg(not(unix))]
    let restart = json!({"injected_exit": false, "reason": "unsupported_platform"});

    for signal in ["logs", "metrics", "traces", "otel", "events"] {
        let payload = if signal == "otel" {
            json!({
                "resourceLogs": [{
                    "resource": {"attributes": [{
                        "key": "service.name",
                        "value": {"stringValue": "csm"}
                    }]},
                    "scopeLogs": [{
                        "scope": {"name": "csm-observability-proof"},
                        "logRecords": [{
                            "timeUnixNano": "1783800000000000000",
                            "severityText": "INFO",
                            "body": {"stringValue": "issue-5117-real-vector"},
                            "attributes": [{
                                "key": "authorization",
                                "value": {"stringValue": "<redacted>"}
                            }]
                        }]
                    }]
                }]
            })
        } else if signal == "metrics" {
            json!({
                "proof": "issue-5117-real-vector",
                "signal": signal,
                "name": "observability_proof",
                "namespace": "ADL/CSM",
                "value": 1.0,
                "authorization": "must-not-survive-redaction"
            })
        } else {
            json!({
                "proof": "issue-5117-real-vector",
                "signal": signal,
                "authorization": "must-not-survive-redaction"
            })
        };
        runtime.append(signal, "audit", &payload)?;
    }
    let delivery_deadline = SystemTime::now() + Duration::from_secs(10);
    loop {
        let all_present = ["logs", "metrics", "traces", "otel", "events"]
            .iter()
            .all(|signal| {
                proof_root
                    .join(format!("observability/durable/{signal}.jsonl"))
                    .is_file()
            });
        if all_present || SystemTime::now() >= delivery_deadline {
            break;
        }
        sleep(Duration::from_millis(250));
    }

    let mut retained = serde_json::Map::new();
    for signal in ["logs", "metrics", "traces", "otel", "events"] {
        let path = proof_root.join(format!("observability/durable/{signal}.jsonl"));
        let body = fs::read_to_string(&path)?;
        if !body.contains("issue-5117-real-vector") || body.contains("must-not-survive-redaction") {
            return Err(format!("invalid retained {signal} proof").into());
        }
        if signal == "events" && !body.contains("issue-5117-replay-during-vector-outage") {
            return Err("Vector replacement did not replay the outage event".into());
        }
        retained.insert(signal.to_string(), json!(path));
    }

    let live = runtime.status();
    runtime.shutdown();
    let stopped = runtime.status();
    let report = json!({
        "schema": "adl.csm.observability.vector_proof.v1",
        "issue": 5117,
        "proof_root": proof_root,
        "vector_version": live.vector_version,
        "startup_health": startup.health,
        "live_health": live.health,
        "stopped_health": stopped.health,
        "vector_pid_was_live": live.vector_pid.is_some(),
        "restart": restart,
        "outage_replay_proven": true,
        "config_validated": live.config_validated,
        "accepted_events": live.accepted_events,
        "redaction_before_egress": live.redaction_before_egress,
        "retained": retained,
        "cloud_delivery_claimed": live.live_cloud_delivery_proven
    });
    let report_path = proof_root.join("proof.json");
    fs::write(&report_path, serde_json::to_vec_pretty(&report)?)?;
    println!("{}", serde_json::to_string(&report)?);
    Ok(())
}
