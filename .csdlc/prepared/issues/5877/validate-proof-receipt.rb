#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../5862/proof-receipt-contract"

Wp04ProofReceiptContract.validate(
  issue: 5877,
  wp: "WP-04.15",
  paths: ["adl-runtime/src/distributed/projection.rs","adl-runtime/tests/distributed_projection.rs","docs/api/runtime-v3/v1/distributed.openapi.json"],
  test: "distributed_projection",
  platforms: []
)
