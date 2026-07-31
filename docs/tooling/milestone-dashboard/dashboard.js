const milestoneData = window.milestoneData;

if (!milestoneData) {
  throw new Error("milestone dashboard data is missing; load a milestone data script before dashboard.js");
}

const state = {
  statusFilter: "all",
  laneFilter: "all"
};

const workcellStates = new Set(["live", "retained", "stale", "unknown", "blocked", "non-authoritative"]);

function byId(id) {
  return document.getElementById(id);
}

function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}

function humanStatus(status) {
  return {
    complete: "Complete",
    active: "Active",
    live: "Live",
    retained: "Retained",
    unknown: "Unknown / unverified",
    blocked: "Blocked / gated",
    stale: "Stale",
    "non-authoritative": "Non-authoritative",
    guarded: "Guarded",
    bounded: "Bounded",
    warn: "Attention",
    good: "Healthy"
  }[status] || status;
}

function statusClass(status) {
  return `status-${status}`;
}

function safeState(status) {
  return workcellStates.has(status) ? status : "unknown";
}

function safeList(value, limit = 20) {
  return Array.isArray(value) ? value.slice(0, limit) : [];
}

function boundedText(value, limit = 240) {
  const text = String(value ?? "unknown");
  return text.length > limit ? `${text.slice(0, limit)}...` : text;
}

function snapshotAgeHours() {
  if (!milestoneData.snapshotGeneratedAt) {
    return null;
  }

  const snapshotMs = Date.parse(milestoneData.snapshotGeneratedAt);
  if (Number.isNaN(snapshotMs)) {
    return null;
  }

  return (Date.now() - snapshotMs) / (1000 * 60 * 60);
}

function freshnessState(baseState) {
  const age = snapshotAgeHours();
  const threshold = milestoneData.snapshotMaxAgeHours;
  if (age === null || !threshold) {
    return baseState;
  }
  return age > threshold ? "stale" : baseState;
}

function renderMeta() {
  byId("milestone-chip").textContent = `${milestoneData.milestone} / ${milestoneData.version}`;
  byId("headline").textContent = `${milestoneData.milestone} compression dashboard`;
  byId("lede").textContent = milestoneData.summary;

  const badge = byId("status-badge");
  badge.textContent = milestoneData.statusLabel;
  badge.className = `status-badge ${statusClass(milestoneData.status)}`;

  const meta = [
    ["Owner", milestoneData.owner],
    ["Updated", milestoneData.updated],
    ["Version", milestoneData.version],
    ["Boundary", "read-only"]
  ];

  byId("meta-list").innerHTML = meta
    .map(([key, value]) => `<dt>${escapeHtml(key)}</dt><dd>${escapeHtml(value)}</dd>`)
    .join("");

  byId("boundary-list").innerHTML = milestoneData.boundary
    .map((item) => `<li>${escapeHtml(item)}</li>`)
    .join("");

  byId("next-actions").innerHTML = milestoneData.nextActions
    .map((item) => `<li>${escapeHtml(item)}</li>`)
    .join("");
}

function renderFreshness() {
  byId("freshness-list").innerHTML = milestoneData.freshness
    .map((item) => {
      const resolvedState = item.label === "Staleness guard" ? freshnessState(item.state) : item.state;
      return `
        <li>
          <span class="inline-pill ${statusClass(resolvedState)}">${escapeHtml(humanStatus(resolvedState))}</span>
          <strong>${escapeHtml(item.label)}</strong>
          <span>${escapeHtml(item.note)}</span>
        </li>
      `;
    })
    .join("");
}

function renderSignals() {
  byId("signal-grid").innerHTML = milestoneData.signals
    .map(
      (signal) => `
        <article class="signal-card ${statusClass(signal.tone)}">
          <span>${escapeHtml(signal.label)}</span>
          <strong>${escapeHtml(signal.value)}</strong>
          <p>${escapeHtml(signal.note)}</p>
        </article>
      `
    )
    .join("");
}

function renderMetrics() {
  const total = milestoneData.workPackages.length;
  const complete = milestoneData.workPackages.filter((wp) => wp.status === "complete").length;
  const active = milestoneData.workPackages.filter((wp) => wp.status === "active").length;
  const unknown = milestoneData.workPackages.filter((wp) => wp.status === "unknown").length;
  const blocked = milestoneData.workPackages.filter((wp) => wp.status === "blocked").length;
  const percent = Math.round((complete / total) * 100);

  const metrics = [
    ["Closed", complete, "Merged or closed WP issues"],
    ["Active", active, "In review or in flight"],
    ["Unknown", unknown, "Open, not yet landed"],
    ["Blocked", blocked, "Release-tail gated"]
  ];

  byId("metric-stack").innerHTML = metrics
    .map(
      ([label, value, note]) => `
        <div class="metric">
          <span class="metric-label">${escapeHtml(label)}</span>
          <strong>${escapeHtml(value)}</strong>
          <span>${escapeHtml(note)}</span>
        </div>
      `
    )
    .join("");

  byId("progress-copy").textContent = `${percent}% closed`;
  requestAnimationFrame(() => {
    byId("progress-bar").style.width = `${percent}%`;
  });
}

function renderWatchlist() {
  byId("watchlist").innerHTML = milestoneData.watchlist
    .map(
      (item) => `
        <li>
          <span class="inline-pill ${statusClass(item.state)}">${escapeHtml(humanStatus(item.state))}</span>
          <strong>${escapeHtml(item.label)}</strong>
          <span>${escapeHtml(item.note)}</span>
        </li>
      `
    )
    .join("");
}

function renderLanes() {
  byId("lane-grid").innerHTML = milestoneData.lanes
    .map((lane) => {
      const wpCount = milestoneData.workPackages.filter((wp) => lane.wps.includes(wp.id)).length;
      return `
        <article class="lane-card">
          <div class="lane-topline">
            <div>
              <p class="section-kicker">${escapeHtml(lane.id)}</p>
              <h4>${escapeHtml(lane.title)}</h4>
            </div>
            <span class="status-pill ${statusClass(lane.status)}">${escapeHtml(humanStatus(lane.status))}</span>
          </div>
          <p>${escapeHtml(lane.purpose)}</p>
          <ul>
            <li>${wpCount} work packages</li>
            <li>${escapeHtml(lane.wps.join(", "))}</li>
          </ul>
        </article>
      `;
    })
    .join("");
}

function renderFilters() {
  const statusOptions = [
    ["all", "All"],
    ["complete", "Complete"],
    ["active", "Active"],
    ["unknown", "Unknown"],
    ["blocked", "Blocked"]
  ];

  const laneOptions = [
    ["all", "All lanes"],
    ...milestoneData.lanes.map((lane) => [lane.id, lane.title])
  ];

  const renderChips = (options, activeValue) =>
    options
      .map(([value, label]) => `<button class="chip${value === activeValue ? " active" : ""}" data-value="${escapeHtml(value)}" type="button">${escapeHtml(label)}</button>`)
      .join("");

  byId("status-filters").innerHTML = renderChips(statusOptions, state.statusFilter);
  byId("lane-filters").innerHTML = renderChips(laneOptions, state.laneFilter);

  byId("status-filters").querySelectorAll(".chip").forEach((chip) => {
    chip.addEventListener("click", () => {
      state.statusFilter = chip.dataset.value;
      renderFilters();
      renderWorkPackages();
    });
  });

  byId("lane-filters").querySelectorAll(".chip").forEach((chip) => {
    chip.addEventListener("click", () => {
      state.laneFilter = chip.dataset.value;
      renderFilters();
      renderWorkPackages();
    });
  });
}

function laneForWorkPackage(wpId) {
  return milestoneData.lanes.find((lane) => lane.wps.includes(wpId));
}

function renderWorkPackages() {
  const filtered = milestoneData.workPackages.filter((wp) => {
    const lane = laneForWorkPackage(wp.id);
    const statusOk = state.statusFilter === "all" || wp.status === state.statusFilter;
    const laneOk = state.laneFilter === "all" || lane?.id === state.laneFilter;
    return statusOk && laneOk;
  });

  byId("wp-list").innerHTML = filtered
    .map((wp) => {
      const lane = laneForWorkPackage(wp.id);
      return `
        <article class="wp-row">
          <div class="wp-title-block">
            <div class="wp-topline">
              <strong class="wp-title">${escapeHtml(wp.id)} · ${escapeHtml(wp.title)}</strong>
            </div>
            <p class="wp-desc">${escapeHtml(wp.evidence)}</p>
            <p class="wp-meta">Lane: ${escapeHtml(lane?.title || "unknown")} · Queue: ${escapeHtml(wp.queue)}</p>
          </div>
          <div><span class="status-pill ${statusClass(wp.status)}">${escapeHtml(humanStatus(wp.status))}</span></div>
          <div>${escapeHtml(wp.issue)}</div>
          <div>${escapeHtml(wp.validation)}</div>
          <div>${escapeHtml(wp.checks)}</div>
          <div>${escapeHtml(wp.action)}</div>
        </article>
      `;
    })
    .join("");
}

function renderAuthority() {
  byId("authority-list").innerHTML = milestoneData.authority
    .map(
      (doc) => `
        <article class="doc-item">
          <a href="${escapeHtml(doc.path)}">${escapeHtml(doc.label)}</a>
          <span class="doc-meta">${escapeHtml(doc.note)}</span>
        </article>
      `
    )
    .join("");
}

function renderValidationProfiles() {
  byId("validation-list").innerHTML = milestoneData.validationProfiles
    .map(
      (profile) => `
        <article class="review-item">
          <div>
            <strong>${escapeHtml(profile.label)}</strong>
            <p>${escapeHtml(profile.profile)}</p>
          </div>
          <span class="status-pill ${statusClass(profile.status)}">${escapeHtml(humanStatus(profile.status))}</span>
          <code>${escapeHtml(profile.command)}</code>
        </article>
      `
    )
    .join("");
}

function renderPrChecks() {
  byId("pr-check-list").innerHTML = milestoneData.prChecks
    .map(
      (item) => `
        <article class="issue-item">
          <span class="inline-pill ${statusClass(item.state)}">${escapeHtml(humanStatus(item.state))}</span>
          <strong>${escapeHtml(item.label)}</strong>
          <span class="issue-copy">${escapeHtml(item.note)}</span>
        </article>
      `
    )
    .join("");
}

function renderWorkcellItems(id, items) {
  byId(id).innerHTML = safeList(items)
    .map((item) => {
      const itemState = safeState(item.state);
      const meta = [item.role, item.source, item.revision, item.freshness, item.note]
        .filter(Boolean)
        .map((value) => boundedText(value, 180))
        .join(" / ");
      return `
        <article class="issue-item">
          <span class="inline-pill ${statusClass(itemState)}">${escapeHtml(humanStatus(itemState))}</span>
          <strong>${escapeHtml(boundedText(item.label, 120))}</strong>
          <span class="issue-copy">${escapeHtml(meta)}</span>
        </article>
      `;
    })
    .join("");
}

function runtimeConfigFromSearch(search, workcell) {
  if (typeof URL === "undefined" || typeof URLSearchParams === "undefined") {
    return null;
  }

  const params = new URLSearchParams(search || "");
  if (params.get("live") !== "1" || params.get("runtime") !== "v3") {
    return null;
  }

  const runtimeApiBase = params.get("runtimeApiBase");
  if (!runtimeApiBase) {
    return { state: "unknown", message: "Runtime v3 live mode requested without runtimeApiBase." };
  }

  let baseUrl;
  try {
    baseUrl = new URL(runtimeApiBase);
  } catch (_error) {
    return { state: "blocked", message: "Runtime v3 Observatory base URL is invalid." };
  }

  if (baseUrl.protocol !== workcell.runtime.requiredProtocol) {
    return { state: "blocked", message: "Runtime v3 Observatory reads require HTTPS." };
  }

  const allowedOrigins = safeList(workcell.runtime.allowedOrigins, 20);
  if (allowedOrigins.length === 0) {
    return { state: "blocked", message: "Runtime v3 Observatory origin allowlist is empty in the snapshot." };
  }
  if (!allowedOrigins.includes(baseUrl.origin)) {
    return { state: "blocked", message: "Runtime v3 Observatory origin is not allowlisted by the snapshot." };
  }

  return {
    state: "live",
    feedUrl: new URL(workcell.runtime.feedPath, baseUrl.origin).toString(),
    timeoutMs: workcell.runtime.timeoutMs,
    maxPayloadBytes: workcell.runtime.maxPayloadBytes,
    tokenStorageKey: workcell.runtime.tokenStorageKey
  };
}

function renderRuntimeStatus(stateName, message) {
  const status = safeState(stateName);
  const statusElement = byId("runtime-live-status");
  statusElement.className = `runtime-status ${statusClass(status)}`;
  statusElement.textContent = `${humanStatus(status)}: ${boundedText(message, 220)}`;
}

function renderRuntimeObservations(observations) {
  byId("runtime-observatory-list").innerHTML = safeList(observations, 8)
    .map(
      (item) => `
        <article class="runtime-observation">
          <span class="inline-pill ${statusClass(safeState(item.state))}">${escapeHtml(humanStatus(safeState(item.state)))}</span>
          <strong>${escapeHtml(boundedText(item.label, 90))}</strong>
          <span>${escapeHtml(boundedText(item.note, 180))}</span>
        </article>
      `
    )
    .join("");
}

function retainedRuntimeObservations(workcell) {
  return [
    {
      label: "Retained fallback",
      state: "retained",
      note: workcell.runtime.fallback
    },
    ...safeList(workcell.blockers, 4)
  ];
}

function observationsFromRuntimePayload(payload) {
  const agents = safeList(payload?.agents || payload?.topology?.agents || payload?.nodes, 6);
  const observedAt = payload?.observed_at || payload?.observedAt || payload?.timestamp || "live response";
  const total = payload?.total_agents || payload?.totalAgents || agents.length;
  const observations = [
    {
      label: "Runtime feed",
      state: "live",
      note: `Observed ${total} runtime agents at ${observedAt}.`
    }
  ];

  agents.forEach((agent) => {
    observations.push({
      label: agent.id || agent.name || agent.label || "runtime agent",
      state: safeState(agent.state || agent.status || "live"),
      note: agent.role || agent.kind || "Runtime v3 Observatory agent sample."
    });
  });

  return observations;
}

async function refreshRuntimeObservatory(workcell) {
  const search = window.location?.search || "";
  const config = runtimeConfigFromSearch(search, workcell);
  if (!config) {
    renderRuntimeStatus("retained", workcell.runtime.fallback);
    renderRuntimeObservations(retainedRuntimeObservations(workcell));
    return;
  }

  if (config.state !== "live") {
    renderRuntimeStatus(config.state, config.message);
    renderRuntimeObservations(retainedRuntimeObservations(workcell));
    return;
  }

  if (typeof fetch !== "function" || typeof AbortController === "undefined" || !window.sessionStorage) {
    renderRuntimeStatus("unknown", "Browser Runtime v3 fetch support or session token storage is unavailable.");
    renderRuntimeObservations(retainedRuntimeObservations(workcell));
    return;
  }

  const token = window.sessionStorage.getItem(config.tokenStorageKey);
  if (!token) {
    renderRuntimeStatus("unknown", "Runtime v3 Observatory session token is missing.");
    renderRuntimeObservations(retainedRuntimeObservations(workcell));
    return;
  }

  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), config.timeoutMs);
  try {
    const response = await fetch(config.feedUrl, {
      method: "GET",
      headers: {
        Accept: "application/json",
        Authorization: `Bearer ${token}`
      },
      signal: controller.signal
    });
    const text = await response.text();
    if (text.length > config.maxPayloadBytes) {
      throw new Error("Runtime v3 Observatory payload exceeded dashboard limit.");
    }
    if (!response.ok) {
      throw new Error(`Runtime v3 Observatory returned HTTP ${response.status}.`);
    }
    const payload = JSON.parse(text);
    renderRuntimeStatus("live", "Runtime v3 Observatory read feed responded.");
    renderRuntimeObservations(observationsFromRuntimePayload(payload));
  } catch (error) {
    renderRuntimeStatus("stale", error.message);
    renderRuntimeObservations(retainedRuntimeObservations(workcell));
  } finally {
    clearTimeout(timer);
  }
}

function renderWorkcellOperator() {
  const workcell = milestoneData.workcellOperator;
  if (!workcell || workcell.schema !== "adl.workcell.operator.snapshot.v1") {
    renderRuntimeStatus("unknown", "Workcell operator snapshot is unavailable.");
    return;
  }

  const workcellState = safeState(workcell.status);
  const status = byId("workcell-status");
  status.textContent = humanStatus(workcellState);
  status.className = `status-badge ${statusClass(workcellState)}`;

  byId("workcell-summary").innerHTML = safeList(workcell.metrics, 8)
    .map(
      (metric) => `
        <div class="metric">
          <span class="metric-label">${escapeHtml(boundedText(metric.label, 80))}</span>
          <strong>${escapeHtml(boundedText(metric.value, 80))}</strong>
          <span>${escapeHtml(boundedText(metric.source, 140))}</span>
        </div>
      `
    )
    .join("");

  renderWorkcellItems("workcell-dependencies", workcell.dependencies);
  renderWorkcellItems("workcell-agents", workcell.agents);
  renderWorkcellItems("workcell-authority", workcell.authority);
  refreshRuntimeObservatory(workcell);
}

function renderReviewTail() {
  byId("review-tail-list").innerHTML = milestoneData.reviewTail
    .map(
      (item) => `
        <article class="issue-item">
          <span class="inline-pill ${statusClass(item.state)}">${escapeHtml(humanStatus(item.state))}</span>
          <strong>${escapeHtml(item.label)}</strong>
          <span class="issue-copy">${escapeHtml(item.note)}</span>
        </article>
      `
    )
    .join("");
}

function renderBlockers() {
  byId("release-blockers").innerHTML = milestoneData.releaseBlockers
    .map((item) => `<li>${escapeHtml(item)}</li>`)
    .join("");
}

function renderDeferredFindings() {
  byId("deferred-findings").innerHTML = milestoneData.deferredFindings
    .map(
      (finding) => `
        <article class="issue-item">
          <span class="inline-pill ${statusClass(finding.state)}">${escapeHtml(humanStatus(finding.state))}</span>
          <strong>${escapeHtml(finding.label)}</strong>
          <span class="issue-copy">${escapeHtml(finding.note)}</span>
        </article>
      `
    )
    .join("");
}

function init() {
  renderMeta();
  renderFreshness();
  renderSignals();
  renderMetrics();
  renderWatchlist();
  renderLanes();
  renderFilters();
  renderWorkPackages();
  renderAuthority();
  renderValidationProfiles();
  renderPrChecks();
  renderWorkcellOperator();
  renderReviewTail();
  renderBlockers();
  renderDeferredFindings();
}

init();

window.dashboardInternals = {
  renderWorkcellOperator,
  runtimeConfigFromSearch,
  observationsFromRuntimePayload
};
