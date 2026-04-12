const milestoneData = {
  milestone: "v0.88",
  version: "0.88",
  owner: "Daniel Austin / Agent Logic",
  updated: "2026-04-11",
  status: "active",
  statusLabel: "Planning package active; issue wave pending",
  summary:
    "v0.88 turns temporal / chronosense and instinct / bounded-agency planning into one coherent public execution package without letting later-band ideas leak into scope.",
  bands: [
    "Temporal / chronosense substrate",
    "Instinct / bounded-agency substrate",
  ],
  nextActions: [
    "Seed the real execution issue wave from WP-02 through WP-20.",
    "Start Sprint 1 with temporal substrate work, not more scope reshaping.",
    "Keep accepted pull-ins bounded to #1614 and #1618.",
    "Preserve the normal v0.86 / v0.87 closeout pattern."
  ],
  watchlist: [
    "Issue wave is still pending even though the planning package is now coherent.",
    "Paper Sonata needs to stay bounded as the flagship demo, not become a catch-all.",
    "Local planning notes must not silently become canonical milestone promises.",
    "Sprint 3 should stay a closeout tail, not a second implementation sprint."
  ],
  sprints: [
    {
      id: "v0.88-s1",
      title: "Sprint 1",
      status: "active",
      purpose: "Lock canonical milestone truth and execute the temporal substrate.",
      wps: ["WP-01", "WP-02", "WP-03", "WP-04", "WP-05", "WP-06", "WP-07", "WP-08"]
    },
    {
      id: "v0.88-s2",
      title: "Sprint 2",
      status: "planned",
      purpose: "Execute PHI metrics, instinct / bounded agency, and the Paper Sonata flagship demo.",
      wps: ["WP-09", "WP-10", "WP-11", "WP-12", "WP-13"]
    },
    {
      id: "v0.88-s3",
      title: "Sprint 3",
      status: "planned",
      purpose: "Converge demos, quality, reviews, remediation, next-milestone planning, and release ceremony.",
      wps: ["WP-14", "WP-15", "WP-16", "WP-17", "WP-18", "WP-19", "WP-20"]
    }
  ],
  docs: [
    { label: "README", path: "../../milestones/v0.88/README.md", note: "Canonical milestone purpose, scope, and issue map." },
    { label: "WBS", path: "../../milestones/v0.88/WBS_v0.88.md", note: "Twenty-work-package execution map with dependencies." },
    { label: "Sprint plan", path: "../../milestones/v0.88/SPRINT_v0.88.md", note: "Three-sprint sequence and exit criteria." },
    { label: "Design", path: "../../milestones/v0.88/DESIGN_v0.88.md", note: "Milestone design boundary and architectural story." },
    { label: "Feature index", path: "../../milestones/v0.88/FEATURE_DOCS_v0.88.md", note: "Promoted feature package and non-promoted local planning boundaries." },
    { label: "Milestone checklist", path: "../../milestones/v0.88/MILESTONE_CHECKLIST_v0.88.md", note: "Closeout gates and release-tail proof expectations." },
    { label: "Release plan", path: "../../milestones/v0.88/RELEASE_PLAN_v0.88.md", note: "Release-tail sequence and package discipline." },
    { label: "Demo matrix", path: "../../milestones/v0.88/DEMO_MATRIX_v0.88.md", note: "Primary proof surfaces and reviewer-facing demos." }
  ],
  issues: [
    { label: "#1527", url: "https://github.com/danielbaustin/agent-design-language/issues/1527", kind: "tracked planning", note: "Initial v0.88 planning shell and milestone scaffolding." },
    { label: "#1579", url: "https://github.com/danielbaustin/agent-design-language/issues/1579", kind: "tracked planning", note: "Promotion of the bounded tracked v0.88 feature-doc package." },
    { label: "#1497", url: "https://github.com/danielbaustin/agent-design-language/issues/1497", kind: "tracked planning", note: "Canonical next-milestone planning reconciliation and scope closure." },
    { label: "#1614", url: "https://github.com/danielbaustin/agent-design-language/issues/1614", kind: "accepted pull-in", note: "Bounded temporal/deadline pressure follow-on." },
    { label: "#1618", url: "https://github.com/danielbaustin/agent-design-language/issues/1618", kind: "accepted pull-in", note: "Bounded comparative-demo / positioning follow-on." }
  ],
  workPackages: [
    { id: "WP-01", title: "Canonical planning package", sprint: "v0.88-s1", status: "complete", issue: "#1527, #1579, #1497", deps: "none", desc: "Reconcile the tracked planning package, promoted feature index, and milestone structure.", deliverable: "Coherent milestone docs + promoted feature set" },
    { id: "WP-02", title: "Chronosense foundation", sprint: "v0.88-s1", status: "pending", issue: "execution issue to be seeded", deps: "WP-01", desc: "Establish the conceptual chronosense substrate.", deliverable: "Runtime-facing chronosense definitions and one bounded proof hook" },
    { id: "WP-03", title: "Temporal schema", sprint: "v0.88-s1", status: "pending", issue: "execution issue to be seeded", deps: "WP-01", desc: "Define temporal anchors, clocks, and execution-policy trace hooks.", deliverable: "Concrete schema fields and targeted tests" },
    { id: "WP-04", title: "Continuity and identity semantics", sprint: "v0.88-s1", status: "pending", issue: "execution issue to be seeded", deps: "WP-02, WP-03", desc: "Ground continuity, interruption, resumption, and identity semantics in temporal structure.", deliverable: "Continuity artifact contract and proof fixture" },
    { id: "WP-05", title: "Temporal query and retrieval", sprint: "v0.88-s1", status: "pending", issue: "execution issue to be seeded", deps: "WP-03", desc: "Make time-aware retrieval and staleness queryable.", deliverable: "Query surface and fixture-backed validation tests" },
    { id: "WP-06", title: "Commitments and deadlines", sprint: "v0.88-s1", status: "pending", issue: "execution issue to be seeded; bounded pull-in #1614", deps: "WP-03, WP-05", desc: "Represent future obligations and missed commitments as first-class temporal records.", deliverable: "Commitment / deadline artifact model and proof fixtures" },
    { id: "WP-07", title: "Temporal causality and explanation", sprint: "v0.88-s1", status: "pending", issue: "execution issue to be seeded", deps: "WP-03, WP-05", desc: "Define bounded causal and explanatory review surfaces.", deliverable: "Explanation artifact format and reviewer-facing examples" },
    { id: "WP-08", title: "Execution policy and cost model", sprint: "v0.88-s1", status: "pending", issue: "execution issue to be seeded", deps: "WP-03", desc: "Tie execution mode and realized cost back to trace reviewability.", deliverable: "Execution-policy contract and cost proof path" },
    { id: "WP-09", title: "PHI-style integration metrics", sprint: "v0.88-s2", status: "planned", issue: "execution issue to be seeded", deps: "WP-02 through WP-08", desc: "Define bounded engineering metrics for integration and adaptive depth.", deliverable: "Metric definitions and reviewable outputs" },
    { id: "WP-10", title: "Instinct model", sprint: "v0.88-s2", status: "planned", issue: "execution issue to be seeded", deps: "WP-01", desc: "Define bounded instinct as an explicit cognitive substrate.", deliverable: "Runtime-facing instinct contract and acceptance tests" },
    { id: "WP-11", title: "Instinct runtime surface and bounded agency hook", sprint: "v0.88-s2", status: "planned", issue: "execution issue to be seeded", deps: "WP-10", desc: "Make instinct visible in runtime declaration, routing, prioritization, trace, and demo proof.", deliverable: "Implementation slice and bounded-agency proof case" },
    { id: "WP-12", title: "Paper Sonata flagship demo", sprint: "v0.88-s2", status: "planned", issue: "execution issue to be seeded", deps: "WP-02 through WP-11", desc: "Build a bounded multi-agent manuscript demo with durable artifacts and truthful runtime proof.", deliverable: "Runner, synthetic packet, artifact tree, and smoke path" },
    { id: "WP-13", title: "Demo matrix + integration demos", sprint: "v0.88-s2", status: "planned", issue: "execution issue to be seeded; supporting pull-in #1618", deps: "WP-02 through WP-12", desc: "Define and implement the primary proof surfaces for temporal, PHI, instinct, and flagship demo bands.", deliverable: "Runnable demos and reviewer-facing matrix" },
    { id: "WP-14", title: "Coverage / quality gate", sprint: "v0.88-s3", status: "closeout", issue: "closeout issue to be seeded", deps: "WP-13", desc: "Enforce milestone quality and coverage posture.", deliverable: "Green quality gate" },
    { id: "WP-15", title: "Docs + review pass", sprint: "v0.88-s3", status: "closeout", issue: "closeout issue to be seeded", deps: "WP-13, WP-14", desc: "Converge reviewer-facing docs against delivered proof.", deliverable: "Reviewer-ready package" },
    { id: "WP-16", title: "Internal review", sprint: "v0.88-s3", status: "closeout", issue: "closeout issue to be seeded", deps: "WP-15", desc: "Perform bounded internal review of milestone truth and proof surfaces.", deliverable: "Internal review record" },
    { id: "WP-17", title: "3rd-party review", sprint: "v0.88-s3", status: "closeout", issue: "closeout issue to be seeded", deps: "WP-15, WP-16", desc: "Perform external review and capture findings.", deliverable: "3rd-party review record" },
    { id: "WP-18", title: "Review findings remediation", sprint: "v0.88-s3", status: "closeout", issue: "closeout issue to be seeded", deps: "WP-16, WP-17", desc: "Resolve or explicitly defer accepted review findings.", deliverable: "Remediation record" },
    { id: "WP-19", title: "Next milestone planning", sprint: "v0.88-s3", status: "closeout", issue: "closeout issue to be seeded", deps: "WP-18", desc: "Prepare the next milestone planning package before v0.88 closeout.", deliverable: "Next-milestone package" },
    { id: "WP-20", title: "Release ceremony", sprint: "v0.88-s3", status: "closeout", issue: "closeout issue to be seeded", deps: "WP-18, WP-19", desc: "Do final validation, notes, tag, cleanup, and closeout record.", deliverable: "Release package" }
  ]
};

const state = {
  statusFilter: "all",
  sprintFilter: "all"
};

function byId(id) {
  return document.getElementById(id);
}

function humanStatus(status) {
  return {
    complete: "Complete",
    active: "Active",
    pending: "Pending issue wave",
    planned: "Planned",
    closeout: "Closeout tail",
    ready: "Ready",
    blocked: "Blocked"
  }[status] || status;
}

function renderMeta() {
  byId("milestone-chip").textContent = `${milestoneData.milestone} / ${milestoneData.version}`;
  byId("headline").textContent = `${milestoneData.milestone} milestone dashboard`;
  byId("lede").textContent = milestoneData.summary;

  const badge = byId("status-badge");
  badge.textContent = milestoneData.statusLabel;
  badge.className = `status-badge status-${milestoneData.status}`;

  const meta = [
    ["Owner", milestoneData.owner],
    ["Updated", milestoneData.updated],
    ["Version", milestoneData.version],
    ["Status", humanStatus(milestoneData.status)]
  ];

  byId("meta-list").innerHTML = meta
    .map(([key, value]) => `<dt>${key}</dt><dd>${value}</dd>`)
    .join("");

  byId("bands-list").innerHTML = milestoneData.bands
    .map((band) => `<li>${band}</li>`)
    .join("");

  byId("next-actions").innerHTML = milestoneData.nextActions
    .map((item) => `<li>${item}</li>`)
    .join("");

  byId("watchlist").innerHTML = milestoneData.watchlist
    .map((item) => `<li>${item}</li>`)
    .join("");
}

function renderMetrics() {
  const total = milestoneData.workPackages.length;
  const complete = milestoneData.workPackages.filter((wp) => wp.status === "complete").length;
  const active = milestoneData.workPackages.filter((wp) => wp.status === "active").length;
  const pending = milestoneData.workPackages.filter((wp) => ["pending", "planned"].includes(wp.status)).length;
  const closeout = milestoneData.workPackages.filter((wp) => wp.status === "closeout").length;
  const percent = Math.round((complete / total) * 100);

  const metrics = [
    ["Completed", complete, "Work packages landed"],
    ["Active", active, "Currently executing"],
    ["Queued", pending, "Need issue-wave work"],
    ["Closeout", closeout, "Review and release tail"]
  ];

  byId("metric-stack").innerHTML = metrics
    .map(
      ([label, value, note]) => `
        <div class="metric">
          <span class="metric-label">${label}</span>
          <strong>${value}</strong>
          <span>${note}</span>
        </div>
      `
    )
    .join("");

  byId("progress-copy").textContent = `${percent}%`;
  requestAnimationFrame(() => {
    byId("progress-bar").style.width = `${percent}%`;
  });
}

function renderSprints() {
  byId("sprint-grid").innerHTML = milestoneData.sprints
    .map((sprint) => {
      const wpCount = milestoneData.workPackages.filter((wp) => wp.sprint === sprint.id).length;
      return `
        <article class="sprint-card">
          <div class="sprint-topline">
            <div>
              <p class="section-kicker">${sprint.id}</p>
              <h4>${sprint.title}</h4>
            </div>
            <span class="status-pill status-${sprint.status}">${humanStatus(sprint.status)}</span>
          </div>
          <p>${sprint.purpose}</p>
          <ul>
            <li>${wpCount} work packages in this sprint</li>
            <li>${sprint.wps.join(", ")}</li>
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
    ["pending", "Queued"],
    ["closeout", "Closeout"]
  ];

  const sprintOptions = [
    ["all", "All sprints"],
    ...milestoneData.sprints.map((sprint) => [sprint.id, sprint.id])
  ];

  const renderChips = (options, activeValue) =>
    options
      .map(([value, label]) => `<button class="chip${value === activeValue ? " active" : ""}" data-value="${value}" type="button">${label}</button>`)
      .join("");

  byId("status-filters").innerHTML = renderChips(statusOptions, state.statusFilter);
  byId("sprint-filters").innerHTML = renderChips(sprintOptions, state.sprintFilter);

  byId("status-filters").querySelectorAll(".chip").forEach((chip) => {
    chip.addEventListener("click", () => {
      state.statusFilter = chip.dataset.value;
      renderFilters();
      renderWorkPackages();
    });
  });

  byId("sprint-filters").querySelectorAll(".chip").forEach((chip) => {
    chip.addEventListener("click", () => {
      state.sprintFilter = chip.dataset.value;
      renderFilters();
      renderWorkPackages();
    });
  });
}

function renderWorkPackages() {
  const filtered = milestoneData.workPackages.filter((wp) => {
    const statusOk =
      state.statusFilter === "all" ||
      (state.statusFilter === "pending" && ["pending", "planned"].includes(wp.status)) ||
      wp.status === state.statusFilter;
    const sprintOk = state.sprintFilter === "all" || wp.sprint === state.sprintFilter;
    return statusOk && sprintOk;
  });

  byId("wp-list").innerHTML = filtered
    .map(
      (wp) => `
        <article class="wp-row">
          <div class="wp-title-block">
            <div class="wp-topline">
              <strong class="wp-title">${wp.id} · ${wp.title}</strong>
            </div>
            <p class="wp-desc">${wp.desc}</p>
            <p class="wp-meta">Deliverable: ${wp.deliverable}</p>
          </div>
          <div><span class="status-pill status-${wp.status}">${humanStatus(wp.status)}</span></div>
          <div>${wp.sprint}</div>
          <div>${wp.deps}</div>
          <div>${wp.issue}</div>
        </article>
      `
    )
    .join("");
}

function renderDocs() {
  byId("docs-list").innerHTML = milestoneData.docs
    .map(
      (doc) => `
        <article class="doc-item">
          <a href="${doc.path}">${doc.label}</a>
          <span class="doc-meta">${doc.note}</span>
        </article>
      `
    )
    .join("");
}

function renderIssues() {
  byId("issues-list").innerHTML = milestoneData.issues
    .map(
      (issue) => `
        <article class="issue-item">
          <a href="${issue.url}">${issue.label}</a>
          <span class="doc-meta">${issue.kind}</span>
          <span class="issue-copy">${issue.note}</span>
        </article>
      `
    )
    .join("");
}

function init() {
  renderMeta();
  renderMetrics();
  renderSprints();
  renderFilters();
  renderWorkPackages();
  renderDocs();
  renderIssues();
}

init();
