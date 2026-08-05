# Structured Intent Prompt

Template: 1.0.0

Issue: 5007

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Prepare issue-specific C-SDLC v2 state for #5007 Memory Palace ADR acceptance while keeping execution blocked on actual completed #4760 implementation proof.

## Required Outcome

The existing preparation worktree contains a clean, committed, pushed, reviewed preparation packet that a later execution session can use after #4760 proof exists; no ADR drafting, implementation, PR, publication, merge, or closeout is claimed by preparation.

## Scope

- Preparation only in `/Volumes/FastWork/adl-wp-5007` on `codex/5007-v0918-wp14-preparation`.
- Integrate `origin/main` exact SHA 51bc5ae51b57c19dbab693af1c5a45142995f4e5 into the preparation branch.
- Complete the six issue-specific cards plus reviewed design/diagram for the later Memory Palace ADR acceptance execution.
- Name exact dependencies, intended paths, COTS boundary, LoC/time budgets, PVF lanes, rollback, and no-deferral boundaries.
- Retain one bounded GPT-5.5 preparation review and fix preparation-only findings.

## Authority

- ADR 0051 remains deferred and is the source obligation for the future accepted ADR.
- #4760 actual completed Memory Palace implementation proof is the execution gate; issue closure metadata, stale claims, and typed closeout receipts are not substitutes.
- Typed closeout receipts and claim reconciliation are deferred to execution-time lifecycle truth and are not blockers for this preparation pass.
- Legacy `.adl` records, if present elsewhere, are context only; the current issue-local packet is `.csdlc/issues/5007` and `.csdlc/prepared/issues/5007`.
- No writes to `main`, `/private/tmp`, ADR candidate paths, runtime source, provider credentials, AWS, PRs, publication, or merge state.

## Assumptions

- #4760 is still open at preparation time, so execution remains blocked.
- #4765, #4768, and #4771 are closed, but execution must inspect retained proof before relying on any claim.
- The next candidate ADR number is expected to be 0058 because `docs/adr/0057-reversible-adl-v2-default-and-rollback.md` is the current highest accepted ADR file.

## Operator Constraints

- Preparation only; no ADR drafting, implementation, PR, publication, merge, or closeout.
- Do not spend time reacquiring stale claims; defer execution-time claim acquisition truthfully.
- Keep every artifact, scratch file, validation output, and build target inside `/Volumes/FastWork/adl-wp-5007` or `/Volumes/FastWork`; never use `/private/tmp`.
- Do not write on `main`; operate only in the named worktree/branch.
- Do not turn #4760 planning, closure metadata, claim reconciliation, or typed closeout receipts into Memory Palace implementation proof.
