#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../5862/proof-receipt-contract"

Wp04ProofReceiptContract.validate(
  issue: 5866,
  wp: "WP-04.04",
  paths: ["adl-runtime/src/distributed/discovery.rs","adl-runtime/tests/distributed_discovery.rs"],
  test: "distributed_discovery",
  platforms: []
)
