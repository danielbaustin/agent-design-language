# v0.6 Demo Matrix

This is the canonical v0.6 integration demo matrix for milestone validation.

Execution assumptions:
- Run from repo root.
- No network dependencies.
- Use local mock provider for run-mode demos: `SWARM_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh`.
- Determinism checks are based on running each demo twice.

| Demo | Features Covered | Commands | Expected Artifacts | Determinism Notes |
|---|---|---|---|---|
| D1 Deterministic Concurrent Workflow | #406 | `SWARM_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin swarm -- swarm/examples/v0-3-concurrency-fork-join.adl.yaml --run --out <out_dir>` | `<out_dir>/fork/alpha.txt`, `<out_dir>/fork/beta.txt`, `<out_dir>/fork/join.txt` | Compare file contents across two runs. Ignore absolute output directory path differences. |
| D2 Pattern Expansion Plan | #401 | `cargo run -q --manifest-path swarm/Cargo.toml --bin swarm -- swarm/examples/v0-5-pattern-fork-join.adl.yaml --print-plan` | Plan text on stdout | Compare stdout byte-for-byte across two runs. |
| D3 HITL Pause/Resume | #402 | `SWARM_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin swarm -- swarm/examples/v0-6-hitl-pause-resume.adl.yaml --run --trace --out <paused_out>` then `SWARM_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin swarm -- swarm/examples/v0-6-hitl-pause-resume.adl.yaml --run --resume .adl/runs/v0-6-hitl-pause-demo/run.json --out <resume_out>` and baseline `SWARM_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin swarm -- swarm/examples/v0-6-hitl-no-pause.adl.yaml --run --out <plain_out>` | `<resume_out>/s3.txt`, `<plain_out>/s3.txt`, `.adl/runs/v0-6-hitl-pause-demo/run.json` | Resume output must match non-paused output; repeat whole cycle twice. |
| D4 Streaming Output Observational | #403 | `SWARM_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin swarm -- swarm/examples/v0-6-hitl-no-pause.adl.yaml --run --trace --out <stream_out>` and `SWARM_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin swarm -- swarm/examples/v0-6-hitl-no-pause.adl.yaml --run --quiet --out <quiet_out>` | `<stream_out>/s1.txt`, `<stream_out>/s2.txt`, `<stream_out>/s3.txt`, `<quiet_out>/s3.txt` | Trace includes `StepOutputChunk`. For trace comparisons, normalize timestamps and compare semantic event lines only. Artifacts must match with/without streaming output. |
| D5 Provider Profiles + Delegation Metadata | #404, #405 | `SWARM_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh cargo run -q --manifest-path swarm/Cargo.toml --bin swarm -- swarm/examples/v0-6-provider-profile-delegation.adl.yaml --allow-unsigned --run --trace --out <out_dir>` | `<out_dir>/b.txt` | Normalize timestamps and compare semantic trace lines across runs. Verify step `a` omits false-only delegation and step `b` emits canonicalized delegation JSON (deduped/sorted tags). |
| D6 Instrumentation (Graph/Replay/Diff) | #407 | `cargo run -q --manifest-path swarm/Cargo.toml --bin swarm -- instrument graph swarm/examples/v0-5-pattern-fork-join.adl.yaml --format json` ; `cargo run -q --manifest-path swarm/Cargo.toml --bin swarm -- instrument graph swarm/examples/v0-5-pattern-fork-join.adl.yaml --format dot` ; `cargo run -q --manifest-path swarm/Cargo.toml --bin swarm -- instrument replay swarm/examples/v0-6-trace-sample-a.json` ; `cargo run -q --manifest-path swarm/Cargo.toml --bin swarm -- instrument diff-plan swarm/examples/v0-5-pattern-linear.adl.yaml swarm/examples/v0-5-pattern-fork-join.adl.yaml` ; `cargo run -q --manifest-path swarm/Cargo.toml --bin swarm -- instrument diff-trace swarm/examples/v0-6-trace-sample-a.json swarm/examples/v0-6-trace-sample-b.json` | stdout artifacts for each command | Compare each command output byte-for-byte across two runs. |

## Twice-Run Determinism Evidence

Evidence directory used for this WP run:
- `/Users/daniel/git/adl-wp-408/.tmp/wp408-20260221-234841`

Results:
- D1: `D1_ALPHA=STABLE`, `D1_BETA=STABLE`, `D1_JOIN=STABLE`
- D2: `D2=STABLE`
- D3: `D3_RESUME=STABLE`, `D3_EQ_PLAIN=YES`
- D4: `D4_TRACE_SEMANTIC=STABLE`, `D4_ARTIFACT_STREAM_EQ_QUIET=YES`, `D4_CHUNK_EVENTS=YES`
- D5: `D5_TRACE_SEMANTIC=STABLE`, `D5_FALSE_ONLY_OMITTED=YES`, `D5_CANONICAL_DELEGATION=YES`
- D6: `D6_GRAPH_JSON=STABLE`, `D6_GRAPH_DOT=STABLE`, `D6_REPLAY=STABLE`, `D6_DIFF_PLAN=STABLE`, `D6_DIFF_TRACE=STABLE`

## Notes

- Raw trace output contains timestamps and elapsed milliseconds, so semantic comparisons remove those prefixes.
- Some raw artifacts include output-directory paths in logs; deterministic checks use content-level comparisons for files and normalized event lines for traces.
