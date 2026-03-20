const ARTIFACTS = {
  stp: {
    label: "Structured Task Prompt",
    extension: "stp.md",
    cardState: "editable",
    cardCopy: "Design surface for the task definition and public bundle intent.",
    metadata: [
      { key: "task_id", label: "Task ID", required: true },
      { key: "issue_number", label: "GitHub Issue Number", required: false },
      { key: "status", label: "Status", required: true, defaultValue: "draft" },
      { key: "action", label: "Action", required: true, defaultValue: "edit" },
      { key: "milestone_sprint", label: "Milestone Sprint", required: true, defaultValue: "Sprint 2" }
    ],
    sections: [
      ["goal", "Goal"],
      ["required_outcome", "Required Outcome"],
      ["acceptance_criteria", "Acceptance Criteria"],
      ["repo_inputs", "Repo Inputs"],
      ["dependencies", "Dependencies"],
      ["demo_expectations", "Demo Expectations"],
      ["non_goals", "Non-goals"],
      ["notes", "Notes"]
    ],
    placeholders: {
      goal: "State the concrete outcome this task should produce.",
      required_outcome: "- Real code, docs, demo, or tests required\n- Docs-only completion is not sufficient",
      acceptance_criteria: "- At least one real surface ships\n- Bounded proof surface exists",
      repo_inputs: "- docs/milestones/v0.85/WBS_v0.85.md",
      dependencies: "- WP-04",
      demo_expectations: "- Required demo: editor-workflow-demo",
      non_goals: "- Full long-term productization",
      notes: "- Future richer editors can build on this first slice."
    }
  },
  sip: {
    label: "Structured Implementation Prompt",
    extension: "sip.md",
    cardState: "editable",
    cardCopy: "Implementation surface for concrete files, commands, and proof paths.",
    metadata: [
      { key: "task_id", label: "Task ID", required: true },
      { key: "run_id", label: "Run ID", required: true },
      { key: "version", label: "Version", required: true, defaultValue: "v0.85" },
      { key: "branch", label: "Branch", required: true },
      { key: "required_outcome_type", label: "Required Outcome Type", required: true, defaultValue: "code" },
      { key: "demo_required", label: "Demo Required", required: true, defaultValue: "true" }
    ],
    sections: [
      ["goal", "Goal"],
      ["required_outcome", "Required Outcome"],
      ["acceptance_criteria", "Acceptance Criteria"],
      ["inputs", "Inputs"],
      ["target_files", "Target Files / Surfaces"],
      ["validation_plan", "Validation Plan"],
      ["demo_requirements", "Demo / Proof Requirements"],
      ["constraints", "Constraints / Policies"],
      ["non_goals", "Non-goals / Out of Scope"],
      ["notes", "Notes / Risks"]
    ],
    placeholders: {
      goal: "State the concrete implementation goal for this run.",
      required_outcome: "- Ship code and docs\n- Produce a bounded proof surface",
      acceptance_criteria: "- Required commands pass\n- Demo/proof surface is captured",
      inputs: "- Source Structured Task Prompt\n- Relevant milestone docs",
      target_files: "- docs/tooling/editor/index.html",
      validation_plan: "- Required commands:\n- Required tests:\n- Required artifacts / traces:",
      demo_requirements: "- Required demo(s): editor-workflow-demo",
      constraints: "- Preserve deterministic behavior\n- No absolute host paths",
      non_goals: "- Full productization",
      notes: "- Keep the slice bounded and honest."
    }
  },
  sor: {
    label: "Structured Output Record",
    extension: "sor.md",
    cardState: "shell",
    cardCopy: "Execution/review card is visibly linked now; richer review flow lands in the next slice.",
    editable: false
  }
};

const ENUM_RULES = {
  stp: {
    status: ["draft", "active", "complete"],
    action: ["create", "edit", "close", "split", "supersede"]
  },
  sip: {
    required_outcome_type: ["code", "docs", "tests", "demo", "combination"],
    demo_required: ["true", "false"]
  }
};

const FORMAT_HINTS = {
  stp: {
    required_outcome: /^- /m,
    acceptance_criteria: /^- /m,
    repo_inputs: /^- /m,
    demo_expectations: /(Required demo|^- )/m
  },
  sip: {
    required_outcome: /^- /m,
    acceptance_criteria: /^- /m,
    inputs: /^- /m,
    target_files: /^- /m,
    validation_plan: /Required commands:/,
    demo_requirements: /Required demo\(s\):/,
    constraints: /Determinism requirements:/
  }
};

const form = document.getElementById("artifact-form");
const preview = document.getElementById("artifact-preview");
const validationList = document.getElementById("validation-list");
const bundleRoot = document.getElementById("bundle-root");
const bundleActivePath = document.getElementById("bundle-active-path");
const bundleCards = document.getElementById("bundle-cards");
const taskIdInput = document.getElementById("task-id");
const titleInput = document.getElementById("title");
const branchInput = document.getElementById("branch");
const copyButton = document.getElementById("copy-preview");
const editorPanelTitle = document.getElementById("editor-panel-title");
const editorPanelCopy = document.getElementById("editor-panel-copy");

let currentArtifact = "stp";

function buildCards() {
  bundleCards.innerHTML = "";
  Object.entries(ARTIFACTS).forEach(([value, artifact]) => {
    const card = document.createElement("button");
    card.type = "button";
    card.className = `bundle-card ${value === currentArtifact ? "active" : ""}`;
    card.dataset.artifact = value;
    card.innerHTML = `
      <span class="bundle-card-status ${artifact.cardState}">${artifact.cardState}</span>
      <span class="bundle-card-label">${artifact.label}</span>
      <span class="bundle-card-meta">${artifact.extension}</span>
      <span class="bundle-card-copy">${artifact.cardCopy}</span>
    `;
    card.addEventListener("click", () => {
      currentArtifact = value;
      buildForm();
    });
    bundleCards.append(card);
  });
}

function buildForm() {
  const artifact = ARTIFACTS[currentArtifact];
  form.innerHTML = "";
  editorPanelTitle.textContent = artifact.label;

  if (artifact.editable === false) {
    editorPanelCopy.textContent = "This card is visible in the workspace shell now; the richer review surface is intentionally deferred.";
    const note = document.createElement("div");
    note.className = "shell-note";
    note.innerHTML = `
      <strong>SOR shell only in WP-05.</strong><br>
      This workspace proves that STP, SIP, and SOR stay linked as one task bundle.
      Full SOR review, validation/provenance display, and decision flow land in the next review-surface slice.
    `;
    form.append(note);
  } else {
    editorPanelCopy.textContent = "Fill the required sections. The preview updates as you type.";
    artifact.metadata.forEach((field) => {
      form.append(createField(field.key, field.label, field.defaultValue || "", false, field.required));
    });

    artifact.sections.forEach(([key, label]) => {
      form.append(createField(key, label, artifact.placeholders[key] || "", true, true));
    });
  }

  updateAll();
}

function createField(key, label, initialValue, isTextarea, required) {
  const wrapper = document.createElement("div");
  wrapper.className = "field-group";

  const title = document.createElement("label");
  title.htmlFor = key;
  title.textContent = label;
  wrapper.append(title);

  const input = isTextarea ? document.createElement("textarea") : document.createElement("input");
  input.id = key;
  input.dataset.required = required ? "true" : "false";
  input.value = initialValue;
  if (!isTextarea) {
    input.type = "text";
  }
  input.addEventListener("input", updateAll);
  wrapper.append(input);

  return wrapper;
}

function gather() {
  const artifact = ARTIFACTS[currentArtifact];
  const metadata = {};
  (artifact.metadata || []).forEach((field) => {
    metadata[field.key] = valueFor(field.key);
  });
  const sections = {};
  (artifact.sections || []).forEach(([key]) => {
    sections[key] = valueFor(key);
  });

  metadata.task_id = taskIdInput.value.trim() || metadata.task_id;
  if (currentArtifact === "sip" || currentArtifact === "sor") {
    metadata.branch = branchInput.value.trim() || metadata.branch;
  }
  if (currentArtifact === "sor") {
    metadata.run_id = taskIdInput.value.trim() || metadata.run_id || "task-id";
    metadata.version = "v0.85";
    metadata.status = "IN_PROGRESS";
  }

  return { artifact, metadata, sections };
}

function artifactKey() {
  return currentArtifact;
}

function looksLikeTaskId(value) {
  return /^task-[a-z0-9][a-z0-9-]*$/.test(value);
}

function normalizedValue(value) {
  return value.trim().toLowerCase();
}

function valueFor(id) {
  const el = document.getElementById(id);
  return el ? el.value.trim() : "";
}

function updateBundlePath() {
  const artifact = ARTIFACTS[currentArtifact];
  const taskId = taskIdInput.value.trim() || "task-id";
  bundleRoot.textContent = `Tracked bundle root: docs/records/v0.85/tasks/${taskId}/`;
  bundleActivePath.textContent = `Active card target: docs/records/v0.85/tasks/${taskId}/${artifact.extension}`;
}

function validate({ artifact, metadata, sections }) {
  const results = [];
  const artifactName = artifactKey();

  if (looksLikeTaskId(taskIdInput.value.trim())) {
    results.push({ ok: true, text: "Task ID uses the public task-bundle format." });
  } else {
    results.push({ ok: false, text: "Task ID should look like task-v085-wp05 or task-0870." });
  }

  if (titleInput.value.trim()) {
    results.push({ ok: true, text: "Title is present for the public record." });
  } else {
    results.push({ ok: false, text: "Title is required." });
  }

  if ((currentArtifact === "sip" || currentArtifact === "sor") && !/^codex\/[a-z0-9][a-z0-9-]*$/.test(branchInput.value.trim())) {
    results.push({ ok: false, text: "Branch should use the codex/<slug> format for SIP work." });
  } else if (currentArtifact === "sip" || currentArtifact === "sor") {
    results.push({ ok: true, text: "Branch format is valid for the bundle execution surfaces." });
  }

  (artifact.metadata || []).forEach((field) => {
    if (field.required && !metadata[field.key]) {
      results.push({ ok: false, text: `${field.label} is required.` });
    }
  });

  (artifact.sections || []).forEach(([key, label]) => {
    if (sections[key]) {
      results.push({ ok: true, text: `${label} is present.` });
    } else {
      results.push({ ok: false, text: `${label} is required.` });
    }
  });

  if (metadata.issue_number && !/^[0-9]+$/.test(metadata.issue_number)) {
    results.push({ ok: false, text: "GitHub Issue Number must be numeric when present." });
  } else if (metadata.issue_number) {
    results.push({ ok: true, text: "GitHub Issue Number is normalized as an integer." });
  }

  if (currentArtifact === "sip" && !looksLikeTaskId(metadata.run_id || "")) {
    results.push({ ok: false, text: "Run ID should use the same task-id format as Task ID." });
  } else if (currentArtifact === "sip") {
    results.push({ ok: true, text: "Run ID uses the normalized task-id format." });
  }

  if (currentArtifact === "sip" && !/^v[0-9]+\.[0-9]+$/.test(metadata.version || "")) {
    results.push({ ok: false, text: "Version should use the vN.N format." });
  } else if (currentArtifact === "sip") {
    results.push({ ok: true, text: "Version uses the normalized vN.N format." });
  }

  const enumRules = ENUM_RULES[artifactName] || {};
  Object.entries(enumRules).forEach(([fieldKey, allowed]) => {
    const raw = metadata[fieldKey] || "";
    const value = normalizedValue(raw);
    if (!value) {
      return;
    }
    if (!allowed.includes(value)) {
      results.push({ ok: false, text: `${fieldLabel(fieldKey)} must be one of: ${allowed.join(", ")}.` });
    } else {
      results.push({ ok: true, text: `${fieldLabel(fieldKey)} matches the stabilized contract vocabulary.` });
    }
  });

  Object.entries(sections).forEach(([key, value]) => {
    if (!value) {
      return;
    }
    const placeholder = artifact.placeholders && artifact.placeholders[key];
    if (placeholder && value === placeholder.trim()) {
      results.push({ ok: false, text: `${sectionLabel(artifact, key)} still uses its placeholder text and needs real content.` });
    }
  });

  const formatHints = FORMAT_HINTS[artifactName] || {};
  Object.entries(formatHints).forEach(([key, pattern]) => {
    const value = sections[key] || "";
    if (!value) {
      return;
    }
    if (!pattern.test(value)) {
      results.push({ ok: false, text: `${sectionLabel(artifact, key)} should follow the expected structured format for this artifact.` });
    } else {
      results.push({ ok: true, text: `${sectionLabel(artifact, key)} follows the expected structured format.` });
    }
  });

  results.push({ ok: true, text: "Task bundle keeps STP, SIP, and SOR visible together in one workspace." });

  if (currentArtifact === "sor") {
    results.push({ ok: true, text: "SOR is visibly linked in the workspace shell; richer review behavior is intentionally deferred." });
  }

  return results;
}

function renderYaml(metadata) {
  const lines = ["---"];
  Object.entries(metadata).forEach(([key, value]) => {
    if (!value) {
      return;
    }
    lines.push(`${key}: ${JSON.stringify(value)}`);
  });
  lines.push("---");
  return lines.join("\n");
}

function fieldLabel(key) {
  const labels = {
    status: "Status",
    action: "Action",
    required_outcome_type: "Required Outcome Type",
    demo_required: "Demo Required"
  };
  return labels[key] || key;
}

function sectionLabel(artifact, key) {
  const match = (artifact.sections || []).find(([sectionKey]) => sectionKey === key);
  return match ? match[1] : key;
}

function renderMarkdown({ artifact, metadata, sections }) {
  const heading = titleInput.value.trim() || artifact.label;
  const lines = [renderYaml({ artifact_type: artifact.label, title: titleInput.value.trim(), ...metadata }), "", `# ${heading}`, ""];
  lines.push("## Summary", "", titleInput.value.trim() || "Replace me.", "");

  if (artifact.editable === false) {
    lines.push("## Workspace Role", "", "Visible execution/review card for the linked task bundle shell.", "");
    lines.push("## Current Boundaries", "", "- Visible in the bundle workspace\n- Full review flow deferred to the dedicated SOR/review issue", "");
    lines.push("## Follow-ups", "", "- Add provenance display\n- Add accept / reject / iterate flow", "");
  } else {
    artifact.sections.forEach(([key, label]) => {
      lines.push(`## ${label}`, "", sections[key] || "", "");
    });
  }

  return lines.join("\n").trimEnd();
}

function renderValidation(results) {
  validationList.innerHTML = "";
  results.forEach((result) => {
    const item = document.createElement("li");
    item.className = `validation-item ${result.ok ? "pass" : "warn"}`;
    item.textContent = result.text;
    validationList.append(item);
  });
}

function updateAll() {
  updateBundlePath();
  const model = gather();
  const results = validate(model);
  renderValidation(results);
  preview.textContent = renderMarkdown(model);
}

copyButton.addEventListener("click", async () => {
  await navigator.clipboard.writeText(preview.textContent);
  copyButton.textContent = "Copied";
  window.setTimeout(() => {
    copyButton.textContent = "Copy Markdown";
  }, 1200);
});

taskIdInput.addEventListener("input", updateAll);
titleInput.addEventListener("input", updateAll);
branchInput.addEventListener("input", updateAll);

taskIdInput.value = "task-v085-wp05";
titleInput.value = "[v0.85][WP-05] First authoring/editor surfaces";
branchInput.value = "codex/870-v085-wp05-first-editor-surfaces";
buildCards();
buildForm();
