const model = window.CSDLC_PROMPT_EDITOR_MODEL;

const cardList = document.getElementById("card-list");
const form = document.getElementById("prompt-form");
const preview = document.getElementById("markdown-preview");
const validationPanel = document.getElementById("validation-panel");
const activeCardTitle = document.getElementById("active-card-title");
const cardStatus = document.getElementById("card-status");
const systemFields = document.getElementById("system-fields");
const templateVersion = document.getElementById("template-version");
const validateButton = document.getElementById("validate-form");
const copyButton = document.getElementById("copy-markdown");
const copyValuesButton = document.getElementById("copy-values");
const toggleMarkdownButton = document.getElementById("toggle-markdown");

let activeKey = "sip";
const drafts = new Map();
const systemTimestamp = new Date().toISOString();
const listFieldKeys = new Set([
  "acceptance_criteria",
  "inputs",
  "target_files_surfaces",
  "demo_proof_requirements",
  "non_goals",
  "notes_risks",
  "deliverables",
  "repo_inputs",
  "dependencies",
  "issue_graph_notes",
  "validation_plan",
]);

if (!model || !Array.isArray(model.cards)) {
  throw new Error("Missing Rust-generated C-SDLC prompt editor model.");
}

templateVersion.textContent = `Template v${model.template_set}`;

function cardByKey(key) {
  return model.cards.find((card) => card.key === key);
}

function sampleValueFor(field) {
  const samples = {
    card_status: "draft",
    issue: "3289",
    issue_padded: "3289",
    version: "v0.91.3",
    slug: "v0-91-3-tools-human-csdlc-prompt-form-editors",
    title: "[v0.91.3][tools] Add human C-SDLC prompt form editors",
    branch: "codex/3289-v0-91-3-tools-human-csdlc-prompt-form-editors",
    issue_url: "https://github.com/danielbaustin/agent-design-language/issues/3289",
    source_issue_prompt:
      ".adl/v0.91.3/bodies/issue-3289-v0-91-3-tools-human-csdlc-prompt-form-editors.md",
    docs_context: "docs/templates/prompts/current.json",
    wp: "tools",
    summary: "Add a Rust-owned local editor for C-SDLC prompt cards.",
    goal: "Make C-SDLC cards editable as deterministic forms instead of regenerated prose.",
    required_outcome:
      "A local browser editor renders all five prompt types from the active SemVer template set.",
    acceptance_criteria:
      "- All five card types render\n- Invalid fields show errors\n- Generated samples validate",
    deliverables:
      "- Rust-owned form model\n- Local browser editor\n- Validator-clean generated samples",
    inputs:
      "- docs/templates/prompts/current.json\n- docs/templates/prompts/1.0.0/",
    repo_inputs:
      "- docs/templates/prompts/current.json\n- docs/templates/prompts/1.0.0/",
    dependencies: "- #3286 SemVer prompt-template substrate",
    target_files_surfaces:
      "- adl/src/csdlc_prompt_editor.rs\n- docs/tooling/csdlc-prompt-editor/",
    validation_plan:
      "- Run focused Rust tests\n- Run generated-card validation\n- Open the local editor page",
    demo_proof_requirements:
      "- Open the editor locally\n- Select each card type\n- Show invalid-field validation",
    non_goals:
      "- Full Jira replacement\n- Cloud persistence\n- Direct browser writes to git",
    issue_graph_note: "Follow-on to #3286.",
    issue_graph_notes: "- Follow-on to #3286.",
    notes_risks:
      "- Browser code must not become a second source of C-SDLC card semantics.",
    tooling_notes: "- Field metadata is generated from Rust.",
    plan_summary:
      "Render prompt cards from Rust-owned form metadata and SemVer templates.",
    dependencies_inline: "#3286 SemVer prompt-template substrate.",
    repo_inputs_inline: "docs/templates/prompts/current.json and docs/templates/prompts/1.0.0/.",
    deliverables_inline: "Rust form model, local browser editor, validator-clean samples.",
    acceptance_criteria_inline: "All five samples validate and the local form catches bad fields.",
    risks_inline: "JavaScript drift is the main risk; keep Rust as the model authority.",
    validation_plan_inline: "Run focused prompt-editor tests and generated-card validators.",
    notes_risks_inline: "Use Rust-generated metadata as the browser model.",
    target_files_surfaces_inline:
      "adl/src/csdlc_prompt_editor.rs and docs/tooling/csdlc-prompt-editor/.",
    non_goals_inline: "Full Jira replacement, cloud persistence, or direct browser writes.",
    required_outcome_type: "combination",
    demo_required: "true",
    stp_card:
      ".adl/v0.91.3/tasks/issue-3289__v0-91-3-tools-human-csdlc-prompt-form-editors/stp.md",
    sip_card:
      ".adl/v0.91.3/tasks/issue-3289__v0-91-3-tools-human-csdlc-prompt-form-editors/sip.md",
    spp_card:
      ".adl/v0.91.3/tasks/issue-3289__v0-91-3-tools-human-csdlc-prompt-form-editors/spp.md",
    srp_card:
      ".adl/v0.91.3/tasks/issue-3289__v0-91-3-tools-human-csdlc-prompt-form-editors/srp.md",
    sor_card:
      ".adl/v0.91.3/tasks/issue-3289__v0-91-3-tools-human-csdlc-prompt-form-editors/sor.md",
    status: "draft",
    activation_state: "draft",
    output_card:
      ".adl/v0.91.3/tasks/issue-3289__v0-91-3-tools-human-csdlc-prompt-form-editors/sor.md",
    branch_action: "Record branch binding truth for this execution.",
  };

  if (field.enum_values && field.enum_values.length > 0) {
    return samples[field.key] || field.enum_values[0];
  }
  return samples[field.key] || "";
}

function draftFor(card) {
  if (!drafts.has(card.key)) {
    const values = {};
    card.fields.forEach((field) => {
      values[field.key] = sampleValueFor(field);
    });
    if (card.key === "spp") {
      values.status = "draft";
      values.activation_state = "draft";
    }
    if (card.key === "sor") {
      values.status = "NOT_STARTED";
    }
    drafts.set(card.key, values);
  }
  return drafts.get(card.key);
}

function renderCardList() {
  cardList.innerHTML = "";
  model.cards.forEach((card) => {
    const button = document.createElement("button");
    button.type = "button";
    button.className = `card-button${card.key === activeKey ? " active" : ""}`;
    button.textContent = card.key.toUpperCase();
    button.addEventListener("click", () => {
      activeKey = card.key;
      render();
    });
    cardList.appendChild(button);
  });
}

function renderForm(card) {
  const draft = draftFor(card);
  form.innerHTML = "";
  card.fields.filter((field) => field.editable !== false).forEach((field) => {
    const wrapper = document.createElement("div");
    wrapper.className = "field";
    wrapper.id = `field-row-${field.key}`;

    const label = document.createElement("label");
    label.htmlFor = `field-${field.key}`;
    label.textContent = field.label;
    wrapper.appendChild(label);

    const help = document.createElement("div");
    help.className = "help";
    help.textContent = field.help;
    wrapper.appendChild(help);

    let input;
    if (field.input === "textarea") {
      input = document.createElement("textarea");
    } else if (field.input === "select") {
      input = document.createElement("select");
      const blank = document.createElement("option");
      blank.value = "";
      blank.textContent = "Select...";
      input.appendChild(blank);
      field.enum_values.forEach((value) => {
        const option = document.createElement("option");
        option.value = value;
        option.textContent = value;
        input.appendChild(option);
      });
    } else {
      input = document.createElement("input");
      input.type = "text";
    }

    input.id = `field-${field.key}`;
    input.name = field.key;
    input.spellcheck = true;
    input.value = draft[field.key] || "";
    input.addEventListener("input", () => {
      draft[field.key] = input.value;
      updatePreviewAndValidation();
    });
    input.addEventListener("change", () => {
      draft[field.key] = input.value;
      updatePreviewAndValidation();
    });
    wrapper.appendChild(input);
    form.appendChild(wrapper);
  });
}

function deriveTemplateValues(card, draft) {
  const values = { ...draft };
  const issue = values.issue || "";
  const slug = values.slug || "unfilled-slug";
  const version = values.version || "v0.0.0";
  const base = `.adl/${version}/tasks/issue-${issue || "0000"}__${slug}`;
  const source = `.adl/${version}/bodies/issue-${issue || "0000"}-${slug}.md`;

  values.card_status = values.card_status || "draft";
  values.issue_padded = issue.padStart(4, "0");
  values.task_id = `issue-${issue || "0000"}`;
  values.run_id = `issue-${issue || "0000"}`;
  values.source_issue_prompt = values.source_issue_prompt || source;
  values.issue_url =
    values.issue_url ||
    `https://github.com/danielbaustin/agent-design-language/issues/${issue || "0000"}`;
  values.docs_context = values.docs_context || "docs/templates/prompts/current.json";
  values.output_card = values.output_card || `${base}/sor.md`;
  values.stp_card = values.stp_card || `${base}/stp.md`;
  values.sip_card = values.sip_card || `${base}/sip.md`;
  values.spp_card = values.spp_card || `${base}/spp.md`;
  values.srp_card = values.srp_card || `${base}/srp.md`;
  values.sor_card = values.sor_card || `${base}/sor.md`;
  values.wp = values.wp || "process";
  values.issue_graph_note = values.issue_graph_note || "No issue graph note supplied.";
  values.issue_graph_notes = values.issue_graph_notes || "- No issue graph notes supplied.";
  values.inputs = values.inputs || "- Source issue prompt\n- Current milestone docs";
  values.target_files_surfaces =
    values.target_files_surfaces || "- Fill in target files or surfaces.";
  values.demo_proof_requirements =
    values.demo_proof_requirements || "- Fill in demo or proof requirements.";
  values.dependencies = values.dependencies || "- No additional dependencies.";
  values.repo_inputs = values.repo_inputs || "- Fill in repo inputs.";
  values.non_goals = values.non_goals || "- Fill in non-goals.";
  values.notes_risks = values.notes_risks || "- Fill in notes and risks.";
  values.tooling_notes = values.tooling_notes || "- No additional tooling notes.";
  values.branch_action =
    values.branch_action || "Record branch binding truth for this execution.";

  values.target_files_surfaces_inline =
    values.target_files_surfaces_inline || values.target_files_surfaces;
  values.non_goals_inline = values.non_goals_inline || values.non_goals;
  values.dependencies_inline = values.dependencies_inline || values.dependencies;
  values.repo_inputs_inline = values.repo_inputs_inline || values.repo_inputs;
  values.deliverables_inline = values.deliverables_inline || values.deliverables;
  values.acceptance_criteria_inline =
    values.acceptance_criteria_inline || values.acceptance_criteria;
  values.risks_inline = values.risks_inline || values.notes_risks;
  values.validation_plan_inline = values.validation_plan_inline || values.validation_plan;
  values.notes_risks_inline = values.notes_risks_inline || values.notes_risks;
  values.plan_summary = values.plan_summary || values.summary || values.goal;
  if (!values.status) {
    if (card.key === "sor") {
      values.status = "NOT_STARTED";
    } else if (card.key === "spp") {
      values.status = "draft";
    }
  }
  values.timestamp = values.timestamp || systemTimestamp;

  return values;
}

function setCardStatus(status) {
  cardStatus.textContent = status;
  cardStatus.dataset.status = status;
}

function renderSystemFields(card) {
  const draft = draftFor(card);
  const locked = [
    ["Card Status", draft.card_status || "draft"],
    ["System Timestamp", systemTimestamp],
    ["Output", card.output_file],
    ...card.fields
      .filter((field) => field.editable === false)
      .map((field) => [field.label, draft[field.key] || ""]),
  ];
  systemFields.innerHTML = locked
    .map(
      ([label, value]) => `
        <div>
          <dt>${escapeHtml(label)}</dt>
          <dd>${escapeHtml(value || "not supplied")}</dd>
        </div>
      `
    )
    .join("");
}

function escapeHtml(value) {
  return String(value)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

function renderTemplate(template, values) {
  return template.replace(/<([a-z0-9_]+)>/g, (match, key) => {
    if (Object.prototype.hasOwnProperty.call(values, key)) {
      return values[key];
    }
    return match;
  });
}

function yamlScalar(value) {
  const text = String(value || "");
  if (text.includes("\n")) {
    return `|-\n${text
      .split(/\r?\n/)
      .map((line) => `    ${line}`)
      .join("\n")}`;
  }
  return `"${text.replace(/\\/g, "\\\\").replace(/"/g, '\\"').replace(/\t/g, "\\t")}"`;
}

function valuesDocumentFor(card, values) {
  const editableKeys = new Set(
    card.fields
      .filter((field) => field.editable !== false)
      .map((field) => field.key)
  );
  const presentKeys = Object.keys(values).sort();
  const systemKeys = presentKeys.filter((key) => !editableKeys.has(key));
  const valueKeys = presentKeys.filter((key) => editableKeys.has(key));
  const lines = [
    "schema: adl.csdlc.prompt_template_values.v1",
    `template_set: ${yamlScalar(model.template_set)}`,
    `card_kind: ${yamlScalar(card.key)}`,
    "system:",
    ...systemKeys.map((key) => `  ${key}: ${yamlScalar(values[key])}`),
    "values:",
    ...valueKeys.map((key) => `  ${key}: ${yamlScalar(values[key])}`),
  ];
  return `${lines.join("\n")}\n`;
}

function addError(errors, field, message) {
  errors.push({ field, message });
}

function fieldLabel(card, key) {
  return card.fields.find((field) => field.key === key)?.label || key;
}

function validateListField(card, key, value, errors) {
  if (!listFieldKeys.has(key) || !value.trim()) {
    return;
  }
  value.split(/\r?\n/).forEach((line, index) => {
    const trimmed = line.trim();
    if (!trimmed) {
      return;
    }
    if (!/^(- |\d+\. )/.test(trimmed)) {
      addError(
        errors,
        key,
        `${fieldLabel(card, key)} line ${index + 1} should be a Markdown list item starting with "- " or "1. ".`
      );
    }
  });
}

function validateSuspiciousText(card, key, value, errors) {
  if (!value.trim()) {
    return;
  }
  const suspicious = value.match(/(&{2,}|-{3,}|_{3,}|[!?.,]{4,})/);
  if (suspicious) {
    addError(
      errors,
      key,
      `${fieldLabel(card, key)} contains suspicious repeated punctuation: "${suspicious[0]}".`
    );
  }
}

function validate(card, values, markdown) {
  const errors = [];
  card.fields.forEach((field) => {
    const value = String(values[field.key] || "").trim();
    if (field.required && !value) {
      addError(errors, field.key, `${field.label} is required.`);
    }
    if (field.enum_values.length > 0 && value && !field.enum_values.includes(value)) {
      addError(errors, field.key, `${field.label} must be one of: ${field.enum_values.join(", ")}.`);
    }
    if (field.editable !== false) {
      validateListField(card, field.key, value, errors);
      validateSuspiciousText(card, field.key, value, errors);
    }
  });

  if (values.issue && !/^[1-9][0-9]*$/.test(values.issue)) {
    addError(errors, "issue", "Issue Number must be a positive integer.");
  }
  if (values.version && !/^v[0-9]+\.[0-9]+(\.[0-9]+)?$/.test(values.version)) {
    addError(errors, "version", "Milestone Version must look like v0.91.3.");
  }
  const missingPlaceholders = [...new Set(markdown.match(/<[a-z0-9_]+>/g) || [])];
  if (missingPlaceholders.length > 0) {
    addError(
      errors,
      null,
      `Markdown preview still contains unresolved template placeholders: ${missingPlaceholders.join(", ")}.`
    );
  }
  if (card.key === "srp" && markdown.includes("Structured Review Policy")) {
    addError(errors, null, "SRP must say Structured Review Prompt, not Structured Review Policy.");
  }

  return errors;
}

function renderValidation(errors) {
  validationPanel.innerHTML = "";
  if (errors.length === 0) {
    const row = document.createElement("div");
    row.className = "validation-row ok";
    row.textContent =
      "Form-valid draft: run the structured prompt validator before treating this card as lifecycle-ready.";
    validationPanel.appendChild(row);
    return;
  }

  errors.forEach((error) => {
    const row = document.createElement("div");
    row.className = "validation-row error";
    row.textContent = error.message;
    validationPanel.appendChild(row);
  });
}

function markInvalidFields(errors) {
  document.querySelectorAll(".field.invalid").forEach((field) => {
    field.classList.remove("invalid");
  });
  errors.forEach((error) => {
    if (!error.field) {
      return;
    }
    const row = document.getElementById(`field-row-${error.field}`);
    if (row) {
      row.classList.add("invalid");
    }
  });
}

function updatePreviewAndValidation(options = {}) {
  const card = cardByKey(activeKey);
  const values = deriveTemplateValues(card, draftFor(card));
  let markdown = renderTemplate(card.template, values);
  let errors = validate(card, values, markdown);
  values.card_status = values.card_status || draftFor(card).card_status || "draft";
  draftFor(card).card_status = values.card_status;
  markdown = renderTemplate(card.template, values);
  errors = validate(card, values, markdown);
  setCardStatus(values.card_status);
  renderSystemFields(card);
  preview.textContent = markdown;
  renderValidation(errors);
  markInvalidFields(errors);
  if (options.focus && errors.length > 0) {
    validationPanel.scrollIntoView({ behavior: "smooth", block: "center" });
  }
  if (options.focus && errors.length === 0) {
    validateButton.textContent = "Validated";
    setTimeout(() => {
      validateButton.textContent = "Validate";
    }, 1100);
  }
}

function render() {
  const card = cardByKey(activeKey);
  activeCardTitle.textContent = card.label;
  renderSystemFields(card);
  renderCardList();
  renderForm(card);
  updatePreviewAndValidation();
}

copyButton.addEventListener("click", async () => {
  await navigator.clipboard.writeText(preview.textContent);
  copyButton.textContent = "Copied";
  copyButton.blur();
  setTimeout(() => {
    copyButton.textContent = "Copy Markdown";
  }, 1100);
});

copyValuesButton.addEventListener("click", async () => {
  const card = cardByKey(activeKey);
  const values = deriveTemplateValues(card, draftFor(card));
  await navigator.clipboard.writeText(valuesDocumentFor(card, values));
  copyValuesButton.textContent = "Copied";
  copyValuesButton.blur();
  setTimeout(() => {
    copyValuesButton.textContent = "Copy Values YAML";
  }, 1100);
});

toggleMarkdownButton.addEventListener("click", () => {
  const hidden = preview.classList.toggle("hidden");
  toggleMarkdownButton.textContent = hidden ? "Show Markdown" : "Hide Markdown";
  toggleMarkdownButton.blur();
});

validateButton.addEventListener("click", () => {
  updatePreviewAndValidation({ focus: true });
  validateButton.blur();
});

render();

if (typeof module !== "undefined") {
  module.exports = {
    cardByKey,
    deriveTemplateValues,
    draftFor,
    renderTemplate,
    valuesDocumentFor,
    validate,
  };
}
