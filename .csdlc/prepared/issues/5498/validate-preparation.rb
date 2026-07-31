#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

root = Pathname.new(__dir__).join("../../../..").cleanpath
issue_dir = root.join(".csdlc", "issues", "5498")
prepared = root.join(".csdlc", "prepared", "issues", "5498")

required = %w[sip stp spp vpp srp sor].map { |card| issue_dir.join("cards", "#{card}.md") }
required += [prepared.join("design.md"), prepared.join("diagram.mmd"), prepared.join("planned-path-sets.json"), prepared.join("check-dependencies.rb"), prepared.join("run-typed-doctor.sh"), prepared.join("validate-task-adapter.sh")]
missing = required.reject(&:file?)
abort("missing preparation artifacts: #{missing.join(', ')}") unless missing.empty?

record = JSON.parse(issue_dir.join("index.json").read)
abort("wrong issue") unless record["issue"] == 5498
abort("preparation must remain bound") unless record["phase"] == "bound"
abort("preparation must not publish") unless record["publication"].nil?
abort("preparation must not close out") unless record["terminal"].nil?

claim = record.fetch("claim")
expected_paths = [".csdlc/issues/5498", ".csdlc/locks/5498.lock", ".csdlc/prepared/issues/5498"]
abort("unexpected protected paths") unless claim.fetch("protected_paths").sort == expected_paths.sort
abort("claim grants product authority") if claim.fetch("protected_paths").any? { |path| path.start_with?("adl-v2/") }

text = required.select { |path| %w[.md .mmd].include?(path.extname) }.map(&:read).join("\n")
%w[#5499 #5349 #4760 #5500 #5502 serde tokio futures blake3 thiserror secrecy 2,500 600 transcript idempotency].each do |needle|
  abort("missing preparation contract term #{needle}") unless text.include?(needle)
end

path_sets = JSON.parse(prepared.join("planned-path-sets.json").read).fetch("issues")
abort("adjacent path inventory is incomplete") unless path_sets.keys.sort == %w[5498 5499 5500 5502]
raw_paths = path_sets.values.flatten
raw_paths.each do |path|
  pathname = Pathname.new(path)
  normalized_path = pathname.cleanpath.to_s.delete_suffix("/")
  abort("adjacent path is not repository-relative canonical: #{path}") if pathname.absolute? || path.split("/").include?("..") || path != normalized_path || normalized_path == "."
end
normalized = raw_paths.map { |path| Pathname.new(path).cleanpath.to_s.delete_suffix("/") }
abort("adjacent path inventory contains duplicate aliases") unless normalized.uniq.length == normalized.length
normalized.combination(2) do |left, right|
  abort("adjacent planned paths overlap: #{left} / #{right}") if left.start_with?("#{right}/") || right.start_with?("#{left}/")
end
confirmations = JSON.parse(prepared.join("planned-path-sets.json").read).fetch("confirmations")
abort("adjacent owner confirmation inventory is incomplete") unless confirmations.keys.sort == %w[5499 5500 5502]
abort("preparation must retain truthful pending confirmations") unless confirmations.values.all? { |state| %w[pending confirmed].include?(state) }

stdout, _stderr, status = Open3.capture3("ruby", prepared.join("check-dependencies.rb").to_s, chdir: root.to_s)
dependency = JSON.parse(stdout)
unless status.success? || (status.exitstatus == 3 && dependency["status"] == "waiting")
  abort("dependency gate did not return ready or truthful waiting")
end

changed = Open3.capture3("git", "diff", "--name-only", "origin/main...HEAD", chdir: root.to_s).first.lines.map(&:strip)
bad = changed.reject { |path| path.start_with?(".csdlc/issues/5498/", ".csdlc/prepared/issues/5498/") || path == ".csdlc/locks/5498.lock" }
abort("product or out-of-scope changes present: #{bad.join(', ')}") unless bad.empty?

puts JSON.pretty_generate(status: "pass", phase: record["phase"], cards: 6, dependency_status: dependency["status"], product_changes: 0)
