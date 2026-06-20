# Codex Usage Watcher

`adl tooling codex-usage-watch` is a local helper for manually copied Codex `/status` output.

It does not call private account APIs, scrape the UI, or use OCR. Version 1 depends on text that the operator copies into a file or passes directly on the command line.

## Commands

```bash
adl tooling codex-usage-watch parse --input /tmp/status.txt --json
adl tooling codex-usage-watch parse --text "Context: 37% left (161,634 used / 258K)
5h limit: 4% left (resets 4:04 PM)
7d limit: 3% left (resets Jun 24)" --json
adl tooling codex-usage-watch watch --input /tmp/status.txt --interval-seconds 60 --iterations 10 --json
```

## Input Shape

The parser expects text equivalent to:

```text
Context: 37% left (161,634 used / 258K)
5h limit: 4% left (resets 4:04 PM)
7d limit: 3% left (resets Jun 24)
```

The parser tolerates commas in token counts and `K` suffixes such as `258K` or `1.5K`.

## Modes

- `normal`: all tracked limits above warning thresholds
- `conserve`: 5h or 7d limit `<= 15%`, or context `<= 20%`
- `emergency`: 5h or 7d limit `<= 5%`
- `reset_ready`: 5h or 7d limit `<= 1%`
- `invoke_reset`: 5h or 7d limit `<= 0.5%`
- `usage_unknown`: input missing or malformed; fail closed

Warnings are emitted on stderr so JSON output on stdout stays machine-readable.

Missing or malformed input is fail-closed:
- the command emits a `usage_unknown` report
- `parse_ok` is `false`
- the process exits nonzero so shell supervision can alert immediately

## History

`watch` appends one JSON object per sample to:

```text
.adl/runs/codex_usage_watch/history.jsonl
```

Use `--history-root <dir>` to redirect the ignored runtime path when needed.

Even when a sample is malformed, the watcher records the `usage_unknown` row before exiting nonzero so operators keep the failure evidence.

## Limitations

- No automatic reset invocation in v1
- No OCR
- No live account polling
- No secret material should appear in input or output
