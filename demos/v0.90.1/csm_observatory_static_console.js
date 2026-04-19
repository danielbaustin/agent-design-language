const fallbackPacket = {
  schema: "adl.csm_visibility_packet.v1",
  packet_id: "csm-observatory-fixture-proto-csm-01",
  source: {
    mode: "fixture",
    evidence_level: "fixture_backed",
    claim_boundary: "Fixture-backed Observatory contract example. This is not a live Runtime v2 capture."
  },
  manifold: {
    manifold_id: "proto-csm-01",
    display_name: "Prototype CSM 01",
    state: "initialized",
    current_tick: 0
  },
  kernel: {
    pulse: {
      status: "bounded_tick_complete",
      completed_through_event_sequence: 8
    },
    service_states: [
      { service_id: "clock_service", service_kind: "clock", lifecycle_state: "ready", last_event_sequence: 1 },
      { service_id: "identity_admission_guard", service_kind: "admission", lifecycle_state: "ready", last_event_sequence: 2 },
      { service_id: "scheduler", service_kind: "scheduler", lifecycle_state: "ready", last_event_sequence: 3 },
      { service_id: "resource_ledger", service_kind: "resource", lifecycle_state: "ready", last_event_sequence: 4 },
      { service_id: "trace_writer", service_kind: "trace", lifecycle_state: "ready", last_event_sequence: 5 },
      { service_id: "snapshot_manager", service_kind: "snapshot", lifecycle_state: "ready", last_event_sequence: 6 },
      { service_id: "invariant_checker", service_kind: "invariant", lifecycle_state: "ready", last_event_sequence: 7 },
      { service_id: "operator_control_interface", service_kind: "operator", lifecycle_state: "ready", last_event_sequence: 8 }
    ]
  },
  citizens: [
    {
      citizen_id: "proto-citizen-alpha",
      display_name: "Prototype Citizen Alpha",
      role: "worker",
      lifecycle_state: "active",
      continuity_status: "provisional_identity_continuity",
      current_episode: "episode-resource-pressure-001",
      resource_balance: { compute_units: 6 },
      capability_envelope: {
        allowed: ["bounded_task_work", "status_report"],
        forbidden: ["direct_runtime_mutation", "cross_polis_export"]
      }
    },
    {
      citizen_id: "proto-citizen-beta",
      display_name: "Prototype Citizen Beta",
      role: "worker",
      lifecycle_state: "proposed",
      continuity_status: "admission_pending",
      current_episode: null,
      resource_balance: { compute_units: 3 },
      capability_envelope: {
        allowed: ["admission_review"],
        forbidden: ["episode_execution", "direct_runtime_mutation"]
      }
    }
  ],
  freedom_gate: {
    recent_docket: [
      { decision_id: "fg-allow-alpha-001", actor: "proto-citizen-alpha", action: "bounded_task_work", decision: "allow" },
      { decision_id: "fg-refuse-cross-polis-001", actor: "proto-citizen-alpha", action: "cross_polis_export", decision: "refuse" },
      { decision_id: "fg-defer-beta-001", actor: "proto-citizen-beta", action: "episode_execution", decision: "defer" }
    ]
  },
  trace: {
    trace_tail: [
      { event_sequence: 1, actor: "kernel.clock_service", event_type: "service_tick", summary: "Clock service observed ready." },
      { event_sequence: 3, actor: "kernel.scheduler", event_type: "service_tick", summary: "Scheduler observed ready." },
      { event_sequence: 8, actor: "kernel.operator_control_interface", event_type: "service_tick", summary: "Operator control interface observed ready." }
    ]
  },
  operator_actions: {
    available_actions: [
      { action: "inspect_citizen", mode: "read_only", status: "available_in_console_prototype" },
      { action: "open_freedom_gate_decision", mode: "read_only", status: "available_in_console_prototype" }
    ],
    disabled_actions: [
      { action: "pause_citizen", reason: "Requires operator command packet design and kernel handling." },
      { action: "request_snapshot", reason: "Requires snapshot command packet and Runtime v2 snapshot implementation." },
      { action: "resume_citizen", reason: "Requires recovery eligibility and wake semantics." }
    ]
  }
};

let packet = fallbackPacket;

const formatId = (value) => value.replaceAll("_", " ");

function renderServices() {
  const target = document.querySelector("#service-ladder");
  target.innerHTML = packet.kernel.service_states.map((service) => `
    <div class="service-row">
      <span class="service-dot" aria-hidden="true"></span>
      <span>${formatId(service.service_id)}</span>
      <span class="service-kind">${service.service_kind} / ${service.lifecycle_state}</span>
    </div>
  `).join("");
}

function renderDocket() {
  const target = document.querySelector("#verdict-stack");
  target.innerHTML = packet.freedom_gate.recent_docket.map((entry) => `
    <button class="verdict-row" type="button" data-decision="${entry.decision}">
      <span class="verdict-dot" aria-hidden="true"></span>
      <span>${formatId(entry.action)} <small>by ${entry.actor}</small></span>
      <span class="verdict-kind">${entry.decision}</span>
    </button>
  `).join("");
}

function renderTrace() {
  const target = document.querySelector("#trace-ribbon");
  target.innerHTML = packet.trace.trace_tail.map((event) => `
    <li class="trace-row">
      <span class="trace-sequence">${String(event.event_sequence).padStart(2, "0")}</span>
      <span>${event.summary} <small>${event.actor}</small></span>
    </li>
  `).join("");
}

function renderActions() {
  const target = document.querySelector("#operator-actions");
  const available = packet.operator_actions.available_actions.map((action) => `
    <div class="action-row" data-mode="read_only">
      <span>${formatId(action.action)}</span>
      <span class="service-kind">read only</span>
    </div>
  `);
  const disabled = packet.operator_actions.disabled_actions.map((action) => `
    <div class="action-row" data-mode="disabled">
      <span>${formatId(action.action)}</span>
      <span class="service-kind">disabled</span>
    </div>
  `);
  target.innerHTML = [...available, ...disabled].join("");
}

function renderInspector(citizenId) {
  const citizen = packet.citizens.find((item) => item.citizen_id === citizenId) || packet.citizens[0];
  document.querySelector("#inspector-heading").textContent = citizen.display_name;
  document.querySelector("#inspector-state").textContent = `${citizen.lifecycle_state} ${citizen.role} / ${formatId(citizen.continuity_status)}`;
  document.querySelector("#inspector-episode").textContent = citizen.current_episode || "no active episode";
  document.querySelector("#inspector-compute").textContent = `${citizen.resource_balance.compute_units} units`;
  document.querySelector("#inspector-allowed").textContent = citizen.capability_envelope.allowed.map(formatId).join(", ");
  document.querySelector("#inspector-forbidden").textContent = citizen.capability_envelope.forbidden.map(formatId).join(", ");
  document.querySelectorAll(".citizen-node").forEach((node) => {
    node.classList.toggle("is-selected", node.dataset.citizen === citizen.citizen_id);
  });
}

function attachInteractions() {
  document.querySelectorAll(".citizen-node").forEach((node) => {
    node.addEventListener("click", () => renderInspector(node.dataset.citizen));
  });
}

function renderConsole() {
  renderServices();
  renderDocket();
  renderTrace();
  renderActions();
  renderInspector(packet.citizens[0].citizen_id);
  attachInteractions();
}

async function loadPacket() {
  const ref = document.querySelector(".observatory-shell")?.dataset.packetRef;
  if (!ref) {
    renderConsole();
    return;
  }

  try {
    const response = await fetch(ref);
    if (response.ok) {
      packet = await response.json();
    }
  } catch (_error) {
    packet = fallbackPacket;
  }

  renderConsole();
}

loadPacket();
