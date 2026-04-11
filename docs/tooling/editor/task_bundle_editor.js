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
    cardState: "review",
    cardCopy: "Review-first execution surface for evidence, integration state, and bounded follow-ups.",
    metadata: [
      { key: "task_id", label: "Task ID", required: true },
      { key: "run_id", label: "Run ID", required: true },
      { key: "version", label: "Version", required: true, defaultValue: "v0.85" },
      { key: "branch", label: "Branch", required: true },
      { key: "status", label: "Status", required: true, defaultValue: "IN_PROGRESS" },
      { key: "integration_state", label: "Integration state", required: true, defaultValue: "pr_open" },
      { key: "verification_scope", label: "Verification scope", required: true, defaultValue: "worktree" }
    ],
    sections: [
      ["summary_text", "Summary"],
      ["artifacts_produced", "Artifacts produced"],
      ["validation", "Validation"],
      ["primary_proof_surface", "Primary proof surface"],
      ["artifact_verification", "Artifact Verification"],
      ["review_focus", "Review focus"],
      ["follow_ups", "Follow-ups / Deferred work"]
    ],
    placeholders: {
      summary_text: "Bounded review surface for the current task-bundle execution record.",
      artifacts_produced: "- docs/tooling/editor/index.html\n- docs/tooling/editor/task_bundle_editor.js",
      validation: "- Required checks executed\n- Reviewer-visible verification surfaced",
      primary_proof_surface: "- bounded git diff over named files\n- deterministic grep over named files",
      artifact_verification: "- required artifacts are present\n- schema changes: none",
      review_focus: "- integration state is clear\n- evidence is visible\n- follow-ups are explicit",
      follow_ups: "- richer workflow decision loop lands in the next slice"
    }
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
  },
  sor: {
    status: ["NOT_STARTED", "IN_PROGRESS", "DONE", "FAILED"],
    integration_state: ["worktree_only", "pr_open", "merged"],
    verification_scope: ["worktree", "pr_branch", "main_repo"]
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
  },
  sor: {
    artifacts_produced: /^- /m,
    validation: /^- /m,
    primary_proof_surface: /^- /m,
    artifact_verification: /^- /m,
    review_focus: /^- /m,
    follow_ups: /^- /m
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
const copyActionButton = document.getElementById("copy-action");
const editorPanelTitle = document.getElementById("editor-panel-title");
const editorPanelCopy = document.getElementById("editor-panel-copy");
const actionSummary = document.getElementById("action-summary");
const actionCommand = document.getElementById("action-command");
const reviewSummary = document.getElementById("review-summary");
const reviewDecision = document.getElementById("review-decision");
const reviewChecklist = document.getElementById("review-checklist");
const reviewNote = document.getElementById("review-note");
const copyReviewNoteButton = document.getElementById("copy-review-note");

let currentArtifact = "stp";
const artifactDrafts = {};

function initialDraftFor(artifactKey) {
  const artifact = ARTIFACTS[artifactKey];
  const metadata = {};
  (artifact.metadata || []).forEach((field) => {
    metadata[field.key] = field.defaultValue || "";
  });
  const sections = {};
  (artifact.sections || []).forEach(([key]) => {
    sections[key] = artifact.placeholders[key] || "";
  });
  return { metadata, sections };
}

function draftFor(artifactKey) {
  if (!artifactDrafts[artifactKey]) {
    artifactDrafts[artifactKey] = initialDraftFor(artifactKey);
  }
  return artifactDrafts[artifactKey];
}

function buildArtifactModel(artifactKey) {
  const artifact = ARTIFACTS[artifactKey];
  const draft = draftFor(artifactKey);
  const metadata = {};
  (artifact.metadata || []).forEach((field) => {
    metadata[field.key] = draft.metadata[field.key] || "";
  });
  const sections = {};
  (artifact.sections || []).forEach(([key]) => {
    sections[key] = draft.sections[key] || "";
  });

  metadata.task_id = taskIdInput.value.trim() || metadata.task_id;
  if (artifactKey === "sip" || artifactKey === "sor") {
    metadata.branch = branchInput.value.trim() || metadata.branch;
  }

  return { artifactKey, artifact, metadata, sections };
}

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
  const draft = draftFor(currentArtifact);
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
    editorPanelCopy.textContent = currentArtifact === "sor"
      ? "Review the execution record in a bounded way. The preview updates as you refine evidence, integration, and follow-ups."
      : "Fill the required sections. The preview updates as you type.";
    artifact.metadata.forEach((field) => {
      form.append(createField(currentArtifact, field.key, field.label, draft.metadata[field.key] || "", false, field.required));
    });

    artifact.sections.forEach(([key, label]) => {
      form.append(createField(currentArtifact, key, label, draft.sections[key] || "", true, true));
    });
  }

  updateAll();
}

function createField(artifactKey, key, label, initialValue, isTextarea, required) {
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
  input.addEventListener("input", (event) => {
    rememberDraftValue(artifactKey, key, event.target.value, isTextarea);
    updateAll();
  });
  wrapper.append(input);

  return wrapper;
}

function rememberDraftValue(artifactKey, key, value, isTextarea) {
  const draft = draftFor(artifactKey);
  if (isTextarea) {
    draft.sections[key] = value;
  } else {
    draft.metadata[key] = value;
  }
}

function gather() {
  const model = buildArtifactModel(currentArtifact);
  (model.artifact.metadata || []).forEach((field) => {
    model.metadata[field.key] = valueFor(field.key);
  });
  (model.artifact.sections || []).forEach(([key]) => {
    model.sections[key] = valueFor(key);
  });
  return model;
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
  bundleRoot.textContent = `Historical demo bundle root: docs/records/v0.85/tasks/${taskId}/`;
  bundleActivePath.textContent = `Historical demo card target: docs/records/v0.85/tasks/${taskId}/${artifact.extension}`;
}

function deriveStartAction() {
  const stpDraft = draftFor("stp");
  const issueNumber = (valueFor("issue_number") || stpDraft.metadata.issue_number || "").trim();
  const branch = branchInput.value.trim();
  const branchMatch = branch.match(/^codex\/([0-9]+)-([a-z0-9][a-z0-9-]*)$/);

  if (!issueNumber) {
    return {
      ready: false,
      summary: "Enter a GitHub Issue Number on the STP card to prepare the bounded pr start action.",
      command: "adl/tools/editor_action.sh start --issue <issue-number> --branch codex/<issue>-<slug>"
    };
  }

  if (!/^[0-9]+$/.test(issueNumber)) {
    return {
      ready: false,
      summary: "GitHub Issue Number must be numeric before the editor can prepare a pr start command.",
      command: "adl/tools/editor_action.sh start --issue <issue-number> --branch codex/<issue>-<slug>"
    };
  }

  if (!branchMatch) {
    return {
      ready: false,
      summary: "Branch must match codex/<issue>-<slug> before the thin pr start adapter can run.",
      command: "adl/tools/editor_action.sh start --issue <issue-number> --branch codex/<issue>-<slug>"
    };
  }

  if (branchMatch[1] !== issueNumber) {
    return {
      ready: false,
      summary: "GitHub Issue Number and branch prefix must match before the adapter can invoke pr start safely.",
      command: "adl/tools/editor_action.sh start --issue <issue-number> --branch codex/<issue>-<slug>"
    };
  }

  return {
    ready: true,
    summary: "Thin control-plane adapter is ready to invoke pr start through the existing validated workflow.",
    command: `adl/tools/editor_action.sh start --issue ${issueNumber} --branch ${branch}`
  };
}

function validate(model) {
  const { artifactKey: modelArtifactKey, artifact, metadata, sections } = model;
  const results = [];

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

  if ((modelArtifactKey === "sip" || modelArtifactKey === "sor") && !/^codex\/[a-z0-9][a-z0-9-]*$/.test(branchInput.value.trim())) {
    results.push({ ok: false, text: "Branch should use the codex/<slug> format for SIP work." });
  } else if (modelArtifactKey === "sip" || modelArtifactKey === "sor") {
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

  if (modelArtifactKey === "sip" && !looksLikeTaskId(metadata.run_id || "")) {
    results.push({ ok: false, text: "Run ID should use the same task-id format as Task ID." });
  } else if (modelArtifactKey === "sip") {
    results.push({ ok: true, text: "Run ID uses the normalized task-id format." });
  }

  if (modelArtifactKey === "sip" && !/^v[0-9]+\.[0-9]+(\.[0-9]+)*$/.test(metadata.version || "")) {
    results.push({ ok: false, text: "Version should use the milestone format vN.N or vN.N.P." });
  } else if (modelArtifactKey === "sip") {
    results.push({ ok: true, text: "Version uses the normalized milestone version format." });
  }

  const enumRules = ENUM_RULES[modelArtifactKey] || {};
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

  const formatHints = FORMAT_HINTS[modelArtifactKey] || {};
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

  if (modelArtifactKey === "sor") {
    results.push({ ok: true, text: "SOR is visibly linked in the workspace shell and participates in the bounded review flow." });
  }

  const startAction = deriveStartAction();
  if (modelArtifactKey === "stp" && startAction.ready) {
    results.push({ ok: true, text: "Thin pr start adapter command is ready from the editor path." });
  } else if (modelArtifactKey === "stp") {
    results.push({ ok: false, text: "Thin pr start adapter needs matching numeric issue number and codex/<issue>-<slug> branch values." });
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
    demo_required: "Demo Required",
    integration_state: "Integration state",
    verification_scope: "Verification scope"
  };
  return labels[key] || key;
}

function sectionLabel(artifact, key) {
  const match = (artifact.sections || []).find(([sectionKey]) => sectionKey === key);
  return match ? match[1] : key;
}

function renderMarkdown({ artifactKey: modelArtifactKey, artifact, metadata, sections }) {
  const heading = titleInput.value.trim() || artifact.label;
  const lines = [renderYaml({ artifact_type: artifact.label, title: titleInput.value.trim(), ...metadata }), "", `# ${heading}`, ""];
  if (modelArtifactKey === "sor") {
    lines.push("## Summary", "", sections.summary_text || titleInput.value.trim() || "Replace me.", "");
    lines.push("## Main Repo Integration", "");
    lines.push(`- Integration state: ${metadata.integration_state || "pr_open"}`);
    lines.push(`- Verification scope: ${metadata.verification_scope || "worktree"}`);
    lines.push(`- Branch: ${metadata.branch || branchInput.value.trim()}`);
    lines.push("");
    artifact.sections
      .filter(([key]) => key !== "summary_text")
      .forEach(([key, label]) => {
        lines.push(`## ${label}`, "", sections[key] || "", "");
      });
  } else if (artifact.editable === false) {
    lines.push("## Summary", "", titleInput.value.trim() || "Replace me.", "");
    lines.push("## Workspace Role", "", "Visible execution/review card for the linked task bundle shell.", "");
    lines.push("## Current Boundaries", "", "- Visible in the bundle workspace\n- Full review flow deferred to the dedicated SOR/review issue", "");
    lines.push("## Follow-ups", "", "- Add provenance display\n- Add accept / reject / iterate flow", "");
  } else {
    lines.push("## Summary", "", titleInput.value.trim() || "Replace me.", "");
    artifact.sections.forEach(([key, label]) => {
      lines.push(`## ${label}`, "", sections[key] || "", "");
    });
  }

  return lines.join("\n").trimEnd();
}

function deriveReviewFlow(activeResults) {
  const sorModel = currentArtifact === "sor" ? gather() : buildArtifactModel("sor");
  const sorResults = currentArtifact === "sor" ? activeResults : validate(sorModel);
  const failing = sorResults.filter((item) => !item.ok);

  const checks = [
    {
      ok: !!sorModel.sections.primary_proof_surface && sorModel.sections.primary_proof_surface !== ARTIFACTS.sor.placeholders.primary_proof_surface,
      text: "Primary proof surface is explicitly recorded in the SOR."
    },
    {
      ok: !!sorModel.sections.artifact_verification && sorModel.sections.artifact_verification !== ARTIFACTS.sor.placeholders.artifact_verification,
      text: "Artifact verification is present and reviewer-visible."
    },
    {
      ok: !!sorModel.sections.review_focus && sorModel.sections.review_focus !== ARTIFACTS.sor.placeholders.review_focus,
      text: "Review focus is explicit instead of implied."
    },
    {
      ok: !!sorModel.sections.follow_ups && sorModel.sections.follow_ups !== ARTIFACTS.sor.placeholders.follow_ups,
      text: "Follow-ups / deferred work are captured."
    },
    {
      ok: !!sorModel.metadata.integration_state && !!sorModel.metadata.verification_scope,
      text: "Integration state and verification scope are both present."
    },
    {
      ok: failing.length === 0,
      text: "Current SOR validation has no remaining warnings."
    }
  ];

  const failedChecks = checks.filter((item) => !item.ok).length;
  const recommendation = failedChecks === 0
    ? {
        status: "ready",
        label: "Ready for review handoff",
        summary: "The bounded review loop is coherent: proof, artifact verification, and follow-ups are present, and the SOR surface is ready for reviewer inspection or closeout discussion."
      }
    : {
        status: "iterate",
        label: "Iterate before handoff",
        summary: "The editor can now drive the review loop, but the SOR still needs evidence or validation cleanup before it is ready for a bounded handoff."
      };

  const noteLines = [
    "## Review Snapshot",
    "",
    `- Recommendation: ${recommendation.label}`,
    `- Integration state: ${sorModel.metadata.integration_state || "pr_open"}`,
    `- Verification scope: ${sorModel.metadata.verification_scope || "worktree"}`,
    `- Primary proof surface: ${inlineValue(sorModel.sections.primary_proof_surface)}`,
    `- Artifact verification: ${inlineValue(sorModel.sections.artifact_verification)}`,
    `- Review focus: ${inlineValue(sorModel.sections.review_focus)}`,
    `- Follow-ups: ${inlineValue(sorModel.sections.follow_ups)}`,
    "",
    "## Reviewer Guidance",
    "",
    recommendation.status === "ready"
      ? "- Review the proof surface and artifact verification, then use the SOR as the bounded closeout record for the task bundle."
      : "- Resolve the missing or placeholder review fields, then re-check the SOR before closeout."
  ];

  return { recommendation, checks, note: noteLines.join("\n") };
}

function inlineValue(value) {
  if (!value) {
    return "missing";
  }
  return value
    .replace(/\n+/g, " ")
    .replace(/^- /, "")
    .trim()
    .slice(0, 140);
}

function renderReviewFlow(reviewModel) {
  reviewSummary.textContent = reviewModel.recommendation.summary;
  reviewDecision.className = `review-decision ${reviewModel.recommendation.status}`;
  reviewDecision.textContent = reviewModel.recommendation.label;
  reviewChecklist.innerHTML = "";
  reviewModel.checks.forEach((item) => {
    const li = document.createElement("li");
    li.className = item.ok ? "pass" : "warn";
    li.textContent = item.text;
    reviewChecklist.append(li);
  });
  reviewNote.textContent = reviewModel.note;
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
  const reviewModel = deriveReviewFlow(results);
  renderValidation(results);
  renderReviewFlow(reviewModel);
  preview.textContent = renderMarkdown(model);
  const startAction = deriveStartAction();
  actionSummary.textContent = startAction.summary;
  actionCommand.textContent = startAction.command;
  copyActionButton.disabled = !startAction.ready;
  copyActionButton.textContent = startAction.ready ? "Copy pr start command" : "Fix issue + branch first";
}

copyButton.addEventListener("click", async () => {
  await navigator.clipboard.writeText(preview.textContent);
  copyButton.textContent = "Copied";
  window.setTimeout(() => {
    copyButton.textContent = "Copy Markdown";
  }, 1200);
});

copyActionButton.addEventListener("click", async () => {
  if (copyActionButton.disabled) {
    return;
  }
  await navigator.clipboard.writeText(actionCommand.textContent);
  copyActionButton.textContent = "Copied";
  window.setTimeout(() => {
    copyActionButton.textContent = "Copy pr start command";
  }, 1200);
});

copyReviewNoteButton.addEventListener("click", async () => {
  await navigator.clipboard.writeText(reviewNote.textContent);
  copyReviewNoteButton.textContent = "Copied";
  window.setTimeout(() => {
    copyReviewNoteButton.textContent = "Copy Review Note";
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
