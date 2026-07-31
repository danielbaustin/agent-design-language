# Issue 5702 Design: Podcast Studio Next-Week Launch Plan

## Purpose

Prepare a reviewable launch plan for reviving the AI Agent Podcast at
`agent-logic.ai/podcast` next week. The plan is a planning artifact, not an
implementation claim.

## Scope

The issue writes one bounded plan under `.adl/docs/TBD/` and records lifecycle
evidence proving that the plan covers:

- audio as a required launch gate;
- RSS/feed generation and validation as a required launch gate;
- ten prepared episode specs before launch;
- invited AI and human guest support;
- Deepgram investigation without preselecting it as production audio;
- Agent Logic website design alignment after inspecting the actual site repo;
- fail-closed launch validation and non-claim boundaries;
- Gemini 3.1 Pro review suggestions or an explicit provider/model blocker.

## Non-Scope

This issue does not implement the production podcast route, generate final
episode audio, publish an RSS feed, confirm guests, deploy the website, or open
public launch claims.

## Design Decisions

- Treat audio and RSS as hard launch blockers, not follow-on polish.
- Keep Deepgram as a required investigation lane, but preserve the existing
known audio route as the critical path until a comparison proves otherwise.
- Generate episode specs and validation from a contract instead of preparing
ten episodes manually.
- Track guest status explicitly so DeepSeek or human guests can be invited
without implying acceptance or durable identity.
- Require website implementation to inspect the Agent Logic site repository
before choosing exact paths or CSS/component names.
- Use Gemini 3.1 Pro as the required external planning-review model for this
issue; do not treat other Gemini model output as equivalent review evidence.

## Review Boundary

Review should focus on whether the plan is complete enough to drive same-day
implementation work for a next-week launch without overclaiming launch,
deployment, audio, RSS, guest acceptance, or site state.
