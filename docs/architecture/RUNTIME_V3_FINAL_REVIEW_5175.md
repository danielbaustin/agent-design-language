# Runtime v3 Final Review — Issue 5175

## Outcome

The final disposition is **continue incubation** and **ready for issue #5175
closeout**. Runtime v2 remains the default. This review does not authorize
automatic or production cutover.

Fable 5 performed a findings-first code and architecture review before the
guardian soak, a full post-remediation review, and a final delta verification.
The final delta verification reported no P0-P3 findings and concluded
`READY_FOR_5175_CLOSEOUT`. Independent code, architecture, security, test, and
documentation reviewers also reported no remaining actionable findings after
remediation.

Provider request, result, review text, and redacted operational logs are retained
locally under `.adl/local-artifacts/fable5-final-review/`. They are review input;
the tracked source, tests, reports, and native execution evidence remain the
authoritative proof.

## Findings Disposition

| Finding group | Disposition | Proof |
|---|---|---|
| Process timeout and descendant leaks | Fixed | concurrent bounded duplex I/O, RAII process-group cleanup, timeout and detached-stream regressions |
| Output artifact, symlink, and disk growth | Fixed | unconditional cleanup, `RLIMIT_FSIZE`, bounded drains, `O_NOFOLLOW`, regular-file validation |
| Tautological parity normalization | Fixed | source-derived terminal completion and terminal-set validation, ordered replay transcription, typed v3 state-hash recomputation, divergence mutation test |
| Vacuous coverage/cutover evidence | Fixed | immutable coverage contract derived from all 18 tracked matrix capabilities; incomplete and uncontracted reports remain ineligible |
| Fixture validation and environment leakage | Fixed | validation at the process boundary, cleared environment, stdin-only fixture input, shared 1 MiB bound |
| Self-certifying relation labels | Fixed | stale redesign, unsupported, and blocked expectations classify as defects |
| String-inferred failure taxonomy | Fixed | explicit `BackendFailureKind` constructors; unclassified strings default to `Other` |
| Concurrent timing methodology | Fixed | sequential-backend 21-sample live run retained with then-current medians |
| Guardian restart and shutdown behavior | Fixed | native Horust fatal restart, SIGTERM/checkpoint, and terminal configuration-exit proofs |
| Stale or mismatched reports | Fixed | generated soak execution matched the tracked projection; parity and guardian reports shared then-current counts and timing |

## Validation

The numeric inventory counts below are the historical snapshot recorded when
issue #5175 closed. They are not current-size claims. The
[current generated Runtime v3 inventory](runtime_v3_current_inventory.v1.json)
is the authoritative reproducible source for present implementation LoC,
direct dependencies, Rust test attributes, and parity-baseline module count.

- Full Runtime v3 crate tests passed.
- Clippy passed for all targets with warnings denied.
- The 100-cycle, 1,600-item bounded soak passed.
- Five explicit native guardian/soak tests passed serially.
- The live v2/v3 bounded-loop fixture passed for 21 sequential samples per
  runtime; the then-current medians were 7,844 microseconds for v2 and 5,717
  microseconds for v3.
- At #5175 closeout, Runtime v3 contained 8,446 Rust source lines and 106 tests,
  below the then-applicable 10,000 LoC and 1,000-test challenge limits.
- At #5175 closeout, the rebased external inventory routed 195 Runtime v2 and
  `adl-runtime` modules, including the newly merged Vector observability module;
  this was ownership routing, not a behavioral-equivalence claim.

## Residual Risk

Issue `#5211` owns production Horust qualification: Linux cgroups, hardened
host identity and limits, cross-host packaging, deliberate session escape,
long-duration endurance, and operational SLO evidence. Process-group cleanup
is appropriate for the trusted parity-fixture boundary but is not presented as
an OS security boundary. In-process parity evidence is not operator
authorization and no production cutover authority exists in this packet.
