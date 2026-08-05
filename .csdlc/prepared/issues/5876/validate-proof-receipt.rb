#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../5862/proof-receipt-contract"

Wp04ProofReceiptContract.validate(
  issue: 5876,
  wp: "WP-04.14",
  paths: ["adl-runtime/src/distributed/recovery.rs","adl-runtime/tests/distributed_recovery.rs"],
  test: "distributed_recovery",
  platforms: []
)
