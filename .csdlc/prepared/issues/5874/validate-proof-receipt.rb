#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../5862/proof-receipt-contract"

Wp04ProofReceiptContract.validate(
  issue: 5874,
  wp: "WP-04.12",
  paths: ["adl-runtime/src/distributed/snapshot_catalog.rs","adl-runtime/tests/distributed_snapshot_catalog.rs"],
  test: "distributed_snapshot_catalog",
  platforms: []
)
