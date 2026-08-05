#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../5862/proof-receipt-contract"

Wp04ProofReceiptContract.validate(
  issue: 5863,
  wp: "WP-04.01",
  paths: ["adl-runtime/src/distributed/identity.rs","adl-runtime/tests/distributed_identity.rs"],
  test: "distributed_identity",
  platforms: []
)
