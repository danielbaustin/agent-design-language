# Remote Gemma Watcher Proof 2026-06-15

Date: 2026-06-15

Issue: `#3724`

Run ID: `v0915-remote-gemma-watcher-20260615`

Status: `useful_with_limits`

## Purpose

This packet records a bounded follow-on probe for the Sprint 2 remote Gemma
watcher non-claim. The historical `#3415` workcell packet kept the watcher lane
truthful as completed but empty. This issue tests whether a larger remote Gemma4
route can produce useful watcher output under a tighter bounded prompt.

## Historical Baseline

The prior tracked watcher output remains unchanged:

- historical empty output: `docs/milestones/v0.91.5/review/multi_agent_workcell/lane_outputs/watcher_remote_gemma4_e2b.md`
- historical state packet: `docs/milestones/v0.91.5/review/multi_agent_workcell/v0915_parallel_csdlc_workcell_state_2026-06-14.json`

That older lane is still true as a historical fact. This packet does not rewrite
that run. It adds new bounded evidence from `#3724`.

## Probe Summary

| Lane | Surface | Model | Status | Output |
| --- | --- | --- | --- | --- |
| `adapter_gemma4_31b` | `adl_provider_adapter` | `gemma4:31b` | `useful_output` | `docs/milestones/v0.91.5/review/remote_gemma_watcher/lane_outputs/adapter_gemma4_31b.md` |
| `raw_gemma4_26b` | `raw_ollama_http` | `gemma4:26b` | `useful_output` | `docs/milestones/v0.91.5/review/remote_gemma_watcher/lane_outputs/raw_gemma4_26b.md` |
| `raw_gemma4_e4b` | `raw_ollama_http` | `gemma4:e4b` | `useful_output` | `docs/milestones/v0.91.5/review/remote_gemma_watcher/lane_outputs/raw_gemma4_e4b.md` |

## What Was Proven

- The remote Ollama host at `http://192.168.68.70:11434` is reachable for bounded watcher probes.
- The larger Gemma4 routes can return non-empty structured watcher text.
- The ADL-native provider path is proven through `adl-provider-adapter` on
  `gemma4:31b`, not only through raw HTTP.
- The historical empty-output issue is no longer the only observed watcher outcome.

## Primary Result

The strongest proving lane is `adapter_gemma4_31b`. It returned reviewer-usable
markdown with the required watcher headings and the exact phrase
`route probe completed` through the real ADL provider adapter surface.

Secondary useful routes also returned structured watcher text: `gemma4:31b`, `gemma4:26b`, `gemma4:e4b`.

## Reliability Gate

This runner fails closed unless `adapter_gemma4_31b` returns `useful_output`
through the ADL provider adapter and at least `2` Gemma lanes return useful
structured watcher text. Smaller or historically weak routes such as
`gemma4:e2b` are not promoted as reliable watcher lanes by this proof.

## Disposition

Remote Gemma watcher usefulness is now **proven in a bounded way** for short,
structured watcher prompts on larger Gemma4 routes. The lane remains
`useful_with_limits` rather than broadly proven because:

- this packet only covers one bounded prompt shape
- it does not prove full multi-agent planning or janitor usefulness
- it does not prove `gemma4:e2b` is universally recovered for the original
  historical workcell prompt

## Validation

- `python3 adl/tools/run_v0915_remote_gemma_watcher_probe.py`
- `python3 adl/tools/validate_v0915_remote_gemma_watcher_probe.py docs/milestones/v0.91.5/review/remote_gemma_watcher`
- `bash adl/tools/test_v0915_remote_gemma_watcher_probe.sh`
- `git diff --check`

## Non-Claims

- This packet does not claim broad remote Gemma autonomy.
- This packet does not claim Sprint 2 multi-agent quality is fully proven.
- This packet does not replace the historical `#3415` workcell packet.
- This packet does not prove every Gemma4 size or prompt shape is equally useful.
