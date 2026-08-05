# Structured Task Prompt

Template: 1.0.0

Issue: 5843

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Deliver current canonical docs, release notes, feature list, ADR plan, skills, agent guidance, and milestone docs.

## Deliverables

- current canonical docs, release notes, feature list, ADR plan, skills, agent guidance, and milestone docs
- docs-review packet, ADR candidate packet if needed, and release-truth diff

## Acceptance

1. AC-1: WP-22 is passing, merged/terminal/ancestral, and every docs change cites its accepted exact-evidence row.
2. AC-2: A complete canonical inventory covers root docs, changelog, v0.92 milestone/features/ADR/release docs, skills, and applicable agent guidance with current ownership and status.
3. AC-3: Stale version/WP/issue/command/link/status text is corrected while historical evidence remains immutable and planned/blocked work remains explicit.
4. AC-4: Release notes and public claims describe only landed reviewed behavior and preserve birthday, provider, platform, privacy, governance, legal, personhood, consciousness, and v0.93 non-claims.
5. AC-5: Markdown, YAML/JSON, relative links, commands, versions, WP ownership, evidence references, redaction, and diff hygiene pass without broad product tests.
6. AC-6: The docs-review and optional ADR-candidate packets receive exact-head review with no unresolved actionable finding.

## Dependencies

- WP-22

## Inputs

- Passing WP-22 matrix, gate record, and blocker dispositions
- README.md, CHANGELOG.md, docs/milestones/v0.92, active feature lists, skills, and root/nested AGENTS.md
- Exact landed issue/PR/review/merge/terminal evidence for every changed claim

## Non Goals

- Product implementation, historical evidence rewrite, release approval, or issue-state cleanup
- WP-24 article or WP-24A podcast publication, WP-25 review execution, or WP-27 remediation
- Unsupported birthday, provider, platform, consciousness, personhood, governance, legal, or v0.93 completion claims
