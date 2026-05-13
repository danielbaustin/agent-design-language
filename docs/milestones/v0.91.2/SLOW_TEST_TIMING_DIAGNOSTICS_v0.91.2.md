# Slow Test Timing Diagnostics v0.91.2

## Purpose

Use `adl/tools/summarize_nextest_timings.py` when a nextest lane is slow enough
that reading the raw log by hand becomes wasteful. The tool parses ordinary
`PASS [   N.NNNs] ...` rows plus `SLOW [> N.NNNs] ...` threshold markers and
turns them into a reviewable hotspot report.

The log path in the commands below is intentionally an operator-supplied,
ignored artifact. In a clean checkout, first save a captured nextest log at that
path or replace the argument with the path to the captured log under review.

This diagnostic path is evidence only. It does not replace passing tests,
coverage gates, release proof, or authoritative slow-proof obligations.

## Operator Command

```bash
python3 adl/tools/summarize_nextest_timings.py .adl/docs/TBD/test-logs.txt --top 12 --min-seconds 45
```

Use `--format json` when a follow-on tool or issue planner needs structured
data:

```bash
python3 adl/tools/summarize_nextest_timings.py .adl/docs/TBD/test-logs.txt --top 12 --min-seconds 45 --format json
```

## Routing Rules

- Create a test-refactor issue when a family has repeated slow completed tests
  with the same setup shape.
- Keep a test in an authoritative slow-proof lane when it is the one
  end-to-end proof path for a feature contract.
- Treat contract-registry/accessor clusters as candidates for shared registry
  proof consolidation or authoritative slow-lane demotion.
- Treat proof-materialization clusters as candidates for explicit-path default
  proof plus root-materialization slow-lane proof.
- Keep broad or ambiguous Rust changes fail-closed; timing diagnostics should
  never narrow a test lane by themselves.

## Example Report From The Supplied #3032 Log

The supplied `.adl/docs/TBD/test-logs.txt` log declares `1,882` tests across
`28` binaries and produced:

- Completed timing rows parsed: `1,881`
- Slow threshold markers parsed: `77`
- Total parsed completed runtime: `7364.644s`
- Threshold markers: `63` over `60s`, `13` over `120s`, and `1` over `180s`

### Top Completed Tests

| Rank | Seconds | Family | Cluster | Test |
| ---: | ---: | --- | --- | --- |
| 1 | 181.281 | `runtime_v2::a2a_adapter_boundary` | `runtime-v2-contract-registry/accessors` | `runtime_v2::tests::a2a_adapter_boundary::runtime_v2_a2a_adapter_boundary_contract_registry_smoke_covers_accessors` |
| 2 | 179.933 | `runtime_v2::theory_of_mind_foundation` | `runtime-v2-contract-registry/accessors` | `runtime_v2::tests::theory_of_mind_foundation::runtime_v2_theory_of_mind_foundation_contract_registry_smoke_covers_accessors` |
| 3 | 178.840 | `runtime_v2::agent_lifecycle_state` | `runtime-v2-contract-registry/accessors` | `runtime_v2::tests::agent_lifecycle_state::runtime_v2_agent_lifecycle_state_contract_registry_accessors_cover_runtime_v2_contracts_module` |
| 4 | 178.825 | `runtime_v2::governed_learning_substrate` | `runtime-v2-contract-registry/accessors` | `runtime_v2::tests::governed_learning_substrate::runtime_v2_governed_learning_substrate_contract_accessors_cover_shared_registry` |
| 5 | 178.801 | `runtime_v2::runtime_inhabitant_integration` | `runtime-v2-contract-registry/accessors` | `runtime_v2::tests::runtime_inhabitant_integration::runtime_v2_runtime_inhabitant_integration_contract_accessors_cover_shared_registry` |
| 6 | 178.785 | `runtime_v2::acip_hardening` | `runtime-v2-contract-registry/accessors` | `runtime_v2::tests::acip_hardening::runtime_v2_acip_hardening_contract_registry_smoke_covers_accessors` |
| 7 | 178.728 | `runtime_v2::citizen_state_substrate` | `runtime-v2-contract-registry/accessors` | `runtime_v2::tests::citizen_state_substrate::runtime_v2_citizen_state_substrate_contract_accessors_cover_shared_registry` |
| 8 | 178.594 | `runtime_v2::memory_identity_architecture` | `runtime-v2-contract-registry/accessors` | `runtime_v2::tests::memory_identity_architecture::runtime_v2_memory_identity_architecture_contract_registry_smoke_covers_accessors` |
| 9 | 178.568 | `runtime_v2::intelligence_metric_architecture` | `runtime-v2-contract-registry/accessors` | `runtime_v2::tests::intelligence_metric_architecture::runtime_v2_intelligence_metric_architecture_contract_accessors_cover_shared_registry` |
| 10 | 163.371 | `runtime_v2::observatory_flagship` | `runtime-v2-other` | `runtime_v2::tests::observatory_flagship::runtime_v2_observatory_flagship_rejects_shape_and_boundary_drift` |
| 11 | 163.157 | `runtime_v2::observatory_flagship` | `runtime-v2-other` | `runtime_v2::tests::observatory_flagship::runtime_v2_observatory_flagship_review_surfaces_are_stable_and_serializable` |
| 12 | 146.979 | `runtime_v2::observatory_flagship` | `runtime-v2-other` | `runtime_v2::tests::observatory_flagship::runtime_v2_observatory_flagship_review_bundle_matches_tracked_artifacts` |

### Hotspot Families

| Rank | Total Seconds | Count | Max Seconds | Family | Routing Hint |
| ---: | ---: | ---: | ---: | --- | --- |
| 1 | 878.262 | 8 | 178.801 | `runtime_v2::runtime_inhabitant_integration` | Repeated family-level setup pattern; consider table/collapse refactor. |
| 2 | 770.216 | 9 | 178.568 | `runtime_v2::intelligence_metric_architecture` | Route to Runtime v2 registry/accessor refactor or authoritative slow-proof consolidation. |
| 3 | 770.212 | 9 | 178.825 | `runtime_v2::governed_learning_substrate` | Route to Runtime v2 registry/accessor refactor or authoritative slow-proof consolidation. |
| 4 | 699.842 | 8 | 179.933 | `runtime_v2::theory_of_mind_foundation` | Route to Runtime v2 registry/accessor refactor or authoritative slow-proof consolidation. |
| 5 | 657.238 | 8 | 181.281 | `runtime_v2::a2a_adapter_boundary` | Repeated family-level setup pattern; consider table/collapse refactor. |
| 6 | 614.812 | 6 | 178.594 | `runtime_v2::memory_identity_architecture` | Repeated family-level setup pattern; consider table/collapse refactor. |

### Hotspot Clusters

| Rank | Total Seconds | Count | Max Seconds | Cluster | Routing Hint |
| ---: | ---: | ---: | ---: | --- | --- |
| 1 | 1860.104 | 12 | 181.281 | `runtime-v2-contract-registry/accessors` | Route to Runtime v2 registry/accessor refactor or authoritative slow-proof consolidation. |
| 2 | 1685.765 | 23 | 163.371 | `runtime-v2-other` | Repeated family-level setup pattern; consider table/collapse refactor. |
| 3 | 1556.955 | 19 | 134.933 | `validation-negative` | Repeated family-level setup pattern; consider table/collapse refactor. |
| 4 | 631.773 | 9 | 100.002 | `golden-fixture` | Repeated family-level setup pattern; consider table/collapse refactor. |
| 5 | 566.277 | 8 | 75.270 | `proof-materialization` | Route to proof-materialization slow-lane review; keep one authoritative root proof where needed. |
| 6 | 537.449 | 7 | 99.923 | `proof-route` | Repeated family-level setup pattern; consider table/collapse refactor. |

## Validation

Focused validation for this diagnostic surface in a clean checkout:

```bash
python3 -m py_compile adl/tools/summarize_nextest_timings.py
bash adl/tools/test_summarize_nextest_timings.sh
```

To reproduce the #3032 example report, provide the captured log locally first,
then run:

```bash
python3 adl/tools/summarize_nextest_timings.py .adl/docs/TBD/test-logs.txt --top 12 --min-seconds 45
```

Do not run full nextest merely to prove the parser. Use an existing captured log
or a small fixture unless the issue being diagnosed independently requires a
real test rerun.
