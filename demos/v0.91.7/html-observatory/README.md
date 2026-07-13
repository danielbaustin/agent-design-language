# ADL HTML Observatory

This demo is the v0.91.7 HTML Observatory integrated proof for #4690.

It adapts the Magic UI Pro AI Agent Template direction, with the Magic UI
Devtool Template used for denser dashboard composition cues, into a reviewable
CSM polis panopticon without importing account credentials or private template
metadata into the repository. The first-class mode is a compact control-room
dashboard over `/status`, `/health`, `/ready`, `/metrics`, and `/events`, with
runtime KPIs, agent graph preview, event tail, CSM API status, CloudWatch
linkage, governance proof, and operator communication status visible in the
first dashboard viewport. It auto-refreshes the retained publishable CSM API
response artifacts from #4976 as a runtime mirror, and upgrades to live loopback
polling when an operator supplies the currently running CSM API base. The
retained runtime packet remains the fallback proof surface if the CSM API mirror
cannot load. The page also consumes the retained CSM runtime Observatory packet
and operator report from the v0.91.7 Soak 2 evidence root, plus the current CSM
runtime administration and AWS linkage evidence.

The primary dashboard is intentionally fixed to the viewport: the page itself
does not scroll, while the event stream and inspector areas own bounded internal
overflow. The visible shell uses local inline SVG icons, role-specific topology
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
mirror by default. When a loopback API base is supplied, it polls the running
CSM API directly. For Runtime v3, the explicit opt-in path is
`?runtime=v3&runtimeApiBase=http://127.0.0.1:20997`, which consumes the
runtime-owned `/v1/observatory` read feed. Runtime v3 control mutation remains
signed-command-only through `/v1/control`; the browser has no shutdown,
mutation, CloudWatch, SNS, or state authority. The CSM API panel intentionally
presents the standalone `csm` runtime ownership boundary from #4929 when the
retained/default mirror is selected. The CloudWatch panel presents the retained
live heartbeat proof from WP-08 #4684. The AWS linkage lane includes #4684
through #4688 so closed heartbeat, ACIP-SNS, and SSM lanes remain distinct from
open full-bridge and S3 archive work. The communication rail can prepare an
ACIP-shaped operator message envelope, mirror the retained #4685 ACIP-SNS proof,
and check a live loopback CSM `/events` endpoint when an operator supplies the
API base. Live SNS/SQS mutation remains runtime/tool-owned and is not performed
by the browser surface.

## Run

From the repository root:

```sh
python3 -m http.server 8765
```

Then open:

```text
http://127.0.0.1:8765/demos/v0.91.7/html-observatory/
```

For the v0.91.7 real-runtime test path, start the existing repo `csm` binary in
a separate terminal and point the dashboard at that loopback base:

```sh
adl/target/debug/csm api serve \
  --spec docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/full/agent.yaml \
  --bind 127.0.0.1:24645 \
  --max-requests 25 \
  --idle-timeout-ms 60000 \
  --json
```

```text
http://127.0.0.1:8765/demos/v0.91.7/html-observatory/?csmApiBase=http://127.0.0.1:24645&live=1
```

For the Runtime v3 opt-in path, start the Runtime v3 kernel control API and
point the dashboard at port `20997`:

```sh
adl-runtime-kernel serve
```

```text
http://127.0.0.1:8765/demos/v0.91.7/html-observatory/?runtime=v3&runtimeApiBase=http://127.0.0.1:20997&live=1
```

The Runtime v3 browser path consumes only the runtime-owned read feed at
`/v1/observatory`. It does not send signed control commands, does not change the
default runtime, and does not authorize Runtime v2 decommission.

The current browser-served dashboard keeps the same loopback-only policy as the
runtime API. If the CSM API is reachable by curl but the browser refuses the
cross-port fetch, the dashboard stays on the retained mirror and reports the
live loopback failure instead of claiming a live WebSocket or remote API path.

Opening `index.html` directly may show the fallback shell in browsers that block
local `fetch()` for files. The retained proof is the local-server path plus the
validator below.

## Validate

```sh
bash adl/tools/test_v0917_html_observatory_integrated_proof.sh
```

## Claim Boundary

This proves that the HTML Observatory can render an auto-refreshing CSM
panopticon over retained publishable runtime API responses, and can upgrade to a
live loopback CSM panopticon when the running CSM API base is supplied. It can
also consume the Runtime v3 `/v1/observatory` read feed when Runtime v3 is
selected explicitly on loopback port `20997`. It renders the retained
bounded runtime capture through a polished investor-facing operator UI, while exposing
CSM API, CSM service, CloudWatch heartbeat, ACIP-SNS projection proof, Runtime
v3 opt-in status, and WP-08 linkage status. It does not claim direct runtime
mutation, browser-owned AWS publish authority, public/remote API exposure, Unity
completion, default Runtime v3 cutover, Runtime v2 decommission, full AWS signal
bridge completion, S3 ObsMem archive completion, or v0.92 runtime completion.
