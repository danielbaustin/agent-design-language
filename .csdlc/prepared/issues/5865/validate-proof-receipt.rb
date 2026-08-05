#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../5862/proof-receipt-contract"

Wp04ProofReceiptContract.validate(
  issue: 5865,
  wp: "WP-04.03",
  paths: ["adl-runtime/src/distributed/transport.rs","adl-runtime/tests/distributed_transport.rs","adl-runtime/Cargo.toml","adl-runtime/Cargo.lock"],
  test: "distributed_transport",
  platforms: []
)
