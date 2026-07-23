# Structured Review Prompt

Template: 1.0.0

Issue: 5349

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-v2/crates/adl-adapters
.csdlc/prepared/issues/5349

## Prompts

- Do the direct dependencies exactly match the canonical WP-06 and WP-08 wave, with WP-07/#5591 transitive through #5341 and #5526 correctly downstream?
- Are preparation and future product protected paths disjoint from every active claim, with no shared-manifest or Runtime source write?
- Does every adapter have explicit preconditions, postconditions, stable errors, bounds, cancellation, and no hidden retry/scheduling/policy/signing authority?
- Can URL parsing, redirects, proxies, DNS/endpoint authority, oversized bodies, malformed JSON, header values, or cancellation bypass the HTTPS contract?
- Can any governed-tool input mint or widen authority, bypass Freedom Gate, suppress denial, execute a shell, alter evidence, or invoke a different tool?
- Do compatibility mappings reject unknown, ambiguous, lossy, extra-field, and alias-drift inputs without incumbent source reuse or silent fallback?
- Do exact COTS versions/features, LoC/module/test/time budgets, secret-canary proof, no-deferral matrix, rollback, and no-credential live-claim gate cover every acceptance criterion?

## Findings

[
  {
    "id": "WP09-R1",
    "severity": "p1",
    "summary": "The SOR named an aggregate validation lane that the runner did not yet expose",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:706ef96511fbdeba8f57cae096c41d0551d070fc:50404c66b91a5924a065f7c2401dbddee58c04c87c9a9fbf5b75b77acaa14ff5",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Preparation-era receipt wording remains immutable historical text but is explicitly superseded by current typed SOR, AC-1, AC-10, planning cards, design, and executable dependency gate; receipts cannot block execution, publication, or integration

## Review Result

Revision: Some("git-blake3:706ef96511fbdeba8f57cae096c41d0551d070fc:50404c66b91a5924a065f7c2401dbddee58c04c87c9a9fbf5b75b77acaa14ff5")

Reviewer: Some("subagent:/root/wp09_review")

Result: pass
