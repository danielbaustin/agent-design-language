#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../5862/proof-receipt-contract"

Wp04ProofReceiptContract.validate(
  issue: 5871,
  wp: "WP-04.09",
  paths: ["adl-runtime/src/distributed/capability_advertisement.rs","adl-runtime/tests/distributed_capability_advertisement.rs"],
  test: "distributed_capability_advertisement",
  platforms: []
)
