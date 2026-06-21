# Issue Resource Telemetry Collector

`#4298` adds the first bounded local collector for issue resource telemetry on
approved host label `wuji`.

## Command

```text
adl tooling issue-resource-telemetry collect \
  --issue <number> \
  --issue-slug <slug> \
  --capture-stage <issue_start|pre_validation|post_validation|review_handoff|custom_stage> \
  [--host-label wuji] \
  [--process <role:pid>] \
  [--pid-file-process <role:path>] \
  [--captured-at <rfc3339>] \
  [--repo-root <path>] \
  [--out <path>] \
  [--json]
```

## Local Output Path

Default local ignored path:

```text
.adl/runs/issues/issue-<issue-number>/telemetry/issue_resource_telemetry.v1.jsonl
```

The collector appends one JSONL row per invocation.

## Capture Stages

Recommended bounded stages:

- `issue_start`
- `pre_validation`
- `post_validation`
- `review_handoff`

Custom snake_case stage names are allowed when the issue needs a narrower local
handoff point.

## Process Inputs

Process summaries stay bounded and never record raw command lines.

- `--process <role:pid>` tracks one explicit pid.
- `--pid-file-process <role:path>` resolves one explicit pid file.
- If no process input is supplied, the collector records its own process as
  role `collector`.

## Privacy Rules

- Host label is restricted to approved first-slice value `wuji`.
- Absolute host paths are rejected from the emitted row.
- GPU remains exact string `not_available` in this first slice.
- Raw command lines, env vars, usernames, and home-directory fragments are not
  recorded.
