# Portable ADL Project Doctor Plan

Status: v0.91.5 Sprint 1 planning surface

Issue: #3569

## Purpose

The portable project doctor is a future read-only command that checks whether
the current repository is ready to run ADL-governed C-SDLC work. It should make
startup failures visible, deterministic, and fast.

This issue defines the doctor contract only. It does not implement the doctor.

## Proposed Command Shape

Preferred future command:

```sh
adl-csdlc project doctor --project-config adl_project.json
```

Compatibility wrapper shape:

```sh
$ADL_HOME/adl/tools/pr.sh project-doctor --project-config adl_project.json
```

The wrapper shape is optional and must not create a second workflow truth.

## Inputs

Required input:

- `adl_project.json`

Optional inputs:

- explicit repo root;
- explicit ADL checkout path;
- JSON output mode;
- profile-specific validation selector.

## Checks

The doctor should check:

- root `AGENTS.md` exists;
- root `adl_project.json` exists and parses;
- `schema_version` is supported;
- all required fields are present;
- enum values are supported;
- ADL tooling discovery resolves exactly one checkout;
- resolved checkout satisfies the valid-checkout contract;
- `prompt_template_registry` exists relative to the ADL checkout;
- issue tracker authority is explicit;
- card and worktree state ownership is explicit;
- local `.adl/`, worktrees, build output, scratch state, and private keys are
  ignored or explicitly declared;
- public artifact policy forbids host-local absolute paths;
- profile validation expectations are listed.

## Output Contract

The doctor should emit:

- `PASS`, `WARN`, or `FAIL`;
- resolved ADL checkout source;
- ADL tooling compatibility status;
- issue/card/worktree ownership summary;
- profile and validation summary;
- public/private artifact policy summary;
- actionable setup instructions for every failure.

JSON output should be machine-readable and stable enough for CI.

## Read-Only Rule

The doctor must not:

- create files;
- modify `.gitignore`;
- create issues;
- create cards;
- bind worktrees;
- run broad tests;
- install tooling;
- migrate repo state.

All mutations should happen in separate, explicitly tracked issues after the
doctor reports readiness gaps.

## Logging Expectations

The doctor should use the standard ADL observability event style so a hang is
visible immediately:

```text
adl_event schema=adl.observability.event.v1 command=adl-csdlc stage=project_doctor result=started
```

Long-running checks should emit stage events before and after each major
operation, including tooling discovery, config parse, checkout validation, and
profile validation.

## Follow-On Implementation Issue Candidate

Title:

```text
[v0.91.5][portable-adl] Implement read-only portable project doctor
```

Scope:

- implement `adl-csdlc project doctor`;
- validate `adl_project.json` shape and enum values;
- validate deterministic ADL checkout discovery;
- emit text and JSON output;
- add focused tests;
- do not migrate external repositories.
