# Structured Intent Prompt

Template: 1.0.0

Issue: 5845

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Produce ten complete Podcast Studio episode packages with final audio, metadata, feed records, and human and machine QA.

## Required Outcome

Each of the first ten episodes is independently reviewable and contains final script, audio, transcript, notes, metadata, RSS enclosure data, redaction proof, listen-check evidence, and editorial/audio review.

## Scope

- demos/podcast/episodes/001-meet-the-ai-coworkers/ through 010-what-does-a-weekly-ai-studio-look-like/
- demos/podcast/audio/
- demos/podcast/feed.xml
- demos/podcast/LAUNCH_READINESS.md
- adl/tools/generate_podcast_launch_packet.py
- adl/tools/validate_podcast_launch_packet.py
- adl/tools/test_podcast_launch_packet.sh
- .csdlc/evidence/5845/

## Authority

- WP-24A owns ten review-ready local episode packages
- #5819 and the route/storage decision identify any canonical public destination
- #3223/#3256 provide retained production-pipeline proof
- The operator retains deployment and publication authority

## Assumptions

- The #5702 ten-episode brief remains the launch sequence
- Podcast Studio v2 proof remains inspectable
- macOS and Linux validation are available before publication review

## Operator Constraints

- Historical smoke WAV/feed proof is not final episode proof
- Human guests remain draft until consent and participation are evidenced
- Episode 002 may invite DeepSeek but cannot imply acceptance or persistent identity
- All test temporary files stay under .csdlc/evidence/5845
- No deployment or directory submission without separate authority
- Release the preparation claim before implementation handoff
