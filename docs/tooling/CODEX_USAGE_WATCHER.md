# Codex Usage Watcher

`adl tooling codex-usage-watch` is a local helper for Codex `/status` output.

It does not call private account APIs, scrape secret-bearing local storage, or claim live UI/OCR collection. The collector path depends on text that the operator or a supported outer UI automation layer provides from the Codex app `/status` panel.

## Commands

```bash
adl tooling codex-usage-watch parse --input /tmp/status.txt --json
adl tooling codex-usage-watch parse --text "Context: 37% left (161,634 used / 258K)
5h limit: 4% left (resets 4:04 PM)
7d limit: 3% left (resets Jun 24)" --json
adl tooling codex-usage-watch collect --text "Status
Session: 019f3d65-2216-7832-804a-26339473b27d
Context: 58% left (109,629 used / 258K)
5h limit: 93% left (resets 9:45 AM)
7d limit: 99% left (resets Jul 19)" --json
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

## Collector Shape

`collect` is the bounded collector ingress for copied or externally provided Codex app `/status` panel text. It normalizes extra panel lines such as `Status`, `Session: ...`, and `Close`, then feeds the required usage lines into the existing parser and classifier.

The collected text must still include all three required lines:

```text
Context: 58% left (109,629 used / 258K)
5h limit: 93% left (resets 9:45 AM)
7d limit: 99% left (resets Jul 19)
```

If no input is provided, or a required limit is missing, `collect` emits `usage_unknown` and exits nonzero. This is intentional: the command fails closed rather than pretending it can read live account state by itself.

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
- No built-in OCR
- No built-in Codex app UI control
- No live account polling or private account API usage
- No secret material should appear in input or output
