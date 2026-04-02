# PR Tooling Skills for v0.87

## Purpose

This document is the direct skill-authoring companion to the v0.87 PR tooling
feature docs.

Use it to write the actual workflow skills without having to infer behavior from
the product-facing feature spec. The feature docs describe the intended product
model. This document describes the operational skill boundaries, inputs,
outputs, stop conditions, and invariants.

## Design Principles

- Keep one skill per workflow step.
- Keep mechanical work separate from qualitative review.
- Delay branch and worktree creation until execution time.
- Preserve sequential merge integrity even when issue bootstrap is parallelized.
- Keep "doctor" as a first-class diagnostic feature rather than forcing it into
  the main four-step workflow.

## Workflow Model

### Step 1: Issue Bootstrap

Intent:
- create or reconcile the issue record and root task bundle mechanically

Current command target:
- `adl/tools/pr.sh init ...` in the long-term target model

Current compatibility reality:
- `adl/tools/pr.sh create ...` currently performs new-issue bootstrap
- existing-issue bootstrap may still use `adl/tools/pr.sh init ...`

Expected outputs:
- GitHub issue exists
- canonical source prompt exists
- root STP exists
- root SIP exists
- root SOR exists

Must not do:
- no qualitative rewriting of STP or SIP
- no branch creation
- no worktree creation
- no code or docs implementation
- no PR creation

Stop condition:
- mechanical bootstrap is complete and the issue is ready for qualitative card review

### Step 2: Qualitative Card Review and Editing

Intent:
- turn the bootstrap prompt surfaces into execution-ready instructions

Current command target:
- no single lifecycle command; this is a human or review-skill phase

Expected inputs:
- source issue prompt
- root STP
- root SIP
- current repo context

Expected outputs:
- revised source prompt if needed
- execution-ready STP
- execution-ready SIP

Must not do:
- no worktree creation
- no branch creation
- no implementation work
- no PR creation

Stop condition:
- STP and SIP are specific, truthful, and ready for execution

### Step 3: Run / Execute

Intent:
- bind execution context at the last responsible moment and perform the task

Current command target:
- `adl/tools/pr.sh run <issue> --slug ... --version ...`

Expected inputs:
- issue number
- slug
- version
- reviewed STP and SIP

Expected outputs:
- branch exists
- worktree exists
- worktree-local task bundle exists
- implementation is complete
- SOR is written
- draft PR is opened

Must do:
- create or reuse the branch/worktree only at execution time
- sync the prepared root bundle into the worktree-local execution context
- run the smallest proving validation set
- keep the output card truthful under the pre-merge model

Must not do:
- no hidden repo repair
- no unreviewed widening of scope
- no fake post-merge phrasing in the SOR

Stop condition:
- draft PR exists and the SOR reflects the real pre-merge state

### Step 4: Review / Closeout

Intent:
- review the SOR and PR, address findings, and close the issue safely

Current command target:
- review is partly human and partly process-driven
- explicit `finish` behavior may remain during compatibility and closeout transition

Expected inputs:
- PR
- SOR
- review findings
- check status

Expected outputs:
- corrected branch if needed
- reviewed SOR
- merged or closed issue state

Must not do:
- no silent rewriting of history
- no claim of closure before review and merge truthfully justify it

Stop condition:
- review findings are addressed and the issue can be merged or closed honestly

## Doctor

"Doctor" is not one of the four main workflow skills. It is a cross-cutting
diagnostic and bounded-repair feature.

Intent:
- inspect readiness
- detect workflow drift
- surface deprecated usage
- perform only small mechanical repairs that are clearly safe

Doctor should be callable:
- before bootstrap, to inspect existing issue state
- after bootstrap, to inspect card readiness
- before run, to confirm execution readiness
- during review, to diagnose workflow drift

Doctor must not become:
- a replacement for qualitative STP/SIP review
- a hidden implementation skill
- a catch-all that obscures the main workflow steps

## Skill Boundaries

### Skill 1: Issue Bootstrap Skill

Responsibilities:
- create or reconcile the issue
- generate the root bundle
- stop at the mechanical boundary

Requires:
- title
- slug
- version
- labels or issue metadata policy

Success proof:
- issue plus root bundle exist and validate

### Skill 2: Card Review Skill

Responsibilities:
- refine source prompt, STP, and SIP
- check truthfulness, completeness, and reviewer legibility

Requires:
- source prompt
- STP
- SIP
- feature context

Success proof:
- STP and SIP are execution-ready and do not contain bootstrap placeholders

### Skill 3: Run Skill

Responsibilities:
- call issue-mode `run`
- create worktree/branch at execution time
- do the work
- write the SOR
- open the PR

Requires:
- issue number
- slug
- version
- reviewed cards

Success proof:
- PR exists
- SOR is filled truthfully
- validation evidence is recorded

### Skill 4: Review / Closeout Skill

Responsibilities:
- review SOR and PR
- track findings
- drive correction and closure

Requires:
- PR
- SOR
- review standard

Success proof:
- review state is explicit
- any findings are tracked
- merge or closeout is justified truthfully

## Command Mapping

### Target public model

- `init`
  - full mechanical bootstrap
- qualitative review
  - human or review-skill step
- `run`
  - execution-time binder plus implementation and PR opening
- review / closeout
  - post-run review and closure
- `doctor`
  - diagnostic surface available throughout

### Compatibility mapping during transition

- `create`
  - temporary compatibility path for new-issue bootstrap
- `start`
  - temporary compatibility shim behind issue-mode `run`
- `ready` / `preflight`
  - converge into `doctor`
- `finish`
  - may remain as an explicit closeout command during transition

## Invariants for All Skills

- No skill should invent workflow steps that are not documented.
- No skill should create a worktree before execution time unless the process is
  explicitly overridden.
- No skill should treat bootstrap placeholders as execution-ready content.
- No skill should overclaim main-repo state before merge.
- No skill should erase the distinction between product behavior docs and
  skill-authoring instructions.

## Reviewer Checklist for This Document

- Can a skill author implement each of the four skills directly from this doc?
- Is the boundary between mechanical and qualitative work explicit?
- Is "doctor" preserved clearly without becoming part of the four-step
  lifecycle?
- Does the document match the post-#1303 lifecycle direction truthfully?
