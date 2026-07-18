# Issue 5509 Design

## Intent

Keep a bounded Runtime v3 change that spans `adl-runtime` and its `adl` CSM
client out of unrelated Runtime v2 coverage. The route must remain narrow:
both owning crates run focused tests and coverage, while every other mixed
crate shape continues to fail closed.

## Contract

The accepted source family is limited to Runtime v3 kernel modules and the CSM
client/long-lived-agent integration. The ordinary PR-fast runner executes each
crate independently. The coverage runner composes their summaries. Path policy
recognizes this exact reviewed shape even when issue-local C-SDLC records and
docs accompany the implementation.

## Invariants

- No Runtime v2 source or tests are selected by this route.
- Unmapped mixed-crate changes still require broad validation.
- Each crate remains independently buildable and testable.
- No AWS execution is introduced.

