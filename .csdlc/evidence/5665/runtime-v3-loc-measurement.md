# Issue 5665 Physical LoC Measurement

Baseline: `main`

Measured command:

```text
git diff --numstat main | awk '{add+=$1; del+=$2} END {print add, del, add-del}'
```

Observed after staging the exact issue payload:

```text
additions=4795 deletions=5347 net=-552
```

Retired executable Runtime v2 / retained-proof wrappers:

```text
adl/src/bin/run_wp12_acip_websocket_transport_proof.rs main=241 current=13
adl/src/bin/run_v0916_acip_aee_memory_integration.rs main=1418 current=13
adl/src/bin/run_v0916_integrated_runtime_soak.rs main=1635 current=13
adl/src/bin/run_v0916_runtime_failure_injection.rs main=1017 current=13
adl/src/bin/run_v0917_integrated_resilience_failure_injection.rs main=1015 current=13
```

Wrapper total: `main=5326`, `current=65`, `net=-5261`.

The retained historical packets remain in `docs/milestones/v0.91.7/review/runtime/`.
The executable generators now fail closed and point to the #5665 Runtime v3 API
WSS proof.
