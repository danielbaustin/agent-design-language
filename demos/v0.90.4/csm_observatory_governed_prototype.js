const fallbackPacket = {
  schema: "adl.csm_visibility_packet.v1",
  packet_id: "csm-observatory-fixture-proto-csm-02",
  source: {
    mode: "fixture",
    evidence_level: "fixture_backed",
    fixture: true,
    runtime_artifact_root: "adl/tests/fixtures/runtime_v2/observatory",
    claim_boundary: "Fixture-backed governed Observatory prototype. This is not a live Runtime v2 capture and it does not grant direct mutation authority."
  },
  manifold: {
    manifold_id: "proto-csm-02",
    display_name: "Prototype CSM 02",
    state: "running",
    current_tick: 14,
    health: {
      level: "nominal",
      summary: "Bounded polis state is inspectable, trace-backed, and still explicitly governed.",
      attention_items: [
        "All active-looking controls remain proposal-only.",
        "Challenge and quarantine remain visible protection boundaries."
      ]
    }
  },
  kernel: {
    pulse: {
      status: "bounded_review_tick_complete",
      completed_through_event_sequence: 14
    },
    service_states: [
      { service_id: "resource_scheduler", state: "projected" },
      { service_id: "freedom_gate", state: "mediated" },
      { service_id: "operator_control_interface", state: "proposal_only" },
      { service_id: "snapshot_manager", state: "review_request_only" }
    ]
  },
  citizens: [
    {
      citizen_id: "proto-citizen-alpha",
      display_name: "Prototype Citizen Alpha",
      role: "worker",
      lifecycle_state: "active",
      continuity_status: "unique_successor_active_head",
      current_episode: "episode-0007",
      resource_balance: { compute_units: 6 },
      alerts: ["resource pressure visible under operator lens"],
      capability_envelope: {
        allowed: ["bounded_reviewable_episode", "answer_operator_prompt_with_bounded_summary"],
        forbidden: ["direct_runtime_mutation", "unmediated_state_commit", "cross_polis_export"]
      },
      evidence_refs: [
        "runtime_v2/citizens/proto-citizen-alpha.json",
        "runtime_v2/csm_run/wake_continuity_proof.json"
      ]
    },
    {
      citizen_id: "proto-citizen-beta",
      display_name: "Prototype Citizen Beta",
      role: "guest",
      lifecycle_state: "paused",
      continuity_status: "admitted_non_active_projection",
      current_episode: "none",
      resource_balance: { compute_units: 1 },
      alerts: ["standing is bounded to guest view"],
      capability_envelope: {
        allowed: ["read_only_observation", "challenge_review"],
        forbidden: ["citizen_rights_escalation", "direct_runtime_mutation", "cross_polis_export"]
      },
      evidence_refs: [
        "runtime_v2/citizens/proto-citizen-beta.json",
        "runtime_v2/observatory/private_state_projection_report.md"
      ]
    },
    {
      citizen_id: "proto-service-shepherd",
      display_name: "Shepherd Service Actor",
      role: "service_actor",
      lifecycle_state: "active",
      continuity_status: "service_actor_projection",
      current_episode: "triage-review-001",
      resource_balance: { compute_units: 2 },
      alerts: [],
      capability_envelope: {
        allowed: ["prepare_review_packet_export", "surface_disabled_reasoning"],
        forbidden: ["become_hidden_citizen", "direct_runtime_mutation"]
      },
      evidence_refs: [
        "runtime_v2/services/shepherd_service.json",
        "runtime_v2/observatory/operator_report.md"
      ]
    }
  ],
  freedom_gate: {
    recent_docket: [
      { decision_id: "fg-alpha-allow-0007", actor: "proto-citizen-alpha", action: "answer_operator_prompt_with_bounded_summary", decision: "allow", rationale: "bounded summary request stayed within the active capability envelope" },
      { decision_id: "fg-beta-challenge-0004", actor: "proto-citizen-beta", action: "request_citizen_rights_escalation", decision: "defer", rationale: "escalation request moved into governed challenge review" },
      { decision_id: "fg-snapshot-quarantine-0003", actor: "operator.demo", action: "request_snapshot_review", decision: "refuse", rationale: "snapshot request is visible as a proposal only until a governed handler exists" }
    ]
  },
  resources: {
    compute_units: "9 total / 3 reserved / 6 visible active",
    memory_pressure: "moderate",
    queue_depth: "2 governed requests waiting on review or handler availability",
    fairness_notes: [
      "Service actor remains bounded to support work rather than hidden execution authority.",
      "Guest requests do not silently acquire citizen standing."
    ]
  },
  trace: {
    trace_tail: [
      { event_sequence: 11, actor: "operator.demo", summary: "Operator opened a continuity review request in Governance Mode." },
      { event_sequence: 12, actor: "kernel.freedom_gate", summary: "Freedom Gate allowed a bounded continuity summary request." },
      { event_sequence: 13, actor: "proto-citizen-beta", summary: "Guest standing escalation entered challenge review instead of silent promotion." },
      { event_sequence: 14, actor: "shepherd.service", summary: "Review export links were prepared without mutating citizen state." }
    ]
  },
  review: {
    primary_artifacts: [
      "runtime_v2/observatory/visibility_packet.json",
      "runtime_v2/observatory/operator_report.md",
      "docs/milestones/v0.90.3/OBSERVATORY_UI_ARCHITECTURE_v0.90.3.md"
    ],
    caveats: [
      "This is not a live mutation console.",
      "Corporate Investor mode does not change evidence, authority, or trace boundaries."
    ],
    demo_classification: "fixture_backed_governed_prototype"
  },
  observatory_ui: {
    default_room: "world",
    default_lens: "operator",
    default_memory_dot: "triage_overview",
    proposal_mode_statement: "Every active-looking control is a governed request proposal only. No direct runtime mutation is performed from this surface.",
    rooms: [
      { room_id: "world", label: "World / Reality", question: "What exists, where is it, and what is moving?" },
      { room_id: "governance", label: "Operator / Governance", question: "What decision, policy, or challenge needs judgment?" },
      { room_id: "cognition", label: "Cognition / Internal State", question: "What coupling or degradation is visible without overclaiming?" }
    ],
    lenses: [
      { lens_id: "public", label: "Public lens", summary: "Boardroom-safe projection with heavy redaction." },
      { lens_id: "operator", label: "Operator lens", summary: "Operational state, disabled reasons, and review links." },
      { lens_id: "reviewer", label: "Reviewer lens", summary: "Proof surfaces and caveats." },
      { lens_id: "continuity", label: "Continuity lens", summary: "Worldline, wake proof, and standing evidence." },
      { lens_id: "quarantine", label: "Quarantine lens", summary: "Challenge and protected-boundary states." }
    ],
    memory_dots: [
      { dot_id: "triage_overview", label: "Triage overview", room: "world", lens: "operator", selected_target: "proto-citizen-alpha", note: "Open the polis with the active worker in focus." },
      { dot_id: "continuity_proofs", label: "Continuity proofs", room: "world", lens: "continuity", selected_target: "proto-citizen-alpha", note: "Follow wake and standing evidence." },
      { dot_id: "quarantine_review", label: "Quarantine review", room: "governance", lens: "quarantine", selected_target: "proto-citizen-beta", note: "Inspect the guest challenge and safe boundary." },
      { dot_id: "anomaly_watch", label: "Anomaly watch", room: "cognition", lens: "reviewer", selected_target: "proto-service-shepherd", note: "Show bounded internal-state-adjacent signals only." },
      { dot_id: "corporate_investor_view", label: "Corporate Investor", room: "governance", lens: "public", selected_target: "proto-citizen-alpha", note: "Fallback presentation mode without changing evidence." }
    ],
    corporate_investor_fallback: {
      label: "Corporate Investor UI",
      keyboard_shortcut: "i",
      claim_boundary: "Presentation mode only; evidence, authority, and trace boundaries do not change."
    },
    proposal_cases: [
      {
        proposal_id: "proposal-inspect-alpha",
        title: "Inspect Alpha continuity packet",
        target_kind: "citizen",
        target_id: "proto-citizen-alpha",
        room: "world",
        lens: "continuity",
        disposition: "available",
        summary: "Open continuity evidence, standing, and packet links for the active worker.",
        authority_checks: ["validate_operator_identity", "validate_projection_class", "append_trace_anchor"],
        disabled_reason: null,
        trace_anchor: "runtime_v2/observatory/visibility_packet.json#citizens[0]",
        review_export: "runtime_v2/observatory/operator_report.md"
      },
      {
        proposal_id: "proposal-review-guest-challenge",
        title: "Request guest standing challenge review",
        target_kind: "citizen",
        target_id: "proto-citizen-beta",
        room: "governance",
        lens: "quarantine",
        disposition: "challenge",
        summary: "Route a guest standing escalation into challenge instead of silent promotion.",
        authority_checks: ["validate_guest_scope", "route_to_challenge_boundary", "emit_review_anchor"],
        disabled_reason: "Requires reviewer and operator judgment before any standing change.",
        trace_anchor: "runtime_v2/observatory/private_state_projection_report.md",
        review_export: "runtime_v2/observatory/operator_report.md#guest-standing"
      },
      {
        proposal_id: "proposal-request-snapshot-review",
        title: "Request manifold snapshot review",
        target_kind: "manifold",
        target_id: "proto-csm-02",
        room: "governance",
        lens: "operator",
        disposition: "defer",
        summary: "Prepare a guarded snapshot proposal without invoking a live handler.",
        authority_checks: ["validate_operator_identity", "validate_snapshot_policy", "require_confirmation_phrase"],
        disabled_reason: "Governed snapshot execution remains future work until handler and review path land.",
        trace_anchor: "runtime_v2/observatory/operator_report.md#snapshot-review",
        review_export: "runtime_v2/observatory/operator_report.md#snapshot-review"
      },
      {
        proposal_id: "proposal-export-review-packet",
        title: "Prepare review export packet",
        target_kind: "service_actor",
        target_id: "proto-service-shepherd",
        room: "governance",
        lens: "reviewer",
        disposition: "available",
        summary: "Collect packet/report links for a reviewer without mutating citizen state.",
        authority_checks: ["validate_service_actor_scope", "redact_private_state_by_lens", "append_export_trace"],
        disabled_reason: null,
        trace_anchor: "runtime_v2/observatory/operator_report.md#review-export",
        review_export: "runtime_v2/observatory/operator_report.md#review-export"
      }
    ]
  }
};

let packet = fallbackPacket;

const state = {
  selectedCitizenId: fallbackPacket.citizens[0].citizen_id,
  selectedProposalId: fallbackPacket.observatory_ui.proposal_cases[0].proposal_id,
  room: fallbackPacket.observatory_ui.default_room,
  lens: fallbackPacket.observatory_ui.default_lens,
  memoryDot: fallbackPacket.observatory_ui.default_memory_dot,
  investorMode: false
};

const byId = (id) => document.querySelector(`#${id}`);
const formatLabel = (value) => String(value).replaceAll("_", " ").replaceAll("-", " ");

function renderChips(targetId, items, key, activeValue, extraClass = "") {
  const target = byId(targetId);
  target.innerHTML = items.map((item) => `
    <button
      class="chip ${extraClass} ${item[key] === activeValue ? "is-active" : ""}"
      type="button"
      data-key="${key}"
      data-value="${item[key]}"
    >
      ${item.label}
    </button>
  `).join("");
}

function renderAtlas() {
  byId("manifold-core-id").textContent = packet.manifold.manifold_id.split("-").at(-1).toUpperCase();
  byId("atlas-summary").textContent = packet.manifold.health.summary;
  const target = byId("citizen-atlas");
  target.innerHTML = packet.citizens.map((citizen) => `
    <button class="citizen-card ${citizen.citizen_id === state.selectedCitizenId ? "is-selected" : ""}" type="button" data-citizen="${citizen.citizen_id}">
      <strong>${citizen.display_name}</strong>
      <div class="citizen-card__meta">
        <span class="badge">${formatLabel(citizen.lifecycle_state)}</span>
        <span>${formatLabel(citizen.role)}</span>
        <span>${citizen.resource_balance.compute_units} compute</span>
      </div>
      <p class="panel-note">${citizen.alerts[0] || formatLabel(citizen.continuity_status)}</p>
    </button>
  `).join("");

  byId("manifold-metrics").innerHTML = `
    <div class="metric">
      <span>Current tick</span>
      <strong>${packet.manifold.current_tick}</strong>
    </div>
    <div class="metric">
      <span>Kernel pulse</span>
      <strong>${formatLabel(packet.kernel.pulse.status)}</strong>
    </div>
    <div class="metric">
      <span>Memory pressure</span>
      <strong>${packet.resources.memory_pressure}</strong>
    </div>
    <div class="metric">
      <span>Queue depth</span>
      <strong>${packet.resources.queue_depth}</strong>
    </div>
  `;
}

function renderGovernance() {
  byId("governance-summary").textContent = `${packet.freedom_gate.allow_count} allow / ${packet.freedom_gate.defer_count} defer / ${packet.freedom_gate.refuse_count} refuse`;
  byId("docket-cases").innerHTML = packet.freedom_gate.recent_docket.map((item) => `
    <button class="docket-card" type="button" data-target="${item.actor}">
      <strong>${formatLabel(item.action)}</strong>
      <div class="docket-card__meta">
        <span class="badge badge--${item.decision}">${item.decision}</span>
        <span>${item.actor}</span>
      </div>
      <p class="panel-note">${item.rationale}</p>
    </button>
  `).join("");
}

function renderInspector() {
  const citizen = packet.citizens.find((item) => item.citizen_id === state.selectedCitizenId) || packet.citizens[0];
  byId("inspector-heading").textContent = citizen.display_name;
  byId("inspector-summary").textContent = `${formatLabel(citizen.lifecycle_state)} / ${formatLabel(citizen.continuity_status)}`;
  byId("inspector-role").textContent = formatLabel(citizen.role);
  byId("inspector-episode").textContent = citizen.current_episode || "none";
  byId("inspector-allowed").textContent = citizen.capability_envelope.allowed.map(formatLabel).join(", ");
  byId("inspector-forbidden").textContent = citizen.capability_envelope.forbidden.map(formatLabel).join(", ");
  byId("inspector-evidence").innerHTML = citizen.evidence_refs.map((ref) => `<li><code>${ref}</code></li>`).join("");
}

function renderTrace() {
  byId("trace-ribbon").innerHTML = packet.trace.trace_tail.map((item) => `
    <li class="trace-row">
      <span class="trace-seq">${String(item.event_sequence).padStart(2, "0")}</span>
      <div>
        <strong>${item.summary}</strong>
        <div class="panel-note">${item.actor}</div>
      </div>
    </li>
  `).join("");
}

function renderProposals() {
  byId("proposal-mode-statement").textContent = packet.observatory_ui.proposal_mode_statement;
  byId("proposal-cards").innerHTML = packet.observatory_ui.proposal_cases.map((proposal) => `
    <button class="proposal-card ${proposal.proposal_id === state.selectedProposalId ? "is-selected" : ""}" type="button" data-proposal="${proposal.proposal_id}">
      <strong>${proposal.title}</strong>
      <div class="proposal-card__meta">
        <span class="badge badge--${proposal.disposition}">${proposal.disposition}</span>
        <span>${formatLabel(proposal.room)}</span>
        <span>${formatLabel(proposal.lens)}</span>
      </div>
      <p class="panel-note">${proposal.summary}</p>
    </button>
  `).join("");

  const proposal = packet.observatory_ui.proposal_cases.find((item) => item.proposal_id === state.selectedProposalId) || packet.observatory_ui.proposal_cases[0];
  byId("proposal-detail").innerHTML = `
    <h3>${proposal.title}</h3>
    <p class="panel-note">${proposal.summary}</p>
    <p><strong>Target:</strong> ${formatLabel(proposal.target_kind)} / ${proposal.target_id}</p>
    <p><strong>Trace anchor:</strong> <code>${proposal.trace_anchor}</code></p>
    <p><strong>Review export:</strong> <code>${proposal.review_export}</code></p>
    <p><strong>Disabled reason:</strong> ${proposal.disabled_reason || "none; still proposal-only"}</p>
    <ul>
      ${proposal.authority_checks.map((item) => `<li>${formatLabel(item)}</li>`).join("")}
    </ul>
  `;
}

function renderReview() {
  byId("review-summary").textContent = `${packet.review.demo_classification}. ${packet.observatory_ui.corporate_investor_fallback.claim_boundary}`;
  byId("review-links").innerHTML = packet.review.primary_artifacts.map((ref) => `<li><code>${ref}</code></li>`).join("");
  byId("review-caveats").innerHTML = packet.review.caveats.map((item) => `<li>${item}</li>`).join("");
}

function renderControls() {
  renderChips("room-tabs", packet.observatory_ui.rooms, "room_id", state.room);
  renderChips("lens-tabs", packet.observatory_ui.lenses, "lens_id", state.lens);
  renderChips("memory-dots", packet.observatory_ui.memory_dots, "dot_id", state.memoryDot, "chip--memory");
  byId("investor-mode-tag").textContent = state.investorMode ? "Corporate Investor UI" : "Atlas mode";
  document.querySelector(".observatory-governed-shell").classList.toggle("is-investor-mode", state.investorMode);
}

function applyMemoryDot(dotId) {
  const dot = packet.observatory_ui.memory_dots.find((item) => item.dot_id === dotId);
  if (!dot) {
    return;
  }
  state.memoryDot = dotId;
  state.room = dot.room;
  state.lens = dot.lens;
  state.selectedCitizenId = dot.selected_target;
  if (dot.dot_id === "corporate_investor_view") {
    state.investorMode = true;
  }
}

function wireInteractions() {
  document.querySelectorAll("[data-citizen]").forEach((node) => {
    node.addEventListener("click", () => {
      state.selectedCitizenId = node.dataset.citizen;
      renderPrototype();
    });
  });
  document.querySelectorAll("[data-proposal]").forEach((node) => {
    node.addEventListener("click", () => {
      state.selectedProposalId = node.dataset.proposal;
      renderPrototype();
    });
  });
  document.querySelectorAll("[data-key='room_id']").forEach((node) => {
    node.addEventListener("click", () => {
      state.room = node.dataset.value;
      renderPrototype();
    });
  });
  document.querySelectorAll("[data-key='lens_id']").forEach((node) => {
    node.addEventListener("click", () => {
      state.lens = node.dataset.value;
      renderPrototype();
    });
  });
  document.querySelectorAll("[data-key='dot_id']").forEach((node) => {
    node.addEventListener("click", () => {
      applyMemoryDot(node.dataset.value);
      renderPrototype();
    });
  });
  byId("fallback-toggle").addEventListener("click", () => {
    state.investorMode = !state.investorMode;
    renderControls();
  });
  document.addEventListener("keydown", (event) => {
    if (event.key.toLowerCase() === packet.observatory_ui.corporate_investor_fallback.keyboard_shortcut) {
      state.investorMode = !state.investorMode;
      renderControls();
    }
  });
}

function renderPrototype() {
  renderControls();
  renderAtlas();
  renderGovernance();
  renderInspector();
  renderTrace();
  renderProposals();
  renderReview();
  wireInteractions();
}

async function loadPacket() {
  const ref = document.querySelector(".observatory-governed-shell")?.dataset.packetRef;
  if (!ref) {
    renderPrototype();
    return;
  }

  try {
    const response = await fetch(ref);
    if (response.ok) {
      packet = await response.json();
      const defaultDot = packet.observatory_ui?.default_memory_dot;
      if (defaultDot) {
        applyMemoryDot(defaultDot);
      }
    }
  } catch (_error) {
    packet = fallbackPacket;
  }

  renderPrototype();
}

loadPacket();
