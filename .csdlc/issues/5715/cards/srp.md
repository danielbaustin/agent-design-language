# Structured Review Prompt

Template: 1.0.0

Issue: 5715

Repository: danielbaustin/agent-design-language

Card: srp

Status: complete

## Scope

.csdlc/issues/5715
.csdlc/locks/5715.lock
.csdlc/prepared/issues/5715
adl/tools/generate_podcast_launch_packet.py
adl/tools/validate_podcast_launch_packet.py
demos/podcast
docs/milestones/v0.91.8/review/podcast_launch_5711/episodes.json

## Prompts

- Does the integrated studio route preserve the exported HTML bytes and image/script assets while using only clean filenames?
- Does the landing page route users to the studio without breaking episode/audio/RSS behavior?
- Does validation fail closed on source/copy digest drift and missing audio/RSS surfaces?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:288d3e214b166ffafe520049e8363d2f2fd3f4c7:8f6be6f0dd407f9a56072f2aa9e79b42aecb38b2e69e18e8697fdfb2fa013d6c")

Reviewer: Some("Pasteur/read-only-exact-head-plus-local-clean-gate")

Result: pass
