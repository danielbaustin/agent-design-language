#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../5862/proof-receipt-contract"

Wp04ProofReceiptContract.validate(
  issue: 5872,
  wp: "WP-04.10",
  paths: ["adl-runtime/src/distributed/resource_weather.rs","adl-runtime/tests/distributed_resource_weather.rs"],
  test: "distributed_resource_weather",
  platforms: []
)
