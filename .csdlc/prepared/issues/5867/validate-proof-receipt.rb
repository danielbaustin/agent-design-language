#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../5862/proof-receipt-contract"

Wp04ProofReceiptContract.validate(
  issue: 5867,
  wp: "WP-04.05",
  paths: ["adl-runtime/src/distributed/membership.rs","adl-runtime/tests/distributed_membership.rs"],
  test: "distributed_membership",
  platforms: []
)
