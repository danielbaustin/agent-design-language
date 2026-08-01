# Issue #5757 Observatory Corrective Design

## Scope

This corrective change is limited to issue #5757 product surfaces:

- `demos/html-observatory/`
- `adl/tools/test_v0917_html_observatory_integrated_proof.sh`
- `adl/tools/validate_v0917_html_observatory.py`
- `adl-runtime-kernel/tests/observatory.rs`
- `infra/runtime-v3/`

The work does not inspect, edit, claim, or depend on #5748 or its worktree.

## Defects

1. Runtime v3 browser selection must reject untrusted API origins before any bearer credential or WebSocket is used.
2. Live, retained, WSS, and fallback completions must obey one monotonic generation so stale async work cannot overwrite current operator intent.
3. The Observatory proof must include real shared localhost certificate evidence across the HTTPS page port `8765` and Runtime API/WSS port `20997`, plus authenticated WSS control proof.

## Design

Runtime v3 API selection now normalizes through a strict trusted-origin function. The accepted browser-control origin is exactly `https://localhost:20997`: HTTPS, hostname `localhost`, explicit port `20997`, root path, and no credentials, query, or fragment. Runtime v3 fetch and WSS connection code calls this normalizer before request construction, bearer login, or socket creation.

The live Observatory binding owns a single increasing generation counter. Polling, retained fallback refreshes, WebSocket messages, close/error completions, and operator stop/connect transitions check the active generation before rendering. Late completions become no-ops instead of replacing newer state.

The integrated proof remains local and bounded. It drives the existing HTML Observatory validation harness, starts two loopback TLS listeners with one generated localhost certificate, captures the certificate visible on ports `8765` and `20997`, compares SHA-256 fingerprints, removes the generated private key before capture, and runs the focused Runtime v3 HTTPS/WSS Rust tests.

## Validation

Typed finalize runs these focused lanes:

- `observatory-js-syntax`: `node --check demos/html-observatory/app.js`
- `observatory-integrated-proof`: `bash adl/tools/test_v0917_html_observatory_integrated_proof.sh`
- `diff-hygiene`: `git diff --check`

The proof artifacts stay under `.csdlc/evidence/5757`.
