# Design: Final Agent Logic repository migration plan

This issue converts the earlier planning draft into the final executable
migration runbook without performing any transfer. The plan is grounded in a
live source-account inventory and the current `agent-logic` organization
inventory.

Exactly five product repositories are migration candidates. `asksifu` remains
personal. `Horust` is an inactive upstream-contribution fork and is excluded.
Repositories already owned by `agent-logic` are verified but not transferred.

The runbook uses one repository per migration window. Each transfer is preceded
by a redacted manifest and followed by exact verification before the next
transfer begins. The central ADL repository transfers last. The `agent-logic.ai`
production and beta links are updated and deployed in the same window as ADL.

Gemini 3.1 Pro reviews the exact final document for GitHub-specific omissions,
unsafe preservation assumptions, website cutover gaps, rollback weaknesses,
and unnecessary complexity. Actionable findings are incorporated before the
plan is considered final.
