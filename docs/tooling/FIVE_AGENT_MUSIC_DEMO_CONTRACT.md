# Five-Agent Music Demo Contract

This contract defines the bounded artifact package for the `v0.89.1` five-agent
Hey Jude MIDI flagship demo.

Required artifacts:

- `performance_manifest.json`
- `cast.json`
- `section_plan.json`
- `cue_timeline.json`
- `transcript.md`
- `performance_summary.md`
- `midi_binding.json`
- `midi_event_log.json`
- `midi_event_summary.json`
- `provider_participation_summary.json`

Contract rules:

- the package must represent five named participants
- the section order must remain `Opening`, `Verse Rotation`, `Chorus Build`,
  `Long Fade`, `Curtain Call`
- the cue layer must be profile-driven and event-logged
- the transcript must stay copyright-safe and section-cue oriented
- the final packet must remain transcript-first rather than becoming an audio
  production artifact
