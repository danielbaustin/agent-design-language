#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../5862/proof-receipt-contract"

Wp04ProofReceiptContract.validate(
  issue: 5873,
  wp: "WP-04.11",
  paths: ["adl-runtime/src/distributed/placement.rs","adl-runtime/tests/distributed_placement.rs"],
  test: "distributed_placement",
  platforms: []
)
