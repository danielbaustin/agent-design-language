# Multi-Agent Podcast Episode Contract

## Purpose

This contract defines the smallest truthful recurring episode shape for the
`v0.91.1` podcast pilot.

## Episode Shape

- format: transcript-first roundtable
- turn count: six explicit turns total
- participant count: three named participants
- stop rule: fixed turn count, visible in the saved transcript
- proof boundary: bounded attributable conversation, not identity continuity

## Recurring Cadence

- target cadence: one episode per week
- release shape: one bounded episode packet at a time
- repeatability requirement: each episode should be runnable from the same
  canonical wrapper with only bounded topic/title overrides
- operations constraint: weekly repetition must not require a new bespoke
  workflow per episode

## Stable Pilot Roles

- `ChatGPT`: host / synthesizer
- `Gemini`: challenger / systems analyst
- `Claude`: refiner / moral stylist

These are pilot defaults for the recurring format, not proof of permanent
long-lived identity.

## Required Proof Surfaces

Each episode should preserve:

- transcript
- proof note
- provider invocation log
- episode contract
- series manifest or equivalent role packet
- optional best-lines excerpt block

## Review Expectations

A reviewer should be able to answer all of the following from the packet alone:

- what the topic was
- who spoke in each turn
- what role each participant was playing
- what the stop rule was
- what the episode proved
- what the episode did not prove

For the recurring weekly format, a reviewer should also be able to answer:

- whether this episode follows the same stable packet shape as the prior one
- which parts changed episode-to-episode: topic, title, models, or operator
  overrides
- whether the episode still stayed inside the same proof boundary

## Audio Boundary

Audio is optional in the pilot.

The recurring format should remain truthful if:

- transcript author identity and audio renderer identity differ
- one provider requires a surrogate TTS path

## Non-Goals

This episode contract does not imply:

- a broad always-on media platform
- native audio support from every provider
- persistent long-term identity continuity
- autonomous federation or social cognition beyond the saved run
- daily or continuous publishing pressure beyond the bounded weekly cadence


## Audio Follow-on Boundary

A later audio path may render the same episode with provider-specific or surrogate
TTS lanes.

The contract must preserve:

- transcript author identity
- audio renderer identity
- explicit disclosure when a surrogate voice path is used
