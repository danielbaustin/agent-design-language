# C-SDLC Prompt Template Dogfood Findings

Date: 2026-05-23

Issue context: `#3286`

Practice targets:

- `#3289` `[v0.91.3][tools] Add human C-SDLC prompt form editors`
- `#3291` `[v0.91.3][process] Plan C-SDLC prompt-template/editor transition`
- `#3296` `[v0.91.3][tools] Enforce C-SDLC card-status transitions in skills`

## Summary

The new template bootstrap path is useful, but the first dogfood pass found several follow-up fixes needed before the process feels deterministic enough for routine C-SDLC use.

## Findings

### P1: Existing bundles are preserved instead of normalized to the new template contract

`pr init 3289 --version v0.91.3` found an existing bundle and preserved its current `SPP`.

The existing `SPP` still says:

- `status: "approved"`
- `activation_state: "design_time_ready"`

That is dangerous during the transition because older bundles can continue to carry pre-template readiness truth even after the template default is fixed to `draft`.

Expected behavior:

- Either `pr init` should report that the existing bundle is legacy and needs editor normalization, or a separate explicit normalization command should upgrade it.
- It should not silently leave a legacy design-ready SPP looking compatible with the new workflow.

### P1: Fresh STP generation can fail validation when the source issue body omits optional-looking sections

`#3291` generated a bundle, but its generated `stp.md` failed validation:

```text
Error: missing required sections: Deliverables, Repo Inputs, Dependencies, Demo Expectations, Issue-Graph Notes, Notes
```

The source issue body has `Summary`, `Goal`, `Required Outcome`, `Acceptance Criteria`, `Non-goals`, and `Tooling Notes`, but not every STP-required section.

Expected behavior:

- Template generation should always emit every required STP section.
- If source material is missing a section, the generator should fill a truthful placeholder such as `None declared in source issue prompt; review before execution.`
- The generated card should validate mechanically before human refinement.

### P1: `pr doctor` lookup still drifts toward the primary checkout for locally initialized worktree bundles

After creating `#3291` and `#3296` bundles in the `#3286` worktree, `pr doctor` looked for source issue prompts under the primary checkout:

```text
/Users/daniel/git/agent-design-language/.adl/v0.91.3/bodies/issue-3291-issue-3291.md
/Users/daniel/git/agent-design-language/.adl/v0.91.3/bodies/issue-3296-issue-3296.md
```

Those paths do not match the worktree-local source prompts that `pr init` created.

Expected behavior:

- Doctor should use the same source-prompt root as init when invoked from a bound or practice worktree.
- If a primary-checkout lookup is intentional, the error should explain that the operator is validating primary-checkout card state, not the worktree-local bundle.

### P2: The validator surface still only exposes `stp`, `sip`, and `sor`

The public validator usage currently says:

```text
adl tooling validate-structured-prompt --type <stp|sip|sor> --input <path> [--phase <phase>]
```

That means the five-card lifecycle cannot be validated through one consistent public entrypoint yet.

Expected behavior:

- Add `spp` and `srp` support to the structured-prompt validator, or provide a clearly named companion validator for those card types.
- Sprint preflight and issue readiness should use the same validation model that humans can run directly.

### P2: Placeholder scanning can flag intentional source text as unresolved template state

`#3296` contains intentional dogfood evidence that mentions `<card_status>` and `<timestamp>` in prose. A naive placeholder scan flags these lines even though they are not unresolved fields in the generated template.

Expected behavior:

- Placeholder checks should distinguish unresolved template fields from quoted or code-formatted historical evidence.
- Alternatively, issue bodies should consistently code-format literal placeholder names and the scanner should understand that convention.

### P2: Init output still uses older lifecycle wording

`pr init` reports:

```text
READ     .../sip.md
WRITE    .../sor.md
CONTRACT minimum v0.86 init = validated source prompt + root stp/sip/sor bundle
```

That wording is understandable historically, but it does not fully match the current five-card `SIP -> STP -> SPP -> SRP -> SOR` lifecycle.

Expected behavior:

- Init output should name all five cards as first-class lifecycle records.
- The reported contract should no longer imply that `stp/sip/sor` alone is the complete issue bundle.

### P3: Some generated metadata still needs stronger deterministic defaults

`#3296` generated:

- `wp: "unassigned"`
- `milestone_sprint: "Pending sprint assignment"`
- `pr_start.enabled: false`

Those values may be truthful for a sidecar issue, but they are risky if not explicitly reviewed because downstream execution gates may read them as inactive or incomplete.

Expected behavior:

- The generator should classify sidecar/tooling issues deliberately.
- If sprint assignment is unknown, the card should make the operational consequence explicit.

## What Worked

- `pr init` created complete five-file bundles for fresh issues `#3291` and `#3296`.
- Fresh SPPs now default to `status: "draft"` and `activation_state: "draft"`.
- `SIP`, `STP`, and `SOR` validation passed for `#3296`.
- `SIP` and `SOR` validation passed for all three practice targets.
- `SRP` uses `Structured Review Prompt`, not the legacy `Structured Review Policy`.

## Follow-Up Routing

These findings should be routed into the next process/tooling follow-ons rather than widening `#3286` indefinitely:

- `#3289`: human form editor and field validation UX should account for locked vs editable fields and all five card statuses.
- `#3291`: transition plan should include legacy-bundle normalization and validator coverage for all five cards.
- `#3296`: card-status enforcement should include deterministic transition rules, doctor/readiness gates, and sprint preflight integration.
