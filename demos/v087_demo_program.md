# v0.87 Demo Program

This is the bounded, truthful demo runbook for the `v0.87` substrate milestone.

Run from repository root.

## Fastest full review path

```bash
bash adl/tools/demo_v087_suite.sh
```

That populates the canonical demo artifact roots under:

- `artifacts/v087/trace_v1/`
- `artifacts/v087/provider_portability/`
- `artifacts/v087/shared_obsmem/`
- `artifacts/v087/skills/`
- `artifacts/v087/control_plane/`
- `artifacts/v087/reviewer_package/`

## Per-demo entrypoints

### D1 Trace v1 substrate truth

```bash
bash adl/tools/demo_v087_trace_truth.sh
```

Primary proof surface:
- `artifacts/v087/trace_v1/runs/v0-6-hitl-no-pause-demo/logs/trace_v1.json`

### D2 Provider portability substrate

```bash
bash adl/tools/demo_v087_provider_portability.sh
```

Primary proof surface:
- `artifacts/v087/provider_portability/provider_substrate_manifest.v1.json`

### D3 Shared ObsMem foundation coherence

```bash
bash adl/tools/demo_v087_shared_obsmem.sh
```

Primary proof surface:
- `artifacts/v087/shared_obsmem/demo-f-obsmem-retrieval/obsmem_retrieval_result.json`

### D4 Operational skills substrate

```bash
bash adl/tools/demo_v087_operational_skills.sh
```

Primary proof surface:
- `artifacts/v087/skills/skills_inventory.txt`

### D5 Control-plane / PR tooling substrate

```bash
bash adl/tools/demo_v087_control_plane.sh
```

Primary proof surface:
- `artifacts/v087/control_plane/runs/v0-4-demo-deterministic-replay/run_summary.json`

### D6 Reviewer-facing substrate package

```bash
bash adl/tools/demo_v087_reviewer_package.sh
```

Primary proof surface:
- `artifacts/v087/reviewer_package/reviewer_package_manifest.txt`

## Truth boundaries

- These demos are repo-local and bounded.
- They use fixture-backed or mock-provider-backed commands where that keeps the
  proof deterministic and runnable without network dependence.
- They prove the `v0.87` substrate surfaces that actually exist now; they do
  not claim later identity, society, routing, or governance capabilities.
