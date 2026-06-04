# Prompt Card Values Import And Round-Trip Normalization

Status: v0.91.5 Sprint 1 tooling contract

Issue: #3643

## Purpose

Prompt cards should move toward values-driven generation without treating rendered Markdown as the editable source of truth. The import path exists for one bounded job:

```text
existing rendered card -> values YAML candidate -> render -> structure validation -> round-trip comparison
```

It lets an agent test whether a current `sip.md`, `stp.md`, `spp.md`, `srp.md`, or `sor.md` can be represented by the active prompt-template values model.

## Command

```sh
adl-csdlc tooling prompt-template import-values \
  --kind <sip|stp|spp|srp|sor> \
  --input <card.md> \
  --out <values.yaml> \
  [--normalized-out <card.md>] \
  [--repo-root <path>]
```

The command:

- validates the source card against the tracked structure schema for the active template;
- extracts values from active template placeholders only;
- populates any required fields that are not represented in the rendered template with deterministic import scaffolding and reports those fields on stdout;
- writes locked fields under `system` and editable fields under `values`;
- validates the imported values with `validate-values`;
- re-renders through the active template registry;
- validates the rendered structure; and
- reports whether the round trip is `exact` or `normalized`.

## Workflow Guidance

Use `import-values` when starting from an existing rendered card and you need to know whether it can be migrated to values YAML safely.

Use `edit-values` when a values YAML file already exists and the change is a declared editable field update.

Use SIP/STP/SPP/SRP/SOR editor skills when lifecycle truth needs human or agent judgment. The importer proves template representability; it does not decide whether issue truth is correct.

Some required fields are values-schema metadata rather than rendered card text for every card kind. When the importer cannot recover those fields from Markdown, it writes deterministic scaffolding values only so the candidate file remains schema-valid and round-trip testable. Treat any stdout `NOTE: populated unrepresented required fields` line as a review marker before reusing the values file as lifecycle truth.

## Fail-Closed Rules

The importer must fail rather than infer when:

- locked template prose has changed;
- headings, fenced blocks, or frontmatter structure drift from the active schema;
- the card cannot be matched against the active template literals;
- repeated placeholders resolve to inconsistent values; or
- imported values fail the values schema or enum checks.

Unsupported historical cards should be routed to an editor skill or a dedicated migration issue, not silently coerced.

## Validation

Focused proof for this contract should include:

- helper tests for all five card kinds;
- CLI tests for import, values validation, render, and fail-closed drift;
- `validate-schemas` when tracked prompt-template schemas change; and
- `git diff --check`.
