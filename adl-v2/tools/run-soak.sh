#!/usr/bin/env bash
set -euo pipefail

usage() {
  printf 'usage: %s (--manifest PATH | --verify-report PATH | --verify-platform PATH | --post-merge PATH)\n' "$0" >&2
  exit 64
}

mode=
input=
while (($#)); do
  case "$1" in
    --manifest)
      [[ -z "$mode" && $# -ge 2 ]] || usage
      mode=run
      input=$2
      shift 2
      ;;
    --verify-report)
      [[ -z "$mode" && $# -ge 2 ]] || usage
      mode=verify
      input=$2
      shift 2
      ;;
    --verify-platform)
      [[ -z "$mode" && $# -ge 2 ]] || usage
      mode=verify_platform
      input=$2
      shift 2
      ;;
    --post-merge)
      [[ -z "$mode" && $# -ge 2 ]] || usage
      mode=post_merge
      input=$2
      shift 2
      ;;
    *)
      usage
      ;;
  esac
done
[[ -n "$mode" && -n "$input" ]] || usage

root=$(git rev-parse --show-toplevel)
cd "$root"
[[ "$input" != /* && "$input" != *".."* && -f "$input" ]] ||
  { printf 'WP-12 input must be a regular repo-relative path\n' >&2; exit 65; }

sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

verify_retained_file() {
  local path=$1 expected_sha256=$2
  [[ "$path" != /* && "$path" != *".."* && -f "$path" ]] || return 1
  [[ "$(sha256 "$path")" == "$expected_sha256" ]]
}

verify_platform_log() {
  local proof=$1 suite=$2
  local platform revision log_ref log_sha256 log_records audit_ref audit_sha256 audit_suite
  audit_suite=$suite
  if [[ "$suite" == preflight ]]; then
    audit_suite=preflight_1x
  fi
  platform=$(jq -r '.native_os // .platform' "$proof")
  revision=$(jq -r '.lifecycle_acceptance.revision' "$proof")
  log_ref=$(jq -r --arg suite "$suite" \
    '.lifecycle_acceptance[$suite].master_log_ref' "$proof")
  log_sha256=$(jq -r --arg suite "$suite" \
    '.lifecycle_acceptance[$suite].master_log_sha256' "$proof")
  log_records=$(jq -r --arg suite "$suite" \
    '.lifecycle_acceptance[$suite].master_log_records' "$proof")
  audit_ref=$(jq -r --arg suite "$suite" \
    '.lifecycle_acceptance[$suite].log_audit_ref' "$proof")
  audit_sha256=$(jq -r --arg suite "$suite" \
    '.lifecycle_acceptance[$suite].log_audit_sha256' "$proof")

  verify_retained_file "$log_ref" "$log_sha256"
  jq -s -e --argjson records "$log_records" \
    '
      all(.[]; type == "object" and (.sequence | type == "number")) and
      (
        sort_by(.sequence)
        | group_by(.sequence)
        | length == $records and
          all(.[];
            (map(tojson) | unique | length) == 1
          )
      )
    ' "$log_ref" >/dev/null

  verify_retained_file "$audit_ref" "$audit_sha256"
  jq -e \
    --arg platform "$platform" \
    --arg suite "$suite" \
    --arg audit_suite "$audit_suite" \
    --arg revision "$revision" \
    --arg log_sha256 "$log_sha256" \
    --argjson records "$log_records" '
      .schema == "adl.runtime.master_log_audit.v1" and
      .status == "pass" and
      .platform == $platform and
      .suite == $audit_suite and
      .revision == $revision and
      ((.master_log_sha256 // $log_sha256) == $log_sha256) and
      .record_count == $records and
      .malformed_records == 0 and
      .missing_required_fields == 0 and
      .sequence_gaps == 0 and
      .error_events == 0 and
      .degraded_events == 0 and
      .unexplained_restarts == 0 and
      .incomplete_drains == 0
    ' "$audit_ref" >/dev/null
}

verify_platform_proof() {
  local path=$1
  jq -e '
    .schema == "adl.wp12.platform_proof.v1" and
    .issue == 5344 and
    .status == "pass" and
    (.platform | IN("macos-arm64", "linux-x86_64", "windows-x86_64-msvc")) and
    .guardian_process_zero == true and
    .native_execution == true and
    (if .platform == "windows-x86_64-msvc"
      then .wsl_used == false and .docker_used == false
      else true
    end) and
    (.lifecycle_acceptance.revision | test("^[0-9a-f]{40}$")) and
    (.lifecycle_acceptance.kernel_sha256 | test("^[0-9a-f]{64}$")) and
    .lifecycle_acceptance.all_logs_clean == true and
    (.lifecycle_acceptance.preflight |
      .status == "pass" and
      .requested_cycles == 1 and
      .completed_cycles == 1 and
      .failed_cycles == 0 and
      .degraded_cycles == 0 and
      .acceptance_eligible == false and
      .logging_complete == true and
      .master_log_status == "clean" and
      (.master_log_ref | type == "string" and length > 0) and
      (.master_log_sha256 | test("^[0-9a-f]{64}$")) and
      (.master_log_records | type == "number" and . > 0) and
      (.log_audit_ref | type == "string" and length > 0) and
      (.log_audit_sha256 | test("^[0-9a-f]{64}$"))) and
    (.lifecycle_acceptance.lifecycle_10000 |
      .status == "pass" and
      .requested_cycles == 10000 and
      .completed_cycles == 10000 and
      .failed_cycles == 0 and
      .degraded_cycles == 0 and
      .logging_complete == true and
      .master_log_status == "clean" and
      (.master_log_ref | type == "string" and length > 0) and
      (.master_log_sha256 | test("^[0-9a-f]{64}$")) and
      (.master_log_records | type == "number" and . > 0) and
      (.log_audit_ref | type == "string" and length > 0) and
      (.log_audit_sha256 | test("^[0-9a-f]{64}$"))) and
    (.lifecycle_acceptance.stress_100x10s as $suite |
      $suite |
      .status == "pass" and
      .requested_runs == 100 and
      .completed_runs == 100 and
      .duration_seconds_per_run == 10 and
      .failed_cycles == 0 and
      .degraded_cycles == 0 and
      .logging_complete == true and
      .master_log_status == "clean" and
      (.master_log_ref | type == "string" and length > 0) and
      (.master_log_sha256 | test("^[0-9a-f]{64}$")) and
      (.master_log_records | type == "number" and . > 0) and
      (.log_audit_ref | type == "string" and length > 0) and
      (.log_audit_sha256 | test("^[0-9a-f]{64}$"))) and
    (.lifecycle_acceptance.endurance_10x600s as $suite |
      $suite |
      .status == "pass" and
      .requested_runs == 10 and
      .completed_runs == 10 and
      .duration_seconds_per_run == 600 and
      .failed_cycles == 0 and
      .degraded_cycles == 0 and
      .logging_complete == true and
      .master_log_status == "clean" and
      (.master_log_ref | type == "string" and length > 0) and
      (.master_log_sha256 | test("^[0-9a-f]{64}$")) and
      (.master_log_records | type == "number" and . > 0) and
      (.log_audit_ref | type == "string" and length > 0) and
      (.log_audit_sha256 | test("^[0-9a-f]{64}$")))
  ' "$path" >/dev/null
  verify_platform_log "$path" preflight
  verify_platform_log "$path" lifecycle_10000
  verify_platform_log "$path" stress_100x10s
  verify_platform_log "$path" endurance_10x600s
}

verify_report() {
  local report=$1
  jq -e '
    .schema == "adl.wp12.soak_report.v1" and
    .issue == 5344 and
    .status == "pass" and
    (.revision | test("^[0-9a-f]{40}$")) and
    (.manifest_sha256 | test("^[0-9a-f]{64}$")) and
    (.results | type == "array" and length > 0) and
    (all(.results[]; .status == "pass")) and
    (.rollback.status == "pass") and
    (.rollback.exact_prior_bytes_restored == true) and
    (.rollback.fresh_install_receipt_ref | type == "string" and length > 0) and
    (.rollback.fresh_install_receipt_sha256 | test("^[0-9a-f]{64}$")) and
    (.default_generation_changed == false) and
    (.runtime_v2_edited == false) and
    (.deferred_acceptance == false)
  ' "$report" >/dev/null
  local receipt_ref receipt_sha256
  receipt_ref=$(jq -r '.rollback.fresh_install_receipt_ref' "$report")
  receipt_sha256=$(jq -r '.rollback.fresh_install_receipt_sha256' "$report")
  if [[ "$receipt_ref" == /* || "$receipt_ref" == *".."* || ! -f "$receipt_ref" ]]; then
    printf 'WP-12 fresh-install receipt is missing or non-portable\n' >&2
    exit 66
  fi
  [[ "$(sha256 "$receipt_ref")" == "$receipt_sha256" ]] || {
    printf 'WP-12 fresh-install receipt digest mismatch\n' >&2
    exit 66
  }
  jq -e --arg sha256 "$(jq -r '.rollback.v2_sha256' "$report")" '
    .schema == "adl.install.receipt.v1" and
    .binary == "adl-v2" and
    .sha256 == $sha256
  ' "$receipt_ref" >/dev/null
  if rg -n '(/Users/|/Volumes/|/private/|[A-Za-z]:\\\\Users\\\\)' "$report" >/dev/null; then
    printf 'WP-12 report contains a machine-local path\n' >&2
    exit 66
  fi
  printf '{"schema":"adl.wp12.report_verification.v1","issue":5344,"status":"pass","report_sha256":"%s"}\n' \
    "$(sha256 "$report")"
}

if [[ "$mode" == verify ]]; then
  verify_report "$input"
  exit 0
fi
if [[ "$mode" == verify_platform ]]; then
  verify_platform_proof "$input"
  exit 0
fi

manifest=$input
jq -e '
  .schema == "adl.wp12.soak_manifest.v1" and
  .issue == 5344 and
  (.scenarios | type == "array" and length == 5) and
  ([.scenarios[].id] == ([.scenarios[].id] | sort)) and
  (all(.scenarios[];
    (.id | test("^[a-z0-9][a-z0-9-]+$")) and
    (.claim_class | IN("runtime_v3_acceptance","runtime_v3_live","rollback")) and
    (.timeout_seconds | type == "number" and . >= 1 and . <= 1800) and
    (.expected_exit | type == "number")
  )) and
  (all(.scenarios[];
    if .kind == "runtime_acceptance" then
      (.landing_revision | test("^[0-9a-f]{40}$")) and
      (.artifact_sha256 | test("^[0-9a-f]{64}$"))
    else true end
  ))
' "$manifest" >/dev/null

manifest_sha256=$(sha256 "$manifest")
revision=$(git rev-parse HEAD)

work_parent="$root/.csdlc/evidence/5344/work"
mkdir -p "$work_parent"
run_root=$(mktemp -d "$work_parent/soak.XXXXXX")
trap 'rm -rf "$run_root"' EXIT
results="$run_root/results.jsonl"

run_bounded() {
  local timeout_seconds=$1
  shift
  ruby -e '
    timeout = Integer(ARGV.shift)
    pid = Process.spawn(*ARGV)
    deadline = Process.clock_gettime(Process::CLOCK_MONOTONIC) + timeout
    loop do
      waited = Process.waitpid(pid, Process::WNOHANG)
      exit($?.exitstatus || 1) if waited
      if Process.clock_gettime(Process::CLOCK_MONOTONIC) >= deadline
        Process.kill("TERM", pid) rescue nil
        sleep 0.2
        Process.kill("KILL", pid) rescue nil
        Process.wait(pid) rescue nil
        exit 124
      end
      sleep 0.02
    end
  ' "$timeout_seconds" "$@"
}

record_result() {
  local id=$1 claim_class=$2 status=$3 exit_code=$4 duration_ms=$5 artifact_sha256=$6
  jq -n -c \
    --arg id "$id" \
    --arg claim_class "$claim_class" \
    --arg status "$status" \
    --argjson exit_code "$exit_code" \
    --argjson duration_ms "$duration_ms" \
    --arg artifact_sha256 "$artifact_sha256" \
    '{
      id:$id,
      claim_class:$claim_class,
      status:$status,
      exit_code:$exit_code,
      duration_ms:$duration_ms,
      artifact_sha256:$artifact_sha256
    }' >>"$results"
}

scenario_count=$(jq '.scenarios | length' "$manifest")
for ((index=0; index<scenario_count; index++)); do
  scenario=$(jq -c ".scenarios[$index]" "$manifest")
  id=$(jq -r .id <<<"$scenario")
  kind=$(jq -r .kind <<<"$scenario")
  claim_class=$(jq -r .claim_class <<<"$scenario")
  timeout_seconds=$(jq -r .timeout_seconds <<<"$scenario")
  expected_exit=$(jq -r .expected_exit <<<"$scenario")
  output="$run_root/$id.out"
  error="$run_root/$id.err"
  started=$(ruby -e 'puts(Process.clock_gettime(Process::CLOCK_MONOTONIC, :millisecond))')
  set +e
  case "$kind" in
    artifact)
      path=$(jq -r .path <<<"$scenario")
      required_schema=$(jq -r .required_schema <<<"$scenario")
      required_status=$(jq -r .required_status <<<"$scenario")
      if [[ "$path" == /* || "$path" == *".."* || ! -f "$path" ]]; then
        exit_code=2
      elif [[ "$required_schema" == "adl.wp12.platform_proof.v1" &&
              "$claim_class" == "runtime_v3_live" ]]; then
        verify_platform_proof "$path" >"$output" 2>"$error"
        exit_code=$?
      else
        jq -e --arg schema "$required_schema" --arg status "$required_status" \
          '.schema == $schema and .status == $status' "$path" >"$output" 2>"$error"
        exit_code=$?
      fi
      ;;
    runtime_acceptance)
      path=$(jq -r .path <<<"$scenario")
      landing_revision=$(jq -r .landing_revision <<<"$scenario")
      expected_artifact_sha256=$(jq -r .artifact_sha256 <<<"$scenario")
      if [[ "$path" == /* || "$path" == *".."* || ! -f "$path" ]]; then
        exit_code=2
      else
        if jq -e '
            .schema == "adl.runtime_v3.acceptance.v1" and
            .issue == 5361 and
            (.revision | test("^[0-9a-f]{40}$")) and
            all(.dependency_proofs[]; .status == "integrated") and
            all(.consumer_proofs[]; .status == "passed") and
            all(.proofs[]; .status == "passed")
          ' "$path" >"$output" 2>"$error" &&
          [[ "$landing_revision" =~ ^[0-9a-f]{40}$ ]] &&
          [[ "$expected_artifact_sha256" =~ ^[0-9a-f]{64}$ ]] &&
          [[ "$(sha256 "$path")" == "$expected_artifact_sha256" ]] &&
          git merge-base --is-ancestor "$landing_revision" "$revision" &&
          [[ "$(git show "$landing_revision:$path" | shasum -a 256 | awk '{print $1}')" == "$expected_artifact_sha256" ]]; then
          exit_code=0
        else
          exit_code=2
        fi
      fi
      ;;
    rollback)
      run_bounded "$timeout_seconds" bash adl-v2/tools/prove-rollback.sh \
        --manifest "$manifest" >"$output" 2>"$error"
      exit_code=$?
      ;;
    *)
      printf 'unsupported scenario kind: %s\n' "$kind" >"$error"
      exit_code=2
      ;;
  esac
  set -e
  finished=$(ruby -e 'puts(Process.clock_gettime(Process::CLOCK_MONOTONIC, :millisecond))')
  duration_ms=$((finished - started))
  artifact_sha256=$(sha256 "$output")
  if [[ "$exit_code" -ne "$expected_exit" ]]; then
    printf 'scenario %s expected exit %s, observed %s\n' \
      "$id" "$expected_exit" "$exit_code" >&2
    sed -n '1,80p' "$error" >&2
    exit 68
  fi
  record_result "$id" "$claim_class" pass "$exit_code" "$duration_ms" "$artifact_sha256"
done

rollback=$(jq -c 'select(.schema == "adl.wp12.rollback_report.v1")' \
  "$run_root/rollback-matrix.out")
[[ -n "$rollback" ]] || { printf 'rollback report missing\n' >&2; exit 69; }

report_path=docs/milestones/v0.91.8/evidence/wp12/report.json
mkdir -p "$(dirname "$report_path")"
jq -s \
  --arg revision "$revision" \
  --arg manifest_sha256 "$manifest_sha256" \
  --arg mode "$mode" \
  --argjson rollback "$rollback" \
  '{
    schema:"adl.wp12.soak_report.v1",
    issue:5344,
    status:"pass",
    revision:$revision,
    manifest_sha256:$manifest_sha256,
    mode:$mode,
    results:sort_by(.id),
    rollback:$rollback,
    default_generation_changed:false,
    runtime_v2_edited:false,
    deferred_acceptance:false
  }' "$results" >"$report_path"

verify_report "$report_path" >/dev/null
cat "$report_path"
