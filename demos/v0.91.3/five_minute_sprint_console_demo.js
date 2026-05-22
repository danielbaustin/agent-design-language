const DEMO_PHASES = [
  {
    minute: "00:00",
    label: "Brief accepted",
    summary:
      "The bounded mini-sprint starts by accepting the cozy asteroid-farming concept and locking the proof vocabulary before anyone broadens the scope.",
    focus: "Creative brief and proof contract",
    focusCopy:
      "The issue wave begins with a governed brief, explicit non-claims, and acceptance surfaces that prevent the sprint from turning into a vague success story.",
    activeIssue: "3220",
    artifactState: "Proof contract established",
    artifactSummary:
      "The shared C-SDLC demo proof contract becomes the first artifact, so later demo claims have a reviewable boundary.",
  },
  {
    minute: "00:40",
    label: "Roles assigned",
    summary:
      "Named responsibilities appear: design, mechanics, art direction, implementation, QA review, and packaging all have visible ownership.",
    focus: "Explicit role separation",
    focusCopy:
      "The console keeps the work legible by showing who owns each lane instead of pretending one opaque agent did everything.",
    activeIssue: "3221",
    artifactState: "Creative lanes in motion",
    artifactSummary:
      "The Starharvest build lane starts with artifact and review expectations already visible.",
  },
  {
    minute: "01:30",
    label: "Mechanics and art locked",
    summary:
      "The brief narrows into a concrete gameplay loop, visual thesis, and scope cuts so the implementation lane has something stable to execute.",
    focus: "Intentional design decisions",
    focusCopy:
      "The C-SDLC story gets stronger when rejected scope and chosen style are explicit rather than left to post-hoc interpretation.",
    activeIssue: "3221",
    artifactState: "Design note landed",
    artifactSummary:
      "The design note and implementation summary start to anchor the artifact story in repo-relative surfaces.",
  },
  {
    minute: "02:25",
    label: "Implementation running",
    summary:
      "The HTML/CSS/JS game artifact takes shape while the work board shows which items are planned, running, or already reviewed.",
    focus: "Bounded artifact production",
    focusCopy:
      "This replay keeps the implementation lane visible without claiming hidden automation or a solved general scheduler.",
    activeIssue: "3221",
    artifactState: "Playable artifact forming",
    artifactSummary:
      "The launch deck now points at the real Starharvest file instead of a conceptual placeholder.",
  },
  {
    minute: "03:15",
    label: "Playable artifact opened",
    summary:
      "A runnable browser artifact exists. The launch panel can now preview the actual game while the proof packet catches up around it.",
    focus: "Artifact before rhetoric",
    focusCopy:
      "The sprint console keeps the demo grounded by centering the real artifact first and the narrative around it second.",
    activeIssue: "3221",
    artifactState: "Starharvest playable",
    artifactSummary:
      "A real open-file demo now exists and can be launched directly from the console.",
  },
  {
    minute: "03:55",
    label: "QA review lands",
    summary:
      "The QA checklist and proof report arrive, showing which claims are supported and which ones still need a more constrained framing.",
    focus: "Reviewable proof surface",
    focusCopy:
      "The process becomes more persuasive when review notes, proof language, and residual risks are visible before publication.",
    activeIssue: "3221",
    artifactState: "Packet proving lane active",
    artifactSummary:
      "The artifact is now paired with validation and review surfaces instead of existing as a naked demo file.",
  },
  {
    minute: "04:30",
    label: "Browser capture stays partial",
    summary:
      "One important friction point remains: full browser-capture proof is limited in this environment, so the packet records that honestly as partial.",
    focus: "Truth boundary held",
    focusCopy:
      "The console shows a blocked lane explicitly so the demo never implies that every proof surface succeeded perfectly.",
    activeIssue: "3222",
    artifactState: "Residual risk recorded",
    artifactSummary:
      "The launch artifact remains real even though one capture-oriented proof lane stays partial.",
  },
  {
    minute: "05:00",
    label: "Launch and verdict",
    summary:
      "The sprint finishes with a launch-ready artifact, visible packet, and a bounded verdict: the process is legible and governed, while literal five-minute success remains unproven.",
    focus: "Launch-ready governed demo",
    focusCopy:
      "The console ends by making the C-SDLC value legible without inflating the result into universal productivity proof.",
    activeIssue: "3222",
    artifactState: "Launch-ready Starharvest packet",
    artifactSummary:
      "The sprint console, Starharvest artifact, and proof packet now read as one governed creative-production lane.",
  },
];

const ISSUE_WAVE = [
  {
    id: "3220",
    title: "WP-01 / Proof contract",
    copy: "Define claim vocabulary, timebox truth, result classes, and review minimums before demo implementation broadens.",
    states: ["running", "complete", "complete", "complete", "complete", "complete", "complete", "complete"],
  },
  {
    id: "3221",
    title: "WP-02 / Starharvest build",
    copy: "Produce the visible HTML game artifact with design, implementation, QA, and proof packet surfaces.",
    states: ["planned", "running", "running", "running", "running", "reviewed", "reviewed", "complete"],
  },
  {
    id: "3222",
    title: "WP-03 / Sprint console",
    copy: "Wrap the mini-sprint in a mission-control replay that keeps the process, artifact, and truth boundary visible together.",
    states: ["planned", "planned", "planned", "planned", "planned", "running", "running", "reviewed"],
  },
  {
    id: "3223",
    title: "WP-04 / Demo publication lane",
    copy: "Prepare the later public-facing packet and handoff surfaces once the process console exists.",
    states: ["planned", "planned", "planned", "planned", "planned", "planned", "planned", "planned"],
  },
  {
    id: "3224",
    title: "WP-05 / Final index and closeout",
    copy: "Index the demo wave and close it out truthfully after the child artifacts are ready.",
    states: ["planned", "planned", "planned", "planned", "planned", "planned", "planned", "planned"],
  },
];

const WORK_ITEMS = [
  {
    title: "Accept brief",
    owner: "Conductor",
    copy: "Lock the cozy asteroid-farming brief and bounded non-claims.",
    states: ["complete", "complete", "complete", "complete", "complete", "complete", "complete", "complete"],
  },
  {
    title: "Assign named roles",
    owner: "Creative room",
    copy: "Show who owns design, mechanics, implementation, QA, and packaging lanes.",
    states: ["planned", "complete", "complete", "complete", "complete", "complete", "complete", "complete"],
  },
  {
    title: "Build Starharvest",
    owner: "Frontend implementer",
    copy: "Create the playable browser artifact with controls, loop, and visible style.",
    states: ["planned", "planned", "running", "running", "complete", "complete", "complete", "complete"],
  },
  {
    title: "Review and QA",
    owner: "QA reviewer",
    copy: "Capture bounded findings, validate the packet, and keep residual risks visible.",
    states: ["planned", "planned", "planned", "running", "running", "reviewed", "reviewed", "reviewed"],
  },
  {
    title: "Browser capture proof",
    owner: "Release packager",
    copy: "Attempt screenshot/video proof without overstating what the environment can support.",
    states: ["planned", "planned", "planned", "planned", "planned", "planned", "blocked", "blocked"],
  },
  {
    title: "Launch and package",
    owner: "Release packager",
    copy: "Publish the artifact and pair it with the final proof report and launch path.",
    states: ["planned", "planned", "planned", "planned", "running", "running", "running", "complete"],
  },
];

const ROLES = [
  {
    title: "Game designer",
    owner: "WP-02 lane",
    meta: "brief + scope cuts",
    copy: "Shapes the cozy asteroid-farming concept into a bounded game brief with a visible loop and no inflated product claims.",
  },
  {
    title: "Mechanic designer",
    owner: "WP-02 lane",
    meta: "loop + scoring",
    copy: "Locks the planting, tending, harvesting, and score-target loop so implementation can stay focused.",
  },
  {
    title: "Art director",
    owner: "WP-02 lane",
    meta: "visual thesis",
    copy: "Owns the warm nebula palette, cockpit tone, and readable screenshot energy of the artifact.",
  },
  {
    title: "Frontend implementer",
    owner: "WP-02 lane",
    meta: "html / css / js",
    copy: "Builds the actual open-file browser game and keeps the code surface lightweight and runnable.",
  },
  {
    title: "QA reviewer",
    owner: "review lane",
    meta: "packet + checklist",
    copy: "Records gameplay checks, validation output, and the exact truth boundary around what was and was not proven.",
  },
  {
    title: "Release packager",
    owner: "WP-03 lane",
    meta: "console + launch",
    copy: "Wraps the artifact in this sprint console so the process and final launch stay legible together.",
  },
];

const ARTIFACTS = [
  {
    path: "demos/v0.91.3/starharvest_five_minute_sprint_demo.html",
    title: "Playable Starharvest artifact",
    states: ["planned", "planned", "planned", "running", "complete", "complete", "complete", "complete"],
  },
  {
    path: "docs/milestones/v0.91.3/review/five_minute_html_game/ct_demo_002_starharvest_design_note.md",
    title: "Design note",
    states: ["planned", "running", "complete", "complete", "complete", "complete", "complete", "complete"],
  },
  {
    path: "docs/milestones/v0.91.3/review/five_minute_html_game/ct_demo_002_starharvest_implementation_summary.md",
    title: "Implementation summary",
    states: ["planned", "planned", "running", "complete", "complete", "complete", "complete", "complete"],
  },
  {
    path: "docs/milestones/v0.91.3/review/five_minute_html_game/ct_demo_002_starharvest_qa_checklist.md",
    title: "QA checklist",
    states: ["planned", "planned", "planned", "planned", "running", "reviewed", "reviewed", "reviewed"],
  },
  {
    path: "docs/milestones/v0.91.3/review/five_minute_html_game/ct_demo_002_starharvest_proof_report.md",
    title: "Proof report",
    states: ["planned", "planned", "planned", "planned", "planned", "running", "reviewed", "complete"],
  },
];

const REVIEW_EVENTS = [
  {
    phase: 1,
    kind: "review",
    title: "Proof contract check",
    copy: "Timebox and result-class vocabulary are set before implementation claims broaden.",
  },
  {
    phase: 4,
    kind: "review",
    title: "Playable artifact accepted",
    copy: "The game is real, open-file runnable, and visually intentional enough to serve as the mini-sprint anchor.",
  },
  {
    phase: 5,
    kind: "review",
    title: "Packet verdict tightened",
    copy: "The proof report separates what the demo proves from what it only suggests.",
  },
  {
    phase: 6,
    kind: "block",
    title: "Browser capture remains partial",
    copy: "Full screenshot/video proof is constrained in this environment, so the lane stays explicitly partial rather than silently green.",
  },
  {
    phase: 7,
    kind: "review",
    title: "Launch-ready bounded result",
    copy: "The final surface is publishable as a governed demo, but it does not claim universal five-minute delivery.",
  },
];

const dom = {
  timerReadout: document.getElementById("timer-readout"),
  phaseLabel: document.getElementById("phase-label"),
  phaseSummary: document.getElementById("phase-summary"),
  focusLabel: document.getElementById("focus-label"),
  focusCopy: document.getElementById("focus-copy"),
  progressFill: document.getElementById("progress-fill"),
  issueWave: document.getElementById("issue-wave"),
  workItems: document.getElementById("work-items"),
  roleGrid: document.getElementById("role-grid"),
  artifactList: document.getElementById("artifact-list"),
  reviewEvents: document.getElementById("review-events"),
  artifactState: document.getElementById("artifact-state"),
  artifactSummary: document.getElementById("artifact-summary"),
  playToggle: document.getElementById("play-toggle"),
  restartReplay: document.getElementById("restart-replay"),
};

const replay = {
  phaseIndex: 0,
  autoplay: true,
  intervalMs: 2100,
  timerId: null,
};

function stateClass(state) {
  return `state-${state}`;
}

function stateLabel(state) {
  return state.replace("_", " ");
}

function renderIssueWave() {
  dom.issueWave.innerHTML = "";
  const activeIssue = DEMO_PHASES[replay.phaseIndex].activeIssue;
  ISSUE_WAVE.forEach((item) => {
    const state = item.states[replay.phaseIndex];
    const li = document.createElement("li");
    li.innerHTML = `
      <div class="issue-wave__row">
        <div>
          <strong class="issue-wave__title">${item.title}${item.id === activeIssue ? " / active" : ""}</strong>
          <p class="issue-wave__copy">${item.copy}</p>
        </div>
        <span class="status-pill ${stateClass(state)}">${stateLabel(state)}</span>
      </div>
    `;
    dom.issueWave.appendChild(li);
  });
}

function renderWorkItems() {
  dom.workItems.innerHTML = "";
  WORK_ITEMS.forEach((item) => {
    const state = item.states[replay.phaseIndex];
    const article = document.createElement("article");
    article.className = "work-card";
    article.innerHTML = `
      <div class="work-card__head">
        <div>
          <strong>${item.title}</strong>
          <p>${item.copy}</p>
        </div>
        <span class="status-pill ${stateClass(state)}">${stateLabel(state)}</span>
      </div>
      <p class="issue-wave__copy"><code>${item.owner}</code></p>
    `;
    dom.workItems.appendChild(article);
  });
}

function renderRoles() {
  dom.roleGrid.innerHTML = "";
  ROLES.forEach((role) => {
    const article = document.createElement("article");
    article.className = "role-card";
    article.innerHTML = `
      <div class="role-card__head">
        <div>
          <strong>${role.title}</strong>
          <span class="role-meta">${role.meta}</span>
        </div>
      </div>
      <p>${role.copy}</p>
      <p class="issue-wave__copy"><code>${role.owner}</code></p>
    `;
    dom.roleGrid.appendChild(article);
  });
}

function renderArtifacts() {
  dom.artifactList.innerHTML = "";
  ARTIFACTS.forEach((artifact) => {
    const state = artifact.states[replay.phaseIndex];
    const li = document.createElement("li");
    li.innerHTML = `
      <div class="artifact-row">
        <div>
          <strong>${artifact.title}</strong>
          <p><code>${artifact.path}</code></p>
        </div>
        <span class="artifact-state-pill ${stateClass(state)}">${stateLabel(state)}</span>
      </div>
    `;
    dom.artifactList.appendChild(li);
  });
}

function renderReviewEvents() {
  dom.reviewEvents.innerHTML = "";
  REVIEW_EVENTS.filter((event) => event.phase <= replay.phaseIndex).forEach((event) => {
    const li = document.createElement("li");
    const kindClass = event.kind === "block" ? "event-kind event-kind--block" : "event-kind";
    li.innerHTML = `
      <div class="review-row">
        <div>
          <strong>${event.title}</strong>
          <p>${event.copy}</p>
        </div>
        <span class="${kindClass}">${event.kind}</span>
      </div>
    `;
    dom.reviewEvents.appendChild(li);
  });
}

function renderHeader() {
  const phase = DEMO_PHASES[replay.phaseIndex];
  dom.timerReadout.textContent = phase.minute;
  dom.phaseLabel.textContent = phase.label;
  dom.phaseSummary.textContent = phase.summary;
  dom.focusLabel.textContent = phase.focus;
  dom.focusCopy.textContent = phase.focusCopy;
  dom.artifactState.textContent = phase.artifactState;
  dom.artifactSummary.textContent = phase.artifactSummary;
  dom.progressFill.style.width = `${(replay.phaseIndex / (DEMO_PHASES.length - 1)) * 100}%`;
}

function render() {
  renderHeader();
  renderIssueWave();
  renderWorkItems();
  renderRoles();
  renderArtifacts();
  renderReviewEvents();
}

function advancePhase() {
  replay.phaseIndex = (replay.phaseIndex + 1) % DEMO_PHASES.length;
  render();
}

function stopAutoplay() {
  if (replay.timerId !== null) {
    window.clearInterval(replay.timerId);
    replay.timerId = null;
  }
}

function startAutoplay() {
  stopAutoplay();
  replay.timerId = window.setInterval(advancePhase, replay.intervalMs);
  replay.autoplay = true;
  dom.playToggle.textContent = "Pause replay";
  dom.playToggle.setAttribute("aria-pressed", "true");
}

function pauseAutoplay() {
  stopAutoplay();
  replay.autoplay = false;
  dom.playToggle.textContent = "Resume replay";
  dom.playToggle.setAttribute("aria-pressed", "false");
}

dom.playToggle.addEventListener("click", () => {
  if (replay.autoplay) {
    pauseAutoplay();
  } else {
    startAutoplay();
  }
});

dom.restartReplay.addEventListener("click", () => {
  replay.phaseIndex = 0;
  render();
  startAutoplay();
});

render();
startAutoplay();
