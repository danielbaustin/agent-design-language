#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "date"
require "yaml"
require "shellwords"

mode = ARGV.fetch(0, "--draft")
abort("usage: validate_handoff.rb --draft|--final") unless %w[--draft --final].include?(mode)

root = File.expand_path("../../../..", __dir__)
draft = File.join(__dir__, "exact-revision-handoff-draft.md")
final = File.join(root, "docs", "milestones", "v0.91.8", "handoff", "issue-5352-v092-consumption-handoff.md")
target = mode == "--final" ? final : draft
abort("missing handoff ledger: #{target}") unless File.file?(target)

text = File.read(target)
required_headings = [
  "## Accepted Platform Revisions",
  "## WP-21 Integration Matrix",
  "## WP-20 Predecessor Gate",
  "## Stable Contracts And Schemas",
  "## Rollback Boundaries",
  "## Residual Risks",
  "## Explicit Non-Claims"
]
required_headings.each { |heading| abort("missing heading: #{heading}") unless text.include?(heading) }

baseline_match = text.match(/Integrated baseline: `origin\/main` at `([0-9a-f]{40})`/)
abort("missing exact integrated baseline") unless baseline_match
recorded_baseline = baseline_match[1]
live_baseline = `git -C #{Shellwords.escape(root)} rev-parse origin/main`.strip
abort("cannot resolve origin/main") unless $?.success?
abort("recorded baseline #{recorded_baseline} differs from origin/main #{live_baseline}") unless recorded_baseline == live_baseline

expected_rows = [
  ["WP-14A platform", "#5384 / #5726", "71e3b70b8f0d235d768ced0383074345547811d4", "72fbf30c74a5193ea41f042c76c5986a48e59d6c"],
  ["C-SDLC v2", "#5358 / #5606", "e048230245b1ad101c8056678123a2747faa4b60", "fc75f4fc697262f89f99461679a406be0b4b3775"],
  ["Runtime v3", "#5361 / #5650", "f7fc71421f4bcf70039b910c9b88b538bb111400", "f7258b07e9da414bfee518f0c89a76071bc03ee8"],
  ["ADL v2 soak / rollback", "#5344 / #5703", "141dfa20ccc3753060687259ad933397331df9c7", "d4825d4be9ed14ed6060dd33cbdafe5eaa5efcd2"],
  ["ADL v2 reversible default", "#5343 / #5704", "e4bbc988cad682cbb2ff8d24085e1a99bccec1ce", "e1b6a34e4763a79d1c40c641e64c0c061a0aa96c"],
  ["Launch readiness", "#4758 / #5739", "c9b5c625ccfb17b1a75fd3a1a93f4810baf4a3e2", "038f718c377549db21df3a1eb08402867beb2cd5"],
  ["Activation bridge", "#4759 / #5738", "32957a21a3fc3fc8a8efb3c3c6ad198db9b0ddd7", "471db0c35dc34c2497682993378948481bdfa213"],
  ["Memory Palace MVP", "#4760 / #5740", "9719252262913351144a20adf0affb7ed4b5480d", "d3dbfb31ba4bd53f4166ee5e09da2a8b9f89968e"],
  ["Capability envelope", "#4761 / #5741", "8c3ef0336570238d26eab0fd49a9a2ff9c1a0c09", "97d4036e0b5c21786d13cd1301b33038d95e3b98"],
  ["Birth witness package", "#4762 / #5744", "d736baca1c82c6ca9b770678ff2c04ce44458fc9", "021be8e33b486d9b66886ff299c20607ed8a071a"],
  ["Birthday / launch docs", "#4763 / #5734", "313268e09b8d9906f61b0e12ac05cce4deea1e3c", "d2b19b3aba092aff871b315d60590731e730cb4a"],
  ["Memory Palace ADR", "#5007 / #5743", "426d0a53fb2b7b0be571b236ca5d0a248b32e1f8", "1bd6f73b1c449ffd132ad9a34c739e16c39186c2"],
  ["Adaptive Learning queue", "#5107 / #5742", "8bf36c9d214a54212e7c483fb29872e9be9e92b3", "b77d020c5c5274e7b64b6ef8f36eed888f34fb4c"]
]
table_rows = text.lines.each_with_object([]) do |line, rows|
  next unless line.start_with?("|")
  rows << line.split("|").map { |cell| cell.strip.delete_prefix("`").delete_suffix("`") }.reject(&:empty?)
end
expected_rows.each do |expected|
  matching = table_rows.select { |row| row.first(4) == expected }
  abort("missing or substituted handoff row: #{expected.join(' | ')}") unless matching.length == 1
end

required_tokens = %w[
  72fbf30c74a5193ea41f042c76c5986a48e59d6c
  fc75f4fc697262f89f99461679a406be0b4b3775
  f7258b07e9da414bfee518f0c89a76071bc03ee8
  038f718c377549db21df3a1eb08402867beb2cd5
  471db0c35dc34c2497682993378948481bdfa213
  d3dbfb31ba4bd53f4166ee5e09da2a8b9f89968e
  97d4036e0b5c21786d13cd1301b33038d95e3b98
  021be8e33b486d9b66886ff299c20607ed8a071a
  d2b19b3aba092aff871b315d60590731e730cb4a
  1bd6f73b1c449ffd132ad9a34c739e16c39186c2
  b77d020c5c5274e7b64b6ef8f36eed888f34fb4c
  2026-08-12T09:04:24Z
  deletion_authorized:
  docs/templates/prompts/current.json
  csdlc-v2/operator/generation-selector.json
  .adl/bin/csdlc-v2/
  #5558
  #5749
  c34f0c9412495039a6374f7ce88fa39e34bb5042
]
required_tokens.each { |token| abort("missing required token: #{token}") unless text.include?(token) }

referenced_paths = text.scan(/`([^`]+)`/).flatten.select do |value|
  value.start_with?(".csdlc/", "adl/", "csdlc-v2/", "docs/") && !value.include?("{")
end
referenced_paths.each do |path|
  next if path.start_with?(".adl/bin/")
  next if path == "docs/milestones/v0.91.8/handoff/issue-5352-v092-consumption-handoff.md" && mode == "--draft"
  abort("missing referenced path: #{path}") unless File.exist?(File.join(root, path))
end

json_contracts = %w[
  .csdlc/evidence/5384/platform-acceptance-ledger.v1.json
  .csdlc/evidence/5361/acceptance-proof-summary.json
  csdlc-v2/operator/generation-selector.json
  csdlc-v2/operator/native-card-shape.json
  docs/templates/prompts/current.json
  docs/milestones/v0.91.8/evidence/wp12/cutover-5343/report.json
]
json_contracts.each do |path|
  JSON.parse(File.read(File.join(root, path)))
rescue JSON::ParserError => error
  abort("invalid JSON contract #{path}: #{error.message}")
end

wave_path = File.join(root, "docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml")
YAML.safe_load(File.read(wave_path), permitted_classes: [Date, Time], aliases: true)

forbidden = [
  "The v0.92 birthday occurred.",
  "Status: production ready",
  "Adaptive Learning runtime is implemented.",
  "Closeout: closed_out",
  "/private/tmp"
]
forbidden.each { |token| abort("forbidden claim: #{token}") if text.include?(token) }

if mode == "--final"
  abort("final ledger still marked draft") if text.include?("Status: issue-local draft")
  abort("final ledger still says #5558 is open") if text.include?("#5558 sunset-guidance repair / PR #5749 | open")
  abort("final ledger still has pending #5558 merge truth") if text.include?("merge revision pending")
  abort("final ledger omits completed #5558 state") unless text.include?("closed, completed")
end

nonblank = text.lines.count { |line| !line.strip.empty? }
abort("handoff ledger exceeds 300 nonblank lines: #{nonblank}") if nonblank > 300
puts "PASS handoff_ledger mode=#{mode.delete_prefix('--')} nonblank_lines=#{nonblank}"
