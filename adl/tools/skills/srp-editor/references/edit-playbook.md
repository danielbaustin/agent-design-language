# SRP Editor Playbook

Use this skill only for bounded `srp.md` editing.

Prefer this order:
1. source issue prompt
2. current `srp.md`
3. linked `stp.md`, `sip.md`, `spp.md`, or `sor.md` if available
4. concrete review findings, reviewer notes, and finding dispositions supplied
   by the caller

Check for:
- legacy Structured Review Policy wording when final Structured Review Prompt
  truth is required
- missing review scope, reviewer instructions, or review-result sections
- review results claimed without supplied review evidence
- unresolved findings hidden as complete work
- finding dispositions that do not distinguish fixed, accepted, deferred, and
  unresolved results
- residual risk omitted after review
- stale placeholders or PR/merge claims

Safe edits:
- normalize `artifact_type` and headings to Structured Review Prompt semantics
- preserve review policy and reviewer instructions
- record supplied findings and dispositions truthfully
- preserve unresolved findings and residual risk
- remove stale or unsupported review-completion claims

Unsafe edits:
- inventing review evidence
- claiming no findings without an actual review
- resolving findings without evidence
- rewriting `STP`, `SIP`, `SPP`, or `SOR` instead of handing off
- widening issue scope
