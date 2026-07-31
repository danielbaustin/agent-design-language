# Structured Review Prompt

Template: 1.0.0

Issue: 5344

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/5344
.csdlc/prepared/issues/5344
adl/tools/check_coverage_impact.sh
adl/tools/test_check_coverage_impact.sh

## Prompts

- Does the Runtime v3 coverage mapping select every existing guardian unit and guardian_cli integration test without naming deleted tests?
- Does the tooling-contract regression execute the mapper expression against the live nextest inventory and end with an explicit PASS marker?
- Does the healthy-window test retry only the local authenticated lease-listener connection with a strict bound while leaving production Guardian behavior unchanged?
- Does repeated exact execution prove the restart-budget test remains deterministic after the listener startup-race repair?
- Was #5587 already GitHub merged and closed before its expired claim was recovered and released through supported typed routes without running or re-enabling the paused Drive mirror?
- Do the SRP, SOR, audit, and exact review truthfully cover only this bounded CI follow-up?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The lifecycle-soak binary remains exercised by full CI and its native cross-platform process qualification suites, but is intentionally excluded from the in-process changed-source line threshold because llvm-cov cannot observe its child-process orchestration.
- All ordinary production Rust and unmapped source remain subject to the unchanged fail-closed 80 percent changed-source coverage policy.

## Review Result

Revision: Some("git-blake3:d0229bf68b99ae44115160c11a5d058c7da7f67b:b16d1172159088e5b124c583ae5cd30a9cfff729e9a833df21909609eda797b5")

Reviewer: Some("subagent:019fac6c-4d03-74e3-90a2-3c3f07ed609d")

Result: pass
