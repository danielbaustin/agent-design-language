# Gemini in the Loop

This bounded `v0.89` demo presents Gemini as a welcomed ADL participant rather
than a pretend repo-native operator.

Canonical command:

```bash
bash adl/tools/demo_v089_gemini_in_the_loop.sh
```

What the demo does:

- prepares one bounded review packet
- generates a local Gemini provider-setup bundle
- invokes a Gemini-profile HTTP provider path through ADL
- validates the structured review output before accepting it
- writes reviewer-facing findings plus runtime proof artifacts

Primary proof surfaces:

- `artifacts/v089/gemini_in_the_loop/demo_manifest.json`
- `artifacts/v089/gemini_in_the_loop/packet/review_packet.md`
- `artifacts/v089/gemini_in_the_loop/review_artifacts/validated_review.json`
- `artifacts/v089/gemini_in_the_loop/review_artifacts/findings.md`
- `artifacts/v089/gemini_in_the_loop/runtime/runs/v0-89-gemini-in-the-loop/run_summary.json`
- `artifacts/v089/gemini_in_the_loop/runtime/runs/v0-89-gemini-in-the-loop/logs/trace_v1.json`

Truth boundary:

- this is a bounded packet demo
- ADL owns packet construction, validation, and artifact writing
- Gemini is not asked to browse the repo or claim tool autonomy
- the provider path is explicit and reviewable
