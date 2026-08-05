# WP-12 Guardian / Observatory TLS Proof

Issue: #5344
Date: 2026-07-23
Host: wuji macOS local proof plus Nessus native Windows probe

## Security Boundary

- No plaintext Runtime v3 API proof was accepted.
- No `curl -k`, disabled verification, WSL, Docker, AWS, Runtime v2, or `/private/tmp` path was used for the proof.
- Local bounded TLS material used a test private CA and a SAN-correct localhost leaf certificate.
- Private keys remained under `.adl/runtime-v3-service/tls/` and were not printed.
- Production requirement remains first-class TLS plus mTLS with managed rotation.

## Test CA And Leaf Certificate

Command class:

- Created `test-ca-cert.pem` / `test-ca-key.pem` as a local private test CA.
- Issued `localhost-leaf-cert.pem` / `localhost-key.pem` with SANs for `localhost`, `127.0.0.1`, and `::1`.
- Runtime and Observatory used `localhost-cert.pem` and `localhost-key.pem`.

Verified metadata:

- CA subject: `CN=ADL Runtime v3 Test CA`
- Leaf subject: `CN=localhost`
- Leaf issuer: `CN=ADL Runtime v3 Test CA`
- Leaf validity: `notBefore=Jul 23 19:34:25 2026 GMT`; `notAfter=Jul 30 19:34:25 2026 GMT`
- Leaf SAN: `DNS:localhost`, `IP Address:127.0.0.1`, `IP Address:0:0:0:0:0:0:0:1`

Verification:

- `openssl verify -CAfile .adl/runtime-v3-service/tls/test-ca-cert.pem -purpose sslserver -verify_hostname localhost .adl/runtime-v3-service/tls/localhost-leaf-cert.pem` passed.
- `openssl verify ... -verify_hostname wrong.local ...` failed closed with `error 62 ... hostname mismatch`.

## Local macOS Guardian / Observatory Proof

Rebuilt binaries from the #5344 worktree:

- `cargo build --locked --manifest-path adl-runtime/Cargo.toml --bin adl-runtime-guardian --target-dir <external-fast-target>` passed after offline lock refresh for the existing `tokio/net` manifest change.
- `cargo build --locked --manifest-path adl-runtime-kernel/Cargo.toml --bin adl-runtime-kernel --target-dir <external-fast-target>` passed.

Launch topology:

- LaunchAgent guardian PID: `75268`
- Child kernel PID: `75269`
- Parent proof: kernel PPID was guardian PID.
- Runtime API: HTTPS `https://localhost:20997`
- HTML Observatory: HTTPS `https://localhost:8765/demos/v0.91.7/html-observatory/index.html`

CA-backed client proof:

- HTML Observatory loaded with `curl --cacert .adl/runtime-v3-service/tls/test-ca-cert.pem https://localhost:8765/...`.
- Runtime `/v1/observatory` loaded with bearer auth and `--cacert test-ca-cert.pem`.
- Browser-style WebSocket upgraded over WSS after sending the required first-frame auth JSON; the client used Python/OpenSSL with `ssl.create_default_context(cafile="test-ca-cert.pem")`, `check_hostname=true`, and `CERT_REQUIRED`.

Observed WSS/feed values:

- `schema=adl.runtime_v3.observatory_feed.v2`
- `runtime_selection=runtime_v3_explicit_opt_in`
- `agents_total_count=10000`
- `agents_rendered_sample_count=6`
- `weather_shutdown_decision=continue`
- `weather_stale=False`
- `control_port=20997`

Forced child termination proof:

- Killed child kernel PID `75269` only.
- Guardian PID `75268` remained alive.
- Replacement kernel PID `80057` appeared on port `20997`.
- Replacement kernel PPID was `75268`.
- Post-restart HTTPS feed remained healthy:
  - `runtime_instance_id=d551e239771d4676a01d45cd0a4fa90e`
  - `agents_total_count=10000`
  - `weather_shutdown_decision=continue`
  - `weather_stale=False`

Clean shutdown proof:

- `launchctl bootout gui/$(id -u)/org.agentlogic.adl-runtime-v3` terminated the launchd-owned guardian and child.
- Old guardian/kernel PIDs were absent after shutdown.
- Guardian log recorded terminal state `shutdown_forwarded`, `attempts=2`, `restarts=1`, and child signal `9` for the forced-kill attempt.
- A transient older non-launchd guardian tree was found after restart (`83302`/`83304`) and terminated by signalling its guardian PID. Final state had only one launchd-owned guardian/kernel tree:
  - guardian PID `86658`
  - kernel PID `86659`
  - kernel PPID `86658`
- Final CA-backed API feed remained healthy:
  - `runtime_instance_id=792f207017dc4409a52d2bf9b6ea36a0`
  - `agents_total_count=10000`
  - `weather_shutdown_decision=continue`
  - `weather_stale=False`

## Nessus Native Windows Probe

Route:

- `nessus` and `nessus.agent-logic.ai` did not resolve.
- `nessus.local` connected over SSH.
- Native platform proof:
  - PowerShell edition: `Desktop`
  - `rustc 1.96.0`
  - Rust host: `x86_64-pc-windows-msvc`

Current source packet:

- Copied the bounded source packet to `D:\adl-wp-5344-min`.
- Built and tested with Cargo output under `D:\adl-wp-5344-target`.
- No WSL, Docker, AWS, plaintext API, disabled certificate verification, or insecure curl flags were used.
- Native runs emitted only Windows target-cache hardlink fallback warnings under `D:\adl-wp-5344-target`; all selected builds/tests completed successfully.

Build proof:

- `cargo build --locked --manifest-path adl-runtime\Cargo.toml --bin adl-runtime-guardian --target-dir D:\adl-wp-5344-target` passed.
- `cargo build --locked --manifest-path adl-runtime-kernel\Cargo.toml --bin adl-runtime-kernel --target-dir D:\adl-wp-5344-target` passed.
- `cargo build --locked --manifest-path adl-runtime-kernel\Cargo.toml --bin adl-runtime-governed-operations --target-dir D:\adl-wp-5344-target` passed, proving `governed_operations` no longer depends on unconditional Unix-only APIs.

Native Windows process-0 proof:

- Command: `cargo test --locked --manifest-path adl-runtime\Cargo.toml --target-dir D:\adl-wp-5344-target child_exit_terminates_descendants_with_windows_job_object -- --nocapture`
- Result: `PASS`; 1 selected guardian test passed.
- Proof meaning: the Guardian assigns the child to a Windows Job Object with kill-on-close semantics, exits the child, and proves the spawned PowerShell descendant process is gone.

Native Windows guardian lease / checkpoint proof:

- Command: `cargo test --locked --manifest-path adl-runtime-kernel\Cargo.toml --test guardian_soak guardian_lease_loss_checkpoints_and_stops_the_real_kernel --target-dir D:\adl-wp-5344-target -- --exact --nocapture`
- Result: `PASS`; 1 selected `guardian_soak` test passed.
- Proof meaning: the real kernel connects to the Guardian lease, reports `control_ready`, detects Guardian lease loss, runs terminal checkpoint serialization, exits cleanly, and retains a signed `generation-1/manifest.json`.

Native Windows strict CA-backed HTTPS/WSS proof:

- Command: `cargo test --locked --manifest-path adl-runtime-kernel\Cargo.toml --test guardian_soak signed_https_wss_shutdown_checkpoints_and_forgery_cannot_stop_the_process --target-dir D:\adl-wp-5344-target -- --exact --nocapture`
- Result: `PASS`; 1 selected `guardian_soak` test passed.
- Test CA and leaf are generated in-process with private keys kept in the test temp directory and never printed.
- Test CA validity: `notBefore=2026-01-01`; `notAfter=2036-01-01`.
- Leaf validity: `notBefore=2026-01-01`; `notAfter=2036-01-01`.
- Leaf SANs: `DNS:localhost`, `IP Address:127.0.0.1`, `IP Address:::1`.
- The client explicitly trusts only the generated test CA through Rustls `RootCertStore`.
- Wrong-host validation for `wrong.local` fails closed before the accepted HTTPS/WSS path.
- HTTPS `/v1/control` and `/v1/observatory` use the CA-backed Rustls client.
- WSS `/v1/observatory/ws` uses `tokio_tungstenite` with Rustls connector and first-frame bearer authentication.
- Forged signed shutdown remains rejected with `HTTP/1.1 401`.

## Classification

- Local macOS guardian-as-process-0, HTTPS Observatory, CA/hostname validation, forced child restart, weather health, 10,000-agent feed, and clean guardian shutdown proof: `PASS`.
- Native Windows build of Guardian, kernel, and governed-operations: `PASS`.
- Native Windows Guardian process-0 descendant cleanup: `PASS`.
- Native Windows guardian lease-loss checkpoint shutdown: `PASS`.
- Native Windows strict CA-backed HTTPS/WSS with SAN verification and wrong-host rejection: `PASS`.
- Remaining blocker: none for the selected native Windows #5344 Guardian/Observatory proof surface. The Windows target-cache hardlink fallback warning is non-functional and did not prevent build or proof completion.
