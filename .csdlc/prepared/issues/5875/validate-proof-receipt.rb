#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../5862/proof-receipt-contract"

Wp04ProofReceiptContract.validate(
  issue: 5875,
  wp: "WP-04.13",
  paths: ["adl-runtime/src/distributed/migration.rs","adl-runtime/tests/distributed_migration.rs"],
  test: "distributed_migration",
  platforms: []
)
