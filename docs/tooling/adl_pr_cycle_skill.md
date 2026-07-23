---
name: adl_pr_cycle
description: "Compatibility entrypoint for routing a tracked ADL issue through the independent C-SDLC v2 Rust lifecycle, from typed bootstrap through reviewable publication and closeout handoff."
---

# adl_pr_cycle

This tracked file is the canonical source for the local Codex skill installed at:

- `$CODEX_HOME/skills/adl_pr_cycle/SKILL.md`

Install or resync the local skill with:

```bash
bash adl/tools/install_adl_pr_cycle_skill.sh
```

## Skill prompt

```text
You are running skill: adl_pr_cycle.

Purpose:
- Route one tracked issue through the independent C-SDLC v2 Rust control plane.
- Preserve the name as a compatibility entrypoint; do not revive sunset v1
  lifecycle wrappers or their card paths.

Inputs:
- issue_num (required)
- slug (required)
- title (required)
- paths (required, comma-separated tracked paths)
- version (required; use the issue's milestone/version label)
- mode (optional: apply|suggest, default apply)
- validation_profile (optional; a named repository validation profile, never an
  arbitrary shell string)
- publish (optional, default false)
- merge (optional, default false; requires explicit operator authorization)

Authority and binaries:
- Resolve the selected generation with `csdlc-install resolve`.
- Use only the installed Rust binaries under `.adl/bin/csdlc-v2/` and the typed
  skills under `csdlc-v2/operator/skills/`.
- The v2 state store and six-card projections under `.csdlc/` are machine truth.
- Cards are constructed and edited through the typed card-editor route, which
  uses the markdown.rs AST and the active template registry. Never patch a
  rendered card by hand.

Hard guardrails:
1) Deterministic state machine:
   preflight -> init -> bind -> design/plan -> implement -> validate -> review
   -> publish -> shepherd -> closeout
2) Never work on `main`. Bind one issue to one branch/worktree before tracked
   implementation edits and keep the primary checkout clean.
3) Do not invoke sunset v1 wrappers, prompt-template commands, or compatibility
   shell lifecycle surfaces. The control plane never evaluates shell/Python
   strings. A repository-declared validation profile may run a bounded external
   proof command only when its typed argv, lane, budget, and evidence contract
   are recorded in VPP truth.
4) Do not edit `.csdlc` state or Markdown cards directly. Use `csdlc-edit` and
   `csdlc-validate`; preserve the canonical SIP -> STP -> SPP -> VPP -> SRP ->
   SOR lifecycle and active session claim/lease invariants.
5) Do not publish until `csdlc-review` has current exact-head review evidence.
   Publication must fail closed without it. Do not merge or close the issue
   unless the operator explicitly authorizes that terminal action.
6) Keep retries bounded and preserve every failure artifact. Never hide a
   stale-generation, claim, review, validation, or ancestry error by retrying
   around it.

Procedure:
1) Preflight
   - Confirm the issue exists, the repository is identified, and the primary
     checkout is clean on `main` (unrelated user changes are preserved and are
     not part of this issue).
   - Resolve the issue's version and derive `codex/<issue_num>-<slug>` plus the
     bound worktree path.
   - Confirm all six cards, the design, and the diagram are present or can be
     generated from the current versioned prompt registry.
2) Init
   - Submit a typed bootstrap request to `csdlc-init`; this is the atomic,
     pre-binding creation of the issue record and six initial projections. It
     includes design and diagram paths, the claim, operator constraints, review
     scope, and explicit validation budgets; it is not implementation editing.
3) Bind
   - Submit a typed `csdlc-bind` request for the issue branch/worktree.
   - Preserve the claim owner, heartbeat/lease, protected paths, and stale-claim
     recovery evidence in the shared ledger.
4) Design/plan, implement, and validate
   - After binding, use `csdlc-v2-card-editor` for all semantic card
     construction/repair and run `csdlc-validate` after every accepted edit.
   - Make only the requested tracked changes in the bound worktree.
   - Run the smallest proving named validation profile through `csdlc-validate`;
     record local proof separately from deferred hosted proof.
5) Review and publish
   - Obtain bounded subagent review of the exact worktree revision and record it
     with `csdlc-review`.
   - Run `csdlc-validate finalize` and then `csdlc-publish` only when review truth,
     ancestry, staged paths, and validation evidence are current.
   - Hand the published PR to `csdlc-shepherd`; do not treat a draft or a green
     local check as merge proof.
6) Closeout
   - After the issue/PR reaches its authorized terminal state, run
     `csdlc-closeout` and retain the observed revision, receipts, and terminal
     transition. Stop and report if any dependency or external guardian blocks
     the terminal transition.

Required evidence/report:
- `.csdlc/issues/<issue_num>/index.json` and its six card projections
- `.git/csdlc-v2/requests/<issue_num>.json` for each typed operation
- bound branch/worktree, exact revision, validation lanes and budgets
- review assignment/result, publication receipt, shepherd observation, and
  closeout receipt when those phases are reached
- one concise report containing inputs, changed tracked paths, commands/typed
  operations attempted, validation results, blockers, and exactly one next
  action

Stop boundaries:
- `mode=suggest` stops after reporting the next typed operation.
- `publish=false` stops after exact-head review and final validation.
- `merge=false` stops before merge/closeout even when the PR is green.
- Any stale claim, stale revision, missing card, missing budget, failed proof,
  missing review, or ancestry drift is a blocker; preserve evidence and report
  the typed recovery operation instead of improvising.
```

## Truth boundaries

- This skill is a compatibility-facing router. The independent Rust v2 binaries
  and their typed operator skills are the only active lifecycle authority.
- Repository-declared validation may invoke bounded external tools, including a
  shell or Python program, but only as an explicit typed proof command. The
  C-SDLC control plane itself never depends on shell/Python lifecycle logic.
- Historical v1 records remain evidence only; they are not executable guidance.

## Failure policy

Fail closed on invalid input, missing authority, stale claims or revisions,
missing review truth, failed validation, publication drift, or incomplete
closeout. Preserve the machine-readable error and report one typed recovery
operation; do not silently fall back to a legacy command surface.
