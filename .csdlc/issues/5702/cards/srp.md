# Structured Review Prompt

Template: 1.0.0

Issue: 5702

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

## Scope

.adl/docs/TBD/PODCAST_STUDIO_NEXT_WEEK_LAUNCH_PLAN_5702.md
.csdlc/evidence/5702/diff-hygiene.log
.csdlc/evidence/5702/gemini-3.1-pro-review-summary.json
.csdlc/evidence/5702/podcast-plan-contract.log
.csdlc/evidence/5702/typed-doctor.log
.csdlc/issues/5702/audit.jsonl
.csdlc/issues/5702/cards/sip.md
.csdlc/issues/5702/cards/sip.values.json
.csdlc/issues/5702/cards/sor.md
.csdlc/issues/5702/cards/sor.values.json
.csdlc/issues/5702/cards/spp.md
.csdlc/issues/5702/cards/spp.values.json
.csdlc/issues/5702/cards/srp.md
.csdlc/issues/5702/cards/srp.values.json
.csdlc/issues/5702/cards/stp.md
.csdlc/issues/5702/cards/stp.values.json
.csdlc/issues/5702/cards/vpp.md
.csdlc/issues/5702/cards/vpp.values.json
.csdlc/issues/5702/index.json
.csdlc/locks/5702.lock
.csdlc/prepared/issues/5702/bind-current-worktree.json
.csdlc/prepared/issues/5702/call_gemini_31_review.py
.csdlc/prepared/issues/5702/complete-spp-s2.json
.csdlc/prepared/issues/5702/complete-spp-s3.json
.csdlc/prepared/issues/5702/complete-spp-s4.json
.csdlc/prepared/issues/5702/design.md
.csdlc/prepared/issues/5702/diagram.mmd
.csdlc/prepared/issues/5702/finalize-plan.json
.csdlc/prepared/issues/5702/reapprove-design-after-gemini-31.json
.csdlc/prepared/issues/5702/record-gemini-31-summary-artifact.json
.csdlc/prepared/issues/5702/record-validation-gemini-31-tracked-review.json
.csdlc/prepared/issues/5702/validate_podcast_launch_plan.py

## Prompts

- Review whether the plan can realistically get the podcast launch-ready today for a next-week launch while keeping audio and RSS as required launch blockers.
- Review whether the plan properly separates planned work, proof requirements, source-backed facts, and non-claims.

## Findings

[
  {
    "id": "5702-review-p1-gemini-disposition-overclaim",
    "severity": "p1",
    "summary": "SOR and plan previously overclaimed complete Gemini review incorporation; exact-head re-review confirmed the plan now incorporates or explicitly dispositions the launch-critical Gemini 3.1 Pro items.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:862d5d006bd35f00ebf35bc2041e50102a07547e:5e3ecb8dd4e12316169019f9bd88cd4644ba375847e63cd8ad65493d0b3194ac",
    "route": null
  },
  {
    "id": "5702-review-p2-path-hygiene",
    "severity": "p2",
    "summary": "Tracked packet previously retained machine-local FastWork paths; exact-head re-review found no /Volumes/FastWork, /Users/, file://, or similar machine-local path leakage outside intentional redaction-rule examples.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:862d5d006bd35f00ebf35bc2041e50102a07547e:5e3ecb8dd4e12316169019f9bd88cd4644ba375847e63cd8ad65493d0b3194ac",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- This is a plan-only issue; production audio, RSS, website deployment, guest acceptance, and weekly cadence remain unproven until follow-on implementation issues execute.

## Review Result

Revision: Some("git-blake3:862d5d006bd35f00ebf35bc2041e50102a07547e:5e3ecb8dd4e12316169019f9bd88cd4644ba375847e63cd8ad65493d0b3194ac")

Reviewer: Some("Lovelace:019faedd-8ef9-71c2-8296-2165d325bf73")

Result: pass
