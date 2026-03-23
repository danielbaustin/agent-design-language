

# HTA Editor Planning (v0.85 / v0.86 Bridge)

## Purpose

Define the **Human Task Artifact (HTA) editor model** for ADL, including:

- Structured Task Prompt (STP) editing
- Structured Implementation Prompt (SIP) editing
- Structured Output Record (SOR) review
- Task-bundle navigation and manipulation
- Alignment with task-bundle public artifact model
- Card-based editing that keeps STP / SIP / SOR visible together as one working bundle

This document consolidates currently distributed planning into a single **canonical HTA editor architecture**.

For v0.85, the HTA editor must be a **working card-based task-bundle editor**. Visual polish is explicitly secondary to correctness, artifact visibility, and control-plane integration. The editor must operate on all three artifacts together. For v0.85, it may only invoke ADL control-plane commands that actually exist in the repo or documented validated adapter surfaces; full direct browser support for `pr init`, `pr create`, `pr start`, `pr run`, and `pr finish` remains target-state architecture rather than current repo truth.

## Command Status For v0.85

This document follows the canonical command-status model in the editing
five-command execution plan.

| Command | v0.85 repo truth | Editor implication |
| --- | --- | --- |
| `pr init` | not yet implemented as a real command | editor may not claim direct support yet |
| `pr create` | not yet implemented as a real command; `pr new` is only a rough precursor | editor may not claim direct support yet |
| `pr start` | implemented and active | editor may expose this through the bounded validated adapter surface |
| `pr run` | not yet implemented as a real command | editor may not claim direct support yet |
| `pr finish` | implemented and active | editor may describe finish as part of the workflow model, but should not claim current browser-launched support unless a validated adapter exists |

---

## Source of Truth

- **STP (Structured Task Prompt)** → canonical design-layer artifact  
- **SIP (Structured Implementation Prompt)** → canonical implementation-layer artifact  
- **SOR (Structured Output Record)** → canonical execution record  

- **Task Bundle** → canonical grouping unit for all three artifacts  
- **Public Record (`docs/records/...`)** → canonical externalized artifact surface  

Editors must operate on these artifacts **directly**, not on derived or duplicated representations.

---

## Design Goals

1. **Direct Artifact Editing**
   - Editors operate directly on STP / SIP / SOR files
   - No hidden transformations or shadow state

2. **Deterministic Structure**
   - Enforced schema + section presence
   - Enumerated metadata fields where possible
   - No free-form structural drift

3. **Task-Bundle Native**
   - Primary unit of navigation = task bundle
   - Editors understand:
     - `stp.md`
     - `sip.md`
     - `sor.md`
     - bundle metadata

4. **Proof-Surface Alignment**
   - Editors must surface:
     - validation results
     - provenance links
     - execution artifacts
   - Editing and verification are tightly coupled

5. **Public Artifact Compatibility**
   - Everything editable locally must map cleanly to:
     - `docs/records/...`
   - No internal-only formats

6. **Card-Based End-to-End Control Surface**
   - The HTA editor presents STP / SIP / SOR as linked cards in one task-bundle workspace
   - The editor is not just a viewer; it is a control surface for the full ADL execution loop
   - The editor must be able to trigger validated control-plane actions rather than forcing the user back into ad hoc shell workflows for every step

---

## Editor Surfaces

### 1. STP Editor (Design Surface)

Capabilities:

- Structured editing of:
  - goals
  - constraints
  - validation plan
  - dependencies
- Enforced sections (non-optional where required)
- Enum-backed metadata fields
- Inline validation feedback

Output:

- Validated STP artifact
- Ready for `pr create` / `pr start`

---

### 2. SIP Editor (Implementation Surface)

Capabilities:

- Execution-brief editing:
  - exact files
  - commands
  - artifacts
- Tight coupling to repo state
- Deterministic validation requirements

Key property:

- Cannot be satisfied with prose alone
- Must specify executable or testable paths

---

### 3. SOR Viewer / Reviewer (Execution Surface)

Capabilities:

- View:
  - produced artifacts
  - logs
  - validation outputs
- Verify:
  - schema compliance
  - provenance linkage
  - expected vs actual outputs
- Structured review surface

Output:

- Accept / reject / iterate decision

---

### 4. Task-Bundle Card Editor (Primary UI Surface)

Capabilities:

- Present STP / SIP / SOR together as linked cards in one workspace
- Navigate bundles:
  - list tasks
  - open STP / SIP / SOR together
- Create new bundles
- Clone / branch bundles
- Link bundles (dependency graph)
- Surface validation status directly on the cards
- Provide control-plane action buttons or commands only for the subset that
  actually exists in the repo or through documented validated adapters.
- For v0.85, this is currently a bounded `pr start`-centric surface, not a
  full browser-executed five-command loop.

This is the **core HTA editor abstraction**.

The minimal viable HTA editor is therefore a **three-card task-bundle editor**:
- STP card = design surface
- SIP card = implementation surface
- SOR card = execution/review surface

These three cards must stay visibly linked as one task-bundle unit rather than behaving like unrelated files in separate tabs.

---

## Integration with Zed

Zed is the preferred near-term host environment.

Planned phases:

### Phase 1 (v0.85)
- File-based card editor for task bundles
- Basic validation hooks
- Task-bundle directory navigation
- Side-by-side STP / SIP / SOR card visibility
- Ability to trigger the currently supported bounded control-plane surface from
  the editor, centered on `pr start`

### Phase 2 (v0.86)
- Inline schema validation
- Structured editing widgets (sections, enums)
- Stronger command integration as additional lifecycle commands become real
- Better task-bundle navigation and state display

### Phase 3 (v0.9+)
- Full HTA-native UI:
  - bundle graph view
  - execution state visualization
  - integrated review flows

---

## Interaction with Control Plane

The intended lifecycle model remains:

- `pr init` → initialize canonical authoring state
- `pr create` → create or reconcile issue from STP
- `pr start` → generate SIP / begin execution context
- `pr run` → execute bounded agent loop
- `pr finish` → finalize SOR and PR handoff

But for v0.85, the editor must not overclaim direct support for commands that
do not yet exist. The HTA editor may only invoke commands that actually exist
in the repo or documented validated adapter layers. Today that means a bounded
adapter centered on `pr start`, with the fuller five-command model preserved as
target-state architecture.

Editors must:

- never bypass control-plane validation
- surface errors instead of masking them
- allow safe recovery paths

---

## Demo Plan (v0.85)

At least one concrete demo must exist:

**Task-Bundle Editor Demo**
- Open a task bundle in the HTA editor
- Edit the STP card
- Validate the STP card
- Trigger `pr start`
- Review or refine the SIP card
- Review the bounded execution/review state the current editor slice actually supports
- Review the resulting SOR card
- Use shell/control-plane follow-on steps where the browser surface does not yet
  expose the command directly

Artifacts:

- existing `docs/tooling/editor/` implementation
- task bundle in `docs/records/v0.85/tasks/...`
- `docs/tooling/editor/index.html`
- `docs/tooling/editor/task_bundle_editor.js`
- `docs/tooling/editor/demo.md`

---

## Non-Goals (This Milestone)

- Full IDE replacement
- Perfect UX
- Complete schema lock-down
- Migration of all historical artifacts

---

## Near-Term Implementation Slice

1. Standardize task-bundle layout:
   - `stp.md`, `sip.md`, `sor.md`

2. Make the editor visibly card-based:
   - STP card
   - SIP card
   - SOR card

3. Add schema validation:
   - headers
   - enums
   - required sections

4. Wire validated control-plane actions into the editor:
   - bounded `pr start` support now
   - richer command integration only as those commands become real

5. Ensure the editor demo works end-to-end on a real task bundle

---

## Open Questions

- How far to push enum normalization vs flexibility?
- How to represent bundle graph visually?
- When to introduce signing / verification in editor UI?
- How to expose public artifact browsing?

---

## Summary

The HTA editor is not “a UI feature.”  
It is the **primary interface to the ADL control plane**.

For v0.85, this means a working, card-based HTA editor that keeps STP / SIP / SOR linked together and can actually drive the control plane. A visually rough but operational editor is preferable to a polished mockup that cannot execute the ADL loop.

This milestone establishes:

- task-bundle-native editing
- deterministic structured prompts
- tight coupling between editing, execution, and verification

This is the foundation for:

- generalized agents
- public artifact systems
- verifiable AI workflows
