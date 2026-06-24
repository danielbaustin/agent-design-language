use super::*;
use crate::cli::pr_cmd::github::transport::pr_validation_report_from_snapshot_with_disposition;

#[test]
fn pr_validation_wait_classifies_pending_failed_successful_and_skipped_states() {
    let snapshot = |checks: Vec<PrValidationCheckSnapshot>| PrValidationSnapshot {
        pr_number: 1159,
        commit_sha: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        state: "OPEN".to_string(),
        is_draft: false,
        checks,
    };

    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(vec![PrValidationCheckSnapshot {
            name: "adl-ci".to_string(),
            status: "IN_PROGRESS".to_string(),
            conclusion: "UNKNOWN".to_string(),
            job_run_id: "8801".to_string(),
        }])),
        PrValidationDisposition::Pending
    );
    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(vec![PrValidationCheckSnapshot {
            name: "adl-ci".to_string(),
            status: "COMPLETED".to_string(),
            conclusion: "FAILURE".to_string(),
            job_run_id: "8801".to_string(),
        }])),
        PrValidationDisposition::Failed
    );
    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(vec![PrValidationCheckSnapshot {
            name: "adl-ci".to_string(),
            status: "COMPLETED".to_string(),
            conclusion: "SUCCESS".to_string(),
            job_run_id: "8801".to_string(),
        }])),
        PrValidationDisposition::Success
    );
    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(Vec::new())),
        PrValidationDisposition::Pending
    );
    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(vec![PrValidationCheckSnapshot {
            name: "optional-lane".to_string(),
            status: "COMPLETED".to_string(),
            conclusion: "SKIPPED".to_string(),
            job_run_id: "8803".to_string(),
        }])),
        PrValidationDisposition::Skipped
    );
    assert_eq!(
        classify_pr_validation_snapshot(&snapshot(vec![PrValidationCheckSnapshot {
            name: "adl-ci".to_string(),
            status: "COMPLETED".to_string(),
            conclusion: "CANCELLED".to_string(),
            job_run_id: "8801".to_string(),
        }])),
        PrValidationDisposition::Cancelled
    );

    let merged_success = PrValidationSnapshot {
        state: "MERGED".to_string(),
        checks: vec![PrValidationCheckSnapshot {
            name: "adl-ci".to_string(),
            status: "COMPLETED".to_string(),
            conclusion: "SUCCESS".to_string(),
            job_run_id: "8804".to_string(),
        }],
        ..snapshot(Vec::new())
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
        PrValidationDisposition::Success,
    );
    assert_eq!(green_draft.projection_status, "checks_green_but_draft");

    let green_ready = pr_validation_report_from_snapshot_with_disposition(
        &snapshot(false, vec![completed("SUCCESS")]),
        PrValidationDisposition::Success,
    );
    assert_eq!(green_ready.projection_status, "ready_to_merge_or_review");
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
