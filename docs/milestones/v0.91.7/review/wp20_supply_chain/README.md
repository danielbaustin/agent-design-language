# WP-20 Supply-Chain Proof

`CSDLC_V2_SUPPLY_CHAIN_PROOF_5546.json` is the retained machine-readable
result for issue #5546.

Generate or refresh it without network access:

```text
bash adl/tools/validate_csdlc_v2_supply_chain.sh \
  docs/milestones/v0.91.7/review/wp20_supply_chain/CSDLC_V2_SUPPLY_CHAIN_PROOF_5546.json
```

The proof currently establishes the locked `csdlc-v2` metadata and lockfile
identity, and records the declared MSRV (`1.85`). Advisory, license-policy,
SBOM, and local MSRV execution are explicitly marked unavailable when the
corresponding approved tool or toolchain is absent. `partial_with_explicit_dispositions`
and `not_ready_for_supply_chain_certification` are intentional fail-closed
release-review outcomes, not green substitutes.
