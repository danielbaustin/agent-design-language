#!/usr/bin/env node
const path = require("path");

const root = process.argv[2];
if (!root) {
  throw new Error("usage: check_csdlc_prompt_editor_browser.js <repo-root>");
}

global.window = {};
global.document = {
  getElementById(id) {
    return stubElement(id);
  },
  createElement(tag) {
    return stubElement(tag);
  },
  querySelectorAll() {
    return [];
  },
};
global.navigator = { clipboard: { writeText: async () => {} } };

function stubElement(id) {
  return {
    id,
    innerHTML: "",
    textContent: "",
    dataset: {},
    className: "",
    classList: { toggle: () => true, remove: () => {}, add: () => {} },
    addEventListener: () => {},
    appendChild: () => {},
    scrollIntoView: () => {},
    blur: () => {},
    set value(value) {
      this._value = value;
    },
    get value() {
      return this._value || "";
    },
  };
}

require(path.join(root, "docs/tooling/csdlc-prompt-editor/editor_model.js"));
const editor = require(path.join(root, "docs/tooling/csdlc-prompt-editor/editor.js"));

for (const key of ["sip", "stp", "spp", "srp", "sor"]) {
  const card = editor.cardByKey(key);
  const values = editor.deriveTemplateValues(card, editor.draftFor(card));
  const markdown = editor.renderTemplate(card.template, values);
  const errors = editor.validate(card, values, markdown);
  if (errors.length > 0) {
    throw new Error(
      `${key} browser sample has validation errors: ${errors
        .map((error) => error.message)
        .join("; ")}`
    );
  }
  if (markdown.includes("<card_status>")) {
    throw new Error(`${key} browser sample left unresolved card_status`);
  }
  if (
    !markdown.includes("Card Status: draft") &&
    !markdown.includes('card_status: "draft"')
  ) {
    throw new Error(
      `${key} browser sample must remain draft until validator/lifecycle approval`
    );
  }
}

const sip = editor.cardByKey("sip");
const bad = editor.deriveTemplateValues(sip, {
  ...editor.draftFor(sip),
  non_goals: "-----broken",
});
const badErrors = editor.validate(sip, bad, editor.renderTemplate(sip.template, bad));
if (!badErrors.some((error) => error.field === "non_goals")) {
  throw new Error("browser validation did not flag suspicious non_goals text");
}

console.log("C-SDLC prompt editor browser sample check passed.");
