# ADL HTML Observatory

This demo is the v0.91.7 HTML Observatory integrated proof for #4690.

It adapts the Magic UI Pro AI Agent Template direction, with the Magic UI
Devtool Template used for denser dashboard composition cues, into a reviewable
CSM polis panopticon without importing account credentials or private template
metadata into the repository. The first-class mode is a compact control-room
dashboard over the Runtime v3 `/v1/observatory` and `/v1/ready` browser
surfaces, with retained CSM `/status`, `/health`, `/ready`, `/metrics`, and
`/events` proof available as fallback. Runtime KPIs, agent graph preview, event
tail, CSM API status, CloudWatch linkage, governance proof, and operator
communication status remain visible in the first dashboard viewport. It
auto-refreshes the retained publishable CSM API response artifacts from #4976 as
a runtime mirror, and upgrades to live loopback polling when an operator
supplies the currently running CSM API base. The retained runtime packet remains
the fallback proof surface if the Runtime v3 feed cannot load. The page reads
the configured Runtime v3 Observatory API base and endpoints from
`runtime-v3.config.json`. It also consumes the retained CSM runtime Observatory
packet and operator report from the v0.91.7 Soak 2 evidence root, plus the
current CSM runtime administration and AWS linkage evidence.

The primary desktop dashboard is fixed to the viewport, while narrower browser
windows use page scrolling so controls are never clipped. Event streams and
inspector areas retain bounded internal overflow. The visible shell uses local inline SVG icons, role-specific topology
glyphs for owner, readiness, scheduler, telemetry, event, and checkpoint lanes,
non-overlapping graph nodes with signal-line affordances, a compact table-style
event stream, rail telemetry, an inspector-style CSM API/gauge stack, and a
bottom runtime status bar to match the approved control-room mockup without
importing external template assets.

- `docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/runtime_v2/observatory/visibility_packet.json`
- `docs/milestones/v0.91.7/review/runtime/soak2_4682/agent_lifecycle/runtime_v2/observatory/operator_report.md`
- `docs/milestones/v0.91.7/review/runtime/csm_service_4903/service/service_manifest.json`
- `docs/milestones/v0.91.7/review/runtime/CSM_RUNTIME_API_4929.md`
- `docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/status.json`
- `docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/health.json`
- `docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/ready.json`
- `docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/metrics.json`
- `docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/events.json`
- `docs/milestones/v0.91.7/review/runtime/wp08_heartbeat_4684/live_heartbeat_summary.json`
- `docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/aws/cloudwatch_recent_events.redacted.json`
- `docs/milestones/v0.91.7/review/runtime/wp08_acip_sns_4685/acip_sns_summary.json`
- `docs/milestones/v0.91.7/review/runtime/wp08_acip_sns_4685/sns_resource_summary.json`

The CSM polis panopticon presents an auto-refreshing agent map, agent roster,
health, readiness, metrics, and operator event stream from the retained CSM API
mirror fallback when Runtime v3 is unavailable. When a loopback API base is
supplied with `runtime=v2` or `csmApiBase`, it polls the running CSM API
directly. For Runtime v3, the default path reads
`demos/html-observatory/runtime-v3.config.json`; query parameters such as
`?runtime=v3&runtimeApiBase=<runtime-api-base>&live=1` remain a troubleshooting
override. The runtime API base must match the configured Runtime v3 control API
host, port, TLS posture, and Observatory allowed-origin policy. The Runtime v3
path consumes the runtime-owned `/v1/observatory` read feed and `/v1/ready`
without bearer credentials. Runtime v3 control mutation remains
signed-command-only through `/v1/control`; the browser has no unsigned
shutdown, mutation, CloudWatch, SNS, or state authority. The CSM API panel intentionally
presents the standalone `csm` runtime ownership boundary from #4929 when the
retained/default mirror is selected. The CloudWatch panel presents the retained
live heartbeat proof from WP-08 #4684. The AWS linkage lane includes #4684
through #4688 so closed heartbeat, ACIP-SNS, and SSM lanes remain distinct from
open full-bridge and S3 archive work. The communication rail can prepare an
ACIP-shaped operator message envelope, mirror the retained #4685 ACIP-SNS proof,
check the Runtime v3 event tail through `/v1/observatory` by default, and check
a live loopback CSM `/events` endpoint when an operator supplies a Runtime v2
API base. Live SNS/SQS mutation remains runtime/tool-owned and is not performed
by the browser surface.

## Run

From the repository root:

```sh
python3 -m http.server 8765
```

Then open:

```text
http://localhost:8765/demos/html-observatory/
```

For the v0.91.7 real-runtime test path, start the existing repo `csm` binary in
a separate terminal and point the dashboard at that loopback base:

```sh
adl/target/debug/csm api serve \
  --spec docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/full/agent.yaml \
  --bind localhost:24645 \
  --max-requests 25 \
  --idle-timeout-ms 60000 \
  --json
```

```text
http://localhost:8765/demos/html-observatory/?csmApiBase=http://localhost:24645&live=1
```

For the Runtime v3 path, start the Runtime v3 kernel control API with the repo
default init file:

```sh
ADL_RUNTIME_OBSERVATORY_TOKEN="<operator-local-token>" \
  adl-runtime-kernel serve --init infra/runtime-v3/runtime-init.toml
```

The default init file keeps the Runtime v3 listener on `localhost:20997`.
Runtime v3 browser/API access is HTTPS-only. Before launch, provision a
localhost certificate and private key at the `[api.tls]` paths in the init file;
the repository does not retain private keys. Set a 32-to-256-character
operator-local write token for the runtime process in
`ADL_RUNTIME_OBSERVATORY_TOKEN`. Runtime v3 health, readiness, metrics,
Observatory snapshots, and the Observatory WSS feed are public read surfaces
and require no token.

Runtime v3 uses versioned operator probes:

```text
GET https://localhost:20997/v1/health
GET https://localhost:20997/v1/ready
GET https://localhost:20997/v1/metrics
GET https://localhost:20997/v1/observatory
```

The HTML Observatory reads its Runtime v3 browser API base and endpoints from
`demos/html-observatory/runtime-v3.config.json`. Keep that file on the same
static host as `index.html`; if it cannot be loaded, the browser falls back to
the versioned defaults listed above.

Do not use unversioned `/health` or `/ready` paths for Runtime v3 overnight
monitoring. `/v1/ready` is the watcher-ready signal: it returns `200` with
`ready: true` only when the runtime is observability-ready and weather
freshness is not stale; it returns `503` with `degraded_reasons` such as
`weather_stale` when the runtime is reachable but should not be reported as
fully ready.

Operator login is required only before the browser sends WSS-authenticated ACIP
work. Signed Runtime v3 control envelopes can be submitted through `/v1/control`
without putting a bearer token in the browser URL; Runtime v3 still verifies the
signature, principal, capability, runtime identity, and command policy before
execution. To enable WSS writes for the current browser tab, set the same token
without putting it in the URL or repository, then reconnect:

```js
sessionStorage.setItem("adl.runtimeV3.observatoryToken", "<operator-local-token>");
```

Serve this Observatory from the allowed HTTPS origin
`https://localhost:8765` using the shared localhost certificate identity for
the static host and Runtime v3 API, then open:

```text
https://localhost:8765/demos/html-observatory/
```

The token elevates only that WSS connection for writes; `/v1/control` remains a
signed-command endpoint and signature verification plus canonical ingress
policy still apply. The shared localhost certificate must be trusted by the
browser for both `https://localhost:8765` and `https://localhost:20997`. The
kernel terminates its own TLS connection; a local API Gateway or sidecar is not
required.

One COTS way to serve the Observatory over local HTTPS is Caddy:

```sh
caddy file-server --root . --domain localhost --listen :8765
```

Caddy provisions the local HTTPS listener; browser trust remains an explicit
operator prerequisite and this repository does not modify the host trust store.

The default init file also configures a high-cardinality Runtime v3 agent
population with `count = 10000` and a bounded sample. The Observatory shows the
true total while rendering only the sample, so live polis-scale demos do not
create 10,000 DOM nodes.

For an externally reachable polis, copy that init file to an operator-local
path and configure the runtime host interface, public route, certificate paths,
and allowed origins there. This packaged browser dashboard only sends Runtime v3
WSS/auth traffic to the trusted localhost API base. Native clients may still
read the runtime-owned Observatory feed without an Origin header when the
operator's deployment policy allows it.

The Runtime v3 browser path consumes the public runtime-owned read feed at
`/v1/observatory`, the watcher readiness surface at `/v1/ready`, and the public
read stream at `/v1/observatory/ws`. The Operator Channel can submit a complete
pre-signed `adl.runtime.control_command.v1` envelope to `/v1/control`, and can
log in for WSS writes when authenticated socket control is available. The
browser never creates or stores the signing key; Runtime v3 verifies the
signature, principal, capability, runtime identity, and command policy before
execution. Logging out reconnects the public read stream without write
authority.

The browser-served dashboard only receives CORS permission when its origin is
listed in `[observatory].allowed_origins`. If the Runtime v3 API is reachable by
curl but the browser refuses the cross-origin fetch, the dashboard stays on the
retained mirror and reports the live fetch failure instead of claiming a live
Runtime v3 path.

Opening `index.html` directly may show the fallback shell in browsers that block
local `fetch()` for files. The retained proof is the local-server path plus the
validator below.

## Validate

```sh
bash adl/tools/test_v0917_html_observatory_integrated_proof.sh
```

## Claim Boundary

The validation lane includes a real TLS client against a running Runtime v3
endpoint and separately retains the JavaScript mock as static
rendering-contract coverage. This proves that the HTML Observatory can render an auto-refreshing CSM
panopticon over retained publishable runtime API responses, and can upgrade to a
live loopback CSM panopticon when the running CSM API base is supplied. It can
also consume the public Runtime v3 `/v1/observatory` read feed when
Runtime v3 is available at its configured local API base. It renders the retained
bounded runtime capture through a polished investor-facing operator UI, while exposing
CSM API, CSM service, CloudWatch heartbeat, ACIP-SNS projection proof, Runtime
v3 status, and WP-08 linkage status. Its Operator Channel can submit
pre-signed commands to `/v1/control`, while Runtime v3 retains signature and
policy authority. It does not claim browser-owned AWS publish authority, Unity
completion, Runtime v2 decommission, full AWS signal
bridge completion, S3 ObsMem archive completion, or v0.92 runtime completion.
