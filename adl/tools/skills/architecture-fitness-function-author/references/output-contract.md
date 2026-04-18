# Architecture Fitness Function Author Output Contract

The architecture fitness-function author emits a plan and check specification
artifact. It does not install checks or mutate repositories.

Required sections:

- Fitness Function Catalog
- Machine-Checkable Invariants
- Human-Judgment Candidates
- Deferred Automation Boundaries
- Validation Command Plan
- Expected Failure Modes
- Implementation Handoffs
- Validation Performed
- Residual Risk

Each fitness-function candidate must include:

- rule id
- source evidence
- invariant statement
- classification: `machine_checkable`, `human_judgment`, or `deferred`
- check type
- suggested implementation surface
- validation command
- expected failure mode
- false-positive risk
- downstream owner

Do not edit tests, CI, docs, policies, production code, issues, PRs, or customer
repository files from this skill.
