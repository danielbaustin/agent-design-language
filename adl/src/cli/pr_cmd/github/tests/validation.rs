use super::*;
use crate::cli::pr_cmd::github::transport::pr_validation_effective_report_from_snapshot_with_disposition;
use crate::cli::pr_cmd::github::transport::pr_validation_report_from_snapshot_with_disposition;
use crate::cli::pr_cmd::github::transport::pr_validation_wait_disposition_is_terminal;

#[test]
fn pr_validation_wait_classifies_pending_failed_successful_and_skipped_states() {
    let snapshot = |is_draft: bool, checks: Vec<PrValidationCheckSnapshot>| PrValidationSnapshot {
        pr_number: 1159,
        commit_sha: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        state: "OPEN".to_string(),
        is_draft,
        checks,
    };

    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(
            false,
            vec![PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "IN_PROGRESS".to_string(),
                conclusion: "UNKNOWN".to_string(),
                job_run_id: "8801".to_string(),
            }]
        )),
        PrValidationDisposition::Pending
    );
    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(
            false,
            vec![PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "COMPLETED".to_string(),
                conclusion: "FAILURE".to_string(),
                job_run_id: "8801".to_string(),
            }]
        )),
        PrValidationDisposition::Failed
    );
    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(
            false,
            vec![PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "COMPLETED".to_string(),
                conclusion: "SUCCESS".to_string(),
                job_run_id: "8801".to_string(),
            }]
        )),
        PrValidationDisposition::Success
    );
    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(false, Vec::new())),
        PrValidationDisposition::Pending
    );
    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(
            false,
            vec![PrValidationCheckSnapshot {
                name: "optional-lane".to_string(),
                status: "COMPLETED".to_string(),
                conclusion: "SKIPPED".to_string(),
                job_run_id: "8803".to_string(),
            }]
        )),
        PrValidationDisposition::Skipped
    );
    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(
            false,
            vec![PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "COMPLETED".to_string(),
                conclusion: "CANCELLED".to_string(),
                job_run_id: "8801".to_string(),
            }]
        )),
        PrValidationDisposition::Cancelled
    );
    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(
            true,
            vec![PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "COMPLETED".to_string(),
                conclusion: "SUCCESS".to_string(),
                job_run_id: "8805".to_string(),
            }]
        )),
        PrValidationDisposition::Success
    );
    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(
            true,
            vec![PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "IN_PROGRESS".to_string(),
                conclusion: "UNKNOWN".to_string(),
                job_run_id: "8806".to_string(),
            }]
        )),
        PrValidationDisposition::Pending
    );
    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(
            true,
            vec![PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "COMPLETED".to_string(),
                conclusion: "FAILURE".to_string(),
                job_run_id: "8807".to_string(),
            }]
        )),
        PrValidationDisposition::Failed
    );
    assert!(
        !pr_validation_wait_disposition_is_terminal(
            &snapshot(
                true,
                vec![PrValidationCheckSnapshot {
                    name: "adl-ci".to_string(),
                    status: "COMPLETED".to_string(),
                    conclusion: "SUCCESS".to_string(),
                    job_run_id: "8808".to_string(),
                }]
            ),
            PrValidationDisposition::Success,
        ),
        "green draft PRs must stay non-terminal for validation wait paths"
    );
    assert!(
        pr_validation_wait_disposition_is_terminal(
            &snapshot(
                false,
                vec![PrValidationCheckSnapshot {
                    name: "adl-ci".to_string(),
                    status: "COMPLETED".to_string(),
                    conclusion: "SUCCESS".to_string(),
                    job_run_id: "8809".to_string(),
                }]
            ),
            PrValidationDisposition::Success,
        ),
        "ready PRs with green checks should stay terminal"
    );

    let merged_success = PrValidationSnapshot {
        state: "MERGED".to_string(),
        checks: vec![PrValidationCheckSnapshot {
            name: "adl-ci".to_string(),
            status: "COMPLETED".to_string(),
            conclusion: "SUCCESS".to_string(),
            job_run_id: "8804".to_string(),
        }],
        ..snapshot(false, Vec::new())
    };
    assert_eq!(
        classify_pr_validation_snapshot(&merged_success),
        PrValidationDisposition::Success
    );
}

#[test]
fn pr_validation_report_exposes_projection_status_for_pr_lifecycle_states() {
    let snapshot = |is_draft: bool, checks: Vec<PrValidationCheckSnapshot>| PrValidationSnapshot {
        pr_number: 1159,
        commit_sha: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        state: "OPEN".to_string(),
        is_draft,
        checks,
    };
    let completed = |conclusion: &str| PrValidationCheckSnapshot {
        name: "adl-ci".to_string(),
        status: "COMPLETED".to_string(),
        conclusion: conclusion.to_string(),
        job_run_id: "8801".to_string(),
    };

    let pending = pr_validation_report_from_snapshot_with_disposition(
        &snapshot(
            false,
            vec![PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "IN_PROGRESS".to_string(),
                conclusion: "UNKNOWN".to_string(),
                job_run_id: "8801".to_string(),
            }],
        ),
        PrValidationDisposition::Pending,
    );
    assert_eq!(pending.disposition, "pending");
    assert_eq!(pending.projection_status, "checks_pending");

    let failed = pr_validation_report_from_snapshot_with_disposition(
        &snapshot(false, vec![completed("FAILURE")]),
        PrValidationDisposition::Failed,
    );
    assert_eq!(failed.projection_status, "checks_failed");

    let green_draft = pr_validation_report_from_snapshot_with_disposition(
        &snapshot(true, vec![completed("SUCCESS")]),
        classify_pr_validation_snapshot(&snapshot(true, vec![completed("SUCCESS")])),
    );
    assert_eq!(green_draft.disposition, "success");
    assert_eq!(green_draft.projection_status, "checks_green_but_draft");

    let green_ready = pr_validation_report_from_snapshot_with_disposition(
        &snapshot(false, vec![completed("SUCCESS")]),
        PrValidationDisposition::Success,
    );
    assert_eq!(green_ready.projection_status, "ready_to_merge_or_review");

    let pending_draft = pr_validation_report_from_snapshot_with_disposition(
        &snapshot(
            true,
            vec![PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "IN_PROGRESS".to_string(),
                conclusion: "UNKNOWN".to_string(),
                job_run_id: "8802".to_string(),
            }],
        ),
        classify_pr_validation_snapshot(&snapshot(
            true,
            vec![PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "IN_PROGRESS".to_string(),
                conclusion: "UNKNOWN".to_string(),
                job_run_id: "8802".to_string(),
            }],
        )),
    );
    assert_eq!(pending_draft.disposition, "pending");
    assert_eq!(pending_draft.projection_status, "checks_pending");
}

#[test]
fn pr_validation_classification_uses_latest_logical_check_state() {
    let snapshot = |checks: Vec<PrValidationCheckSnapshot>| PrValidationSnapshot {
        pr_number: 3933,
        commit_sha: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string(),
        state: "MERGED".to_string(),
        is_draft: false,
        checks,
    };

    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(vec![
            PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "COMPLETED".to_string(),
                conclusion: "CANCELLED".to_string(),
                job_run_id: "9101".to_string(),
            },
            PrValidationCheckSnapshot {
                name: "adl-coverage".to_string(),
                status: "COMPLETED".to_string(),
                conclusion: "CANCELLED".to_string(),
                job_run_id: "9102".to_string(),
            },
            PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "COMPLETED".to_string(),
                conclusion: "SUCCESS".to_string(),
                job_run_id: "9201".to_string(),
            },
            PrValidationCheckSnapshot {
                name: "adl-coverage".to_string(),
                status: "COMPLETED".to_string(),
                conclusion: "SUCCESS".to_string(),
                job_run_id: "9202".to_string(),
            },
        ])),
        PrValidationDisposition::Success
    );

    let reversed_rollup = snapshot(vec![
        PrValidationCheckSnapshot {
            name: "adl-ci".to_string(),
            status: "COMPLETED".to_string(),
            conclusion: "SUCCESS".to_string(),
            job_run_id: "9201".to_string(),
        },
        PrValidationCheckSnapshot {
            name: "adl-coverage".to_string(),
            status: "COMPLETED".to_string(),
            conclusion: "SUCCESS".to_string(),
            job_run_id: "9202".to_string(),
        },
        PrValidationCheckSnapshot {
            name: "adl-ci".to_string(),
            status: "COMPLETED".to_string(),
            conclusion: "CANCELLED".to_string(),
            job_run_id: "9101".to_string(),
        },
        PrValidationCheckSnapshot {
            name: "adl-coverage".to_string(),
            status: "COMPLETED".to_string(),
            conclusion: "CANCELLED".to_string(),
            job_run_id: "9102".to_string(),
        },
    ]);
    assert_eq!(
        classify_pr_validation_snapshot(&reversed_rollup),
        PrValidationDisposition::Success
    );
    let report = pr_validation_report_from_snapshot_with_disposition(
        &reversed_rollup,
        classify_pr_validation_snapshot(&reversed_rollup),
    );
    assert_eq!(report.disposition, "success");
    assert_eq!(report.checks.len(), 4);
    assert!(
        report.failed_checks.is_empty(),
        "stale duplicate failures must not remain in failed_checks: {:#?}",
        report.failed_checks
    );
    assert!(
        report.pending_checks.is_empty(),
        "stale duplicate pending checks must not remain in pending_checks: {:#?}",
        report.pending_checks
    );
    let inventory_report = pr_validation_effective_report_from_snapshot_with_disposition(
        &reversed_rollup,
        classify_pr_validation_snapshot(&reversed_rollup),
    );
    assert_eq!(
        inventory_report.checks.len(),
        2,
        "inventory reports must expose only effective checks"
    );

    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(vec![
            PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "COMPLETED".to_string(),
                conclusion: "SUCCESS".to_string(),
                job_run_id: "9201".to_string(),
            },
            PrValidationCheckSnapshot {
                name: "adl-coverage".to_string(),
                status: "COMPLETED".to_string(),
                conclusion: "FAILURE".to_string(),
                job_run_id: "9202".to_string(),
            },
        ])),
        PrValidationDisposition::Failed
    );
}

#[test]
fn pr_validation_wait_emits_tail_friendly_events_for_terminal_and_pending_states() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-validation-wait-log");
    let log_path = temp.join("events.log");
    unsafe {
        std::env::set_var("ADL_OBSERVABILITY_STDERR", "0");
        std::env::set_var(
            "ADL_OBSERVABILITY_LOG",
            log_path.to_str().expect("log path utf8"),
        );
    }
    let snapshot = PrValidationSnapshot {
        pr_number: 1159,
        commit_sha: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        state: "OPEN".to_string(),
        is_draft: false,
        checks: vec![
            PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "IN_PROGRESS".to_string(),
                conclusion: "UNKNOWN".to_string(),
                job_run_id: "8801".to_string(),
            },
            PrValidationCheckSnapshot {
                name: "adl-coverage".to_string(),
                status: "COMPLETED".to_string(),
                conclusion: "FAILURE".to_string(),
                job_run_id: "8802".to_string(),
            },
        ],
    };

    emit_pr_validation_wait_snapshot(
        &snapshot,
        PrValidationDisposition::Failed,
        Instant::now(),
        3,
        Duration::from_millis(250),
    );
    let success = PrValidationSnapshot {
        checks: vec![PrValidationCheckSnapshot {
            name: "adl-ci".to_string(),
            status: "COMPLETED".to_string(),
            conclusion: "SUCCESS".to_string(),
            job_run_id: "8801".to_string(),
        }],
        ..snapshot.clone()
    };
    emit_pr_validation_wait_snapshot(
        &success,
        PrValidationDisposition::Success,
        Instant::now(),
        5,
        Duration::ZERO,
    );
    emit_pr_validation_wait_timeout(&snapshot, Instant::now(), 4, Duration::ZERO);
    let no_checks_pending = PrValidationSnapshot {
        checks: Vec::new(),
        ..snapshot.clone()
    };
    emit_pr_validation_wait_snapshot(
        &no_checks_pending,
        PrValidationDisposition::Pending,
        Instant::now(),
        1,
        Duration::from_millis(250),
    );
    let draft_no_checks = PrValidationSnapshot {
        is_draft: true,
        checks: Vec::new(),
        ..snapshot.clone()
    };
    emit_pr_validation_wait_snapshot(
        &draft_no_checks,
        PrValidationDisposition::Pending,
        Instant::now(),
        2,
        Duration::from_millis(250),
    );

    let log = fs::read_to_string(&log_path).expect("read validation wait log");
    assert!(log.contains("stage=pr.validation.wait"));
    assert!(log.contains("result=pending"));
    assert!(log.contains("result=success"));
    assert!(log.contains("result=failed"));
    assert!(log.contains("result=timed_out"));
    assert!(log.contains("pr_number=1159"));
    assert!(log.contains("commit_sha=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
    assert!(log.contains("check_name=adl-ci"));
    assert!(log.contains("job_run_id=8801"));
    assert!(log.contains("pr_state=OPEN"));
    assert!(log.contains("is_draft=true"));
    assert!(log.contains("wait_reason=pr_draft"));
    assert!(log.contains("wait_reason=checks_not_reported"));
    assert!(log.contains("poll_count=3"));
    assert!(log.contains("next_poll_delay_ms=250"));

    unsafe {
        std::env::remove_var("ADL_OBSERVABILITY_STDERR");
        std::env::remove_var("ADL_OBSERVABILITY_LOG");
    }
}

#[test]
fn pr_validation_status_query_retries_transient_transport_and_returns_success_snapshot() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let temp = unique_temp_dir("adl-pr-validation-status-retry");
    let log_path = temp.join("events.log");
    let (base_uri, server) = spawn_validation_status_transient_server();
    unsafe {
        std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
        std::env::set_var("GITHUB_TOKEN", "test-token");
        std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
        std::env::set_var("ADL_GITHUB_OCTOCRAB_MAX_ATTEMPTS", "2");
        std::env::set_var("ADL_OBSERVABILITY_STDERR", "0");
        std::env::set_var(
            "ADL_OBSERVABILITY_LOG",
            log_path.to_str().expect("log path utf8"),
        );
    }

    let snapshot = pr_validation_status_octocrab("owner/repo", "1159")
        .expect("validation status snapshot after retry");
    assert_eq!(snapshot.pr_number, 1159);
    assert_eq!(
        snapshot.commit_sha,
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    );
    assert_eq!(snapshot.checks.len(), 1);
    assert_eq!(snapshot.checks[0].name, "adl-ci");
    assert_eq!(snapshot.checks[0].status, "COMPLETED");
    assert_eq!(snapshot.checks[0].conclusion, "SUCCESS");
    assert_eq!(snapshot.checks[0].job_run_id, "8801");

    let log = fs::read_to_string(&log_path).expect("read validation status log");
    assert!(log.contains("stage=github_octocrab"));
    assert!(log.contains("operation=pr.validation.status"));
    assert!(log.contains("result=retry"));
    assert!(log.contains("result=completed"));
    let seen = server.join().expect("server join");
    assert_eq!(
        seen.len(),
        2,
        "unexpected validation status calls: {seen:#?}"
    );
    assert!(seen.iter().all(|call| call.starts_with("POST /graphql ")));

    unsafe {
        std::env::remove_var("ADL_GITHUB_OCTOCRAB_MAX_ATTEMPTS");
        std::env::remove_var("ADL_OBSERVABILITY_STDERR");
        std::env::remove_var("ADL_OBSERVABILITY_LOG");
    }
    restore_github_policy_env(policy_env);
}

#[test]
fn pr_validation_wait_reports_folded_slow_anomaly_packets_with_safe_body_files() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let temp = unique_temp_dir("adl-pr-validation-anomaly-report");
    let reports_dir = temp.join("reports");
    let (base_uri, server) = spawn_validation_status_pending_then_success_server();
    unsafe {
        std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
        std::env::set_var("GITHUB_TOKEN", "test-token");
        std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
        std::env::set_var("ADL_REPORT_TOOLING_ANOMALIES", "1");
        std::env::set_var("ADL_PR_VALIDATION_WAIT_POLL_MS", "1");
        std::env::set_var("ADL_PR_VALIDATION_ANOMALY_THRESHOLD_MS", "0");
        std::env::set_var("ADL_TOOLING_ANOMALY_REPORT_DIR", &reports_dir);
    }

    let report =
        wait_for_pr_validation_report("owner/repo", "1159").expect("validation wait report");
    assert_eq!(report.disposition, "success");

    let mut packets = fs::read_dir(&reports_dir)
        .expect("read reports dir")
        .map(|entry| entry.expect("entry").path())
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("json"))
        .collect::<Vec<_>>();
    packets.sort();
    assert_eq!(packets.len(), 1, "expected one folded anomaly packet");

    let packet: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&packets[0]).expect("read anomaly packet"))
            .expect("parse anomaly packet");
    assert_eq!(packet["schema"], "adl.tooling_anomaly.v1");
    assert_eq!(packet["anomaly_type"], "pr_validation_slow");
    assert_eq!(packet["pr_number"], 1159);
    assert_eq!(packet["disposition"], "success");
    assert_eq!(packet["observation_count"], 2);
    assert_eq!(packet["linked_issue_numbers"][0], 4493);

    let issue_body_relpath = packet["issue_body_relpath"]
        .as_str()
        .expect("issue body relpath");
    let issue_body_path = reports_dir.join(
        PathBuf::from(issue_body_relpath)
            .file_name()
            .expect("issue body filename"),
    );
    let issue_body = fs::read_to_string(&issue_body_path).expect("read issue body");
    assert!(issue_body.contains("--body-file"));
    assert!(issue_body.contains("Machine-readable packet"));
    assert!(issue_body.contains("`adl-ci`"));

    let seen = server.join().expect("server join");
    assert!(
        seen.iter()
            .any(|call| call.contains("closingIssuesReferences")),
        "anomaly reporting should resolve linked issues: {seen:#?}"
    );

    unsafe {
        std::env::remove_var("ADL_GITHUB_OCTOCRAB_BASE_URI");
        std::env::remove_var("ADL_REPORT_TOOLING_ANOMALIES");
        std::env::remove_var("ADL_PR_VALIDATION_WAIT_POLL_MS");
        std::env::remove_var("ADL_PR_VALIDATION_ANOMALY_THRESHOLD_MS");
        std::env::remove_var("ADL_TOOLING_ANOMALY_REPORT_DIR");
    }
    restore_github_policy_env(policy_env);
}

#[test]
fn pr_validation_wait_keeps_validation_result_when_anomaly_capture_fails() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let temp = unique_temp_dir("adl-pr-validation-anomaly-failure");
    let blocker = temp.join("not-a-directory");
    let log_path = temp.join("events.log");
    fs::write(&blocker, "occupied").expect("write blocking file");
    let report_dir = blocker.join("reports");
    let (base_uri, server) =
        spawn_validation_status_once_server("COMPLETED", Some("SUCCESS"), "adl-ci");
    unsafe {
        std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
        std::env::set_var("GITHUB_TOKEN", "test-token");
        std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
        std::env::set_var("ADL_REPORT_TOOLING_ANOMALIES", "1");
        std::env::set_var("ADL_PR_VALIDATION_ANOMALY_THRESHOLD_MS", "0");
        std::env::set_var("ADL_TOOLING_ANOMALY_REPORT_DIR", &report_dir);
        std::env::set_var("ADL_OBSERVABILITY_STDERR", "0");
        std::env::set_var("ADL_OBSERVABILITY_LOG", &log_path);
    }

    let report =
        wait_for_pr_validation_report("owner/repo", "1159").expect("validation result should win");
    assert_eq!(report.disposition, "success");
    assert!(
        !report_dir.exists(),
        "broken anomaly sink should not materialize report dir"
    );

    let log = fs::read_to_string(&log_path).expect("read observability log");
    assert!(log.contains("stage=pr.validation.wait.anomaly_capture"));
    assert!(log.contains("result=failed"));
    assert!(log.contains("detail=create tooling anomaly report dir"));
    assert!(log.contains("stage=pr.validation.wait"));

    let seen = server.join().expect("server join");
    assert_eq!(
        seen.len(),
        1,
        "unexpected validation-status calls: {seen:#?}"
    );

    unsafe {
        std::env::remove_var("ADL_GITHUB_OCTOCRAB_BASE_URI");
        std::env::remove_var("ADL_REPORT_TOOLING_ANOMALIES");
        std::env::remove_var("ADL_PR_VALIDATION_ANOMALY_THRESHOLD_MS");
        std::env::remove_var("ADL_TOOLING_ANOMALY_REPORT_DIR");
        std::env::remove_var("ADL_OBSERVABILITY_STDERR");
        std::env::remove_var("ADL_OBSERVABILITY_LOG");
    }
    restore_github_policy_env(policy_env);
}
