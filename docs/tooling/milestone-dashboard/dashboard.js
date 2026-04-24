const milestoneData = window.milestoneData;

if (!milestoneData) {
  throw new Error("milestone dashboard data is missing; load a milestone data script before dashboard.js");
}

const state = {
  statusFilter: "all",
  laneFilter: "all"
};

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
    unknown: "Unknown / unverified",
    blocked: "Blocked / gated",
    stale: "Stale",
    guarded: "Guarded",
    bounded: "Bounded",
    warn: "Attention",
    good: "Healthy"
  }[status] || status;
}

function statusClass(status) {
  return `status-${status}`;
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
  renderReviewTail();
  renderBlockers();
  renderDeferredFindings();
}

init();
