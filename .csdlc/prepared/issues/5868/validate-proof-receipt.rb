#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../5862/proof-receipt-contract"

Wp04ProofReceiptContract.validate(
  issue: 5868,
  wp: "WP-04.06",
  paths: ["adl-runtime/src/distributed/failure_detection.rs","adl-runtime/tests/distributed_failure_detection.rs"],
  test: "distributed_failure_detection",
  platforms: []
)
