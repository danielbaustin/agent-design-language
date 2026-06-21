# Nessus Local Remote Rust Validation Runner Proof for `#4317`

Status: `completed_for_issue_scope`
Issue: `#4317`
Related issues: `#4310`, `#4316`, `#4318`, `#4320`
Date: 2026-06-21

## Scope

This packet records the bounded proof that `nessus.local` can execute a real
ADL Rust build and focused validation cycle as an owned local remote runner.

This issue proves:

- the correct remote login identity for runner work
- the active WSL2/Linux execution surface on Nessus
- a real remote ADL checkout pinned to a known `origin/main` commit
- a real Rust toolchain + `sccache` setup path inside WSL
- one successful remote build of `adl-pr-doctor`
- one successful focused remote validation lane using
  `provider_communication`
- tail-friendly artifact and log retention under a deterministic run directory
- one real host-readiness problem that must be accounted for in future runner
  automation

This issue does not prove:

- migration of CI to Nessus
- generic support for all ADL validation lanes
- provider/model execution on Nessus
- distributed scheduling or automatic lane fan-out
- long-running cache-hit behavior across repeated remote runs
- rootless or non-admin setup flows on the host

## Decision

Recommendation: treat `nessus.local` WSL2 Ubuntu as a viable Phase 1 owned local
remote Rust validation runner for heavy or experimental validation lanes, with
one important readiness caveat:

- the WSL apt configuration currently contains stale third-party sources that
  can break unattended package bootstrap unless the automation masks or repairs
  them first

Interpretation:

- the remote-runner abstraction is proven on owned local hardware before cloud
  rollout
- `#4316` was correct to keep CodeBuild as a later candidate rather than the
  next narrowest proof step
- follow-on wrapper/tooling should make the successful WSL path deterministic
  and should codify the apt-source hygiene workaround or permanent cleanup

## Identity and access proof

The runner proof surfaced an important access fact:

- `ssh danie@nessus.local` is the correct login path for the usable WSL-backed
  runner environment
- `ssh nessus.local` / `nessus\\daniel` lands in a different Windows user
  context that does not see the active WSL distro

This means future automation must not assume the default Nessus SSH identity is
sufficient. The bound login/user is part of runner truth.

Related repo truth already existed but was incomplete for this issue's runner
needs:

- `#4318` proved bounded SSM enrollment for `nessus.local`
- `#4320` proved a Codex login preparation path for Nessus
- this issue proved which account/user context actually reaches the live WSL2
  runner surface needed for Rust work

## Live runner environment

Remote host execution surface during proof:

- Windows host: `nessus.local`
- SSH login used: `danie@nessus.local`
- Linux execution surface: WSL2 Ubuntu run as `root`
- Kernel: `5.15.153.1-microsoft-standard-WSL2`
- Distro: `Ubuntu 20.04.3 LTS (Focal Fossa)`
- CPU visibility: `16` logical CPUs
- Memory visibility: about `62 GiB` total, about `60 GiB` free during proof
- Swap visibility: `16 GiB`

Captured host-facts artifact:

- `/root/adl-remote-runner/logs/20260621-070301/host-facts.log`

## Remote workspace and cache layout

The proof used this deterministic remote layout:

- workspace root: `/root/adl-remote-runner`
- repo checkout: `/root/adl-remote-runner/agent-design-language`
- target dir: `/root/adl-remote-runner/cache/target`
- `sccache` dir: `/root/adl-remote-runner/cache/sccache`
- run logs: `/root/adl-remote-runner/logs/<run-id>`

Pinned ADL checkout for the successful proof:

- `origin/main` SHA: `b69f4010e483fc5120bf0e1d7abf24437850a2af`

This commit was checked out in detached-HEAD mode specifically to avoid branch
ambiguity during the proof run.

## Toolchain setup proof

The successful WSL proof established:

- Rust installed and working in WSL
- `rustc 1.96.0`
- `cargo 1.96.0`
- `sccache 0.16.0`

Relevant proof artifacts:

- `/root/adl-remote-runner/logs/20260621-070301/rustc-version.log`
- `/root/adl-remote-runner/logs/20260621-070301/cargo-version.log`
- `/root/adl-remote-runner/logs/20260621-070301/sccache-version.log`
- `/root/adl-remote-runner/logs/20260621-070301/install-sccache.log`

## Real remote build proof

Successful build command:

```bash
cargo build --manifest-path /root/adl-remote-runner/agent-design-language/adl/Cargo.toml --locked --bin adl-pr-doctor
```

Observed result:

- build completed successfully in WSL on Nessus
- terminal artifact shows:
  - `Finished 'dev' profile [unoptimized + debuginfo] target(s) in 1m 34s`

Primary artifact:

- `/root/adl-remote-runner/logs/20260621-070301/cargo-build.log`

## Real focused validation proof

Successful focused validation command:

```bash
cargo test --manifest-path /root/adl-remote-runner/agent-design-language/adl/Cargo.toml --locked provider_communication -- --nocapture
```

Observed result:

- focused provider-communication validation passed on the remote runner
- the main unit-test surface reported:
  - `21 passed; 0 failed`
- no failing filtered sub-binaries or test harnesses were observed afterward

Primary artifact:

- `/root/adl-remote-runner/logs/20260621-070301/cargo-test-provider-communication.log`

## `sccache` proof

The proof also captured first-run `sccache` stats:

- compile requests: `436`
- compile requests executed: `281`
- cache hits: `0`
- cache misses: `276`
- Rust cache misses: `246`
- cache location:
  `/root/adl-remote-runner/cache/sccache`
- cache size after proof: about `217 MiB`

Interpretation:

- the first remote run is behaving like a cold-cache proof, which is expected
- the cache surface is functioning and durable enough to support a second-pass
  warm-cache follow-on issue if needed
- this issue does not claim a measured warm-cache performance improvement yet

Primary artifact:

- `/root/adl-remote-runner/logs/20260621-070301/sccache-stats.log`

## Apt-source hygiene finding

The successful proof required a temporary workaround for stale WSL apt sources.

Observed failure before workaround:

- `apt-get update` failed because:
  - `/etc/apt/sources.list.d/kubernetes.list` pointed to
    `https://apt.kubernetes.io/ kubernetes-xenial`, which now returns a 404
  - `/etc/apt/sources.list` contained a HashiCorp apt source with a missing
    public key for unattended verification on this host

Workaround used during proof:

- temporarily mask `kubernetes.list`
- temporarily comment the HashiCorp apt source
- restore both on script exit

Why this matters:

- the runner itself is viable
- host package-source hygiene is not yet automation-safe
- future runner bootstrap must either:
  - permanently repair those source entries on Nessus, or
  - codify the temporary masking logic as explicit runner preflight behavior

This is a real runner-readiness finding, not a proof failure.

## Logging and artifact-return expectations

The proof established a usable logging contract for remote runner work:

- each command writes a dedicated log file under the run directory
- remote artifacts are tail-friendly and human-readable
- the run writes a small machine-readable summary file:
  - `/root/adl-remote-runner/logs/20260621-070301/summary.json`

Recommended contract for future tooling:

- keep one stable run root on the remote host
- keep one timestamped subdirectory per run
- keep one log file per command or major phase
- emit a tiny summary pointer at the end of the run
- preserve fail-closed behavior when any setup/build/test command exits nonzero

## Recommended command contract for future automation

The next safe wrapper/tooling surface should:

1. connect to Nessus using the proven user context
2. enter WSL explicitly rather than assuming native Windows Rust
3. preflight the apt-source state and either repair or temporarily mask known
   stale third-party entries
4. ensure Rust + `sccache` are present
5. materialize or refresh a pinned repo checkout
6. set:
   - `CARGO_TARGET_DIR`
   - `SCCACHE_DIR`
   - `RUSTC_WRAPPER`
7. run one declared build or validation command
8. emit a summary path and stable log directory
9. fail closed on any command failure

## Non-claims

- This packet does not claim the default Windows user on Nessus can use WSL for
  runner work.
- This packet does not claim the host's apt sources are already clean.
- This packet does not claim warm-cache timings.
- This packet does not claim all ADL tests or lanes have been remote-proven.
- This packet does not claim provider credentials or GitHub credentials were
  copied to Nessus for the proof.
- This packet does not claim CI replacement or merge-gate readiness.

## Follow-on issue candidates

Recommended follow-on work if we continue this direction:

1. Add a repo-native Nessus remote-runner wrapper that codifies the proven WSL
   path, cache layout, summary output, and fail-closed logging.
2. Add a host-readiness remediation issue to permanently clean or manage stale
   WSL apt sources on Nessus.
3. Add a warm-cache measurement issue for repeated `sccache` runs on the remote
   WSL runner.
4. Add a bounded PVF routing issue for deciding which remote-capable validation
   lanes are worth running on Nessus first.
