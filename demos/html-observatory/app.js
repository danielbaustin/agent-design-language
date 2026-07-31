const FALLBACK_PACKET = {
  schema: "adl.csm_visibility_packet.v1",
  packet_id: "html-observatory-fallback",
  generated_at: "unloaded",
  source: {
    mode: "fallback",
    evidence_level: "fallback_only",
    claim_boundary: "The retained runtime packet did not load; this fallback only preserves the static UI shell."
  },
  manifold: {
    manifold_id: "unknown",
    display_name: "Runtime packet unavailable",
    state: "fallback",
    current_tick: 0,
    health: {
      summary: "Packet load failed; no runtime proof is claimed from fallback content."
    }
  },
  kernel: {
    pulse: { status: "fallback", completed_through_event_sequence: 0 },
    service_states: []
  },
  citizens: [],
  freedom_gate: {
    recent_docket: [],
    allow_count: 0,
    defer_count: 0,
    refuse_count: 0
  },
  invariants: [],
  trace: { trace_tail: [] },
  operator_actions: {
    available_actions: [],
    disabled_actions: [
      {
        action: "runtime_mutation",
        reason: "Disabled unless a retained ADL runtime packet is loaded."
      }
    ]
  },
  review: {
    primary_artifacts: [],
    caveats: ["Fallback shell is not runtime evidence."]
  }
};

const formatLabel = (value) =>
  String(value ?? "unknown")
    .replaceAll("_", " ")
    .replaceAll("-", " ");

const asArray = (value) => (Array.isArray(value) ? value : []);

function formatTimestampLabel(value) {
  if (!value) {
    return "retained";
  }
  const parsed = new Date(value);
  if (Number.isNaN(parsed.getTime())) {
    return String(value);
  }
  return parsed.toLocaleString([], {
    month: "short",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit"
  });
}

function formatCurrentTimestampLabel() {
  return formatTimestampLabel(new Date());
}

let livePollTimer = null;
let retainedPollTimer = null;
const OBSERVATORY_VERSION = "Runtime v3";
const OBSERVATORY_MANIFOLD_LABEL = `${OBSERVATORY_VERSION} CSM runtime mirror`;
const OBSERVATORY_PACKET_LABEL = `${OBSERVATORY_VERSION} Observatory proof packet`;
const RUNTIME_V3_OBSERVATORY_ENDPOINT = "/v1/observatory";
const RUNTIME_V3_OBSERVATORY_WS_ENDPOINT = "/v1/observatory/ws";
const RUNTIME_V3_OBSERVATORY_SCHEMA = "adl.runtime_v3.observatory_feed.v2";
const RUNTIME_V3_OBSERVATORY_WS_AUTH_SCHEMA = "adl.runtime_v3.observatory_ws_auth.v1";

const AWS_LINKAGES = [
  {
    issue: 4684,
    label: "Heartbeat publisher",
    state: "closed",
    proof: "Live CloudWatch heartbeat proof retained for the Agent Logic AWS profile.",
    evidence: "wp08_heartbeat_4684/live_heartbeat_summary.json"
  },
  {
    issue: 4685,
    label: "ACIP to SNS",
    state: "closed",
    proof: "Issue is closed and routed as the ACIP-SNS linkage lane for this demo.",
    evidence: "GitHub issue #4685"
  },
  {
    issue: 4686,
    label: "AWS signal integration",
    state: "open",
    proof: "Full runtime AWS signal bridge remains visible as pending integration work.",
    evidence: "GitHub issue #4686"
  },
  {
    issue: 4687,
    label: "Local polis SSM operations",
    state: "closed",
    proof: "Local polis SSM operations issue is closed and available as the SSM linkage lane.",
    evidence: "GitHub issue #4687"
  },
  {
    issue: 4688,
    label: "S3 ObsMem archive",
    state: "open",
    proof: "Community-memory archive policy remains planned until archive proof is retained.",
    evidence: "GitHub issue #4688"
  }
];

function parseCloudWatchEventMessage(event) {
  if (!event || typeof event !== "object") {
    return {};
  }
  const message = event.message;
  if (typeof message !== "string") {
    return {};
  }
  try {
    const parsed = JSON.parse(message);
    return parsed && typeof parsed === "object" ? parsed : {};
  } catch (_error) {
    return {};
  }
}

function buildViewModel(packet, reportText = "") {
  const safePacket = packet && typeof packet === "object" ? packet : FALLBACK_PACKET;
  const citizens = asArray(safePacket.citizens);
  const services = asArray(safePacket.kernel?.service_states);
  const decisions = asArray(safePacket.freedom_gate?.recent_docket);
  const invariants = asArray(safePacket.invariants);
  const traceTail = asArray(safePacket.trace?.trace_tail);
  const availableActions = asArray(safePacket.operator_actions?.available_actions);
  const disabledActions = asArray(safePacket.operator_actions?.disabled_actions);
  const artifacts = asArray(safePacket.review?.primary_artifacts);
  const caveats = asArray(safePacket.review?.caveats);
  const latestEvent = traceTail.reduce(
    (max, event) => Math.max(max, Number(event.event_sequence || 0)),
    0
  );

  return {
    packet: safePacket,
    reportText,
    citizens,
    services,
    decisions,
    invariants,
    traceTail,
    availableActions,
    disabledActions,
    artifacts,
    caveats,
    latestEvent,
    decisionCounts: {
      allow: safePacket.freedom_gate?.allow_count ?? decisions.filter((item) => item.decision === "allow").length,
      defer: safePacket.freedom_gate?.defer_count ?? decisions.filter((item) => item.decision === "defer").length,
      refuse: safePacket.freedom_gate?.refuse_count ?? decisions.filter((item) => item.decision === "refuse").length
    }
  };
}

function buildIntegrationViewModel({
  serviceManifest = {},
  apiText = "",
  cloudwatchSummary = {},
  cloudwatchEvents = {},
  acipSnsSummary = {},
  snsResourceSummary = {}
} = {}) {
  const events = asArray(cloudwatchEvents.events);
  const parsedEvents = events.map(parseCloudWatchEventMessage).filter((event) => Object.keys(event).length > 0);
  const latestEvent = parsedEvents.at(-1) || {};
  const cloudwatch = cloudwatchSummary.cloudwatch || {};
  const heartbeat = cloudwatchSummary.heartbeat || {};
  const redaction = cloudwatchSummary.redaction || {};
  const acipProjection = acipSnsSummary.acip_projection || {};
  const acipSns = acipSnsSummary.sns || {};
  const snsResource = snsResourceSummary.sns || {};
  const acipRedaction = acipSnsSummary.redaction || {};
  const acipRetainsFullAccountSha = Boolean(acipSnsSummary.aws_account_sha256 || snsResourceSummary.aws_account_sha256);
  const acipRedactionSafe =
    acipRedaction.credentials_recorded === false &&
    acipRedaction.raw_message_content_recorded === false &&
    !acipRetainsFullAccountSha;

  return {
    serviceManifest,
    apiText,
    cloudwatchSummary,
    cloudwatchEvents,
    acipSnsSummary,
    snsResourceSummary,
    parsedEvents,
    latestEvent,
    serviceRows: [
      {
        label: "Runtime owner",
        value: serviceManifest.runtime_owner || "unknown",
        detail: serviceManifest.csm_bin ? "Standalone csm binary owns runtime service startup." : "Service manifest did not record csm binary ownership.",
        state: serviceManifest.runtime_owner === "csm" ? "closed" : "open"
      },
      {
        label: "Service manager",
        value: serviceManifest.manager || "unknown",
        detail: serviceManifest.label || "No service label recorded.",
        state: serviceManifest.manager ? "closed" : "open"
      },
      {
        label: "Local API",
        value: apiText.includes("csm api serve --spec <agent-spec.yaml>") ? "csm api serve" : "not detected",
        detail: apiText.includes("/status") ? "/status, /health, /ready, /metrics, and /events documented." : "Expected CSM API endpoints were not detected.",
        state: apiText.includes("csm api serve --spec <agent-spec.yaml>") ? "closed" : "open"
      }
    ],
    cloudwatchRows: [
      {
        label: "Heartbeat status",
        value: cloudwatchSummary.status || "unknown",
        detail: `${heartbeat.signal_kind || "signal"} / ${heartbeat.runtime_id || "runtime unknown"}`,
        state: cloudwatchSummary.status || "open"
      },
      {
        label: "CloudWatch target",
        value: cloudwatch.log_group || "unknown log group",
        detail: `${cloudwatch.log_stream || "unknown stream"} / ${cloudwatch.event_count ?? events.length} events`,
        state: cloudwatch.target_kind === "cloudwatch_logs" || heartbeat.target_kind === "cloudwatch_logs" ? "passed" : "open"
      },
      {
        label: "Redaction posture",
        value: redaction.credentials_recorded === false ? "operations safe" : "needs review",
        detail: redaction.raw_account_id_recorded === false ? "No raw account id or credentials recorded in retained summary." : "Retained summary needs redaction review.",
        state: redaction.credentials_recorded === false && redaction.raw_account_id_recorded === false ? "passed" : "blocked"
      }
    ],
    acipRows: [
      {
        label: "ACIP projection",
        value: acipSnsSummary.status || "unknown",
        detail: `${acipProjection.signal_kind || "signal unknown"} / ${acipProjection.route_class || "route unknown"}; retained proof passed redaction hygiene.`,
        state: acipSnsSummary.status === "passed" && !acipRetainsFullAccountSha ? "passed" : "blocked"
      },
      {
        label: "SNS topic",
        value: acipSns.topic_name || snsResource.topic_name || "unknown topic",
        detail: acipSns.message_id ? `Retained SNS message ${acipSns.message_id}.` : "No retained SNS message id loaded.",
        state: acipSns.message_id ? "passed" : "open"
      },
      {
        label: "Redaction",
        value: acipRedactionSafe ? "operations safe" : "needs review",
        detail: acipRedactionSafe ? "No raw credentials, account id, topic ARN, private ACIP content, or full account SHA retained." : "Retained proof needs redaction review before operations-safe claim.",
        state: acipRedactionSafe ? "passed" : "blocked"
      }
    ]
  };
}

function setText(id, value) {
  const target = document.getElementById(id);
  if (target) {
    target.textContent = value;
  }
}

function setState(id, value) {
  const target = document.getElementById(id);
  if (target) {
    target.dataset.state = stateTone(value);
  }
}

function setHref(id, value) {
  const target = document.getElementById(id);
  if (target) {
    target.href = value;
  }
}

function setDataset(id, key, value) {
  const target = document.getElementById(id);
  if (target) {
    target.dataset[key] = value;
  }
}

function renderRows(targetId, rows) {
  const target = document.getElementById(targetId);
  if (target) {
    target.innerHTML = rows.join("");
  }
}

function escapeHtml(value) {
  return String(value ?? "")
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}

function buildOperatorEnvelope({ channel = "events", message = "", packetId = "", acipSnsSummary = {}, snsResourceSummary = {} } = {}) {
  const acipProjection = acipSnsSummary.acip_projection || {};
  const acipSns = acipSnsSummary.sns || {};
  const snsResource = snsResourceSummary.sns || {};
  const sanitizedMessage = String(message || "").slice(0, 800);
  return {
    schema: "adl.html_observatory.operator_message.v1",
    channel,
    intent: "operator_communication",
    delivery: "prepared_client_side",
    runtime_mutation_claimed: false,
    packet_id: packetId,
    message: sanitizedMessage,
    acip_message: {
      schema: "acip.message.v1",
      sender: "html-observatory-operator",
      recipient: channel === "events" ? "csm-runtime-api" : "csm-runtime-owner",
      mode: channel === "events" ? "read_request" : "projection_request",
      ordering: "client_draft",
      visibility: "operator_visible",
      traceability: "prepared_envelope_only",
      authority_granted: false,
      content_summary: sanitizedMessage || "No operator message supplied."
    },
    aws_projection: channel === "acip_sns" ? {
      schema: "adl.runtime.aws_signal.v1",
      signal_kind: "acip_projection",
      route_class: acipProjection.route_class || "cross_boundary_deferred",
      target_kind: "sns",
      topic_name: acipSns.topic_name || snsResource.topic_name || "unknown",
      retained_message_id: acipSns.message_id || null,
      retained_proof_status: acipSnsSummary.status || "unknown",
      retained_hygiene_issue: null,
      live_publish_claimed: false
    } : null,
    allowed_live_check: channel === "events" ? "/events" : null
  };
}

function renderEnvelope(envelope) {
  const target = document.getElementById("message-envelope");
  if (target) {
    target.textContent = JSON.stringify(envelope, null, 2);
  }
}

const DASHBOARD_FOCUS = {
  runtime: {
    kicker: "Runtime",
    title: "Runtime mirror",
    status: "active",
    target: "#runtime-proof",
    focusTarget: "#hero-ready-state",
    detail: "Runtime readiness, event tail, CloudWatch proof, and retained/live mode are visible in the fixed dashboard.",
    facts: ["Readiness and kernel state", "Event preview and gauges", "Retained/live mode status"]
  },
  agents: {
    kicker: "Agents",
    title: "CSM polis topology",
    status: "map ready",
    target: "#panopticon",
    focusTarget: "#hero-agent-map",
    detail: "Agent roster, scheduler, telemetry, event stream, and checkpoint lanes are mirrored in the panopticon map.",
    facts: ["Role-specific topology icons", "Agent roster summary", "Health and signal lanes"]
  },
  "csm-api": {
    kicker: "CSM API",
    title: "Local control plane",
    status: "public reads + governed writes",
    target: "#csm-api",
    focusTarget: "#hero-api-list",
    detail: "Runtime state is publicly readable; login and a trusted signature are required for control writes.",
    facts: ["/health, /metrics, /observatory", "Signed /v1/control commands", "Authenticated full-duplex WSS"]
  },
  cloudwatch: {
    kicker: "AWS",
    title: "CloudWatch heartbeat",
    status: "source-linked",
    target: "#cloudwatch",
    focusTarget: "#hero-cloudwatch-state",
    detail: "CloudWatch rows and event-tail evidence are loaded from retained redacted AWS proof artifacts.",
    facts: ["WP-08 heartbeat proof", "Redacted event tail", "No browser AWS write authority"]
  },
  communication: {
    kicker: "Operator communication",
    title: "ACIP/SNS envelope",
    status: "prepared",
    target: "#communication",
    focusTarget: ".compact-composer",
    detail: "Comms prepare ACIP messages and mirror retained SNS proof; live AWS mutation remains runtime-owned.",
    facts: ["ACIP message draft", "SNS projection proof", "Redaction hygiene passed"]
  },
  governance: {
    kicker: "Governance",
    title: "Freedom gate",
    status: "bounded",
    target: "#governance",
    focusTarget: "#hero-governance-state",
    detail: "Decision, invariant, and proposal-only action surfaces preserve the packet claim boundary.",
    facts: ["Freedom gate decisions", "Runtime invariants", "Proposal-only actions"]
  },
  evidence: {
    kicker: "Evidence",
    title: "Proof packet",
    status: "linked",
    target: "#evidence",
    focusTarget: "#packet-link",
    detail: "Packet, operator report, CSM API proof, metrics mirror, and CloudWatch artifacts remain source-linked.",
    facts: ["Visibility packet", "Operator report", "CSM/AWS proof refs"]
  }
};

function updateDashboardFocus(key = "runtime", extraDetail = "") {
  const selected = DASHBOARD_FOCUS[key] || DASHBOARD_FOCUS.runtime;
  setText("dashboard-focus-kicker", selected.kicker);
  setText("dashboard-focus-title", selected.title);
  setText("dashboard-focus-status", selected.status);
  setState("dashboard-focus-status", selected.status);
  setText("dashboard-focus-detail", extraDetail || selected.detail);
  setHref("dashboard-focus-link", selected.focusTarget);
  setText("dashboard-focus-link", `View ${selected.title}`);
  renderRows("dashboard-focus-list", asArray(selected.facts).map((fact) => `
    <span class="dashboard-focus-item">${escapeHtml(fact)}</span>
  `));
  document.querySelectorAll("[data-dashboard-link]").forEach((link) => {
    const isActive = link.dataset.dashboardLink === key;
    if (isActive) {
      link.setAttribute("aria-current", "page");
    } else {
      link.removeAttribute("aria-current");
    }
  });
}

function bindDashboardNavigation(packet = FALLBACK_PACKET) {
  document.querySelectorAll("[data-dashboard-link]").forEach((link) => {
    link.addEventListener("click", (event) => {
      event.preventDefault();
      const key = link.dataset.dashboardLink || "runtime";
      updateDashboardFocus(key);
      const selected = DASHBOARD_FOCUS[key] || DASHBOARD_FOCUS.runtime;
      globalThis.history?.replaceState(null, "", selected.target);
      const panel = document.getElementById("dashboard-focus-panel");
      panel?.setAttribute("tabindex", "-1");
      panel?.focus({ preventScroll: true });
      if (key === "communication") {
        document.getElementById("prepare-envelope")?.click();
      }
    });
  });

  document.getElementById("dashboard-focus-link")?.addEventListener("click", (event) => {
    event.preventDefault();
    const target = document.querySelector(document.getElementById("dashboard-focus-link")?.getAttribute("href") || "#hero-ready-state");
    if (target) {
      target.setAttribute("tabindex", "-1");
      target.focus();
    }
  });

  document.getElementById("export-proof")?.addEventListener("click", () => {
    const manifest = {
      schema: "adl.html_observatory.export_manifest.v1",
      version: OBSERVATORY_VERSION,
      packet_id: displayPacketId(packet.packet_id || ""),
      exported_at: new Date().toISOString(),
      runtime_mode: document.getElementById("statusbar-mode")?.textContent || "unknown",
      runtime_status: document.getElementById("dashboard-live-test-status")?.textContent || "unknown",
      csm_api_base: document.getElementById("dashboard-live-api-base")?.value || "",
      cloudwatch_status: document.getElementById("cloudwatch-status")?.textContent || "unknown",
      communication_status: document.getElementById("communication-status")?.textContent || "unknown",
      mutation_claimed: false
    };
    const blob = new Blob([JSON.stringify(manifest, null, 2)], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = "adl-html-observatory-proof-manifest.json";
    document.body.appendChild(link);
    link.click();
    link.remove();
    globalThis.setTimeout(() => URL.revokeObjectURL(url), 1000);
    updateDashboardFocus("evidence", "Export prepared a local proof manifest from the visible dashboard state.");
  });

  updateDashboardFocus("runtime");
}

function normalizeApiBase(value) {
  return String(value || "").trim().replace(/\/+$/, "");
}

function displayManifoldId(_value) {
  return OBSERVATORY_MANIFOLD_LABEL;
}

function displayPacketId(_value) {
  return OBSERVATORY_PACKET_LABEL;
}

function displayClaimBoundary(source = {}) {
  const evidenceLevel = formatLabel(source.evidence_level || "bounded local runtime capture");
  return `${OBSERVATORY_VERSION} Observatory consumes a ${evidenceLevel} from runtime-owned artifacts, supports public monitoring and runtime-governed signed writes, and does not claim browser-owned authority or v0.92 coherence.`;
}

function displayMilestoneText(value) {
  return String(value ?? "")
    .replaceAll("v0.91.6", OBSERVATORY_VERSION)
    .replaceAll("v0916", "runtime-v3")
    .replaceAll("v0.91.7", OBSERVATORY_VERSION)
    .replaceAll("v0917", "runtime-v3");
}

function isLoopbackApiBase(value) {
  const base = normalizeApiBase(value);
  try {
    const parsed = new URL(base);
    return (
      ["http:", "https:"].includes(parsed.protocol) &&
      parsed.hostname === "localhost"
    );
  } catch (_error) {
    return false;
  }
}

function getQueryApiBase() {
  const params = new URLSearchParams(window.location.search);
  const candidate = params.get("runtimeApiBase") || params.get("csmApiBase") || params.get("apiBase") || "";
  const normalized = normalizeApiBase(candidate);
  if (requestedRuntimeSelection() === "v3") {
    return isRuntimeV3ApiBase(normalized) ? normalized : "";
  }
  return isLoopbackApiBase(normalized) ? normalized : "";
}

function requestedRuntimeSelection() {
  const params = new URLSearchParams(window.location.search);
  return String(params.get("runtime") || params.get("runtimeSelection") || "v2").toLowerCase();
}

function isRuntimeV3ApiBase(value) {
  try {
    return new URL(normalizeApiBase(value)).protocol === "https:";
  } catch (_error) {
    return false;
  }
}

function shouldAutoConnectLive() {
  const params = new URLSearchParams(window.location.search);
  return ["1", "true", "live", "connect"].includes(String(params.get("live") || params.get("connect") || "").toLowerCase());
}

async function checkEventsEndpoint(apiBase) {
  const base = normalizeApiBase(apiBase);
  if (!base) {
    throw new Error("Enter a loopback CSM API base first.");
  }
  if (!isLoopbackApiBase(base)) {
    throw new Error("Only loopback CSM API bases are allowed.");
  }
  const response = await fetch(`${base}/events`, { method: "GET" });
  if (!response.ok) {
    throw new Error(`/events returned ${response.status}`);
  }
  return response.json();
}

async function fetchRuntimeEndpoint(apiBase, endpoint) {
  const base = normalizeApiBase(apiBase);
  if (!base) {
    throw new Error("Enter a loopback CSM API base first.");
  }
  if (!isLoopbackApiBase(base)) {
    throw new Error("Only loopback CSM API bases are allowed.");
  }
  const response = await fetch(`${base}${endpoint}`, { method: "GET" });
  if (!response.ok) {
    throw new Error(`${endpoint} returned ${response.status}`);
  }
  return response.json();
}

async function fetchRuntimeSnapshot(apiBase) {
  if (requestedRuntimeSelection() === "v3") {
    if (!isRuntimeV3ApiBase(apiBase)) {
      throw new Error("Runtime v3 selection requires a configured HTTPS runtime API base.");
    }
    return fetchRuntimeV3ObservatorySnapshot(apiBase);
  }
  const endpoints = ["/status", "/health", "/ready", "/metrics", "/events"];
  const settled = await Promise.allSettled(
    endpoints.map((endpoint) => fetchRuntimeEndpoint(apiBase, endpoint))
  );
  const snapshot = {
    mode: "live",
    fetchedAt: new Date().toISOString(),
    errors: {}
  };
  endpoints.forEach((endpoint, index) => {
    const key = endpoint.slice(1);
    const result = settled[index];
    if (result.status === "fulfilled") {
      snapshot[key] = result.value;
    } else {
      snapshot.errors[key] = result.reason instanceof Error ? result.reason.message : "unknown error";
    }
  });
  return snapshot;
}

async function fetchRuntimeV3ObservatorySnapshot(apiBase) {
  const base = normalizeApiBase(apiBase);
  if (!isRuntimeV3ApiBase(base)) {
    throw new Error("Runtime v3 selection requires a configured HTTPS runtime API base.");
  }
  const response = await fetch(`${base}${RUNTIME_V3_OBSERVATORY_ENDPOINT}`, { method: "GET" });
  if (!response.ok) {
    throw new Error(`${RUNTIME_V3_OBSERVATORY_ENDPOINT} returned ${response.status}`);
  }
  const feed = await response.json();
  return runtimeV3SnapshotFromFeed(feed);
}

function runtimeV3SnapshotFromFeed(feed) {
  if (feed.schema !== RUNTIME_V3_OBSERVATORY_SCHEMA) {
    throw new Error(`Unsupported Runtime v3 Observatory schema: ${feed.schema || "missing"}`);
  }
  const snapshot = feed.health?.snapshot || {};
  const weather = feed.weather || {};
  const weatherFreshness = feed.weather_freshness || {};
  const events = asArray(feed.events);
  return {
    mode: "live",
    runtimeSelection: feed.runtime_selection || "runtime_v3_explicit_opt_in",
    fetchedAt: new Date().toISOString(),
    status: {
      schema: feed.schema,
      runtime_owner: "runtime-v3",
      runtime_id: feed.runtime_instance_id,
      agent_instance_id: feed.runtime_instance_id,
      agent_population: feed.agents,
      status: snapshot.lifecycle || "unknown",
      observability: snapshot.observability,
      topology_generation: snapshot.topology_generation,
      control: feed.control,
      proof: feed.proof
    },
    health: {
      status: feed.health?.observability_ready ? "healthy" : "pending",
      summary: "Runtime v3 observatory feed",
      components: snapshot.components || {},
      queues: snapshot.queues || {}
    },
    ready: {
      status: snapshot.observability_ready ? "ready" : "pending",
      blocking_reasons: snapshot.observability_ready ? [] : ["observability_not_ready"]
    },
    metrics: {
      gauges: {
        event_count: snapshot.event_count || events.length,
        component_count: Object.keys(snapshot.components || {}).length,
        agent_count: feed.agents?.total_count ?? null,
        agent_sample_count: feed.agents?.rendered_sample_count ?? null,
        queue_count: Object.keys(snapshot.queues || {}).length,
        weather_cpu_basis_points: weather.sample?.cpu_basis_points?.value ?? null,
        network_received_bytes: weather.sample?.network_received_bytes?.value ?? null,
        network_transmitted_bytes: weather.sample?.network_transmitted_bytes?.value ?? null,
        weather_age_millis: weatherFreshness.age_millis ?? null,
        weather_stale_after_millis: weatherFreshness.stale_after_millis ?? null
      },
      states: {
        lifecycle: snapshot.lifecycle || "unknown",
        resource_state: weather.resource_state || "unknown",
        shutdown_decision: weather.shutdown_decision || "unknown",
        gpu_proof_state: weather.gpu_proof_state || "unknown",
        weather_stale: weatherFreshness.stale ?? null
      }
    },
    events: { events },
    continuity: feed.continuity,
    proof: feed.proof,
    errors: {}
  };
}

function connectRuntimeV3ObservatoryWebSocket(
  apiBase,
  onSnapshot,
  onError,
  onClose = onError,
  onControlFrame = () => {}
) {
  const base = normalizeApiBase(apiBase);
  if (!isRuntimeV3ApiBase(base)) {
    throw new Error("Runtime v3 selection requires a configured HTTPS runtime API base.");
  }
  const endpoint = new URL(`${base}${RUNTIME_V3_OBSERVATORY_WS_ENDPOINT}`);
  endpoint.protocol = "wss:";
  const socket = new WebSocket(endpoint.toString());
  socket.addEventListener("open", () => {
    const writeToken = globalThis.sessionStorage?.getItem("adl.runtimeV3.observatoryToken") || "";
    if (writeToken) {
      authenticateRuntimeV3ObservatorySocket(socket, writeToken);
    }
  });
  socket.addEventListener("message", (event) => {
    try {
      const frame = JSON.parse(String(event.data));
      if (frame.schema === RUNTIME_V3_OBSERVATORY_SCHEMA) {
        onSnapshot(runtimeV3SnapshotFromFeed(frame));
      } else if (frame.schema === "adl.runtime_v3.observatory_ws_control_result.v1" ||
                 frame.schema === "adl.csm.acip_carrier.websocket_frame.v1") {
        onControlFrame(frame);
      }
    } catch (error) {
      onError(error instanceof Error ? error : new Error("Runtime v3 Observatory frame is invalid."));
      socket.close(1008, "invalid_observatory_frame");
    }
  });
  socket.addEventListener("error", () => {
    onError(new Error("Runtime v3 Observatory WebSocket failed."));
  });
  socket.addEventListener("close", (event) => {
    onClose(new Error(`Runtime v3 Observatory WebSocket closed (${event.code}).`));
  });
  return socket;
}

function authenticateRuntimeV3ObservatorySocket(socket, token) {
  if (!socket || socket.readyState !== WebSocket.OPEN) {
    throw new Error("Runtime v3 Observatory WebSocket is not open.");
  }
  const writeToken = String(token || "").trim();
  if (!writeToken) {
    throw new Error("A runtime operator token is required for writes.");
  }
  socket.send(JSON.stringify({
    schema: RUNTIME_V3_OBSERVATORY_WS_AUTH_SCHEMA,
    bearer_token: writeToken
  }));
}

async function fetchRetainedRuntimeSnapshot(refs = {}) {
  const [status, health, ready, metrics, events] = await Promise.all([
    loadJson(refs.statusRef).catch((error) => ({ __load_error: error instanceof Error ? error.message : "status load failed" })),
    loadJson(refs.healthRef).catch((error) => ({ __load_error: error instanceof Error ? error.message : "health load failed" })),
    loadJson(refs.readyRef).catch((error) => ({ __load_error: error instanceof Error ? error.message : "ready load failed" })),
    loadJson(refs.metricsRef).catch((error) => ({ __load_error: error instanceof Error ? error.message : "metrics load failed" })),
    loadJson(refs.eventsRef).catch((error) => ({ __load_error: error instanceof Error ? error.message : "events load failed" }))
  ]);
  return {
    mode: "published",
    fetchedAt: new Date().toISOString(),
    status,
    health,
    ready,
    metrics,
    events,
    errors: Object.fromEntries(
      Object.entries({ status, health, ready, metrics, events })
        .filter(([, value]) => value?.__load_error)
        .map(([key, value]) => [key, value.__load_error])
    )
  };
}

function flattenStatusRows(value, prefix = "", rows = []) {
  if (rows.length >= 14 || value == null) {
    return rows;
  }
  if (typeof value !== "object") {
    rows.push({ label: prefix || "value", value });
    return rows;
  }
  if (Array.isArray(value)) {
    rows.push({ label: prefix || "items", value: value.length });
    return rows;
  }
  Object.entries(value).forEach(([key, nested]) => {
    if (rows.length >= 14) {
      return;
    }
    const label = prefix ? `${prefix}.${key}` : key;
    if (nested == null || typeof nested !== "object") {
      rows.push({ label, value: nested });
    } else if (Array.isArray(nested)) {
      rows.push({ label, value: `${nested.length} items` });
    } else {
      flattenStatusRows(nested, label, rows);
    }
  });
  return rows;
}

function stateTone(value) {
  const normalized = String(value || "").toLowerCase();
  if (["running", "active", "ready", "healthy", "ok", "completed", "closed", "clear", "awake"].some((token) => normalized.includes(token))) {
    return "active";
  }
  if (["failed", "error", "refuse", "blocked", "timeout"].some((token) => normalized.includes(token))) {
    return "failed";
  }
  if (["degraded", "not_ready", "pending", "paused", "stale", "saturated"].some((token) => normalized.includes(token))) {
    return "degraded";
  }
  return normalized || "unknown";
}

function iconForAgent(agent = {}) {
  const text = `${agent.role || ""} ${agent.label || ""} ${agent.id || ""}`.toLowerCase();
  if (text.includes("owner")) {
    return "icon-agent";
  }
  if (text.includes("scheduler") || text.includes("cadence")) {
    return "icon-clock";
  }
  if (text.includes("telemetry") || text.includes("observability")) {
    return "icon-pulse";
  }
  if (text.includes("event") || text.includes("stream")) {
    return "icon-bolt";
  }
  if (text.includes("checkpoint") || text.includes("continuity") || text.includes("custody")) {
    return "icon-database";
  }
  if (text.includes("policy") || text.includes("gate") || text.includes("readiness")) {
    return "icon-shield";
  }
  return "icon-bot";
}

function eventMessageToObject(event) {
  if (!event || typeof event !== "object") {
    return {};
  }
  if (typeof event.message === "string") {
    try {
      const parsed = JSON.parse(event.message);
      return parsed && typeof parsed === "object" ? parsed : { message: event.message };
    } catch (_error) {
      return { message: event.message };
    }
  }
  if (event.details && typeof event.details === "object") {
    return {
      ...event,
      event_type: event.event || event.details.event || event.details.event_name,
      status: event.result || event.details.result || event.status,
      runtime_id: event.runtime_id || event.agent_instance_id || event.details.runtime_id,
      timestamp: event.at || event.timestamp || event.details.timestamp
    };
  }
  return {
    ...event,
    event_type: event.event || event.event_type,
    runtime_id: event.runtime_id || event.agent_instance_id,
    timestamp: event.at || event.timestamp
  };
}

function normalizeEventEntries(eventEnvelope = {}) {
  if (Array.isArray(eventEnvelope)) {
    return eventEnvelope;
  }
  if (Array.isArray(eventEnvelope.events)) {
    return eventEnvelope.events;
  }
  if (Array.isArray(eventEnvelope.entries)) {
    return eventEnvelope.entries;
  }
  if (Array.isArray(eventEnvelope.events?.entries)) {
    return eventEnvelope.events.entries;
  }
  if (Array.isArray(eventEnvelope.tail)) {
    return eventEnvelope.tail;
  }
  return [];
}

function normalizeMetricRows(metrics = {}) {
  const rows = flattenStatusRows(metrics)
    .filter((row) => ["number", "string", "boolean"].includes(typeof row.value))
    .slice(0, 8);
  return rows.length ? rows : [{ label: "metrics", value: "not exposed" }];
}

function buildRuntimeAgentRows({ status = {}, health = {}, ready = {}, metrics = {}, events = [], packet = FALLBACK_PACKET } = {}) {
  const hasApiStatus = Object.keys(status || {}).length > 0 && !status.__load_error;
  const retainedCitizens = asArray(packet.citizens);
  const primaryAgentId =
    status.agent_instance_id ||
    status.agent_id ||
    status.runtime_id ||
    status.instance_id ||
    packet.manifold?.manifold_id ||
    "csm-runtime";
  const primaryState =
    status.status ||
    status.state ||
    status.current_agent_status?.state ||
    status.agent_status?.state ||
    ready.status ||
    ready.ready ||
    health.status ||
    packet.manifold?.state ||
    "unknown";
  const agentPopulation = status.agent_population || {};
  const agentSample = asArray(agentPopulation.sample);

  if (!hasApiStatus) {
    return retainedCitizens.map((citizen) => ({
      id: citizen.citizen_id || citizen.display_name,
      label: citizen.display_name,
      role: citizen.role || "agent",
      state: citizen.lifecycle_state || citizen.continuity_status,
      detail: citizen.continuity_status || "retained citizen lane"
    })).slice(0, 6);
  }

  if (agentSample.length) {
    return agentSample.slice(0, 12).map((agent) => ({
      id: agent.id,
      label: agent.label || agent.id,
      role: agent.role || "runtime agent",
      state: agent.state || primaryState,
      detail: agent.detail || `${agentPopulation.total_count || agentSample.length} configured agents`
    }));
  }

  return [
    {
      id: primaryAgentId,
      label: status.agent_name || status.display_name || status.agent_instance_id || "CSM runtime",
      role: "runtime owner",
      state: primaryState,
      detail: status.runtime_owner ? `owner: ${status.runtime_owner}` : "loopback API status"
    },
    {
      id: `${primaryAgentId}:readiness`,
      label: "Readiness gate",
      role: "control gate",
      state: ready.status || ready.ready || primaryState,
      detail: asArray(ready.blocking_reasons).length ? ready.blocking_reasons.join(", ") : "no blocking reasons"
    },
    {
      id: `${primaryAgentId}:scheduler`,
      label: "Scheduler watcher",
      role: "cadence",
      state: status.scheduler?.status || metrics.states?.agent_state || status.agent_status?.state || primaryState,
      detail: status.scheduler?.cadence_source || `cycles: ${metrics.gauges?.completed_cycle_count ?? status.agent_status?.completed_cycle_count ?? "unknown"}`
    },
    {
      id: `${primaryAgentId}:observability`,
      label: "Observability bridge",
      role: "telemetry",
      state: status.otel?.status?.status || status.otel?.log?.status || health.status || "unknown",
      detail: status.otel?.status?.schema || status.otel?.log?.ref || "OTel and event logs"
    },
    {
      id: `${primaryAgentId}:events`,
      label: "Event stream tail",
      role: "operator stream",
      state: events.length ? "active" : "quiet",
      detail: `${events.length} retained CSM events`
    },
    {
      id: `${primaryAgentId}:continuity`,
      label: "Continuity checkpoint",
      role: "state custody",
      state: status.checkpoint?.status || status.continuity?.checkpoint?.status || "unknown",
      detail: status.checkpoint?.checkpoint_ref || status.continuity?.checkpoint?.ref || "continuity state"
    }
  ].slice(0, 6);
}

function buildPanopticonViewModel(snapshot = {}, packet = FALLBACK_PACKET) {
  const status = snapshot.status || {};
  const health = snapshot.health || {};
  const ready = snapshot.ready || {};
  const metrics = snapshot.metrics || {};
  const eventEnvelope = snapshot.events || {};
  const events = normalizeEventEntries(eventEnvelope).map(eventMessageToObject);
  const statusRows = flattenStatusRows(status);
  const liveAgents = buildRuntimeAgentRows({ status, health, ready, metrics, events, packet });
  const agentTotal = Number(status.agent_population?.total_count ?? metrics.gauges?.agent_count ?? liveAgents.length);

  const signalRows = [
    {
      label: "health",
      value: health.status || health.state || "unknown",
      detail: health.reason || health.summary || health.message || "CSM /health"
    },
    {
      label: "readiness",
      value: ready.status || ready.state || ready.ready || "unknown",
      detail: ready.reason || ready.summary || ready.message || "CSM /ready"
    },
    {
      label: "events",
      value: `${events.length} events`,
      detail: eventEnvelope.event_stream_ref || eventEnvelope.source || eventEnvelope.schema || "CSM /events"
    },
    {
      label: "errors",
      value: Object.keys(snapshot.errors || {}).length ? "partial" : "none",
      detail: Object.values(snapshot.errors || {}).join("; ") || "all requested endpoints responded"
    }
  ];

  return {
    mode: snapshot.mode || "retained",
    fetchedAt: snapshot.fetchedAt || "",
    agents: liveAgents,
    agentTotal,
    signals: signalRows,
    metrics: normalizeMetricRows(metrics),
    events,
    statusRows,
    readyState: ready.status || ready.state || ready.ready || "unknown"
  };
}

function renderPanopticon(snapshot = {}, packet = FALLBACK_PACKET) {
  const vm = buildPanopticonViewModel(snapshot, packet);
  setText("live-status", vm.mode === "live" ? "live loopback" : vm.mode === "published" ? "published runtime mirror" : "retained fallback");
  setText("hero-live-mode", vm.mode === "live" ? "Online" : vm.mode === "published" ? "Published" : "Retained");
  setText("hero-map-mode", vm.mode === "live" ? "live graph" : vm.mode === "published" ? "published graph" : "retained graph");
  setText("hero-event-title", vm.mode === "live" ? "Event Stream (Live Loopback)" : "Event Stream");
  setText("statusbar-mode", vm.mode === "live" ? "Live Loopback" : vm.mode === "published" ? "Published Mirror" : "Retained Mirror");
  const modeSelect = document.getElementById("top-mode-select");
  if (modeSelect) {
    modeSelect.value = vm.mode === "live" ? "live" : vm.mode === "published" ? "published" : "retained";
  }
  setText("statusbar-updated", vm.mode === "live" ? formatTimestampLabel(vm.fetchedAt) : formatCurrentTimestampLabel());
  setDataset("statusbar-indicator", "state", vm.mode === "live" ? "live" : vm.mode === "published" ? "published" : "fallback");
  setText("agent-count", `${vm.agentTotal.toLocaleString()} agents`);
  setText("hero-agent-count", `${vm.agentTotal.toLocaleString()} Agents`);
  setText("live-readiness", formatLabel(vm.readyState));
  setText("hero-ready-state", formatLabel(vm.readyState));
  setDataset("hero-agent-map", "state", formatLabel(vm.readyState));
  setText("live-updated", vm.fetchedAt ? new Date(vm.fetchedAt).toLocaleTimeString() : "not connected");
  setText("live-event-count", `${vm.events.length} events`);
  setText("hero-event-count", `${vm.events.length} Events`);
  setText("hero-gauge-agents", vm.agentTotal.toLocaleString());
  setText("hero-gauge-events", String(vm.events.length));
  setText("hero-gauge-metrics", String(vm.metrics.length));
  setText("hero-gauge-ready", formatLabel(vm.readyState));
  setText("agent-heartbeat", vm.fetchedAt ? new Date(vm.fetchedAt).toLocaleTimeString() : "retained");
  setText("agent-state", formatLabel(vm.readyState));
  setText("hero-event-detail", vm.events.length ? `${vm.events.length} retained or live CSM events visible.` : "No CSM events visible yet.");
  setText("live-metric-count", `${vm.metrics.length} gauges`);
  setText("hero-ready-detail", vm.signals.find((signal) => signal.label === "readiness")?.detail || "CSM /ready");
  setText("hero-latest-event", vm.events.length ? `event ${vm.events.length}` : "event 0");
  setState("hero-ready-state", vm.readyState);

  renderRows("panopticon-map", vm.agents.map((agent) => `
    <article class="agent-node" data-state="${escapeHtml(stateTone(agent.state))}">
      <span class="row-kicker">${escapeHtml(formatLabel(agent.role))}</span>
      <strong>${escapeHtml(agent.label || agent.id)}</strong>
      <p class="row-detail">${escapeHtml(formatLabel(agent.state))} / ${escapeHtml(agent.detail || agent.id)}</p>
    </article>
  `));

  renderRows("hero-agent-map", vm.agents.length ? vm.agents.slice(0, 6).map((agent) => `
    <article class="hero-agent-node" data-state="${escapeHtml(stateTone(agent.state))}">
      <svg class="node-icon"><use href="#${escapeHtml(iconForAgent(agent))}"></use></svg>
      <span class="row-kicker">${escapeHtml(formatLabel(agent.role))}</span>
      <strong>${escapeHtml(agent.label || agent.id)}</strong>
      <p class="row-detail">${escapeHtml(formatLabel(agent.state))}</p>
    </article>
  `) : [`
    <article class="hero-agent-node" data-state="pending">
      <svg class="node-icon"><use href="#icon-bot"></use></svg>
      <span class="row-kicker">agents</span>
      <strong>Waiting</strong>
      <p class="row-detail">No retained or live CSM agents visible yet.</p>
    </article>
  `]);

  renderRows("live-agent-list", vm.agents.map((agent) => `
    <article class="agent-row" data-state="${escapeHtml(stateTone(agent.state))}">
      <span class="row-kicker">${escapeHtml(agent.id)}</span>
      <strong>${escapeHtml(agent.label || agent.id)}</strong>
      <p class="row-detail">${escapeHtml(formatLabel(agent.state))} / ${escapeHtml(formatLabel(agent.role))}</p>
    </article>
  `));

  renderRows("live-signal-list", vm.signals.map((signal) => `
    <article class="signal-row" data-state="${escapeHtml(stateTone(signal.value))}">
      <span class="row-kicker">${escapeHtml(formatLabel(signal.label))}</span>
      <strong>${escapeHtml(formatLabel(signal.value))}</strong>
      <p class="row-detail">${escapeHtml(signal.detail)}</p>
    </article>
  `));

  renderRows("live-metric-list", vm.metrics.map((metric) => `
    <article class="metric-row">
      <strong>${escapeHtml(formatLabel(metric.label))}</strong>
      <span class="metric-value">${escapeHtml(metric.value)}</span>
    </article>
  `));

  renderRows("live-event-stream", vm.events.slice(-8).map((event, index) => `
    <li class="trace-row">
      <span class="trace-seq">${String(index + 1).padStart(2, "0")}</span>
      <span><strong>${escapeHtml(formatLabel(event.signal_kind || event.event_type || event.status || "event"))}</strong><br><span class="row-detail">${escapeHtml(event.runtime_id || event.agent_id || event.correlation_id || event.timestamp || event.message || "retained event")}</span></span>
    </li>
  `));

  const heroEventRows = vm.events.length ? vm.events.slice(-6).map((event, index) => {
    const eventName = formatLabel(event.signal_kind || event.event_type || event.status || "event");
    const source = event.agent_id || event.agent_instance_id || event.runtime_id || "csm";
    const state = formatLabel(event.status || event.result || event.details?.result || "ok");
    const tick = event.manifold_tick || event.tick || event.sequence || event.event_sequence || index + 1;
    const severity = stateTone(state) === "failed" ? "ERROR" : stateTone(state) === "degraded" ? "WARN" : "INFO";
    const eventTime = event.timestamp ? new Date(event.timestamp).toLocaleTimeString([], { hour12: false }) : `T-${String(vm.events.length - index).padStart(2, "0")}`;
    return `
    <li class="trace-row event-table-row">
      <span class="trace-seq">${escapeHtml(eventTime)}</span>
      <span class="event-severity" data-state="${escapeHtml(stateTone(state))}">${escapeHtml(severity)}</span>
      <span class="event-source">${escapeHtml(source)}</span>
      <span><strong>${escapeHtml(eventName)}</strong></span>
      <span class="event-state" data-state="${escapeHtml(stateTone(state))}">${escapeHtml(state)}</span>
      <span class="event-tick">${escapeHtml(tick)}</span>
    </li>
  `;
  }) : [`
    <li class="trace-row event-table-row">
      <span class="trace-seq">00</span>
      <span class="event-severity" data-state="degraded">WAIT</span>
      <span class="event-source">CSM API</span>
      <span><strong>Waiting</strong></span>
      <span class="event-state" data-state="degraded">pending</span>
      <span class="event-tick">0</span>
    </li>
  `];
  renderRows("hero-event-stream", [
    `<li class="event-table-header" aria-hidden="true">
      <span>Time</span>
      <span>Severity</span>
      <span>Source</span>
      <span>Event</span>
      <span>State</span>
      <span>Tick</span>
    </li>`,
    ...heroEventRows
  ]);
}

function renderObservatory(packet, reportText = "", state = "ok") {
  const vm = buildViewModel(packet, reportText);
  const source = vm.packet.source || {};
  const manifold = vm.packet.manifold || {};
  const pulse = vm.packet.kernel?.pulse || {};

  setText("packet-status", state === "ok" ? "CSM Runtime" : "Fallback shell");
  document.getElementById("packet-status")?.setAttribute("data-state", state);
  setText("claim-boundary", displayClaimBoundary(source));
  setText("evidence-level", formatLabel(source.evidence_level));
  document.getElementById("evidence-level")?.setAttribute("data-tone", state === "ok" ? "ok" : "warn");
  setText("packet-heading", "Owner Agent (owner-v2)");
  setText("manifold-id", displayManifoldId(manifold.manifold_id));
  setText("manifold-state", formatLabel(manifold.state));
  setText("manifold-tick", String(manifold.current_tick ?? 0));
  setText("packet-id", displayPacketId(vm.packet.packet_id));
  setText("hero-uptime", formatCurrentTimestampLabel());
  setText("rail-capture-time", formatCurrentTimestampLabel());
  setText("rail-manifold-id", displayManifoldId(manifold.manifold_id));
  setText("rail-state", formatLabel(manifold.state));
  setText("rail-tick", String(manifold.current_tick ?? 0));
  setText("statusbar-source", displayManifoldId(manifold.manifold_id || vm.packet.packet_id));
  setText("kernel-status", formatLabel(pulse.status));
  setText("latest-event", `event ${vm.latestEvent}`);
  setText("decision-counts", `${vm.decisionCounts.allow} / ${vm.decisionCounts.defer} / ${vm.decisionCounts.refuse}`);
  setText(
    "report-summary",
    reportText.includes("CSM Observatory Operator Report")
      ? "The operator report loaded from the same retained runtime artifact root as the packet."
      : "The operator report link is retained; report text did not load in this browser context."
  );

  renderRows("orbit-map", [
    `<div class="orbit-center"><strong>${formatLabel(manifold.state)}</strong><span class="row-kicker">${formatLabel(source.mode)}</span></div>`,
    ...vm.citizens.slice(0, 3).map((citizen) => `
      <article class="orbit-node">
        <span class="row-kicker">${formatLabel(citizen.lifecycle_state)} / ${formatLabel(citizen.role)}</span>
        <strong>${citizen.display_name}</strong>
        <p class="row-detail">${formatLabel(citizen.continuity_status)}</p>
      </article>
    `)
  ]);

  renderRows("service-list", vm.services.map((service) => `
    <article class="service-row">
      <span class="row-kicker">${formatLabel(service.service_id)}</span>
      <strong>${formatLabel(service.state || service.lifecycle_state)}</strong>
    </article>
  `));

  renderRows("decision-stack", vm.decisions.map((decision) => `
    <article class="decision-row" data-decision="${decision.decision}">
      <span class="row-kicker">${formatLabel(decision.decision)} / ${decision.actor}</span>
      <strong>${formatLabel(decision.action)}</strong>
      <p class="row-detail">${decision.rationale || decision.evidence_ref || "No rationale recorded."}</p>
    </article>
  `));

  renderRows("trace-list", vm.traceTail.map((event) => `
    <li class="trace-row">
      <span class="trace-seq">${String(event.event_sequence).padStart(2, "0")}</span>
      <span><strong>${formatLabel(event.event_type)}</strong><br><span class="row-detail">${event.summary}</span></span>
    </li>
  `));

  renderRows("invariant-list", vm.invariants.map((invariant) => `
    <article class="invariant-row">
      <span class="row-kicker">${formatLabel(invariant.severity)} / ${formatLabel(invariant.state)}</span>
      <strong>${invariant.name}</strong>
      <p class="row-detail">${invariant.evidence_ref}</p>
    </article>
  `));

  renderRows("action-list", [
    ...vm.availableActions.map((action) => `
      <article class="action-row">
      <span class="row-kicker">available / ${displayMilestoneText(formatLabel(action.mode))}</span>
      <strong>${displayMilestoneText(formatLabel(action.action))}</strong>
      <p class="row-detail">${displayMilestoneText(formatLabel(action.status))}</p>
      </article>
    `),
    ...vm.disabledActions.map((action) => `
      <article class="action-row">
        <span class="row-kicker">disabled</span>
      <strong>${displayMilestoneText(formatLabel(action.action))}</strong>
      <p class="row-detail">${displayMilestoneText(action.reason)}</p>
      </article>
    `)
  ]);
}

function renderIntegrations(integrationInputs = {}) {
  const vm = buildIntegrationViewModel(integrationInputs);

  const csmApiStatus = vm.serviceRows.every((row) => row.state === "closed") ? "wired" : "check evidence";
  const cloudwatchStatus = vm.cloudwatchSummary.status || "pending";
  setText("csm-api-status", csmApiStatus);
  setText("hero-csm-api-status", csmApiStatus);
  setText("cloudwatch-status", cloudwatchStatus === "passed" ? "live proof" : formatLabel(cloudwatchStatus));
  setText("hero-cloudwatch-state", cloudwatchStatus === "passed" ? "CloudWatch Proven" : formatLabel(cloudwatchStatus));
  setText(
    "hero-cloudwatch-detail",
    vm.cloudwatchRows.find((row) => row.label === "CloudWatch target")?.detail || "CloudWatch heartbeat proof pending load."
  );
  setState("hero-cloudwatch-state", vm.cloudwatchSummary.status || "pending");
  setText("cloudwatch-event-count", `${vm.parsedEvents.length} events`);

  renderRows("csm-api-list", vm.serviceRows.map((row) => `
    <article class="integration-row" data-state="${row.state}">
      <span class="row-kicker">${formatLabel(row.label)}</span>
      <strong>${formatLabel(row.value)}</strong>
      <p class="row-detail">${row.detail}</p>
    </article>
  `));

  renderRows("hero-api-list", vm.serviceRows.slice(0, 3).map((row, index) => `
    <span class="api-mini-row" data-state="${escapeHtml(stateTone(row.state))}">
      <span>GET ${index === 0 ? "/status" : index === 1 ? "/health" : "/ready"}</span>
      <strong>${row.state === "closed" ? "proved" : escapeHtml(formatLabel(row.state))}</strong>
      <em>retained</em>
    </span>
  `));

  renderRows("cloudwatch-list", vm.cloudwatchRows.map((row) => `
    <article class="integration-row" data-state="${row.state}">
      <span class="row-kicker">${formatLabel(row.label)}</span>
      <strong>${formatLabel(row.value)}</strong>
      <p class="row-detail">${row.detail}</p>
    </article>
  `));

  renderRows("cloudwatch-events-list", vm.parsedEvents.slice(-5).map((event, index) => `
    <li class="trace-row">
      <span class="trace-seq">${String(index + 1).padStart(2, "0")}</span>
      <span><strong>${formatLabel(event.signal_kind)} / ${formatLabel(event.payload?.state)}</strong><br><span class="row-detail">${event.runtime_id || "unknown runtime"} / ${event.timestamp || "unknown timestamp"}</span></span>
    </li>
  `));

  renderRows("aws-linkage-list", AWS_LINKAGES.map((linkage) => `
    <article class="linkage-row" data-state="${linkage.state}">
      <span class="row-kicker">#${linkage.issue} / ${formatLabel(linkage.state)}</span>
      <strong>${linkage.label}</strong>
      <p class="row-detail">${linkage.proof} Evidence: ${linkage.evidence}.</p>
    </article>
  `));

  renderRows("communication-proof-list", vm.acipRows.map((row) => `
    <article class="communication-proof-row" data-state="${row.state}">
      <span class="row-kicker">${formatLabel(row.label)}</span>
      <strong>${formatLabel(row.value)}</strong>
      <p class="row-detail">${row.detail}</p>
    </article>
  `));

  renderRows("compact-comms-proof", vm.acipRows.slice(0, 3).map((row) => `
    <span class="compact-proof-chip" data-state="${row.state}">
      <span>${formatLabel(row.label)}</span>
      <strong>${formatLabel(row.value)}</strong>
    </span>
  `));
}

function bindCommunication(packet = FALLBACK_PACKET, acipSnsSummary = {}, snsResourceSummary = {}) {
  const channel = document.getElementById("operator-channel");
  const message = document.getElementById("operator-message");
  const compactMessage = document.getElementById("compact-operator-message");
  const apiBase = document.getElementById("runtime-api-base");
  const prepare = document.getElementById("prepare-envelope");
  const checkEvents = document.getElementById("check-events");
  const compactClear = document.getElementById("compact-clear-envelope");
  const packetId = displayPacketId(packet.packet_id || "");
  const setCommunicationStatus = (status) => {
    setText("communication-status", status);
    setText("hero-communication-status", status);
  };

  const updateEnvelope = () => {
    const envelope = buildOperatorEnvelope({
      channel: channel?.value || "events",
      message: compactMessage?.value || message?.value || "",
      packetId,
      acipSnsSummary,
      snsResourceSummary
    });
    renderEnvelope(envelope);
    setCommunicationStatus("envelope ready");
  };

  prepare?.addEventListener("click", updateEnvelope);
  compactMessage?.addEventListener("input", () => {
    if (message) {
      message.value = compactMessage.value;
    }
  });
  compactClear?.addEventListener("click", () => {
    if (message) {
      message.value = "";
    }
    if (compactMessage) {
      compactMessage.value = "";
    }
    renderEnvelope({});
    setCommunicationStatus("draft cleared");
  });
  checkEvents?.addEventListener("click", async () => {
    setCommunicationStatus("checking /events");
    try {
      const events = await checkEventsEndpoint(apiBase?.value || "");
      const eventEntries = normalizeEventEntries(events);
      const envelope = buildOperatorEnvelope({
        channel: "events",
        message: `Read ${eventEntries.length} retained CSM events from live API.`,
        packetId,
        acipSnsSummary,
        snsResourceSummary
      });
      renderEnvelope({ ...envelope, live_event_count: eventEntries.length });
      setCommunicationStatus("events reachable");
    } catch (error) {
      renderEnvelope({
        ...buildOperatorEnvelope({
          channel: channel?.value || "events",
          message: compactMessage?.value || message?.value || "",
          packetId,
          acipSnsSummary,
          snsResourceSummary
        }),
        live_check_error: error instanceof Error ? error.message : "unknown error"
      });
      setCommunicationStatus("offline draft");
    }
  });

  updateEnvelope();
}

function bindLivePanopticon(packet = FALLBACK_PACKET) {
  const apiBase = document.getElementById("live-api-base");
  const dashboardBase = document.getElementById("dashboard-live-api-base");
  const communicationBase = document.getElementById("runtime-api-base");
  const connect = document.getElementById("connect-live");
  const refresh = document.getElementById("refresh-live");
  const stop = document.getElementById("stop-live");
  const dashboardConnect = document.getElementById("dashboard-connect-live");
  const dashboardRefresh = document.getElementById("dashboard-refresh-live");
  const dashboardStop = document.getElementById("dashboard-stop-live");
  const modeSelect = document.getElementById("top-mode-select");
  const operatorToken = document.getElementById("operator-write-token");
  const operatorLogin = document.getElementById("operator-login");
  const operatorLogout = document.getElementById("operator-logout");
  const operatorAuthStatus = document.getElementById("operator-auth-status");
  const signedControlCommand = document.getElementById("signed-control-command");
  const sendSignedCommand = document.getElementById("send-signed-command");
  const operatorControlResult = document.getElementById("operator-control-result");
  let lastLiveError = null;
  let runtimeBaseActive = false;
  let liveSocket = null;
  let liveStoppedByOperator = false;
  let liveRequestGeneration = 0;
  const refs = {
    statusRef: document.querySelector(".observatory")?.dataset.csmStatusRef || "",
    healthRef: document.querySelector(".observatory")?.dataset.csmHealthRef || "",
    readyRef: document.querySelector(".observatory")?.dataset.csmReadyRef || "",
    metricsRef: document.querySelector(".observatory")?.dataset.csmMetricsRef || "",
    eventsRef: document.querySelector(".observatory")?.dataset.csmEventsRef || ""
  };

  const mirrorApiBase = (base) => {
    [apiBase, dashboardBase, communicationBase].forEach((input) => {
      if (input && base && !input.value) {
        input.value = base;
      }
    });
  };

  const setRuntimeTestStatus = (status, detail = "") => {
    setText("dashboard-live-test-status", status);
    setState("dashboard-live-test-status", status);
    if (detail) {
      setText("dashboard-live-test-detail", detail);
    }
  };

  const setWriteAccess = (enabled, status, detail) => {
    if (operatorAuthStatus) {
      operatorAuthStatus.textContent = status;
      operatorAuthStatus.dataset.state = enabled ? "passed" : "open";
    }
    if (sendSignedCommand) {
      sendSignedCommand.disabled = !enabled;
    }
    if (operatorControlResult && detail) {
      operatorControlResult.textContent = detail;
    }
  };

  const renderControlFrame = (frame) => {
    if (frame.status === "authenticated") {
      setWriteAccess(true, "write access enabled", JSON.stringify(frame, null, 2));
      return;
    }
    if (frame.error === "credential_revoked" ||
        frame.error === "authentication_failed" ||
        frame.error === "write_authentication_required") {
      setWriteAccess(false, "public read", JSON.stringify(frame, null, 2));
      return;
    }
    if (operatorControlResult) {
      operatorControlResult.textContent = JSON.stringify(frame, null, 2);
    }
  };

  const readApiBase = () => normalizeApiBase(dashboardBase?.value || apiBase?.value || communicationBase?.value || "");
  const setLiveConnectionState = (state) => {
    document.querySelector(".observatory")?.setAttribute("data-live-connection", state);
  };
  mirrorApiBase(getQueryApiBase());

  const renderMinimalFallback = (error) => {
    renderPanopticon({
      mode: "retained",
      fetchedAt: new Date().toISOString(),
      errors: {
        retained: error instanceof Error ? error.message : "unknown retained mirror error"
      }
    }, packet);
    setText("live-status", "retained fallback");
    setRuntimeTestStatus("retained fallback", error instanceof Error ? error.message : "Retained runtime mirror is available; live loopback is not connected.");
  };

  const refreshRetained = async (extraErrors = {}) => {
    try {
      const snapshot = await fetchRetainedRuntimeSnapshot(refs);
      const mergedSnapshot = {
        ...snapshot,
        errors: {
          ...(snapshot.errors || {}),
          ...(lastLiveError ? { live: lastLiveError } : {}),
          ...extraErrors
        }
      };
      renderPanopticon(mergedSnapshot, packet);
      const status = Object.keys(mergedSnapshot.errors || {}).length ? "published partial" : "published runtime mirror";
      setText("live-status", status);
      setRuntimeTestStatus(status, lastLiveError ? `Live loopback not proved: ${lastLiveError}` : "Using retained publishable CSM API artifacts until a loopback runtime is connected.");
    } catch (error) {
      renderMinimalFallback(error);
    }
  };

  const renderLiveError = async (error) => {
    lastLiveError = error instanceof Error ? error.message : "unknown live polling error";
    await refreshRetained({
      live: lastLiveError
    });
  };

  const refreshLive = async () => {
    const requestGeneration = liveRequestGeneration;
    const base = readApiBase();
    if (!base) {
      runtimeBaseActive = false;
      await refreshRetained();
      return;
    }
    runtimeBaseActive = true;
    if (communicationBase && base && !communicationBase.value) {
      communicationBase.value = base;
    }
    mirrorApiBase(base);
    setText("live-status", "polling loopback");
    setRuntimeTestStatus("polling loopback", `Checking ${base}/status, /health, /ready, /metrics, and /events.`);
    try {
      const snapshot = await fetchRuntimeSnapshot(base);
      if (liveStoppedByOperator || requestGeneration !== liveRequestGeneration) {
        return;
      }
      const endpointKeys = ["status", "health", "ready", "metrics", "events"];
      const successfulEndpoints = endpointKeys.filter((key) => snapshot[key]);
      if (successfulEndpoints.length === 0) {
        throw new Error("No CSM runtime API endpoints responded from the browser context.");
      }
      lastLiveError = null;
      renderPanopticon(snapshot, packet);
      const status = Object.keys(snapshot.errors || {}).length ? "live partial" : "live loopback";
      setText("live-status", status);
      const runtimeKind = snapshot.runtimeSelection === "runtime_v3_explicit_opt_in" ? "Runtime v3 observatory feed" : "loopback CSM server";
      setRuntimeTestStatus(status, Object.keys(snapshot.errors || {}).length ? "Runtime reached, but one or more endpoints failed." : `Runtime API endpoints responded from the ${runtimeKind}.`);
    } catch (error) {
      if (liveStoppedByOperator || requestGeneration !== liveRequestGeneration) {
        return;
      }
      await renderLiveError(error);
    }
  };

  const stopPolling = () => {
    liveStoppedByOperator = true;
    liveRequestGeneration += 1;
    setLiveConnectionState("stopped");
    if (liveSocket) {
      liveSocket.close(1000, "operator_stop");
      liveSocket = null;
    }
    if (livePollTimer) {
      clearInterval(livePollTimer);
      livePollTimer = null;
    }
    if (retainedPollTimer) {
      clearInterval(retainedPollTimer);
      retainedPollTimer = null;
    }
    lastLiveError = null;
    runtimeBaseActive = false;
    setText("live-status", "polling stopped");
    setText("statusbar-websocket", "stopped");
    setRuntimeTestStatus("polling stopped", "Live polling is stopped; retained mirror remains available.");
  };

  const connectLive = () => {
    stopPolling();
    liveStoppedByOperator = false;
    setLiveConnectionState("connecting");
    if (requestedRuntimeSelection() === "v3") {
      runtimeBaseActive = true;
      setText("live-status", "connecting secure stream");
      setText("statusbar-websocket", "connecting");
      try {
        const base = readApiBase();
        const socketEndpoint = new URL(`${base}${RUNTIME_V3_OBSERVATORY_WS_ENDPOINT}`);
        socketEndpoint.protocol = "wss:";
        setRuntimeTestStatus("connecting secure stream", `Opening ${socketEndpoint}.`);
        let socket;
        socket = connectRuntimeV3ObservatoryWebSocket(
          base,
          (snapshot) => {
            if (liveStoppedByOperator || liveSocket !== socket) {
              return;
            }
            lastLiveError = null;
            renderPanopticon(snapshot, packet);
            setText("live-status", "live secure stream");
            setText("statusbar-websocket", "connected");
            setLiveConnectionState("connected");
            setRuntimeTestStatus("live secure stream", "Runtime v3 public WebSocket feed is active; operator login is required only for writes.");
          },
          (error) => {
            if (liveStoppedByOperator || liveSocket !== socket) {
              return;
            }
            setText("statusbar-websocket", "disconnected");
            renderLiveError(error);
          },
          (error) => {
            if (liveStoppedByOperator) {
              return;
            }
            if (liveSocket === socket) {
              liveSocket = null;
              setText("statusbar-websocket", "disconnected");
              setWriteAccess(false, "public read", "The live connection closed. Public monitoring can reconnect without login.");
              renderLiveError(error);
            }
          },
          (frame) => {
            if (liveStoppedByOperator || liveSocket !== socket) {
              return;
            }
            renderControlFrame(frame);
          }
        );
        liveSocket = socket;
      } catch (error) {
        setText("statusbar-websocket", "disconnected");
        renderLiveError(error);
      }
      return;
    }
    refreshLive();
    livePollTimer = setInterval(refreshLive, 3000);
  };

  if (connect) connect.onclick = connectLive;
  if (refresh) refresh.onclick = refreshLive;
  if (stop) stop.onclick = stopPolling;
  if (dashboardConnect) dashboardConnect.onclick = connectLive;
  if (dashboardRefresh) dashboardRefresh.onclick = refreshLive;
  if (dashboardStop) dashboardStop.onclick = stopPolling;
  modeSelect?.addEventListener("change", () => {
    if (modeSelect.value === "live") {
      connectLive();
      return;
    }
    stopPolling();
    if (modeSelect.value === "published") {
      refreshRetained();
      return;
    }
    renderPanopticon({
      mode: "retained",
      fetchedAt: new Date().toISOString(),
      status: {},
      health: {},
      ready: {},
      metrics: {},
      events: [],
      errors: {}
    }, packet);
    setText("live-status", "retained mirror");
    setRuntimeTestStatus("retained mirror", "Showing the retained proof packet without live or published endpoint polling.");
  });
  operatorLogin?.addEventListener("click", () => {
    const token = operatorToken?.value.trim() || "";
    if (!token) {
      setWriteAccess(false, "login required", "Enter the operator write token.");
      return;
    }
    globalThis.sessionStorage?.setItem("adl.runtimeV3.observatoryToken", token);
    if (!liveSocket || liveSocket.readyState !== WebSocket.OPEN) {
      setWriteAccess(false, "connecting", "Opening the public stream before operator login.");
      connectLive();
      return;
    }
    setWriteAccess(false, "logging in", "Authenticating write access...");
    authenticateRuntimeV3ObservatorySocket(liveSocket, token);
  });
  operatorLogout?.addEventListener("click", () => {
    globalThis.sessionStorage?.removeItem("adl.runtimeV3.observatoryToken");
    if (operatorToken) {
      operatorToken.value = "";
    }
    liveSocket?.close(1000, "operator_logout");
    liveSocket = null;
    setWriteAccess(false, "public read", "Write access cleared. Public monitoring remains available.");
    connectLive();
  });
  sendSignedCommand?.addEventListener("click", () => {
    if (!liveSocket || liveSocket.readyState !== WebSocket.OPEN) {
      setWriteAccess(false, "connection required", "Connect to the Runtime v3 Observatory before sending a command.");
      return;
    }
    try {
      const command = JSON.parse(signedControlCommand?.value || "");
      if (command.schema !== "adl.runtime.control_command.v1") {
        throw new Error("Expected an adl.runtime.control_command.v1 signed envelope.");
      }
      liveSocket.send(JSON.stringify(command));
      if (operatorControlResult) {
        operatorControlResult.textContent = "Signed command submitted; awaiting runtime verification.";
      }
    } catch (error) {
      if (operatorControlResult) {
        operatorControlResult.textContent = error instanceof Error ? error.message : "Invalid signed command.";
      }
    }
  });

  const queryApiBase = getQueryApiBase();
  if (queryApiBase) {
    refreshLive();
  } else {
    refreshRetained();
  }
  if (queryApiBase && shouldAutoConnectLive()) {
    connectLive();
  }
  if (!retainedPollTimer && !runtimeBaseActive && !queryApiBase) {
    retainedPollTimer = setInterval(refreshRetained, 3000);
  }
}

async function loadText(ref) {
  if (!ref) {
    return "";
  }
  const response = await fetch(ref);
  if (!response.ok) {
    throw new Error(`failed to load ${ref}: ${response.status}`);
  }
  return response.text();
}

async function loadJson(ref) {
  const text = await loadText(ref);
  return JSON.parse(text);
}

async function bootObservatory() {
  const root = document.querySelector(".observatory");
  const packetRef = root?.dataset.packetRef || "";
  const reportRef = root?.dataset.reportRef || "";
  const csmServiceRef = root?.dataset.csmServiceRef || "";
  const csmApiRef = root?.dataset.csmApiRef || "";
  const cloudwatchRef = root?.dataset.cloudwatchRef || "";
  const cloudwatchEventsRef = root?.dataset.cloudwatchEventsRef || "";
  const acipSnsRef = root?.dataset.acipSnsRef || "";
  const snsResourceRef = root?.dataset.snsResourceRef || "";
  setHref("packet-link", packetRef);
  setHref("report-link", reportRef);
  const runtimeApiBase = getQueryApiBase();
  if (requestedRuntimeSelection() === "v3" && runtimeApiBase) {
    setHref("packet-link", `${runtimeApiBase}${RUNTIME_V3_OBSERVATORY_ENDPOINT}`);
    setHref("report-link", `${runtimeApiBase}/v1/observatory/docs/`);
  }

  try {
    const [packet, reportText, serviceManifest, apiText, cloudwatchSummary, cloudwatchEvents, acipSnsSummary, snsResourceSummary] = await Promise.all([
      loadJson(packetRef),
      loadText(reportRef).catch(() => ""),
      loadJson(csmServiceRef).catch(() => ({})),
      loadText(csmApiRef).catch(() => ""),
      loadJson(cloudwatchRef).catch(() => ({})),
      loadJson(cloudwatchEventsRef).catch(() => ({})),
      loadJson(acipSnsRef).catch(() => ({})),
      loadJson(snsResourceRef).catch(() => ({}))
    ]);
    renderObservatory(packet, reportText, "ok");
    renderIntegrations({ serviceManifest, apiText, cloudwatchSummary, cloudwatchEvents, acipSnsSummary, snsResourceSummary });
    bindDashboardNavigation(packet);
    bindCommunication(packet, acipSnsSummary, snsResourceSummary);
    bindLivePanopticon(packet);
  } catch (_error) {
    renderObservatory(FALLBACK_PACKET, "", "fallback");
    renderIntegrations();
    bindDashboardNavigation(FALLBACK_PACKET);
    bindCommunication(FALLBACK_PACKET);
    bindLivePanopticon(FALLBACK_PACKET);
  }
}

if (typeof document !== "undefined") {
  bootObservatory();
}

globalThis.AdlHtmlObservatory = {
  FALLBACK_PACKET,
  AWS_LINKAGES,
  formatLabel,
  parseCloudWatchEventMessage,
  buildOperatorEnvelope,
  normalizeApiBase,
  isLoopbackApiBase,
  getQueryApiBase,
  fetchRuntimeSnapshot,
  fetchRuntimeV3ObservatorySnapshot,
  runtimeV3SnapshotFromFeed,
  connectRuntimeV3ObservatoryWebSocket,
  authenticateRuntimeV3ObservatorySocket,
  fetchRetainedRuntimeSnapshot,
  requestedRuntimeSelection,
  isRuntimeV3ApiBase,
  buildRuntimeAgentRows,
  buildViewModel,
  buildIntegrationViewModel,
  buildPanopticonViewModel,
  normalizeEventEntries,
  renderObservatory,
  renderIntegrations,
  bindDashboardNavigation,
  updateDashboardFocus,
  bindLivePanopticon,
  renderPanopticon
};
