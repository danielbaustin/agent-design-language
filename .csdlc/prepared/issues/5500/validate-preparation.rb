#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

root = Pathname.new(__dir__).join("../../../..").cleanpath
issue_dir = root.join(".csdlc", "issues", "5500")
prepared = root.join(".csdlc", "prepared", "issues", "5500")

required = %w[sip stp spp vpp srp sor].map { |card| issue_dir.join("cards", "#{card}.md") }
required += %w[design.md diagram.mmd check-dependencies.rb check-product-paths.rb product-paths.json validate-dashboard.sh check-diagram.sh run-typed-doctor.rb check-preparation-diff.sh validate-preparation.json].map { |name| prepared.join(name) }
missing = required.reject(&:file?)
abort("missing preparation artifacts: #{missing.join(', ')}") unless missing.empty?

record = JSON.parse(issue_dir.join("index.json").read)
abort("wrong issue") unless record["issue"] == 5500
abort("preparation must remain bound") unless record["phase"] == "bound"
design_review = record["design_review"]
abort("design must be independently approved") unless design_review.is_a?(Hash) && design_review.key?("approved")
abort("preparation must not publish") unless record["publication"].nil?
abort("preparation must not close out") unless record["terminal"].nil?

claim = record.fetch("claim")
expected_paths = [".csdlc/issues/5500", ".csdlc/locks/5500.lock", ".csdlc/prepared/issues/5500"]
abort("unexpected protected paths") unless claim.fetch("protected_paths").sort == expected_paths.sort
abort("claim grants product authority") if claim.fetch("protected_paths").any? { |path| path.start_with?("docs/tooling/", "adl/tools/") }

text = required.select { |path| %w[.md .mmd].include?(path.extname) }.map(&:read).join("\n")
%w[#5498 #5349 #5502 read-only HTTPS stale unknown non-authoritative audit-only 2,000 3,600].each do |needle|
  abort("missing preparation contract term #{needle}") unless text.include?(needle)
end
abort("design creates a second dashboard") unless text.include?("No second dashboard framework")
lower_text = text.downcase
abort("COTS posture does not default to zero new dependencies") unless lower_text.include?("zero new direct dependencies by default")
abort("browser-platform COTS posture is missing") unless lower_text.include?("platform apis")

cards = %w[sip stp spp vpp srp sor].to_h do |card|
  [card, JSON.parse(issue_dir.join("cards", "#{card}.values.json").read)]
end
abort("wrong native template generation") unless cards.values.all? { |card| card.dig("identity", "template_version") == "1.0.0" }
abort("complete validation budget drift") unless cards.dig("vpp", "content", "values", "planned_validation_seconds") == 3600
lanes = cards.dig("vpp", "content", "values", "lanes")
abort("focused dashboard lane budget drift") unless lanes.any? { |lane| lane["lane"] == "dashboard-contract" && lane["budget_seconds"] == 120 }

pvf = JSON.parse(prepared.join("validate-preparation.json").read)
abort("wrong PVF schema") unless pvf.dig("manifest", "schema") == "csdlc.pvf.manifest.v1"
abort("PVF network must be denied") unless pvf.dig("selection", "allow_network") == false
abort("future dashboard lane selected during preparation") if pvf.dig("selection", "requested_lanes").include?("dashboard-contract")

path_out, path_err, path_status = Open3.capture3("ruby", prepared.join("check-product-paths.rb").to_s, chdir: root.to_s)
abort("product path proof failed: #{path_out}#{path_err}") unless path_status.success?

stdout, _stderr, status = Open3.capture3("ruby", prepared.join("check-dependencies.rb").to_s, chdir: root.to_s)
dependency = JSON.parse(stdout)
unless status.success? || (status.exitstatus == 3 && dependency["status"] == "waiting")
  abort("dependency gate did not return ready or truthful waiting")
end
abort("dependency predicate drift") unless dependency["predicate"] == "live merge on origin/main plus ancestry to HEAD"
abort("typed closeout receipts must remain audit-only") unless dependency.fetch("audit_only").include?("typed closeout receipts")

_diff_out, diff_err, diff_status = Open3.capture3("bash", prepared.join("check-preparation-diff.sh").to_s, chdir: root.to_s)
abort("diff hygiene failed: #{diff_err}") unless diff_status.success?

puts JSON.pretty_generate(status: "pass", phase: record["phase"], cards: 6, dependency_status: dependency["status"], product_path_proof: "pass", product_changes: 0)
