- **High — preparation validation cannot pass with a dormant claim.** [validate_preparation.rb](/Volumes/FastWork/adl-wp-4759/.csdlc/prepared/issues/4759/validate_preparation.rb:29) unconditionally aborts unless `index["claim"]` is a hash. This incorrectly makes execution-time claim acquisition a preparation gate and contradicts the operator correction.

- **Medium — recorded design and diagram digests are stale.** [spp.values.json](/Volumes/FastWork/adl-wp-4759/.csdlc/issues/4759/cards/spp.values.json:83) and [vpp.values.json](/Volumes/FastWork/adl-wp-4759/.csdlc/issues/4759/cards/vpp.values.json:43), plus their rendered cards, record digests that differ from the current files:

  - `design.md`: recorded `030180…`, actual `d65f25…`
  - `diagram.mmd`: recorded `24b888…`, actual `7c50d0…`

- **Medium — “exactly one concern” is not represented unambiguously.** [spp.md](/Volumes/FastWork/adl-wp-4759/.csdlc/issues/4759/cards/spp.md:71) contains three risks and four stop conditions, while [design.md](/Volumes/FastWork/adl-wp-4759/.csdlc/prepared/issues/4759/design.md:21) has one blocker. There is no explicit concern field identifying one canonical concern.

- **Low — PVF budgets disagree across truth surfaces.** [vpp.values.json](/Volumes/FastWork/adl-wp-4759/.csdlc/issues/4759/cards/vpp.values.json:29) assigns the lane 30 seconds/1,000 tokens and totals 1,200 seconds/10,000 tokens, while [validate.json](/Volumes/FastWork/adl-wp-4759/.csdlc/prepared/issues/4759/validate.json:10) declares 30 seconds/500 tokens. Network denial, no credentials, deterministic local Ruby execution, and the Ruby standard-library dependency are otherwise truthful; no separate COTS dependency is claimed.

Disposition: **preparation not ready**. The six issue-specific cards exist and are coherent with the design/diagram. Dependency truth is correct: #5384 merge plus current `origin/main` ancestry is the execution gate; #5335 and receipts remain audit-only. JSON and Ruby syntax pass. Correct the dormant-claim validator requirement, refresh the digests, and make the single concern and PVF budgets canonical before accepting preparation validation. No files were edited, and the validator itself was not executed.

## Fix Dispositions

- **Dormant claim gate: fixed.** The preparation validator now accepts a deferred execution claim and validates issue-local protected paths if a claim is present.
- **Design and diagram digests: no fix required.** The review compared SHA-256 output with BLAKE3 fields. Typed doctor classified the record as `block` only for the dormant claim, not corrupt or projection-stale, so the recorded BLAKE3 digests remain authoritative.
- **Single concern: fixed.** `preparation-contract.json` declares exactly one concern: live #5384 merge plus current `origin/main` ancestry before execution. Risks and stop conditions remain safeguards for that concern.
- **PVF budget: fixed.** `validate.json` now matches the VPP lane at 30 seconds and 1,000 tokens. The larger VPP totals describe future execution planning and do not expand this preparation lane.

Final disposition: **findings resolved; preparation validation may run.** This report is preparation evidence only and does not populate lifecycle SRP review truth or authorize implementation, publication, merge, or closeout.
