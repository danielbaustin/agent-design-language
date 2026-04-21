const milestoneData = {
  milestone: "v0.90.2",
  version: "0.90.2",
  owner: "Daniel Austin / Agent Logic",
  updated: "2026-04-21",
  status: "active",
  statusLabel: "Active compression visibility; read-only dashboard",
  summary:
    "v0.90.2 is the first bounded CSM run milestone. This dashboard mirrors milestone-compression state so operators can see issue-wave progress, PR posture, validation risk, review-tail gates, release blockers, deferred findings, and the next safe action without treating the page as release authority.",
  boundary: [
    "Read-only visibility surface; it must not mutate issues, PRs, branches, cards, releases, or closeout state.",
    "Canonical truth remains in milestone docs, GitHub issues/PRs, task cards, validation output, and review records.",
    "Unknown or stale evidence is shown as unknown/stale rather than green."
  ],
  authority: [
    { label: "v0.90.2 WBS", path: "../../milestones/v0.90.2/WBS_v0.90.2.md", note: "20-WP execution map and dependencies." },
    { label: "v0.90.2 issue wave", path: "../../milestones/v0.90.2/WP_ISSUE_WAVE_v0.90.2.yaml", note: "Canonical WP-to-issue mapping for #2245-#2264." },
    { label: "v0.90.2 checklist", path: "../../milestones/v0.90.2/MILESTONE_CHECKLIST_v0.90.2.md", note: "Current milestone truth markers and release gates." },
    { label: "v0.90.2 release plan", path: "../../milestones/v0.90.2/RELEASE_PLAN_v0.90.2.md", note: "Release blockers, evidence requirements, and closeout order." },
    { label: "v0.90 compression model", path: "../../milestones/v0.90/milestone_compression/README.md", note: "Read-only compression pilot and authority boundary." },
    { label: "finish validation profiles", path: "../../milestones/v0.90/milestone_compression/FINISH_VALIDATION_PROFILES_v0.90.md", note: "Focused validation profile boundaries inherited by v0.90.2." }
  ],
  signals: [
    { label: "WP wave", value: "20", note: "#2245-#2264 open/closed issue set", tone: "good" },
    { label: "Closed WPs", value: "9", note: "WP-01 through WP-09 are closed", tone: "good" },
    { label: "Active PR", value: "#2296", note: "WP-10 draft PR checks in progress", tone: "warn" },
    { label: "Release tail", value: "open", note: "WP-15 through WP-20 still require review/closeout", tone: "warn" }
  ],
  nextActions: [
    "Watch #2296 for WP-10 check completion and review state.",
    "Continue WP-11 recovery eligibility only after the WP-10 visibility packet is safely reviewed or explicitly unblocked.",
    "Keep review-tail work WP-15 through WP-20 visibly gated until the integrated first CSM run demo lands.",
    "Preserve v0.91/v0.92 boundaries: no birthday, moral/emotional civilization, or capability-rebinding claims in v0.90.2."
  ],
  watchlist: [
    { label: "Observatory packet", state: "active", note: "WP-10 is in draft PR; dashboard must not mark Observatory evidence green until review lands." },
    { label: "Recovery/quarantine", state: "unknown", note: "WP-11 and WP-12 are still open, so recovery and quarantine proof is not complete." },
    { label: "Adversarial hook", state: "unknown", note: "WP-13 remains bounded future work, not a full security ecology." },
    { label: "Review tail", state: "blocked", note: "Internal/external review and remediation are intentionally not green before WP-14/WP-15." }
  ],
  lanes: [
    {
      id: "lane-setup",
      title: "Setup and inherited truth",
      status: "complete",
      wps: ["WP-01", "WP-02", "WP-03", "WP-04"],
      purpose: "Promote the package, audit inherited Runtime v2/CSM surfaces, and define first-run contracts."
    },
    {
      id: "lane-first-run",
      title: "First bounded CSM run",
      status: "active",
      wps: ["WP-05", "WP-06", "WP-07", "WP-08", "WP-09", "WP-10"],
      purpose: "Boot citizens, run a governed episode, mediate actions, reject invalid actions, prove continuity, and expose Observatory evidence."
    },
    {
      id: "lane-hardening",
      title: "Recovery and hardening",
      status: "unknown",
      wps: ["WP-11", "WP-12", "WP-13", "WP-14"],
      purpose: "Distinguish recovery from quarantine, add bounded hardening probes, and package the integrated CSM run demo."
    },
    {
      id: "lane-release-tail",
      title: "Review and release tail",
      status: "blocked",
      wps: ["WP-15", "WP-16", "WP-17", "WP-18", "WP-19", "WP-20"],
      purpose: "Converge docs, complete internal/external review, remediate findings, plan next milestone, and run ceremony."
    }
  ],
  workPackages: [
    { id: "WP-01", issue: "#2245", title: "Milestone package and issue wave", queue: "docs", status: "complete", validation: "docs/package", checks: "issue-wave docs", action: "closed", evidence: "Planning docs promoted and issue wave opened." },
    { id: "WP-02", issue: "#2246", title: "Runtime v2 inheritance and compression audit", queue: "docs", status: "complete", validation: "docs/compression", checks: "inheritance audit", action: "closed", evidence: "Inheritance and compression report landed." },
    { id: "WP-03", issue: "#2247", title: "CSM run packet contract", queue: "runtime", status: "complete", validation: "contract", checks: "fixture contract", action: "closed", evidence: "First-run packet contract and fixture landed." },
    { id: "WP-04", issue: "#2248", title: "Invariant and violation artifact contract", queue: "runtime", status: "complete", validation: "schema/fixture", checks: "artifact contract", action: "closed", evidence: "Invariant map and violation artifact proof landed." },
    { id: "WP-05", issue: "#2249", title: "Manifold boot and citizen admission", queue: "runtime", status: "complete", validation: "runtime", checks: "boot/admission", action: "closed", evidence: "Boot and citizen admission artifacts landed." },
    { id: "WP-06", issue: "#2250", title: "Governed episode and resource scheduling", queue: "runtime", status: "complete", validation: "runtime", checks: "scheduler proof", action: "closed", evidence: "Governed episode and scheduler explanation landed." },
    { id: "WP-07", issue: "#2251", title: "Freedom Gate mediation", queue: "runtime", status: "complete", validation: "runtime", checks: "decision artifact", action: "closed", evidence: "Non-trivial citizen action routes through Freedom Gate." },
    { id: "WP-08", issue: "#2252", title: "Invalid action rejection", queue: "runtime", status: "complete", validation: "negative test", checks: "violation packet", action: "closed", evidence: "Invalid action rejection proof landed." },
    { id: "WP-09", issue: "#2253", title: "Snapshot rehydrate and wake continuity", queue: "runtime", status: "complete", validation: "runtime", checks: "continuity proof", action: "closed", evidence: "Snapshot/rehydrate/wake proof landed." },
    { id: "WP-10", issue: "#2254 / PR #2296", title: "Observatory packet and operator report integration", queue: "runtime", status: "active", validation: "runtime/docs", checks: "CI in progress", action: "review #2296", evidence: "Draft PR open; do not mark green until merged." },
    { id: "WP-11", issue: "#2255", title: "Recovery eligibility model", queue: "runtime", status: "unknown", validation: "runtime", checks: "not started/unknown", action: "queue after WP-10", evidence: "Open issue; recovery proof not landed." },
    { id: "WP-12", issue: "#2256", title: "Quarantine state machine", queue: "runtime", status: "unknown", validation: "runtime", checks: "not started/unknown", action: "wait on WP-11", evidence: "Open issue; quarantine proof not landed." },
    { id: "WP-13", issue: "#2257", title: "Governed adversarial hook and hardening probes", queue: "runtime", status: "unknown", validation: "security/runtime", checks: "not started/unknown", action: "wait on WP-11/WP-12", evidence: "Open issue; bounded hardening probes not landed." },
    { id: "WP-14", issue: "#2258", title: "Integrated first CSM run demo", queue: "demo", status: "unknown", validation: "demo", checks: "not started/unknown", action: "wait on WP-05-WP-13", evidence: "Open issue; integrated demo not landed." },
    { id: "WP-15", issue: "#2259", title: "Docs, quality, and review convergence", queue: "docs", status: "blocked", validation: "docs/quality", checks: "blocked by WP-14", action: "hold", evidence: "Review package cannot converge before integrated demo." },
    { id: "WP-16", issue: "#2260", title: "Internal review", queue: "review", status: "blocked", validation: "review", checks: "blocked by WP-15", action: "hold", evidence: "Internal review waits for docs/quality convergence." },
    { id: "WP-17", issue: "#2261", title: "External / 3rd-party review", queue: "review", status: "blocked", validation: "review", checks: "blocked by WP-16", action: "hold", evidence: "External review waits for internal review." },
    { id: "WP-18", issue: "#2262", title: "Review findings remediation", queue: "review", status: "blocked", validation: "review", checks: "blocked by WP-16/WP-17", action: "hold", evidence: "No accepted review findings can be closed before reviews occur." },
    { id: "WP-19", issue: "#2263", title: "Next milestone planning and v0.91/v0.92 handoff", queue: "docs", status: "blocked", validation: "docs", checks: "blocked by WP-18", action: "hold", evidence: "Handoff waits for findings disposition." },
    { id: "WP-20", issue: "#2264", title: "Release ceremony", queue: "release", status: "blocked", validation: "release", checks: "blocked by WP-19", action: "hold", evidence: "Release ceremony waits for next-milestone planning." }
  ],
  validationProfiles: [
    { label: "Docs/static dashboard", status: "active", profile: "Focused local validation plus static integrity scan.", command: "bash adl/tools/test_milestone_dashboard.sh" },
    { label: "Runtime/schema/security", status: "blocked", profile: "Fuller validation required when implementation, schema, security, or release gates change.", command: "issue-specific cargo/tests/demo proof" },
    { label: "Release ceremony", status: "blocked", profile: "No focused validation can approve release; human ceremony and review evidence remain required.", command: "WP-20 release closeout only" }
  ],
  releaseBlockers: [
    "Missing Observatory-visible run evidence until WP-10 merges.",
    "Recovery/quarantine distinction is still open until WP-11/WP-12 land.",
    "Governed adversarial hook and hardening probes are still open until WP-13 lands.",
    "Integrated first CSM run demo is still open until WP-14 lands.",
    "Internal/external review, remediation, next-milestone planning, and release ceremony remain open."
  ],
  deferredFindings: [
    { label: "Birthday boundary", state: "guarded", note: "v0.90.2 must not claim the first true Gödel-agent birthday." },
    { label: "Moral/emotional civilization", state: "guarded", note: "v0.91 scope remains later; do not backport into v0.90.2 claims." },
    { label: "Capability rebinding / richer memory", state: "guarded", note: "v0.92 scope remains later; do not present as current proof." },
    { label: "Security ecology", state: "bounded", note: "WP-13 is one governed adversarial hook plus probes, not a complete red/blue ecology." }
  ]
};

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
    unknown: "Unknown / not landed",
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
  renderSignals();
  renderMetrics();
  renderWatchlist();
  renderLanes();
  renderFilters();
  renderWorkPackages();
  renderAuthority();
  renderValidationProfiles();
  renderBlockers();
  renderDeferredFindings();
}

init();
