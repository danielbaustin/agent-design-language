const milestoneData = {
  milestone: "v0.90.3",
  version: "0.90.3",
  owner: "Daniel Austin / Agent Logic",
  updated: "2026-04-21",
  status: "active",
  statusLabel: "Active compression visibility; read-only dashboard",
  summary:
    "v0.90.3 is the citizen-state substrate milestone. This dashboard mirrors milestone-compression state so operators can see issue-wave progress, PR posture, validation risk, review-tail gates, release blockers, deferred findings, and the next safe action without treating the page as release authority.",
  boundary: [
    "Read-only visibility surface; it must not mutate issues, PRs, branches, cards, releases, or closeout state.",
    "Canonical truth remains in milestone docs, GitHub issues/PRs, task cards, validation output, and review records.",
    "Unknown or stale evidence is shown as unknown/stale rather than green."
  ],
  authority: [
    { label: "v0.90.3 WBS", path: "../../milestones/v0.90.3/WBS_v0.90.3.md", note: "21-WP execution map, including the WP-14A demo/proof lane." },
    { label: "v0.90.3 issue wave", path: "../../milestones/v0.90.3/WP_ISSUE_WAVE_v0.90.3.yaml", note: "Canonical WP-to-issue mapping for #2327-#2347." },
    { label: "v0.90.3 checklist", path: "../../milestones/v0.90.3/MILESTONE_CHECKLIST_v0.90.3.md", note: "Current milestone truth markers and release gates." },
    { label: "v0.90.3 release plan", path: "../../milestones/v0.90.3/RELEASE_PLAN_v0.90.3.md", note: "Citizen-state release evidence, non-claims, and handoff boundaries." },
    { label: "v0.90 compression model", path: "../../milestones/v0.90/milestone_compression/README.md", note: "Read-only compression pilot and authority boundary." },
    { label: "finish validation profiles", path: "../../milestones/v0.90/milestone_compression/FINISH_VALIDATION_PROFILES_v0.90.md", note: "Focused validation profile boundaries inherited by v0.90.3." }
  ],
  signals: [
    { label: "WP wave", value: "21", note: "#2327-#2347 open/closed issue set, including WP-14A", tone: "good" },
    { label: "Closed WPs", value: "7", note: "WP-01 through WP-07 are closed", tone: "good" },
    { label: "Current edge", value: "WP-08", note: "Anti-equivocation is next in the citizen-state sequence", tone: "warn" },
    { label: "Release tail", value: "open", note: "WP-15 through WP-20 still require review/closeout", tone: "warn" }
  ],
  nextActions: [
    "Execute WP-08 #2334 anti-equivocation before sanctuary/quarantine widens the state machine.",
    "Keep WP-10, WP-11, and WP-12 blocked on their required standing, projection, and access-control prerequisites.",
    "Preserve the WP-14A demo/proof lane before WP-15 quality and docs convergence.",
    "Preserve later-scope boundaries: no birthday, moral/emotional civilization, full economics, or cloud-enclave dependency claims in v0.90.3."
  ],
  watchlist: [
    { label: "Anti-equivocation", state: "active", note: "WP-08 is the current execution edge; conflicting signed continuity claims are not proven until #2334 lands." },
    { label: "Sanctuary/quarantine", state: "unknown", note: "WP-09 remains open, so ambiguous or unsafe continuity handling is not complete." },
    { label: "Redacted projections and standing", state: "unknown", note: "WP-10 and WP-11 must prove safe Observatory views and citizen/guest standing boundaries." },
    { label: "Review tail", state: "blocked", note: "Internal/external review and remediation are intentionally not green before WP-14A/WP-15." }
  ],
  lanes: [
    {
      id: "lane-setup",
      title: "Setup and state foundations",
      status: "complete",
      wps: ["WP-01", "WP-02", "WP-03", "WP-04", "WP-05", "WP-06", "WP-07"],
      purpose: "Promote the package, audit inherited CSM surfaces, and establish private state, envelopes, keys, lineage, witnesses, and receipts."
    },
    {
      id: "lane-safety",
      title: "Citizen-state safety",
      status: "active",
      wps: ["WP-08", "WP-09", "WP-10", "WP-11", "WP-12", "WP-13"],
      purpose: "Prove anti-equivocation, sanctuary/quarantine, redacted projections, standing, access control, challenge/appeal, threat model, and economics placement."
    },
    {
      id: "lane-demo",
      title: "Demo and proof coverage",
      status: "unknown",
      wps: ["WP-14", "WP-14A"],
      purpose: "Package the integrated citizen-state demo, then prove every feature claim has a runnable demo, fixture-backed artifact, non-proving status, or explicit deferral."
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
    { id: "WP-01", issue: "#2327", title: "Promote v0.90.3 milestone package", queue: "docs", status: "complete", validation: "docs/package", checks: "issue-wave docs", action: "closed", evidence: "Planning docs promoted and issue wave opened." },
    { id: "WP-02", issue: "#2328", title: "Citizen-state inheritance and gap audit", queue: "docs", status: "complete", validation: "docs/audit", checks: "inheritance audit", action: "closed", evidence: "Prior-milestone inheritance and unsafe-assumption audit complete." },
    { id: "WP-03", issue: "#2329", title: "Canonical private state format", queue: "runtime", status: "complete", validation: "schema/fixture", checks: "private-state fixture", action: "closed", evidence: "Private-state format decision and fixture landed." },
    { id: "WP-04", issue: "#2330", title: "Signed envelope and trust root", queue: "runtime", status: "complete", validation: "schema/negative test", checks: "envelope/trust-root", action: "closed", evidence: "Signed envelope and local trust-root fixture landed." },
    { id: "WP-05", issue: "#2331", title: "Local-first key management and sealing", queue: "runtime", status: "complete", validation: "runtime/fixture", checks: "sealed checkpoint path", action: "closed", evidence: "Local key lifecycle and sealed checkpoint boundary landed." },
    { id: "WP-06", issue: "#2332", title: "Append-only lineage ledger", queue: "runtime", status: "complete", validation: "schema/tamper test", checks: "ledger proof", action: "closed", evidence: "Ledger schema, head calculation, and tamper evidence landed." },
    { id: "WP-07", issue: "#2333", title: "Continuity witnesses and receipts", queue: "runtime", status: "complete", validation: "schema/fixture", checks: "witness/receipt proof", action: "closed", evidence: "Witness and receipt fixtures landed." },
    { id: "WP-08", issue: "#2334", title: "Anti-equivocation", queue: "runtime", status: "active", validation: "negative test", checks: "not landed", action: "execute next", evidence: "Open issue; conflict fixture and negative test not landed." },
    { id: "WP-09", issue: "#2335", title: "Sanctuary and quarantine behavior", queue: "runtime", status: "unknown", validation: "runtime", checks: "blocked by WP-08", action: "wait on WP-08", evidence: "Open issue; sanctuary/quarantine proof not landed." },
    { id: "WP-10", issue: "#2336", title: "Redacted Observatory projections", queue: "runtime", status: "unknown", validation: "redaction/runtime", checks: "blocked by WP-03/WP-07", action: "queue after prerequisites", evidence: "Open issue; projection schema and leakage scan not landed." },
    { id: "WP-11", issue: "#2337", title: "Citizen, guest, standing, and communication boundary", queue: "runtime", status: "unknown", validation: "runtime/negative test", checks: "blocked by WP-03", action: "queue after prerequisites", evidence: "Open issue; standing and communication proof not landed." },
    { id: "WP-12", issue: "#2338", title: "Access-control semantics", queue: "runtime", status: "unknown", validation: "authority/denial test", checks: "blocked by WP-10/WP-11", action: "wait on WP-10/WP-11", evidence: "Open issue; access matrix and denial tests not landed." },
    { id: "WP-13", issue: "#2339", title: "Continuity challenge, appeal, threat model, and economics placement", queue: "runtime", status: "unknown", validation: "runtime/docs", checks: "blocked by WP-07/WP-09-WP-12", action: "wait on safety lane", evidence: "Open issue; challenge, appeal, threat model, and economics placement not landed." },
    { id: "WP-14", issue: "#2340", title: "Integrated citizen-state demo", queue: "demo", status: "unknown", validation: "demo", checks: "blocked by WP-03-WP-13", action: "wait on feature lane", evidence: "Open issue; integrated citizen-state demo not landed." },
    { id: "WP-14A", issue: "#2341", title: "Demo matrix and feature proof demos", queue: "demo", status: "unknown", validation: "demo matrix", checks: "blocked by WP-03-WP-14", action: "wait on integrated demo", evidence: "Open issue; feature proof coverage record not landed." },
    { id: "WP-15", issue: "#2342", title: "Quality gate, docs, and review convergence", queue: "docs", status: "blocked", validation: "docs/quality", checks: "blocked by WP-14A", action: "hold", evidence: "Review package cannot converge before feature proof coverage lands." },
    { id: "WP-16", issue: "#2343", title: "Internal review", queue: "review", status: "blocked", validation: "review", checks: "blocked by WP-15", action: "hold", evidence: "Internal review waits for docs/quality convergence." },
    { id: "WP-17", issue: "#2344", title: "External / third-party review", queue: "review", status: "blocked", validation: "review", checks: "blocked by WP-16", action: "hold", evidence: "External review waits for internal review." },
    { id: "WP-18", issue: "#2345", title: "Review findings remediation", queue: "review", status: "blocked", validation: "review", checks: "blocked by WP-16/WP-17", action: "hold", evidence: "No accepted review findings can be closed before reviews occur." },
    { id: "WP-19", issue: "#2346", title: "Next-milestone planning and handoff", queue: "docs", status: "blocked", validation: "docs", checks: "blocked by WP-18", action: "hold", evidence: "Handoff waits for findings disposition." },
    { id: "WP-20", issue: "#2347", title: "Release ceremony", queue: "release", status: "blocked", validation: "release", checks: "blocked by WP-19", action: "hold", evidence: "Release ceremony waits for next-milestone planning." }
  ],
  validationProfiles: [
    { label: "Docs/static dashboard", status: "active", profile: "Focused local validation plus static integrity scan.", command: "bash adl/tools/test_milestone_dashboard.sh" },
    { label: "Runtime/schema/security", status: "blocked", profile: "Fuller validation required when implementation, schema, security, or release gates change.", command: "issue-specific cargo/tests/demo proof" },
    { label: "Release ceremony", status: "blocked", profile: "No focused validation can approve release; human ceremony and review evidence remain required.", command: "WP-20 release closeout only" }
  ],
  releaseBlockers: [
    "Anti-equivocation negative test is still open until WP-08 lands.",
    "Sanctuary/quarantine behavior is still open until WP-09 lands.",
    "Redacted Observatory projection, standing, and access-control proofs remain open until WP-10 through WP-12 land.",
    "Challenge/appeal proof, threat model, and economics placement remain open until WP-13 lands.",
    "Integrated citizen-state demo, demo matrix, internal/external review, remediation, planning handoff, and release ceremony remain open."
  ],
  deferredFindings: [
    { label: "Birthday boundary", state: "guarded", note: "v0.90.3 must not claim the first true Gödel-agent birthday." },
    { label: "Moral/emotional civilization", state: "guarded", note: "v0.91 scope remains later; do not backport into v0.90.3 claims." },
    { label: "Birthday and rebinding", state: "guarded", note: "v0.92 scope remains later; do not present as current proof." },
    { label: "Economics and contract markets", state: "bounded", note: "Full economics belongs to v0.90.4 unless WP-13 records only a narrow resource-stewardship bridge." }
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
