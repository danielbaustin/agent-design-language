#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

root = File.expand_path("../../../../", __dir__)
issue_dir = File.join(root, ".csdlc/issues/5526")
prepared_dir = File.join(root, ".csdlc/prepared/issues/5526")

required = [
  ".csdlc/issues/5526/index.json",
  ".csdlc/issues/5526/cards/sip.md",
  ".csdlc/issues/5526/cards/stp.md",
  ".csdlc/issues/5526/cards/spp.md",
  ".csdlc/issues/5526/cards/vpp.md",
  ".csdlc/issues/5526/cards/srp.md",
  ".csdlc/issues/5526/cards/sor.md",
  ".csdlc/prepared/issues/5526/design.md",
  ".csdlc/prepared/issues/5526/diagram.mmd"
]

missing = required.reject { |path| File.file?(File.join(root, path)) }
abort("missing required prep files: #{missing.join(", ")}") unless missing.empty?

index = JSON.parse(File.read(File.join(issue_dir, "index.json")))
abort("issue mismatch") unless index["issue"] == 5526
abort("branch mismatch") unless index.dig("claim", "branch") == "codex/5526-v0918-provider-expansion"
abort("phase must remain bound for preparation") unless index["phase"] == "bound"

combined = Dir[File.join(issue_dir, "cards/*.md")]
  .concat(Dir[File.join(prepared_dir, "*.{md,mmd}")])
  .map { |path| File.read(path) }
  .join("\n")

required_terms = [
  "live WP-09 merge",
  "ancestry",
  "receipts are non-blocking audit evidence",
  "No AWS",
  "No provider credential access",
  "One exact pre-PR review"
]

missing_terms = required_terms.reject { |term| combined.include?(term) }
abort("missing required packet terms: #{missing_terms.join(", ")}") unless missing_terms.empty?

puts "issue 5526 preparation packet OK"
