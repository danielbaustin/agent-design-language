#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../5862/proof-receipt-contract"

Wp04ProofReceiptContract.validate(
  issue: 5870,
  wp: "WP-04.08",
  paths: ["adl-runtime/src/distributed/fencing.rs","adl-runtime/tests/distributed_fencing.rs"],
  test: "distributed_fencing",
  platforms: []
)
