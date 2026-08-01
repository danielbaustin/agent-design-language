#!/usr/bin/env python3
"""Validate the v0.91.7 HTML Observatory integrated proof surface."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import textwrap
from pathlib import Path
from typing import Any


PACKET_REF = (
    "../../../docs/milestones/v0.91.7/review/runtime/soak2_4682/"
    "agent_lifecycle/runtime_v2/observatory/visibility_packet.json"
)
REPORT_REF = (
    "../../../docs/milestones/v0.91.7/review/runtime/soak2_4682/"
    "agent_lifecycle/runtime_v2/observatory/operator_report.md"
)
CSM_SERVICE_REF = "../../../docs/milestones/v0.91.7/review/runtime/csm_service_4903/service/service_manifest.json"
CSM_API_REF = "../../../docs/milestones/v0.91.7/review/runtime/CSM_RUNTIME_API_4929.md"
CLOUDWATCH_REF = "../../../docs/milestones/v0.91.7/review/runtime/wp08_heartbeat_4684/live_heartbeat_summary.json"
CLOUDWATCH_EVENTS_REF = "../../../docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/aws/cloudwatch_recent_events.redacted.json"
ACIP_SNS_REF = "../../../docs/milestones/v0.91.7/review/runtime/wp08_acip_sns_4685/acip_sns_summary.json"
SNS_RESOURCE_REF = "../../../docs/milestones/v0.91.7/review/runtime/wp08_acip_sns_4685/sns_resource_summary.json"
CSM_STATUS_REF = "../../../docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/status.json"
CSM_HEALTH_REF = "../../../docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/health.json"
CSM_READY_REF = "../../../docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/ready.json"
CSM_METRICS_REF = "../../../docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/metrics.json"
CSM_EVENTS_REF = "../../../docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api/events.json"
RUNTIME_V3_OBSERVATORY_ENDPOINT = "https://localhost:20997/v1/observatory"


def fail(message: str) -> None:
    raise SystemExit(f"FAIL: {message}")


def read_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        value = json.load(handle)
    if not isinstance(value, dict):
        fail(f"{path} must contain a JSON object")
    return value


def assert_contains(label: str, haystack: str, needle: str) -> None:
    if needle not in haystack:
        fail(f"{label} missing {needle!r}")


def assert_not_contains(label: str, haystack: str, needle: str) -> None:
    if needle in haystack:
        fail(f"{label} contains forbidden text {needle!r}")


def run_js_view_model(
    js_path: Path,
    packet_path: Path,
    report_path: Path,
    service_path: Path,
    api_path: Path,
    cloudwatch_path: Path,
    cloudwatch_events_path: Path,
    acip_sns_path: Path,
    sns_resource_path: Path,
    csm_status_path: Path,
    csm_health_path: Path,
    csm_ready_path: Path,
    csm_metrics_path: Path,
    csm_events_path: Path,
) -> dict[str, Any]:
    node_program = textwrap.dedent(
        f"""
        (async () => {{
        const fs = require("fs");
        const vm = require("vm");
        const source = fs.readFileSync({json.dumps(str(js_path))}, "utf8");
        const packet = JSON.parse(fs.readFileSync({json.dumps(str(packet_path))}, "utf8"));
        const reportText = fs.readFileSync({json.dumps(str(report_path))}, "utf8");
        const serviceManifest = JSON.parse(fs.readFileSync({json.dumps(str(service_path))}, "utf8"));
        const apiText = fs.readFileSync({json.dumps(str(api_path))}, "utf8");
        const cloudwatchSummary = JSON.parse(fs.readFileSync({json.dumps(str(cloudwatch_path))}, "utf8"));
        const cloudwatchEvents = JSON.parse(fs.readFileSync({json.dumps(str(cloudwatch_events_path))}, "utf8"));
        const acipSnsSummary = JSON.parse(fs.readFileSync({json.dumps(str(acip_sns_path))}, "utf8"));
        const snsResourceSummary = JSON.parse(fs.readFileSync({json.dumps(str(sns_resource_path))}, "utf8"));
        const retainedRefs = {{
          statusRef: {json.dumps(str(csm_status_path))},
          healthRef: {json.dumps(str(csm_health_path))},
          readyRef: {json.dumps(str(csm_ready_path))},
          metricsRef: {json.dumps(str(csm_metrics_path))},
          eventsRef: {json.dumps(str(csm_events_path))}
        }};
        const retainedFiles = new Map([
          [retainedRefs.statusRef, fs.readFileSync(retainedRefs.statusRef, "utf8")],
          [retainedRefs.healthRef, fs.readFileSync(retainedRefs.healthRef, "utf8")],
          [retainedRefs.readyRef, fs.readFileSync(retainedRefs.readyRef, "utf8")],
          [retainedRefs.metricsRef, fs.readFileSync(retainedRefs.metricsRef, "utf8")],
          [retainedRefs.eventsRef, fs.readFileSync(retainedRefs.eventsRef, "utf8")]
        ]);
        const runtimeV3Feed = JSON.stringify({{
          schema: "adl.runtime_v3.observatory_feed.v2",
          runtime_instance_id: "runtime-v3-test",
          default_runtime_changed: false,
          runtime_selection: "runtime_v3_explicit_opt_in",
          control: {{
            port: 20997,
            read_endpoint: "/v1/observatory",
            signed_command_endpoint: "/v1/control",
            signed_commands_required_for_mutation: true,
            browser_mutation_authority: false
          }},
          health: {{
            observability_ready: true,
            snapshot: {{
              schema: "adl.runtime.control_snapshot.v1",
              revision: 7,
              topology_generation: 3,
              components: {{ runtime_api: "running", checkpoint: "running" }},
              restart_counts: {{}},
              queues: {{}},
              clock: {{ status: "authoritative", source: "sntp", unix_millis: 1789000000 }},
              continuity_head: {{ generation: 2, accepted_through: 19, topology_hash: "topology", config_hash: "config", integrity: "snapshot" }},
              lifecycle: "running",
              event_count: 2,
              observability: {{ status: "ready" }},
              observability_ready: true
            }}
          }},
          weather: {{
            schema: "adl.runtime.weather_health.v1",
            resource_state: "healthy",
            shutdown_decision: "continue",
            gpu_proof_state: "unavailable_not_pass",
            cloudwatch_route: "vector.runtime_v3_cloudwatch_emf",
            sample: {{
              platform: "test",
              cpu_basis_points: {{ value: 250, source: "fixture" }},
              per_core_basis_points: {{ value: [250], source: "fixture" }},
              memory_total_bytes: {{ value: 1024, source: "fixture" }},
              memory_available_bytes: {{ value: 768, source: "fixture" }},
              disks: {{ value: [], source: "fixture" }},
              network_received_bytes: {{ value: 13, source: "fixture" }},
              network_transmitted_bytes: {{ value: 21, source: "fixture" }},
              max_temperature_millicelsius: {{ value: 42000, source: "fixture" }},
              gpus: {{ value: [], source: "optional_platform_adapter" }}
            }}
          }},
          weather_freshness: {{
            observed_at_unix_millis: 1789000000,
            age_millis: 250,
            stale_after_millis: 2000,
            stale: false
          }},
          continuity: {{ checkpoint: {{ generation: 2, accepted_through: 19, topology_hash: "topology", config_hash: "config", integrity: "snapshot" }} }},
          agents: {{
            total_count: 10000,
            rendered_sample_count: 3,
            sample: [
              {{ id: "agent-00001", label: "Runtime agent 1", role: "runtime agent", state: "running", detail: "sample 1 of 10000" }},
              {{ id: "agent-00002", label: "Runtime agent 2", role: "runtime agent", state: "running", detail: "sample 2 of 10000" }},
              {{ id: "agent-00003", label: "Runtime agent 3", role: "runtime agent", state: "running", detail: "sample 3 of 10000" }}
            ]
          }},
          proof: {{
            default_runtime_switch_authorized: false,
            runtime_v2_decommission_authorized: false,
            sidecar_required: false,
            vector_cloudwatch_route: "vector.runtime_v3_cloudwatch_emf"
          }},
          events: [
            {{ sequence: 1, monotonic_millis: 5, component: "runtime_api", event: "components_ready", correlation_id: null }}
          ]
        }});
        const livePayloads = new Map([
          ["http://localhost:49210/status", retainedFiles.get(retainedRefs.statusRef)],
          ["http://localhost:49210/health", retainedFiles.get(retainedRefs.healthRef)],
          ["http://localhost:49210/ready", retainedFiles.get(retainedRefs.readyRef)],
          ["http://localhost:49210/metrics", retainedFiles.get(retainedRefs.metricsRef)],
          ["http://localhost:49210/events", retainedFiles.get(retainedRefs.eventsRef)],
          ["https://localhost:20997/v1/observatory", runtimeV3Feed]
        ]);
        const textWrites = [];
        const datasetWrites = [];
        const timers = [];
        const elements = new Map();
        function element(id, extra = {{}}) {{
          const node = {{
            id,
            value: "",
            textContent: "",
            dataset: {{}},
            innerHTML: "",
            href: "",
            setAttribute: (name, value) => {{ node[name] = value; }},
            removeAttribute: (name) => {{ delete node[name]; }},
            addEventListener: (name, callback) => {{ node[`on${{name}}`] = callback; }},
            ...extra
          }};
          elements.set(id, node);
          return node;
        }}
        [
          "live-api-base",
          "dashboard-live-api-base",
          "runtime-api-base",
          "connect-live",
          "refresh-live",
          "stop-live",
          "dashboard-connect-live",
          "dashboard-refresh-live",
          "dashboard-stop-live",
          "dashboard-live-test-status",
          "dashboard-live-test-detail",
          "live-status",
          "hero-live-mode",
          "hero-map-mode",
          "hero-event-title",
          "statusbar-mode",
          "statusbar-websocket",
          "statusbar-updated",
          "statusbar-indicator",
          "agent-count",
          "hero-agent-count",
          "live-readiness",
          "hero-ready-state",
          "hero-agent-map",
          "live-updated",
          "live-event-count",
          "hero-event-count",
          "hero-gauge-agents",
          "hero-gauge-events",
          "hero-gauge-metrics",
          "hero-gauge-ready",
          "agent-heartbeat",
          "agent-state",
          "hero-event-detail",
          "live-metric-count",
          "hero-ready-detail",
          "hero-latest-event",
          "panopticon-map",
          "live-agent-list",
          "live-signal-list",
          "live-metric-list",
          "live-event-stream",
          "hero-event-stream",
          "dashboard-focus-kicker",
          "dashboard-focus-title",
          "dashboard-focus-status",
          "dashboard-focus-detail",
          "dashboard-focus-list",
          "dashboard-focus-link",
          "compact-comms-proof",
          "export-proof",
          "prepare-envelope",
          "operator-write-token",
          "operator-login",
          "operator-logout",
          "operator-auth-status",
          "signed-control-command",
          "send-signed-command",
          "operator-control-result",
          "top-mode-select"
        ].forEach((id) => element(id));
        elements.get("dashboard-live-api-base").value = "";
        const dashboardLinks = [
          {{ dataset: {{ dashboardLink: "runtime" }}, setAttribute: () => {{}}, removeAttribute: () => {{}}, addEventListener: () => {{}} }},
          {{ dataset: {{ dashboardLink: "agents" }}, setAttribute: () => {{}}, removeAttribute: () => {{}}, addEventListener: () => {{}} }},
          {{ dataset: {{ dashboardLink: "csm-api" }}, setAttribute: () => {{}}, removeAttribute: () => {{}}, addEventListener: () => {{}} }},
          {{ dataset: {{ dashboardLink: "cloudwatch" }}, setAttribute: () => {{}}, removeAttribute: () => {{}}, addEventListener: () => {{}} }},
          {{ dataset: {{ dashboardLink: "communication" }}, setAttribute: () => {{}}, removeAttribute: () => {{}}, addEventListener: () => {{}} }},
          {{ dataset: {{ dashboardLink: "governance" }}, setAttribute: () => {{}}, removeAttribute: () => {{}}, addEventListener: () => {{}} }},
          {{ dataset: {{ dashboardLink: "evidence" }}, setAttribute: () => {{}}, removeAttribute: () => {{}}, addEventListener: () => {{}} }}
        ];
        const observatoryElement = {{ dataset: {{
          csmStatusRef: retainedRefs.statusRef,
          csmHealthRef: retainedRefs.healthRef,
          csmReadyRef: retainedRefs.readyRef,
          csmMetricsRef: retainedRefs.metricsRef,
          csmEventsRef: retainedRefs.eventsRef
        }}, setAttribute: (name, value) => {{ observatoryElement[name] = value; }} }};
        const mockDocument = {{
          getElementById: (id) => elements.get(id) || null,
          querySelector: (selector) => selector === ".observatory" ? observatoryElement : null,
          querySelectorAll: (selector) => selector === "[data-dashboard-link]" ? dashboardLinks : []
        }};
        let fetchMode = "immediate";
        const pendingFetches = [];
        function responseFor(ref) {{
          const key = String(ref);
          const body = retainedFiles.get(key) || livePayloads.get(key);
          return body == null
            ? {{ ok: false, status: 404, text: async () => "", json: async () => {{ throw new Error("missing mock payload"); }} }}
            : {{ ok: true, status: 200, text: async () => body, json: async () => JSON.parse(body) }};
        }}
        const mockFetch = async (ref) => {{
          if (fetchMode === "defer") {{
            return new Promise((resolve) => pendingFetches.push({{ ref, resolve }}));
          }}
          return responseFor(ref);
        }};
        const resolvePendingFetches = () => {{
          while (pendingFetches.length) {{
            const pending = pendingFetches.shift();
            pending.resolve(responseFor(pending.ref));
          }}
        }};
        const sessionValues = new Map();
        class MockWebSocket {{
          static CONNECTING = 0;
          static OPEN = 1;
          static CLOSING = 2;
          static CLOSED = 3;
          static instances = [];
          constructor(url) {{
            this.url = String(url);
            this.readyState = MockWebSocket.CONNECTING;
            this.sent = [];
            this.listeners = {{}};
            MockWebSocket.instances.push(this);
          }}
          addEventListener(name, callback) {{
            this.listeners[name] = this.listeners[name] || [];
            this.listeners[name].push(callback);
          }}
          send(value) {{
            this.sent.push(String(value));
          }}
          close(code = 1000, reason = "") {{
            this.readyState = MockWebSocket.CLOSED;
            this.emit("close", {{ code, reason }});
          }}
          emit(name, event = {{}}) {{
            if (name === "open") {{
              this.readyState = MockWebSocket.OPEN;
            }}
            for (const callback of this.listeners[name] || []) {{
              callback(event);
            }}
          }}
        }}
        const mockLocation = {{ search: "?csmApiBase=http://localhost:49210" }};
        const context = {{
          console,
          URL,
          URLSearchParams,
          fetch: mockFetch,
          document: mockDocument,
          location: mockLocation,
          window: {{ location: mockLocation }},
          setInterval: (fn, ms) => {{
            timers.push({{ ms, name: fn && fn.name ? fn.name : "anonymous" }});
            return timers.length;
          }},
          clearInterval: () => {{}},
          sessionStorage: {{
            getItem: (key) => sessionValues.get(String(key)) || null,
            setItem: (key, value) => sessionValues.set(String(key), String(value)),
            removeItem: (key) => sessionValues.delete(String(key))
          }},
          WebSocket: MockWebSocket,
          globalThis: {{}}
        }};
        context.globalThis = context;
        vm.runInNewContext(source, context);
        const viewModel = context.AdlHtmlObservatory.buildViewModel(packet, reportText);
        const integrationViewModel = context.AdlHtmlObservatory.buildIntegrationViewModel({{
          serviceManifest,
          apiText,
          cloudwatchSummary,
          cloudwatchEvents,
          acipSnsSummary,
          snsResourceSummary
        }});
        const operatorEnvelope = context.AdlHtmlObservatory.buildOperatorEnvelope({{
          channel: "acip_sns",
          message: "Request current CSM event tail and runtime readiness.",
          packetId: packet.packet_id,
          acipSnsSummary,
          snsResourceSummary
        }});
        const panopticonViewModel = context.AdlHtmlObservatory.buildPanopticonViewModel({{
          mode: "live",
          fetchedAt: "2026-07-06T19:30:00Z",
          status: {{
            runtime_owner: "csm",
            agent_instance_id: "polis-runtime-1",
            status: "running"
          }},
          health: {{ status: "healthy" }},
          ready: {{ status: "ready" }},
          metrics: {{ active_agents: 3, event_tail_size: 2 }},
          events: {{
            events: [
              {{ message: "{{\\"signal_kind\\":\\"heartbeat\\",\\"runtime_id\\":\\"polis-runtime-1\\",\\"status\\":\\"completed\\"}}" }}
            ]
          }}
        }}, packet);
        const retainedPanopticonViewModel = context.AdlHtmlObservatory.buildPanopticonViewModel({{
          mode: "published",
          fetchedAt: "2026-07-06T19:35:00Z",
          status: {{
            runtime_owner: "csm",
            agent_instance_id: "csm-liveness-4976-full",
            status: "healthy",
            agent_status: {{ state: "idle" }}
          }},
          health: {{ status: "healthy" }},
          ready: {{ ready: "ready" }},
          metrics: {{
            gauges: {{ completed_cycle_count: 87, operator_event_count_observed: 348 }},
            states: {{ agent_state: "idle" }}
          }},
          events: {{
            events: {{
              entries: [
                {{ event: "checkpoint_write", agent_instance_id: "csm-liveness-4976-full", details: {{ result: "completed" }} }}
              ]
            }}
          }}
        }}, packet);
        const retainedSnapshot = await context.AdlHtmlObservatory.fetchRetainedRuntimeSnapshot(retainedRefs);
        const retainedFetchPanopticon = context.AdlHtmlObservatory.buildPanopticonViewModel(retainedSnapshot, packet);
        const liveSnapshot = await context.AdlHtmlObservatory.fetchRuntimeSnapshot("http://localhost:49210");
        const liveFetchPanopticon = context.AdlHtmlObservatory.buildPanopticonViewModel(liveSnapshot, packet);
        mockLocation.search = "?runtime=v3&runtimeApiBase=https://localhost:20997";
        const runtimeV3Snapshot = await context.AdlHtmlObservatory.fetchRuntimeSnapshot("https://localhost:20997");
        const runtimeV3Panopticon = context.AdlHtmlObservatory.buildPanopticonViewModel(runtimeV3Snapshot, packet);
        mockLocation.search = "?csmApiBase=http://localhost:49210";
        context.AdlHtmlObservatory.bindLivePanopticon(packet);
        await new Promise((resolve) => setImmediate(resolve));
        const initialLiveBinding = {{
          base: elements.get("dashboard-live-api-base").value,
          retainedIntervalCount: timers.filter((timer) => timer.name === "refreshRetained").length,
          liveIntervalCount: timers.filter((timer) => timer.name === "refreshLive").length,
          runtimeStatus: elements.get("dashboard-live-test-status").textContent,
          statusbarMode: elements.get("statusbar-mode").textContent
        }};
        elements.get("dashboard-live-api-base").value = "";
        fetchMode = "defer";
        elements.get("refresh-live").onclick();
        elements.get("stop-live").onclick();
        resolvePendingFetches();
        await new Promise((resolve) => setImmediate(resolve));
        const retainedRace = {{
          status: elements.get("live-status").textContent,
          runtimeStatus: elements.get("dashboard-live-test-status").textContent,
          connectionState: observatoryElement["data-live-connection"]
        }};
        fetchMode = "immediate";
        mockLocation.search = "?runtime=v3&runtimeApiBase=https://localhost:20997";
        elements.get("dashboard-live-api-base").value = "https://localhost:20997";
        elements.get("operator-write-token").value = "operator-write-token-5757";
        elements.get("dashboard-connect-live").onclick();
        const socket = MockWebSocket.instances[MockWebSocket.instances.length - 1];
        socket.emit("open");
        elements.get("operator-login").onclick();
        const trustedWss = {{
          endpoint: socket.url,
          authFrame: socket.sent.find((frame) => frame.includes("observatory_ws_auth")) || ""
        }};
        elements.get("stop-live").onclick();
        socket.emit("message", {{ data: runtimeV3Feed }});
        await new Promise((resolve) => setImmediate(resolve));
        const wssRace = {{
          status: elements.get("live-status").textContent,
          runtimeStatus: elements.get("dashboard-live-test-status").textContent,
          websocketStatus: elements.get("statusbar-websocket").textContent
        }};
        const socketCountBeforeRejected = MockWebSocket.instances.length;
        let rejectedUntrustedWss = false;
        try {{
          context.AdlHtmlObservatory.connectRuntimeV3ObservatoryWebSocket(
            "https://example.com?runtimeApiBase=https://localhost:20997",
            () => {{}},
            () => {{}}
          );
        }} catch (_error) {{
          rejectedUntrustedWss = true;
        }}
        const blockedCloudwatchViewModel = context.AdlHtmlObservatory.buildIntegrationViewModel({{
          serviceManifest,
          apiText,
          cloudwatchSummary: {{ ...cloudwatchSummary, status: "blocked" }},
          cloudwatchEvents,
          acipSnsSummary,
          snsResourceSummary
        }});
        process.stdout.write(JSON.stringify({{
          packetId: viewModel.packet.packet_id,
          evidenceLevel: viewModel.packet.source.evidence_level,
          manifoldState: viewModel.packet.manifold.state,
          citizenCount: viewModel.citizens.length,
          serviceCount: viewModel.services.length,
          decisionCounts: viewModel.decisionCounts,
          invariantCount: viewModel.invariants.length,
          latestEvent: viewModel.latestEvent,
          actionCount: viewModel.availableActions.length + viewModel.disabledActions.length,
          reportLoaded: viewModel.reportText.includes("CSM Observatory Operator Report"),
          serviceRows: integrationViewModel.serviceRows,
          cloudwatchRows: integrationViewModel.cloudwatchRows,
          acipRows: integrationViewModel.acipRows,
          parsedCloudWatchEventCount: integrationViewModel.parsedEvents.length,
          latestCloudWatchTarget: integrationViewModel.latestEvent.transport?.target_kind || "",
          awsLinkageCount: context.AdlHtmlObservatory.AWS_LINKAGES.length,
          openAwsLinkageCount: context.AdlHtmlObservatory.AWS_LINKAGES.filter((item) => item.state === "open").length,
          operatorEnvelope,
          loopbackPolicy: {{
            localhostHttp: context.AdlHtmlObservatory.isLoopbackApiBase("http://localhost:49210"),
            runtimeTrustedLocalhost: context.AdlHtmlObservatory.isRuntimeV3ApiBase("https://localhost:20997"),
            runtimeRemoteHttps: context.AdlHtmlObservatory.isRuntimeV3ApiBase("https://runtime-gateway-host"),
            runtimeWrongPort: context.AdlHtmlObservatory.isRuntimeV3ApiBase("https://localhost:8765"),
            runtimeUrlCredentials: context.AdlHtmlObservatory.isRuntimeV3ApiBase("https://operator:token@localhost:20997"),
            runtimeUrlQuery: context.AdlHtmlObservatory.isRuntimeV3ApiBase("https://localhost:20997?runtimeApiBase=https://example.com"),
            runtimePath: context.AdlHtmlObservatory.isRuntimeV3ApiBase("https://localhost:20997/collect"),
            runtimeHttp: context.AdlHtmlObservatory.isRuntimeV3ApiBase("http://localhost:20997"),
            remoteHttp: context.AdlHtmlObservatory.isLoopbackApiBase("https://example.com"),
            malformed: context.AdlHtmlObservatory.isLoopbackApiBase("not a url")
          }},
          closedAwsIssues: context.AdlHtmlObservatory.AWS_LINKAGES.filter((item) => item.state === "closed").map((item) => item.issue),
          openAwsIssues: context.AdlHtmlObservatory.AWS_LINKAGES.filter((item) => item.state === "open").map((item) => item.issue),
          panopticon: {{
            mode: panopticonViewModel.mode,
            agentCount: panopticonViewModel.agents.length,
            signalCount: panopticonViewModel.signals.length,
            metricCount: panopticonViewModel.metrics.length,
            eventCount: panopticonViewModel.events.length,
            readyState: panopticonViewModel.readyState
          }},
          retainedPanopticon: {{
            mode: retainedPanopticonViewModel.mode,
            agentLabels: retainedPanopticonViewModel.agents.map((agent) => agent.label),
            eventCount: retainedPanopticonViewModel.events.length,
            readyState: retainedPanopticonViewModel.readyState
          }},
          retainedFetchPanopticon: {{
            mode: retainedFetchPanopticon.mode,
            agentLabels: retainedFetchPanopticon.agents.map((agent) => agent.label),
            eventCount: retainedFetchPanopticon.events.length,
            metricCount: retainedFetchPanopticon.metrics.length,
            readyState: retainedFetchPanopticon.readyState,
            errorCount: Object.keys(retainedSnapshot.errors || {{}}).length
          }},
          liveFetchPanopticon: {{
            mode: liveFetchPanopticon.mode,
            agentLabels: liveFetchPanopticon.agents.map((agent) => agent.label),
            eventCount: liveFetchPanopticon.events.length,
            metricCount: liveFetchPanopticon.metrics.length,
            readyState: liveFetchPanopticon.readyState,
            errorCount: Object.keys(liveSnapshot.errors || {{}}).length
          }},
          runtimeV3Panopticon: {{
            mode: runtimeV3Panopticon.mode,
            runtimeSelection: runtimeV3Snapshot.runtimeSelection,
            agentLabels: runtimeV3Panopticon.agents.map((agent) => agent.label),
            agentTotal: runtimeV3Panopticon.agentTotal,
            eventCount: runtimeV3Panopticon.events.length,
            metricCount: runtimeV3Panopticon.metrics.length,
            readyState: runtimeV3Panopticon.readyState,
            controlPort: runtimeV3Snapshot.status.control.port,
            mutationAuthority: runtimeV3Snapshot.status.control.browser_mutation_authority,
            decommissionAuthorized: runtimeV3Snapshot.proof.runtime_v2_decommission_authorized,
            defaultSwitchAuthorized: runtimeV3Snapshot.proof.default_runtime_switch_authorized,
            sidecarRequired: runtimeV3Snapshot.proof.sidecar_required,
            weatherAgeMillis: runtimeV3Snapshot.metrics.gauges.weather_age_millis,
            weatherStaleAfterMillis: runtimeV3Snapshot.metrics.gauges.weather_stale_after_millis,
            weatherStale: runtimeV3Snapshot.metrics.states.weather_stale
          }},
          dashboardMirrors: {{
            heroCloudwatchOkLabel: integrationViewModel.cloudwatchSummary.status === "passed" ? "CloudWatch Proven" : context.AdlHtmlObservatory.formatLabel(integrationViewModel.cloudwatchSummary.status || "pending"),
            heroCloudwatchBlockedLabel: blockedCloudwatchViewModel.cloudwatchSummary.status === "passed" ? "CloudWatch Proven" : context.AdlHtmlObservatory.formatLabel(blockedCloudwatchViewModel.cloudwatchSummary.status || "pending"),
            heroReadyLabel: context.AdlHtmlObservatory.formatLabel(retainedFetchPanopticon.readyState),
            heroAgentCount: String(retainedFetchPanopticon.agents.length),
            heroEventCount: String(retainedFetchPanopticon.events.length)
          }},
          liveBinding: {{
            ...initialLiveBinding
          }},
          asyncRace: {{
            retainedStopStatus: retainedRace.status,
            retainedStopRuntimeStatus: retainedRace.runtimeStatus,
            retainedStopConnectionState: retainedRace.connectionState,
            wssStopStatus: wssRace.status,
            wssStopRuntimeStatus: wssRace.runtimeStatus,
            wssStopWebsocketStatus: wssRace.websocketStatus
          }},
          trustedWss: {{
            endpoint: trustedWss.endpoint,
            authFrameSent: trustedWss.authFrame.includes("operator-write-token-5757"),
            rejectedUntrustedWss,
            rejectedUntrustedCreatedSocket: MockWebSocket.instances.length !== socketCountBeforeRejected
          }}
        }}));
        }})().catch((error) => {{
          console.error(error && error.stack ? error.stack : String(error));
          process.exit(1);
        }});
        """
    )
    try:
        completed = subprocess.run(
            ["node", "-e", node_program],
            check=True,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
    except FileNotFoundError:
        fail("node is required for HTML Observatory JS validation")
    except subprocess.CalledProcessError as exc:
        fail(f"HTML Observatory JS validation failed: {exc.stderr.strip()}")
    return json.loads(completed.stdout)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--html", type=Path, required=True)
    parser.add_argument("--css", type=Path, required=True)
    parser.add_argument("--js", type=Path, required=True)
    parser.add_argument("--packet", type=Path, required=True)
    parser.add_argument("--report", type=Path, required=True)
    parser.add_argument("--csm-service", type=Path, required=True)
    parser.add_argument("--csm-api", type=Path, required=True)
    parser.add_argument("--cloudwatch", type=Path, required=True)
    parser.add_argument("--cloudwatch-events", type=Path, required=True)
    parser.add_argument("--acip-sns", type=Path, required=True)
    parser.add_argument("--sns-resource", type=Path, required=True)
    parser.add_argument("--csm-status", type=Path, required=True)
    parser.add_argument("--csm-health", type=Path, required=True)
    parser.add_argument("--csm-ready", type=Path, required=True)
    parser.add_argument("--csm-metrics", type=Path, required=True)
    parser.add_argument("--csm-events", type=Path, required=True)
    args = parser.parse_args()

    html = args.html.read_text(encoding="utf-8")
    css = args.css.read_text(encoding="utf-8")
    js = args.js.read_text(encoding="utf-8")
    packet = read_json(args.packet)
    report = args.report.read_text(encoding="utf-8")
    service = read_json(args.csm_service)
    api_text = args.csm_api.read_text(encoding="utf-8")
    cloudwatch = read_json(args.cloudwatch)
    cloudwatch_events = read_json(args.cloudwatch_events)
    acip_sns = read_json(args.acip_sns)
    sns_resource = read_json(args.sns_resource)
    smoke = run_js_view_model(
        args.js,
        args.packet,
        args.report,
        args.csm_service,
        args.csm_api,
        args.cloudwatch,
        args.cloudwatch_events,
        args.acip_sns,
        args.sns_resource,
        args.csm_status,
        args.csm_health,
        args.csm_ready,
        args.csm_metrics,
        args.csm_events,
    )

    assert_contains("HTML packet ref", html, f'data-packet-ref="{PACKET_REF}"')
    assert_contains("HTML report ref", html, f'data-report-ref="{REPORT_REF}"')
    assert_contains("HTML CSM service ref", html, f'data-csm-service-ref="{CSM_SERVICE_REF}"')
    assert_contains("HTML CSM API ref", html, f'data-csm-api-ref="{CSM_API_REF}"')
    assert_contains("HTML CloudWatch ref", html, f'data-cloudwatch-ref="{CLOUDWATCH_REF}"')
    assert_contains("HTML CloudWatch events ref", html, f'data-cloudwatch-events-ref="{CLOUDWATCH_EVENTS_REF}"')
    assert_contains("HTML ACIP-SNS ref", html, f'data-acip-sns-ref="{ACIP_SNS_REF}"')
    assert_contains("HTML SNS resource ref", html, f'data-sns-resource-ref="{SNS_RESOURCE_REF}"')
    assert_contains("HTML retained CSM status ref", html, f'data-csm-status-ref="{CSM_STATUS_REF}"')
    assert_contains("HTML retained CSM health ref", html, f'data-csm-health-ref="{CSM_HEALTH_REF}"')
    assert_contains("HTML retained CSM ready ref", html, f'data-csm-ready-ref="{CSM_READY_REF}"')
    assert_contains("HTML retained CSM metrics ref", html, f'data-csm-metrics-ref="{CSM_METRICS_REF}"')
    assert_contains("HTML retained CSM events ref", html, f'data-csm-events-ref="{CSM_EVENTS_REF}"')
    assert_contains("HTML title", html, "ADL HTML Observatory - Runtime Proof")
    assert_contains("HTML live Observatory copy", html, "Runtime v3 / live Observatory")
    assert_not_contains("HTML obsolete visible version chip", html, '<span class="version-chip">v0.91.7</span>')
    assert_contains("HTML inline icon sprite", html, 'class="icon-sprite"')
    assert_contains("HTML pulse icon", html, 'href="#icon-pulse"')
    assert_contains("HTML packet icon", html, 'href="#icon-packet"')
    assert_contains("HTML clock icon", html, 'id="icon-clock"')
    assert_contains("HTML database icon", html, 'id="icon-database"')
    assert_contains("HTML bolt icon", html, 'id="icon-bolt"')
    assert_contains("HTML dashboard rail", html, 'class="dashboard-rail"')
    assert_contains("HTML dashboard hero title", html, '<h1 id="hero-title">Panopticon</h1>')
    assert_contains("HTML dashboard runtime KPI", html, 'id="hero-ready-state"')
    assert_contains("HTML dashboard agent KPI", html, 'id="hero-agent-count"')
    assert_contains("HTML dashboard event KPI", html, 'id="hero-event-count"')
    assert_contains("HTML dashboard CloudWatch KPI", html, 'id="hero-cloudwatch-state"')
    assert_contains("HTML dashboard KPI icons", html, 'class="stat-icon"')
    assert_contains("HTML dashboard graph", html, 'id="hero-agent-map"')
    assert_contains("HTML dashboard graph mode", html, 'id="hero-map-mode"')
    assert_contains("HTML dashboard event preview", html, 'id="hero-event-stream"')
    assert_contains("HTML dashboard event preview aria", html, 'aria-live="polite"')
    assert_contains("HTML dashboard CSM inspector", html, 'id="hero-csm-api-status"')
    assert_contains("HTML dashboard CSM mini API list", html, 'id="hero-api-list"')
    assert_contains("HTML dashboard real runtime test card", html, "Real Runtime Test")
    assert_contains("HTML dashboard real runtime base input", html, 'id="dashboard-live-api-base"')
    assert_contains("HTML dashboard real runtime connect", html, 'id="dashboard-connect-live"')
    assert_contains("HTML dashboard real runtime refresh", html, 'id="dashboard-refresh-live"')
    assert_contains("HTML dashboard real runtime stop", html, 'id="dashboard-stop-live"')
    assert_contains("HTML Runtime v3 opt-in port", html, "20997")
    assert_contains("HTML Runtime v3 explicit opt-in query", html, "runtime=v3&runtimeApiBase=https://localhost:20997")
    assert_contains("HTML dashboard communication inspector", html, 'id="hero-communication-status"')
    assert_contains("HTML dashboard status bar", html, 'class="dashboard-statusbar"')
    assert_contains("HTML topbar capture time field", html, "Capture Time")
    assert_contains("HTML statusbar last update field", html, 'Last Update <strong id="statusbar-updated">pending</strong>')
    assert_contains("HTML statusbar state indicator", html, 'id="statusbar-indicator"')
    assert_contains("HTML statusbar Runtime v3 opt-in source", html, "Runtime v3 opt-in + CSM Runtime + AWS CloudWatch + ACIP/SNS")
    assert_contains("HTML source-driven capture readout", html, 'id="hero-uptime">pending</strong>')
    assert_contains("HTML source-driven rail capture", html, 'id="rail-capture-time">pending</strong>')
    assert_contains("HTML source-driven gauge agents", html, 'id="hero-gauge-agents"')
    assert_contains("HTML published mirror default mode", html, '<option value="published">Published Mirror</option>')
    assert_contains("HTML retained mirror mode", html, '<option value="retained">Retained Mirror</option>')
    assert_contains("HTML live loopback mode", html, '<option value="live">Live Loopback</option>')
    assert_contains("HTML truthful runtime mirror label", html, "<span>Runtime Mirror</span>")
    assert_contains("HTML source-driven event title", html, 'id="hero-event-title">Event Stream</h2>')
    assert_contains("HTML dashboard truthful operator CTA", html, "Draft operator probe")
    assert_contains("HTML dashboard focus action", html, "Focus panopticon")
    assert_contains("HTML dashboard selected-surface list", html, 'id="dashboard-focus-list"')
    assert_contains("HTML dashboard comms rail link", html, 'data-dashboard-link="communication"')
    assert_contains("HTML dashboard compact ACIP proof", html, 'id="compact-comms-proof"')
    if "Open panopticon" in html or "Open surface" in html:
      fail("dashboard links overpromise opening hidden lower sections")
    if "Send operator message" in html:
      fail("dashboard CTA overclaims live operator send capability")
    if "Send Message" in html:
      fail("dashboard CTA overclaims live operator send capability")
    if "Panopticon online" in html:
      fail("dashboard title overclaims live panopticon state")
    if "Event Stream (Live)" in html or "<option>Live</option>" in html:
      fail("dashboard statically overclaims live mode")
    if "Live Mirror" in html:
      fail("dashboard statically overclaims live mirror mode")
    for hardcoded_live_value in ("02:14", "10:42", "1,284", "120ms"):
      if hardcoded_live_value in html:
        fail(f"dashboard contains hard-coded live-looking telemetry: {hardcoded_live_value}")
    assert_contains("HTML panopticon section", html, "CSM polis panopticon")
    assert_contains("HTML live connect control", html, 'id="connect-live"')
    assert_contains("HTML live agents surface", html, 'id="live-agent-list"')
    assert_contains("HTML live event stream", html, 'id="live-event-stream"')
    assert_contains("HTML CSM API section", html, "CSM local control plane")
    assert_contains("HTML CloudWatch section", html, "CloudWatch heartbeat")
    assert_contains("HTML AWS linkages section", html, "AWS runtime linkages")
    assert_contains("HTML communication section", html, "ACIP event channel")
    assert_contains("HTML communication input", html, 'id="runtime-api-base"')
    assert_contains("HTML communication proof list", html, 'id="communication-proof-list"')
    if '<option value="cloudwatch">CloudWatch heartbeat</option>' in html:
      fail("communication channel exposes CloudWatch option without a CloudWatch envelope")
    assert_contains("HTML governance section", html, "Freedom gate")
    assert_contains("HTML evidence section", html, "Same packet, same report, same boundary.")
    assert_contains("CSS responsive layout", css, "@media (max-width: 980px)")
    assert_contains("CSS dashboard rail", css, ".dashboard-rail")
    assert_contains("CSS dashboard core", css, ".dashboard-core")
    assert_contains("CSS dashboard graph", css, ".hero-agent-map")
    assert_contains("CSS dashboard graph nodes", css, ".hero-agent-node")
    assert_contains("CSS dashboard icons", css, ".stat-icon")
    assert_contains("CSS fixed cockpit overflow", css, ".observatory > .panopticon-shell")
    assert_contains("CSS dashboard event table", css, ".event-table-header")
    assert_contains("CSS dashboard API mini rows", css, ".api-mini-row")
    assert_contains("CSS dashboard real runtime test card", css, ".runtime-test-card")
    assert_contains("CSS dashboard real runtime actions", css, ".runtime-test-actions")
    assert_contains("CSS responsive inspector strip", css, ".ops-sidecar .inspector-stack")
    assert_contains("CSS dashboard status bar", css, ".dashboard-statusbar")
    assert_contains("CSS dashboard selected-surface chips", css, ".dashboard-focus-item")
    assert_contains("CSS compact ACIP proof chips", css, ".compact-proof-chip")
    assert_contains("CSS statusbar state indicator", css, '.dashboard-statusbar b[data-state="published"]')
    assert_contains("CSS graph node icon treatment", css, ".node-icon")
    assert_contains("CSS orbit visualization", css, ".orbit-map")
    assert_contains("CSS Magic UI inspired card styling", css, ".proof-card")
    assert_contains("JS packet loader", js, "loadJson(packetRef)")
    assert_contains("JS report loader", js, "loadText(reportRef)")
    assert_contains("JS view model", js, "buildViewModel")
    assert_contains("JS CSM integration view model", js, "buildIntegrationViewModel")
    assert_contains("JS AWS linkage state", js, "AWS_LINKAGES")
    assert_contains("JS communication envelope", js, "buildOperatorEnvelope")
    assert_contains("JS ACIP envelope", js, 'schema: "acip.message.v1"')
    assert_contains("JS SNS projection envelope", js, 'live_publish_claimed: false')
    assert_contains("JS events endpoint check", js, "checkEventsEndpoint")
    assert_contains("JS runtime snapshot polling", js, "fetchRuntimeSnapshot")
    assert_contains("JS Runtime v3 observatory feed polling", js, "fetchRuntimeV3ObservatorySnapshot")
    assert_contains("JS Runtime v3 observatory endpoint", js, 'RUNTIME_V3_OBSERVATORY_ENDPOINT = "/v1/observatory"')
    assert_contains("JS Runtime v3 observatory schema", js, 'RUNTIME_V3_OBSERVATORY_SCHEMA = "adl.runtime_v3.observatory_feed.v2"')
    assert_contains("JS trusted Runtime v3 origin normalizer", js, "normalizeTrustedRuntimeV3ApiBase")
    assert_contains("JS trusted Runtime v3 localhost port", js, 'parsed.port !== "20997"')
    assert_contains("JS trusted Runtime v3 root path", js, 'parsed.pathname !== "/"')
    assert_contains("JS shared live generation guard", js, "isCurrentLiveGeneration")
    assert_contains("JS retained generation guard", js, "refreshRetained = async (extraErrors = {}, requestGeneration = nextLiveGeneration())")
    assert_not_contains("JS public Runtime v3 reads omit bearer authentication", js, "Authorization: `Bearer ${readToken}`")
    assert_contains("JS Runtime v3 write login", js, "authenticateRuntimeV3ObservatorySocket")
    assert_contains("JS Runtime v3 login result handling", js, 'frame.status === "authenticated"')
    assert_contains("JS Runtime v3 signed command send", js, 'liveSocket.send(JSON.stringify(command))')
    assert_contains("HTML Runtime v3 write login", html, 'id="operator-login"')
    assert_contains("HTML Runtime v3 signed command input", html, 'id="signed-control-command"')
    assert_contains("JS Runtime v3 weather staleness", js, "weather_stale_after_millis")
    assert_contains("JS Runtime v3 explicit opt-in selection", js, "runtime_v3_explicit_opt_in")
    assert_contains("JS runtime query base bootstrap", js, "getQueryApiBase")
    assert_contains("JS runtime auto-connect gate", js, "shouldAutoConnectLive")
    assert_contains("JS dashboard runtime status mirror", js, 'setText("dashboard-live-test-status"')
    assert_contains("JS Runtime v3 display version", js, 'OBSERVATORY_VERSION = "Runtime v3"')
    assert_contains("HTML Runtime v3 version chip", html, '<span class="version-chip">Runtime v3</span>')
    assert_contains(
        "HTML Runtime v3 live Observatory eyebrow",
        html,
        "Runtime v3 / live Observatory / CSM polis control room",
    )
    assert_contains("JS display claim boundary normalization", js, "displayClaimBoundary")
    assert_contains("JS display packet label normalization", js, "displayPacketId")
    assert_contains("JS retained runtime mirror polling", js, "fetchRetainedRuntimeSnapshot")
    assert_contains("JS dashboard timestamp formatter", js, "formatTimestampLabel")
    assert_contains("JS CSM events entries normalizer", js, "normalizeEventEntries")
    assert_contains("JS panopticon view model", js, "buildPanopticonViewModel")
    assert_contains("JS panopticon renderer", js, "renderPanopticon")
    assert_contains("JS loopback API policy", js, "isLoopbackApiBase")
    assert_contains("JS dashboard graph renderer", js, 'renderRows("hero-agent-map"')
    assert_contains("JS role-based graph icons", js, "iconForAgent")
    assert_contains("JS graph node SVG icons", js, 'class="node-icon"')
    assert_contains("JS dashboard event renderer", js, 'renderRows("hero-event-stream"')
    assert_contains("JS dashboard event table header", js, "event-table-header")
    assert_contains("JS dashboard CSM mirror", js, 'setText("hero-csm-api-status"')
    assert_contains("JS dashboard CSM API mini renderer", js, 'renderRows("hero-api-list"')
    assert_contains("JS dashboard CSM API actual status endpoint", js, '"/status"')
    if "/api/status" in js or "200 OK" in js or "${index + 9}ms" in js:
      fail("dashboard CSM API mini rows contain fake paths, status, or latency")
    assert_contains("JS dashboard communication mirror", js, 'setText("hero-communication-status"')
    assert_contains("JS dashboard selected-surface renderer", js, 'renderRows("dashboard-focus-list"')
    assert_contains("JS compact ACIP proof renderer", js, 'renderRows("compact-comms-proof"')
    assert_contains("JS dashboard CloudWatch fail-closed label", js, 'formatLabel(cloudwatchStatus)')
    assert_contains("JS source-driven capture readout", js, 'setText("hero-uptime"')
    assert_contains("JS current operator time formatter", js, "formatCurrentTimestampLabel")
    assert_contains("JS source-driven kernel state", js, 'setDataset("hero-agent-map", "state"')
    assert_contains("JS source-driven gauges", js, 'setText("hero-gauge-agents"')
    assert_contains("JS source-driven event title", js, 'setText("hero-event-title"')
    assert_contains("JS source-driven statusbar", js, 'setText("statusbar-mode"')
    assert_contains("HTML WebSocket lifecycle statusbar", html, 'id="statusbar-websocket"')
    assert_contains("JS WebSocket lifecycle statusbar", js, 'setText("statusbar-websocket"')
    for websocket_state in ("connecting", "connected", "disconnected", "stopped"):
      assert_contains(
          f"JS WebSocket {websocket_state} status",
          js,
          f'setText("statusbar-websocket", "{websocket_state}")',
      )
    assert_contains("JS statusbar current update timestamp", js, 'setText("statusbar-updated", vm.mode === "live" ? formatTimestampLabel(vm.fetchedAt) : formatCurrentTimestampLabel())')
    assert_contains("JS statusbar state indicator", js, 'setDataset("statusbar-indicator"')

    if packet.get("packet_id") != "v0916-runtime-soak-observatory-packet-0001":
      fail("unexpected runtime packet id")
    if packet.get("source", {}).get("evidence_level") != "bounded_local_runtime_capture":
      fail("runtime packet is not the retained bounded local runtime capture")
    if "CSM Observatory Operator Report" not in report:
      fail("operator report identity missing")
    if service.get("schema") != "adl.csm.service_manifest.v1":
      fail("CSM service manifest schema mismatch")
    if service.get("runtime_owner") != "csm":
      fail("CSM service manifest does not record csm runtime ownership")
    for endpoint in ("csm api serve --spec <agent-spec.yaml>", "/status", "/health", "/ready", "/metrics", "/events"):
      if endpoint not in api_text:
        fail(f"CSM API proof missing {endpoint}")
    if cloudwatch.get("schema") != "adl.wp08.heartbeat_live_proof.v1":
      fail("CloudWatch heartbeat proof schema mismatch")
    if cloudwatch.get("status") != "passed":
      fail("CloudWatch heartbeat proof did not pass")
    if cloudwatch.get("cloudwatch", {}).get("event_count", 0) < 1:
      fail("CloudWatch heartbeat proof has no retained events")
    if cloudwatch.get("heartbeat", {}).get("target_kind") != "cloudwatch_logs":
      fail("CloudWatch heartbeat target is not cloudwatch_logs")
    redaction = cloudwatch.get("redaction", {})
    if redaction.get("credentials_recorded") is not False or redaction.get("raw_account_id_recorded") is not False:
      fail("CloudWatch proof redaction posture is not operations safe")
    if len(cloudwatch_events.get("events", [])) < 1:
      fail("CloudWatch event tail is empty")
    if acip_sns.get("schema") != "adl.wp08.acip_sns_live_proof.v1":
      fail("ACIP-SNS proof schema mismatch")
    if acip_sns.get("status") != "passed":
      fail("ACIP-SNS proof did not pass")
    if acip_sns.get("acip_projection", {}).get("signal_kind") != "acip_projection":
      fail("ACIP-SNS proof is not an ACIP projection")
    if acip_sns.get("acip_projection", {}).get("route_class") != "cross_boundary_deferred":
      fail("ACIP-SNS proof route class mismatch")
    if acip_sns.get("sns", {}).get("topic_name") != "adl-v0917-wp08-acip-sns-4685":
      fail("ACIP-SNS proof topic mismatch")
    acip_redaction = acip_sns.get("redaction", {})
    if acip_redaction.get("credentials_recorded") is not False or acip_redaction.get("raw_message_content_recorded") is not False:
      fail("ACIP-SNS proof redaction posture is not operations safe")
    if acip_sns.get("aws_account_sha256"):
      fail("ACIP-SNS proof must not retain a full account SHA")
    if not acip_sns.get("aws_account_hash"):
      fail("ACIP-SNS proof must retain only the short account hash")
    if sns_resource.get("schema") != "adl.wp08.acip_sns_resource.v1":
      fail("SNS resource proof schema mismatch")
    if sns_resource.get("aws_account_sha256"):
      fail("SNS resource proof must not retain a full account SHA")
    if not sns_resource.get("aws_account_hash"):
      fail("SNS resource proof must retain only the short account hash")
    if smoke["packetId"] != packet["packet_id"]:
      fail("JS view model did not consume the retained packet")
    if smoke["evidenceLevel"] != "bounded_local_runtime_capture":
      fail("JS view model evidence level mismatch")
    if smoke["manifoldState"] != packet["manifold"]["state"]:
      fail("JS view model manifold state mismatch")
    if smoke["citizenCount"] < 3:
      fail("expected three runtime lanes in HTML Observatory view model")
    if smoke["serviceCount"] < 4:
      fail("expected runtime services in HTML Observatory view model")
    if smoke["decisionCounts"] != {"allow": 1, "defer": 1, "refuse": 1}:
      fail(f"unexpected decision counts: {smoke['decisionCounts']!r}")
    if smoke["invariantCount"] < 3:
      fail("expected retained invariants in HTML Observatory view model")
    if smoke["latestEvent"] < 5:
      fail("expected retained trace tail through event 5")
    if smoke["actionCount"] < 5:
      fail("expected available and disabled operator actions")
    if not smoke["reportLoaded"]:
      fail("JS view model did not receive the operator report text")
    if len(smoke["serviceRows"]) < 3:
      fail("JS integration view model did not build CSM service rows")
    if len(smoke["cloudwatchRows"]) < 3:
      fail("JS integration view model did not build CloudWatch rows")
    if len(smoke["acipRows"]) < 3:
      fail("JS integration view model did not build ACIP-SNS rows")
    if not any(row.get("label") == "ACIP projection" and row.get("value") == "passed" for row in smoke["acipRows"]):
      fail(f"ACIP-SNS retained proof pass was not exposed: {smoke['acipRows']!r}")
    if not any(row.get("label") == "Redaction" and row.get("value") == "operations safe" for row in smoke["acipRows"]):
      fail(f"ACIP-SNS operations-safe redaction was not exposed: {smoke['acipRows']!r}")
    if smoke["parsedCloudWatchEventCount"] < 1:
      fail("JS integration view model did not parse CloudWatch events")
    if smoke["latestCloudWatchTarget"] != "cloudwatch_logs":
      fail("latest CloudWatch event is not a cloudwatch_logs signal")
    if smoke["awsLinkageCount"] != 5 or smoke["openAwsLinkageCount"] != 2:
      fail("AWS linkage lane did not preserve open WP-08 work truth")
    if smoke["closedAwsIssues"] != [4684, 4685, 4687]:
      fail(f"closed AWS linkage issues mismatch: {smoke['closedAwsIssues']!r}")
    if smoke["openAwsIssues"] != [4686, 4688]:
      fail(f"open AWS linkage issues mismatch: {smoke['openAwsIssues']!r}")
    envelope = smoke["operatorEnvelope"]
    if envelope.get("schema") != "adl.html_observatory.operator_message.v1":
      fail("operator communication envelope schema mismatch")
    if envelope.get("runtime_mutation_claimed") is not False:
      fail("operator communication envelope overclaims runtime mutation")
    acip_message = envelope.get("acip_message") or {}
    if acip_message.get("schema") != "acip.message.v1":
      fail("operator communication envelope does not contain an ACIP message")
    if acip_message.get("authority_granted") is not False:
      fail("ACIP message overclaims authority")
    aws_projection = envelope.get("aws_projection") or {}
    if aws_projection.get("schema") != "adl.runtime.aws_signal.v1":
      fail("operator communication envelope does not contain AWS projection")
    if aws_projection.get("target_kind") != "sns":
      fail("operator communication envelope does not target SNS projection")
    if aws_projection.get("live_publish_claimed") is not False:
      fail("operator communication envelope overclaims live SNS publish")
    if aws_projection.get("retained_proof_status") != "passed":
      fail("operator communication envelope did not expose passed ACIP-SNS proof")
    if aws_projection.get("retained_hygiene_issue") is not None:
      fail("operator communication envelope retained a stale ACIP-SNS hygiene issue")
    if envelope.get("allowed_live_check") is not None:
      fail("ACIP-SNS envelope should not claim a live /events read")
    loopback_policy = smoke["loopbackPolicy"]
    if not loopback_policy["localhostHttp"]:
      fail(f"loopback CSM API bases were not accepted: {loopback_policy!r}")
    if not loopback_policy["runtimeTrustedLocalhost"]:
      fail(f"trusted Runtime v3 localhost API base was not accepted: {loopback_policy!r}")
    if loopback_policy["runtimeRemoteHttps"] or loopback_policy["runtimeWrongPort"] or loopback_policy["runtimeUrlCredentials"] or loopback_policy["runtimeUrlQuery"] or loopback_policy["runtimePath"]:
      fail(f"untrusted Runtime v3 API base was accepted before bearer/WSS use: {loopback_policy!r}")
    if loopback_policy["runtimeHttp"]:
      fail(f"non-HTTPS Runtime v3 API base was accepted: {loopback_policy!r}")
    if loopback_policy["remoteHttp"] or loopback_policy["malformed"]:
      fail(f"non-CSM-loopback or malformed API base was accepted: {loopback_policy!r}")
    panopticon = smoke["panopticon"]
    if panopticon.get("mode") != "live":
      fail("panopticon view model did not preserve live mode")
    if panopticon.get("agentCount", 0) < 3:
      fail("panopticon did not expose the runtime agent roster")
    if panopticon.get("signalCount", 0) < 4:
      fail("panopticon did not expose health/readiness/event/error signals")
    if panopticon.get("metricCount", 0) < 2:
      fail("panopticon did not expose runtime metrics")
    if panopticon.get("eventCount", 0) < 1:
      fail("panopticon did not expose live event tail data")
    if panopticon.get("readyState") != "ready":
      fail("panopticon readiness state mismatch")
    retained_panopticon = smoke["retainedPanopticon"]
    if retained_panopticon.get("mode") != "published":
      fail("retained CSM API mirror did not preserve published mode")
    if retained_panopticon.get("eventCount", 0) < 1:
      fail("retained CSM API mirror did not normalize events.entries")
    if retained_panopticon.get("readyState") != "ready":
      fail("retained CSM API mirror readiness state mismatch")
    retained_fetch_panopticon = smoke["retainedFetchPanopticon"]
    if retained_fetch_panopticon.get("mode") != "published":
      fail("retained CSM API fetch path did not preserve published mode")
    if retained_fetch_panopticon.get("eventCount", 0) < 1:
      fail("retained CSM API fetch path did not load events.entries")
    if retained_fetch_panopticon.get("metricCount", 0) < 2:
      fail("retained CSM API fetch path did not expose metrics")
    if retained_fetch_panopticon.get("readyState") != "ready":
      fail("retained CSM API fetch path readiness state mismatch")
    if retained_fetch_panopticon.get("errorCount") != 0:
      fail(f"retained CSM API fetch path had unexpected errors: {retained_fetch_panopticon!r}")
    live_fetch_panopticon = smoke["liveFetchPanopticon"]
    if live_fetch_panopticon.get("mode") != "live":
      fail("live CSM API fetch path did not preserve live mode")
    if live_fetch_panopticon.get("eventCount", 0) < 1:
      fail("live CSM API fetch path did not load events.entries")
    if live_fetch_panopticon.get("metricCount", 0) < 2:
      fail("live CSM API fetch path did not expose metrics")
    if live_fetch_panopticon.get("readyState") != "ready":
      fail("live CSM API fetch path readiness state mismatch")
    if live_fetch_panopticon.get("errorCount") != 0:
      fail(f"live CSM API fetch path had unexpected errors: {live_fetch_panopticon!r}")
    stale_roster_labels = {"Runtime lane alpha", "Runtime lane beta", "Runtime lane gamma"}
    if stale_roster_labels.intersection(retained_fetch_panopticon.get("agentLabels", [])):
      fail(f"published panopticon roster included stale packet citizens: {retained_fetch_panopticon!r}")
    dashboard_mirrors = smoke["dashboardMirrors"]
    if dashboard_mirrors.get("heroCloudwatchOkLabel") != "CloudWatch Proven":
      fail(f"dashboard CloudWatch pass mirror mismatch: {dashboard_mirrors!r}")
    if dashboard_mirrors.get("heroCloudwatchBlockedLabel") != "blocked":
      fail(f"dashboard CloudWatch blocked mirror overclaims or hides failure: {dashboard_mirrors!r}")
    if dashboard_mirrors.get("heroReadyLabel") != "ready":
      fail(f"dashboard readiness mirror mismatch: {dashboard_mirrors!r}")
    if int(dashboard_mirrors.get("heroAgentCount", "0")) < 1:
      fail(f"dashboard agent-count mirror did not expose retained agents: {dashboard_mirrors!r}")
    if int(dashboard_mirrors.get("heroEventCount", "0")) < 1:
      fail(f"dashboard event-count mirror did not expose retained events: {dashboard_mirrors!r}")
    live_binding = smoke["liveBinding"]
    if live_binding.get("base") != "http://localhost:49210":
      fail(f"live query-param base was not mirrored into the dashboard input: {live_binding!r}")
    if live_binding.get("retainedIntervalCount") != 0:
      fail(f"retained polling interval can overwrite a supplied live runtime base: {live_binding!r}")
    if live_binding.get("runtimeStatus") != "live loopback":
      fail(f"live binding did not preserve proved loopback status: {live_binding!r}")
    if live_binding.get("statusbarMode") != "Live Loopback":
      fail(f"live binding did not preserve statusbar live mode: {live_binding!r}")
    async_race = smoke["asyncRace"]
    if async_race.get("retainedStopStatus") != "polling stopped":
      fail(f"late retained completion overwrote Stop state: {async_race!r}")
    if async_race.get("retainedStopRuntimeStatus") != "polling stopped":
      fail(f"late retained completion overwrote Stop runtime status: {async_race!r}")
    if async_race.get("retainedStopConnectionState") != "stopped":
      fail(f"Stop did not remain browser-visible after retained completion: {async_race!r}")
    if async_race.get("wssStopStatus") != "polling stopped":
      fail(f"late WSS completion overwrote Stop state: {async_race!r}")
    if async_race.get("wssStopRuntimeStatus") != "polling stopped":
      fail(f"late WSS completion overwrote Stop runtime status: {async_race!r}")
    if async_race.get("wssStopWebsocketStatus") != "stopped":
      fail(f"late WSS completion overwrote stopped WebSocket status: {async_race!r}")
    trusted_wss = smoke["trustedWss"]
    if trusted_wss.get("endpoint") != "wss://localhost:20997/v1/observatory/ws":
      fail(f"trusted WSS endpoint was not localhost:20997: {trusted_wss!r}")
    if trusted_wss.get("authFrameSent") is not True:
      fail(f"operator token was not sent after trusted localhost WSS open: {trusted_wss!r}")
    if trusted_wss.get("rejectedUntrustedWss") is not True or trusted_wss.get("rejectedUntrustedCreatedSocket") is not False:
      fail(f"untrusted WSS was not rejected before WebSocket/token use: {trusted_wss!r}")
    runtime_v3_panopticon = smoke["runtimeV3Panopticon"]
    if runtime_v3_panopticon.get("mode") != "live":
      fail(f"Runtime v3 observatory feed did not preserve live mode: {runtime_v3_panopticon!r}")
    if runtime_v3_panopticon.get("runtimeSelection") != "runtime_v3_explicit_opt_in":
      fail(f"Runtime v3 observatory feed did not preserve explicit opt-in selection: {runtime_v3_panopticon!r}")
    if runtime_v3_panopticon.get("controlPort") != 20997:
      fail(f"Runtime v3 observatory feed did not preserve control port 20997: {runtime_v3_panopticon!r}")
    if runtime_v3_panopticon.get("mutationAuthority") is not False:
      fail("Runtime v3 Observatory browser path overclaims mutation authority")
    if runtime_v3_panopticon.get("agentTotal") != 10000:
      fail(f"Runtime v3 Observatory did not preserve high-cardinality agent total: {runtime_v3_panopticon!r}")
    if len(runtime_v3_panopticon.get("agentLabels", [])) != 3:
      fail(f"Runtime v3 Observatory rendered sample should stay bounded: {runtime_v3_panopticon!r}")
    if runtime_v3_panopticon.get("defaultSwitchAuthorized") is not False:
      fail("Runtime v3 Observatory browser path overclaims default switch authorization")
    if runtime_v3_panopticon.get("decommissionAuthorized") is not False:
      fail("Runtime v3 Observatory browser path overclaims Runtime v2 decommission")
    if runtime_v3_panopticon.get("sidecarRequired") is not False:
      fail("Runtime v3 Observatory browser path introduced a sidecar requirement")
    if runtime_v3_panopticon.get("eventCount", 0) < 1:
      fail("Runtime v3 observatory feed did not expose operator-visible events")
    if runtime_v3_panopticon.get("metricCount", 0) < 4:
      fail("Runtime v3 observatory feed did not expose health/weather metrics")
    if runtime_v3_panopticon.get("readyState") != "ready":
      fail("Runtime v3 observatory feed readiness state mismatch")
    if runtime_v3_panopticon.get("weatherAgeMillis") != 250:
      fail(f"Runtime v3 Observatory dropped weather age: {runtime_v3_panopticon!r}")
    if runtime_v3_panopticon.get("weatherStaleAfterMillis") != 2000:
      fail(f"Runtime v3 Observatory dropped weather staleness bound: {runtime_v3_panopticon!r}")
    if runtime_v3_panopticon.get("weatherStale") is not False:
      fail(f"Runtime v3 Observatory weather freshness state mismatch: {runtime_v3_panopticon!r}")

    secret_pattern = re.compile(
        r"/Users/|/private/var/|192\\.168\\.|"
        r"bearer\\s+[A-Za-z0-9._-]{8,}|"
        r"(api[_-]?key|secret|token)\\s*[:=]\\s*[A-Za-z0-9._-]{8,}",
        re.IGNORECASE,
    )
    for label, content in {"html": html, "css": css, "js": js}.items():
      if secret_pattern.search(content):
        fail(f"{label} contains private path, endpoint, or secret-like text")

    print("PASS: v0.91.7 HTML Observatory integrated proof validation passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
