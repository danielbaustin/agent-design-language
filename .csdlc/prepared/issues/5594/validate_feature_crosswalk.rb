#!/usr/bin/env ruby

require "digest"
require "json"
require_relative "feature_decisions_5594"

root = File.expand_path("../../../..", __dir__)
path = File.join(root, "docs/planning/ADL_FEATURE_LIST.md")
rows = []

File.readlines(path).each_with_index do |line, index|
  next unless line.start_with?("|")

  columns = line.split("|").map(&:strip)[1..-2]
  next unless columns && columns.length == 4
  next if columns[0] == "Feature band" || columns[0].match?(/^-+$/)

  rows << [index + 1, columns]
end

expected_count = 122
expected_digest = "4cf6dcde57bab59523ef715d39552b7f1d9daeac963caae51817c8d88a8ceaaa"
digest = Digest::SHA256.hexdigest(rows.map { |_, row| row.join("\u001f") }.join("\n"))

abort("feature-row count changed: #{rows.length}") unless rows.length == expected_count
abort("feature-row digest changed: #{digest}") unless digest == expected_digest
abort("feature row has an empty field") if rows.any? { |_, row| row.any?(&:empty?) }

names = rows.map { |_, row| row.first }
abort("duplicate feature names") unless names.uniq.length == names.length

counts = Hash.new(0)
artifact_path = File.join(root, "docs/milestones/v0.91.8/feature_preservation_crosswalk_5594.v1.json")
artifact = JSON.parse(File.read(artifact_path))
abort("wrong feature-crosswalk schema") unless artifact["schema"] == "adl.v0918.feature_preservation_crosswalk.v1"
abort("feature-crosswalk source count mismatch") unless artifact["source_row_count"] == rows.length
abort("feature-crosswalk source digest mismatch") unless artifact["source_row_digest"] == digest
entries = artifact.fetch("entries")
abort("feature-crosswalk entry count mismatch") unless entries.length == rows.length
allowed_dispositions = FeatureDecisions5594::GROUPS.values.map { |group| group.fetch(:disposition) }.uniq

source_lines = rows.map(&:first)
decision_lines = FeatureDecisions5594::BY_SOURCE_LINE.keys.sort
abort("feature decisions do not exactly cover source rows") unless decision_lines == source_lines

rows.each_with_index do |(source_line, row), index|
  code = FeatureDecisions5594::BY_SOURCE_LINE.fetch(source_line)
  decision = FeatureDecisions5594::GROUPS.fetch(code)
  name = decision.fetch(:classification)
  owner = decision.fetch(:owner_issues)
  abort("feature row has no owner: #{row.first}") if owner.nil? || Array(owner).empty?
  entry = entries.fetch(index)
  abort("feature-crosswalk row order mismatch: #{row.first}") unless entry["row"] == index + 1 && entry["source_line"] == source_line && entry["feature"] == row[0]
  abort("feature-crosswalk canonical fields mismatch: #{row.first}") unless entry.values_at("canonical_status", "canonical_evidence", "canonical_next_target") == row[1, 3]
  abort("feature-crosswalk class mismatch: #{row.first}") unless entry["classification"] == name
  abort("feature-crosswalk owner mismatch: #{row.first}") unless entry["owner_issues"] == Array(owner)
  abort("feature-crosswalk disposition mismatch: #{row.first}") unless entry["cutover_disposition"] == decision.fetch(:disposition) && allowed_dispositions.include?(entry["cutover_disposition"])
  abort("feature-crosswalk decision basis mismatch: #{row.first}") unless entry["decision_basis"] == decision.fetch(:basis)
  counts[name] += 1
end

abort("not every feature row was classified") unless counts.values.sum == rows.length
puts "feature crosswalk ok rows=#{rows.length} digest=#{digest} classes=#{counts.sort.to_h}"
