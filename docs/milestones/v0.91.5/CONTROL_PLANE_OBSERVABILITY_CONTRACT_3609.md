# Control-Plane Observability Contract (#3609)

Issue: #3609
Status: implementation contract

## Purpose

ADL control-plane commands must tell operators what they are doing before they
block, mutate state, call GitHub, delegate to Rust, or fail. Silent waits are a
workflow defect because agents cannot distinguish useful work from a wedge.

## Event Shape

Shell and Rust command-dispatch events use one line per stage:

```text
adl_event schema=adl.observability.event.v1 command=<command> stage=<stage> result=<result> key=value ...
```

Required fields:

- `schema`: currently `adl.observability.event.v1`
- `command`: command family such as `pr.sh`, `adl-runtime`, or
  `attach_post_merge_closeout.sh`
- `stage`: bounded stage name such as `dispatch`, `doctor`, `rust_delegate`,
  `github_fetch`, `closeout`, or `watcher_attached`
- `result`: `started`, `exec`, `ok`, `blocked`, `skipped`, or `failed`

Optional fields may include `issue`, `subcommand`, `operation`, `branch`,
`delegate`, `log`, `summary`, `elapsed_ms`, and other bounded diagnostic keys.

## Redaction Rules

Events must not expose:

- raw credentials, secret markers, API keys, tokens, or private key material
- raw prompts or private tool arguments
- host-local absolute paths
- unbounded command output

Repo paths should be normalized to `<repo>/...`, home paths to `<home>/...`,
and temp or unrelated absolute paths to `<tmp>` or `<path>`.

## Terminal And Durable Logs

By default, shell and Rust dispatch events are visible on stderr. If
`ADL_OBSERVABILITY_LOG` is set, shell and Rust events may also be mirrored into
that compatibility log path.

If a machine-readable command needs completely quiet stderr while preserving
observability, set:

- `ADL_OBSERVABILITY_STDERR=0`
- `ADL_OBSERVABILITY_LOG=<durable-log-path>`

Under that explicit compatibility mode, the JSON payload remains stdout-only
while observability events continue to flow into the compatibility mirror file.

`ADL_OBSERVABILITY=0` disables event emission for compatibility tests that need
a completely quiet stderr surface.

## PR Validation Wait Runbook

When `pr.sh finish --merge` waits on GitHub PR validation, operators should be
able to understand the wait from `adl_event` lines alone. To keep stdout clean
while tailing the wait surface:

```bash
ADL_OBSERVABILITY_STDERR=0 \
ADL_OBSERVABILITY_LOG=.adl/logs/pr-validation-wait.log \
bash adl/tools/pr.sh finish <issue> --title "<title>" --paths "<paths>" --ready --merge

tail -f .adl/logs/pr-validation-wait.log | rg 'stage=pr.validation.wait|operation=pr.validation.status'
```

The wait events use `stage=pr.validation.wait` with `result=pending`, `success`,
`failed`, `cancelled`, `skipped`, or `timed_out`. Each event records
`pr_number`, `commit_sha`, `check_name`, `job_run_id`, `pr_state`, `is_draft`,
`wait_reason`, `status`, `conclusion`, `elapsed_ms`, `poll_count`, and
`next_poll_delay_ms`. Empty check rollups remain `pending` until checks report
or the wait times out; explicit skipped check conclusions remain `skipped`.
Transient GitHub
transport retries remain separate `stage=github_octocrab result=retry` events
with `operation=pr.validation.status`, so a check failure and a GitHub transport
failure are distinguishable in the same tail.

For direct PR validation diagnosis without `gh pr checks`, use the Rust-owned
status surface:

```bash
bash adl/tools/pr.sh validation <pr-number-or-url> --json
bash adl/tools/pr.sh validation <pr-number-or-url> --watch --json
```

The JSON report includes the PR number, head commit, PR state, draft state,
aggregate disposition, all checks, failed checks, and pending checks. `--watch`
polls through the same `pr.validation.wait` event stream before printing the
final report. The status query follows `statusCheckRollup.contexts` pagination
instead of assuming the first page is complete. When the PR argument is a
GitHub URL, the command infers `owner/repo` from that URL unless `-R` is
supplied explicitly. Shell exit status is non-zero for `pending`, `failed`,
`cancelled`, and `timed_out` dispositions so automation does not confuse “not
done yet” with success.

## OTEL Mapping

The event vocabulary is intentionally OTEL-ready:

| ADL field | OTEL-style mapping |
| --- | --- |
| `command` | span name prefix or `service.operation` attribute |
| `stage` | span name suffix or event name |
| `result` | status/event result attribute |
| `issue`, `branch`, `operation` | span attributes |
| `elapsed_ms` | span/event duration field |

This issue does not require a hosted collector. Exporter wiring belongs behind
a later implementation gate after local deterministic logs are stable.

## Implemented First Slice

The first slice instruments:

- `adl/tools/pr.sh` command dispatch and Rust delegation boundaries
- high-pain `pr.sh` commands: `run`, `doctor`, `finish`, and `closeout`
- `adl/tools/attach_post_merge_closeout.sh` watch, GitHub fetch, attach, and
  closeout stages
- Rust dispatch entrypoints for `adl`, `adl-csdlc`, `adl-runtime`, and
  `adl-review`

Runtime action-level logs remain owned by #3556. This contract keeps the
vocabulary aligned so runtime and control-plane spans can converge later.

## Non-Claims

- This contract does not claim complete OpenTelemetry export.
- This contract does not claim every Rust internal function is span-instrumented.
- This contract does not change command semantics or exit-code policy.
