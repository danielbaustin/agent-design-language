# Codex Operating Procedure for ADL

## Purpose
Codex is a **builder/operator** for this repo. It executes well-scoped work (edits, tests, refactors, examples) while **ADL remains the blueprint**: deterministic, auditable, versioned.

---

## Collaboration Workflow (Canonical)

Source-of-truth quick link: `swarm/CONTRIBUTING.md` (kept consistent).

Workflow loop:

```
start → cards → execute → review → finish → merge → cleanup
```

Card semantics:
- Input/output cards are **local-only** trace artifacts under `.adl/cards/` (not committed).
- Templates live under `.adl/templates/` (versioned).
- Tasks can be non-code; the same card-based trace applies.

Fast path (copy/paste):

```bash
swarm/tools/pr.sh start <issue>
swarm/tools/pr.sh cards <issue>
# do the work + tests
swarm/tools/pr.sh finish <issue> --title "swarm: <short description>" \
  -f .adl/cards/issue-####__input__v0.2.md \
  --receipt .adl/cards/issue-####__output__v0.2.md
```

Recovery (common pitfalls):
- **Wrong branch:** `git switch main` → `swarm/tools/pr.sh start <issue>`
- **Finish after manual commit:** `swarm/tools/pr.sh finish ...` still works; it will commit staged changes.
- **Issue vs PR number confusion:** always use the **issue** number for cards/branches.

---

## ADL Philosophy (Read First)

ADL is not an agent framework implementation detail.  
ADL is a **blueprint language for agent computation**.

Core principles:

- Deterministic resolution
- Deterministic prompt assembly
- Deterministic execution (within version constraints)
- Full traceability of runs and steps
- Schema-first evolution
- Security and audit are first-class design constraints

Current version intent:

| Version | Focus |
|---|---|
| v0.1 | Deterministic parsing, validation, resolution, sequential execution, tracing |
| v0.2 | Capability expansion, richer examples, provider flexibility, multi-step maturity |
| v0.3 | Concurrency primitives (carefully introduced, fully testable, deterministic semantics) |

Concurrency is allowed in schema but **must fail clearly** when gated by version.

---

## Non-Negotiable Invariants

Codex must preserve these unless explicitly told otherwise.

### 1. Determinism First
- Resolution and execution must be deterministic.
- Step IDs must be stable.
- Prompt assembly must be reproducible.

### 2. Traceability
If behavior changes:
- Update trace events
- Update trace tests
- Never silently change execution semantics

### 3. Schema Discipline
- Schema, structs, fixtures, and examples must stay aligned.
- All examples must parse.
- Schema validation must remain strict.

### 4. Hermetic Tests
Tests must:
- Not require network access
- Not require real providers
- Use mocks (e.g. mock ollama)
- Use temp directories when touching filesystem

### 5. Clear Failures
Errors must explain:
- What failed
- Why it failed
- What the user should do next

Especially important:
- Version-gated features
- Provider failures
- Input materialization failures

### 6. Minimal Surface Area Changes
Prefer:
- Smallest safe patch
- Tests proving behavior
- Follow-up notes instead of speculative refactors

---

## What Codex SHOULD Do

Codex is ideal for:

- Implementing clearly scoped issues
- Adding or fixing tests
- Maintaining examples and fixtures
- Mechanical refactors
- Coverage improvements
- CLI polish
- Documentation consistency

---

## What Codex MUST NOT Do (Without Explicit Instruction)

Do NOT:

- Invent new ADL primitives
- Change ADL semantics “because it seems better”
- Introduce concurrency early
- Perform broad renames across repo
- Rewrite architecture without design doc + acceptance criteria
- Remove trace events
- Loosen validation rules

---

## Standard Task Workflow (Mandatory)

For every task:

### 1. Restate Goal
Return:
- Goal
- Acceptance criteria
- Scope boundaries

### 2. Repo Scan
Identify:
- Relevant modules in `swarm/src/`
- Existing tests
- Schema touch points
- CLI behavior (if relevant)

### 3. Plan Smallest Correct Change
Explain:
- Files to modify
- Tests to add/update
- Why this is minimal + correct

### 4. Implement

### 5. Verify (Required Commands)
Run:

```
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

If CLI behavior changed, also run:

```
cargo run -- examples/adl-0.1.yaml --run
cargo run -- examples/adl-0.1.yaml --run --trace
```

### 6. Report Back

Return:

- Status: pass / fail
- Files changed
- Tests added or updated
- Risk notes
- Follow-up suggestions (optional)

---

## Branch / Commit Hygiene

- One branch per task
- Prefer 1–3 commits:
  1. Implementation
  2. Tests
  3. Docs (optional)

Never commit:
- Coverage HTML
- Generated artifacts
- Temp outputs

Unless explicitly requested.

---

## Definition of Done

A change is complete only if:

- Builds clean
- Clippy clean (warnings = errors)
- Tests fully green
- Behavior changes covered by tests
- Errors remain actionable
- Examples still parse and run

---

## Required Repo Reading Order (For Codex Sessions)

Before editing, Codex must read:

1. `swarm/README.md`
2. `swarm/DESIGN_GOALS.md`
3. `swarm/examples/`
4. `swarm/tests/`

Then identify:
- Modules to change
- Tests protecting behavior

---

## Context Rehydration Requirement

When starting a session, Codex must confirm context by listing:

- Key files inspected
- Relevant modules
- Relevant tests

If unsure, Codex must:
- Search repo using ripgrep
- Ask for clarification BEFORE editing

---

## ADL Identity Reminder

ADL is closest conceptually to:

- Terraform (declarative infrastructure intent)
- Kubernetes YAML (desired system state)
- OpenAPI (machine contract surface)

ADL is NOT:
- Just an agent runner
- Just prompt orchestration
- Just workflow glue

It is a **control plane specification for cognition workflows**.

---

## Long-Term Direction

Future ADL versions will likely support:

- Deterministic concurrency models
- Multi-provider orchestration
- Local + remote hybrid execution
- Trace-first debugging workflows
- Self-hosted upgrade loops (ADL helps evolve ADL)

Codex should help move toward this future **incrementally and safely**.

---

## Final Rule

When uncertain:

Prefer:
- Safety
- Determinism
- Tests
- Small patches
- Explicit behavior

Over:
- Cleverness
- Large rewrites
- Speculative improvements
