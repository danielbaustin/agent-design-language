#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../5862/proof-receipt-contract"

Wp04ProofReceiptContract.validate(
  issue: 5864,
  wp: "WP-04.02",
  paths: ["adl-runtime/src/distributed/certificates.rs","adl-runtime/tests/distributed_certificates.rs"],
  test: "distributed_certificates",
  platforms: []
)
