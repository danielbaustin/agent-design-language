# Review To Test Planner Output Contract

The review-to-test planner emits a planning artifact, not tests.

Required sections:

- Findings To Test Map
- Generation Status Summary
- Test Task Briefs
- Fixture And Assertion Map
- Validation Command Plan
- Test Generator Handoffs
- Deferred And Unsafe Tasks
- Validation Performed
- Residual Risk

Each task must include:

- source finding id or title
- priority
- affected source path or explicit `unknown`
- behavior under test
- suggested test location
- fixture needs
- expected assertions
- validation command
- generation status: `generated`, `recommended`, `deferred`, or `unsafe`
- handoff owner

Safe tasks may include a `test-generator` handoff, but this skill must not run
that handoff.

Do not write tests, fixtures, snapshots, production code, issues, PRs, or
customer-repo changes from this skill.
