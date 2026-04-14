# Paper Sonata Demo Refresh

This `v0.89` refresh keeps the original bounded Paper Sonata workflow but makes
the reviewer package stronger and easier to inspect.

Canonical command:

```bash
bash adl/tools/demo_v088_paper_sonata.sh
```

## What Changed

- added `input_packet/packet_manifest.json`
- added `manuscript_package/abstract.md`
- added `manuscript_package/claim_matrix.md`
- added `manuscript_package/figures_spec.json`
- added `manuscript_package/revision_requests.json`
- added `manuscript_package/reviewer_brief.md`
- upgraded the manifest to `adl.paper_sonata_demo.v2`

## Why It Matters

The original demo already proved bounded manuscript assembly.

This refresh makes the review flow clearer:

1. inspect the input packet manifest
2. inspect the reviewer brief and abstract
3. inspect the draft and claim matrix
4. inspect revision requests and role outputs
5. inspect runtime evidence

That gives Paper Sonata a stronger reviewer-facing and public-facing package
without widening the underlying workflow into something less truthful.
