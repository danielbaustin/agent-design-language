#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"

issue = "4758"
root = File.expand_path("../../../..", __dir__)
base = File.join(root, ".csdlc", "issues", issue)
prepared = File.join(root, ".csdlc", "prepared", "issues", issue)

required = [
  File.join(base, "index.json"),
  File.join(base, "cards", "sip.md"),
  File.join(base, "cards", "stp.md"),
  File.join(base, "cards", "spp.md"),
  File.join(base, "cards", "vpp.md"),
  File.join(base, "cards", "srp.md"),
  File.join(base, "cards", "sor.md"),
  File.join(prepared, "design.md"),
  File.join(prepared, "diagram.mmd"),
  File.join(prepared, "launch-readiness-preparation.v1.md"),
  File.join(prepared, "prep-review.v1.md")
]

missing = required.reject { |path| File.file?(path) }
abort("missing preparation files: #{missing.join(", ")}") unless missing.empty?

index = JSON.parse(File.read(File.join(base, "index.json")))
abort("wrong issue") unless index["issue"] == issue.to_i
abort("unexpected phase") unless index["phase"] == "bound"
abort("preparation claim must remain deferred") unless index["claim"].nil?
abort("implementation evidence present") unless index["phase"] != "implemented" && index["publication"].nil?

text = required.grep(/\.md$/).map { |path| File.read(path) }.join("\n")
%w[#5384 #5363 #5362 #5352 #5335 launch-readiness origin/main ancestry COTS Rollback No-Deferral].each do |needle|
  abort("missing required gate text: #{needle}") unless text.include?(needle)
end

contract = File.read(File.join(prepared, "launch-readiness-preparation.v1.md"))
%w[SIP STP SPP VPP SRP SOR].each do |card|
  abort("missing six-card contract section: #{card}") unless contract.include?("### #{card}")
end
abort("missing WP-21 identity correction") unless contract.include?("[v0.91.8][WP-21][launch]")
abort("missing issue-local artifact root") unless contract.include?(".csdlc/evidence/4758/launch-readiness/")
abort("missing deferred claim boundary") unless contract.include?("claim acquisition is deferred")
abort("missing LoC budget") unless contract.include?("## LoC, Time, And Token Budgets")
abort("missing no-deferral gate") unless contract.include?("defer_reason` must remain null")

puts "issue #{issue} preparation contract OK; execution claim deferred"
