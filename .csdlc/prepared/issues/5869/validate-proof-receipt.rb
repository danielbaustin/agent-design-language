#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../5862/proof-receipt-contract"

Wp04ProofReceiptContract.validate(
  issue: 5869,
  wp: "WP-04.07",
  paths: ["adl-runtime/src/distributed/lease.rs","adl-runtime/tests/distributed_lease.rs"],
  test: "distributed_lease",
  platforms: []
)
