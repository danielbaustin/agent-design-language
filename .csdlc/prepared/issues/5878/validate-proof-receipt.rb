#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative "../5862/proof-receipt-contract"

Wp04ProofReceiptContract.validate(
  issue: 5878,
  wp: "WP-04.16",
  paths: ["adl-runtime/src/distributed/mod.rs","adl-runtime/src/lib.rs","adl-runtime/tests/distributed_guardian.rs","adl/tools/validate_v092_distributed_guardian.sh","adl/tools/validate_v092_distributed_native_receipts.rb"],
  test: "distributed_guardian",
  platforms: ["macos","linux","windows"],
  required_commands: [
    ["bash", "adl/tools/validate_v092_distributed_guardian.sh"],
    ["ruby", "adl/tools/validate_v092_distributed_native_receipts.rb"]
  ]
)
