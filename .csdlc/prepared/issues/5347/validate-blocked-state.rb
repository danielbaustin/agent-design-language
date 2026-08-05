#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

ROOT = File.expand_path("../../../..", __dir__)
RUNNER = File.join(__dir__, "run-validation-lane.rb")
DEPENDENCIES = File.join(__dir__, "check-dependencies.rb")
GATES = [
  ["dependency-terminal-gate", ["ruby", DEPENDENCIES]],
  ["manifest-disjointness", ["ruby", RUNNER, "manifest-disjointness"]],
  ["owner-and-consumer-proof", ["ruby", RUNNER, "owner-and-consumer-proof"]],
  ["deletion-budgets-and-evidence", ["ruby", RUNNER, "deletion-budgets-and-evidence"]],
  ["post-deletion-exact", ["ruby", RUNNER, "post-deletion-exact"]]
].freeze

unexpected = GATES.each_with_object([]) do |(name, argv), found|
  _out, _err, status = Open3.capture3(*argv, chdir: ROOT)
  found << name if status.success?
end
unless unexpected.empty?
  warn("#5347 preparation is not fail-closed; unexpectedly green gates: #{unexpected.join(', ')}")
  exit(1)
end

puts(JSON.generate(schema: "adl.wp13.blocked_admission_proof.v1", issue: 5347, status: "pass", blocked_gates: GATES.map(&:first)))
