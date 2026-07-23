# Structured Review Prompt

Template: 1.0.0

Issue: 5502

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-v2/Cargo.toml
adl-v2/Cargo.lock
adl-v2/crates/adl-workcell-convergence
.csdlc/prepared/issues/5502/run-validation-lane.rb

## Prompts

- Can any missing, stale, forged, overlapping, out-of-scope, or revision-discontinuous output bypass convergence rejection?
- Can changed assumptions or partial success be hidden, reordered nondeterministically, or converted into silent scope expansion?
- Does the component retain pure decision authority and avoid task, filesystem, network, GitHub, review, merge, and closeout mutation?
- Are dependency, COTS, protected-path, LoC/test/module/time, PVF, no-deferral, CI, exact-review, and post-merge contracts complete?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:9f89413d33327b8a523af20057c0fa0a5f0f4f60:e0442c8b5a955e39ceffb266340b623d8f33393e4377557137bf34e4a7463afc")

Reviewer: Some("gpt-5.5:codex-exec-review:019f8c5a-2717-7bc0-b649-33f7843dcd72")

Result: pass
