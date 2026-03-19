const ARTIFACTS = {
  stp: {
    label: "Structured Task Prompt",
    extension: "stp.md",
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
  }
};

const typeSelect = document.getElementById("artifact-type");
const form = document.getElementById("artifact-form");
const preview = document.getElementById("artifact-preview");
const validationList = document.getElementById("validation-list");
const bundlePath = document.getElementById("bundle-path");
const taskIdInput = document.getElementById("task-id");
const titleInput = document.getElementById("title");
const branchInput = document.getElementById("branch");
const copyButton = document.getElementById("copy-preview");

Object.entries(ARTIFACTS).forEach(([value, artifact]) => {
  const option = document.createElement("option");
  option.value = value;
  option.textContent = artifact.label;
  typeSelect.append(option);
});

function buildForm() {
  const artifact = ARTIFACTS[typeSelect.value];
  form.innerHTML = "";

  artifact.metadata.forEach((field) => {
    form.append(createField(field.key, field.label, field.defaultValue || "", false, field.required));
  });

  artifact.sections.forEach(([key, label]) => {
    form.append(createField(key, label, artifact.placeholders[key] || "", true, true));
  });

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
  const artifact = ARTIFACTS[typeSelect.value];
  const metadata = {};
  artifact.metadata.forEach((field) => {
    metadata[field.key] = valueFor(field.key);
  });
  const sections = {};
  artifact.sections.forEach(([key]) => {
    sections[key] = valueFor(key);
  });

  metadata.task_id = taskIdInput.value.trim() || metadata.task_id;
  if (typeSelect.value === "sip") {
    metadata.branch = branchInput.value.trim() || metadata.branch;
  }

  return { artifact, metadata, sections };
}

function valueFor(id) {
  const el = document.getElementById(id);
  return el ? el.value.trim() : "";
}

function updateBundlePath() {
  const artifact = ARTIFACTS[typeSelect.value];
  const taskId = taskIdInput.value.trim() || "task-id";
  bundlePath.textContent = `Tracked bundle target: docs/records/v0.85/tasks/${taskId}/${artifact.extension}`;
}

function validate({ artifact, metadata, sections }) {
  const results = [];

  if (/^task-[a-z0-9][a-z0-9-]*$/.test(taskIdInput.value.trim())) {
    results.push({ ok: true, text: "Task ID uses the public task-bundle format." });
  } else {
    results.push({ ok: false, text: "Task ID should look like task-v085-wp05 or task-0870." });
  }

  if (titleInput.value.trim()) {
    results.push({ ok: true, text: "Title is present for the public record." });
  } else {
    results.push({ ok: false, text: "Title is required." });
  }

  if (typeSelect.value === "sip" && !/^codex\/[a-z0-9][a-z0-9-]*$/.test(branchInput.value.trim())) {
    results.push({ ok: false, text: "Branch should use the codex/<slug> format for SIP work." });
  } else if (typeSelect.value === "sip") {
    results.push({ ok: true, text: "Branch format is valid for the implementation prompt." });
  }

  artifact.metadata.forEach((field) => {
    if (field.required && !metadata[field.key]) {
      results.push({ ok: false, text: `${field.label} is required.` });
    }
  });

  artifact.sections.forEach(([key, label]) => {
    if (sections[key]) {
      results.push({ ok: true, text: `${label} is present.` });
    } else {
      results.push({ ok: false, text: `${label} is required.` });
    }
  });

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

function renderMarkdown({ artifact, metadata, sections }) {
  const lines = [renderYaml({ artifact_type: artifact.label, title: titleInput.value.trim(), ...metadata }), "", `# ${artifact.label}`, ""];
  lines.push("## Summary", "", titleInput.value.trim() || "Replace me.", "");
  artifact.sections.forEach(([key, label]) => {
    lines.push(`## ${label}`, "", sections[key] || "", "");
  });
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
typeSelect.addEventListener("change", buildForm);

taskIdInput.value = "task-v085-wp05";
titleInput.value = "[v0.85][WP-05] First authoring/editor surfaces";
branchInput.value = "codex/870-v085-wp05-first-editor-surfaces";
buildForm();
