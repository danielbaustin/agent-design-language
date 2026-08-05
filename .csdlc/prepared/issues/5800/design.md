# Issue 5800 Design: Browser-Trusted Observatory HTTPS

## Outcome And Boundary

Issue 5800 establishes one supported local HTTPS identity for the separate HTML
Observatory and Runtime v3 API. Chrome, curl, the static server, Runtime TLS,
configured origins, health probes, and operator documentation must agree on the
same `localhost` trust contract without a warning bypass. The issue owns local
certificate issuance, installation guidance, reissue, and URL consistency; it
does not own WP-03 launch resilience or public-domain certificate automation.

## Source Baseline

- `adl-runtime/src/local_tls.rs` already implements atomic local certificate
  generations, SAN verification, Rustls pair validation, private-key permission
  checks, and preservation of the last committed generation.
- `adl-runtime/tests/local_tls.rs` owns focused local TLS contract coverage.
- `infra/runtime-v3/runtime-init.toml` defines the Runtime API TLS and allowed
  Observatory origin contract.
- `demos/html-observatory/runtime-v3.config.json` points the client at
  `https://localhost:20997`; `demos/html-observatory/README.md` requires the
  separate static host at `https://localhost:8765` but still contains stale
  plaintext startup guidance that must be reconciled.
- `docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml` requires trusted-browser,
  curl, HTML, Runtime-feed, configuration, and negative certificate proof.

## Design

Use the existing Runtime local TLS bootstrap as the certificate material owner.
The implementation must select and document one host trust mechanism that can
install the generated local CA or certificate into the operator browser trust
store with explicit consent. The Observatory static server and Runtime API use
certificate/key material representing the same `localhost` identity, while
each listener keeps its existing ownership boundary.

Reissue is explicit and atomic. A new certificate is validated for SANs,
Rustls compatibility, expiry, and file permissions before the current manifest
is replaced. Trust installation never commits private material or silently
modifies a host trust store. Unsupported or incomplete host setup remains a
reported prerequisite, not an instruction to disable verification.

## Owned Paths

- `adl-runtime/src/local_tls.rs`
- `adl-runtime/src/bin/adl-runtime-local-tls-bootstrap.rs`
- `adl-runtime/tests/local_tls.rs`
- `demos/html-observatory/runtime-v3.config.json`
- `demos/html-observatory/README.md`
- `adl/tools/validate_v092_browser_trusted_observatory.mjs`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Invariants And Failure Semantics

- TLS verification remains enabled for browser and command-line clients.
- Certificate SANs include the configured localhost DNS/IP identities.
- No private key, trust-store export, token, or secret enters Git or logs.
- A failed, mismatched, expired, malformed, or partial replacement cannot
  delete or supersede the last valid committed pair.
- Runtime TLS failure may block API readiness but must not silently start a
  plaintext API or take down unrelated preparation tooling.
- CORS origin and endpoint configuration must match the actual HTTPS listeners.

## Dependencies And Coordination

WP-01 and WP-02A must be terminal and ancestral before implementation. Sprint
5855 coordinates ownership. Issue 5800 may prepare independently, but any
overlap with issue 5820 on Runtime init, startup, or TLS configuration collapses
to serial execution. Issue 5795 consumes the trusted path later and cannot
redefine it.

## Validation Boundary

Deterministic proof covers generation, SAN, expiry, permission, replacement,
and last-valid-pair behavior. Live macOS proof covers Chrome trust, static HTML,
Runtime health/readiness/feed, and curl verification. Linux and native Windows
lanes must either reproduce the supported setup or retain an explicit platform
blocker; a macOS screenshot alone does not prove portable trust installation.
The implementation must add `adl/tools/validate_v092_browser_trusted_observatory.mjs`.
That Playwright validator launches the real HTTPS static server and Runtime
candidate, opens Chrome with the operator-approved trust root, fails on any
certificate interstitial or console/network TLS error, exercises HTML plus
health, readiness, and feed requests, and writes redacted browser-visible
evidence. `curl` remains a separate endpoint probe and cannot satisfy browser
trust acceptance.

## Rollback

Rollback restores the previous committed TLS generation and prior HTTPS
configuration, removes only issue-created public trust entries using the same
documented mechanism, restarts both listeners, and reruns verified health
checks. It never deletes unknown operator certificates or falls back to HTTP.

## Non-Goals

- AWS, ACM, Route53, or Let's Encrypt issuance for `localhost`.
- Public production-domain certificate automation.
- Disabling browser warnings, Rustls verification, or CORS policy.
- Serving Observatory HTML from Runtime or redesigning the Observatory.
- Claiming WP-03 resilience, WP-14 protocol, or WP-18A consumer completion.
