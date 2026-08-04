#!/usr/bin/env ruby
# frozen_string_literal: true

require "open3"

mode = ARGV.fetch(0, "--draft")
abort("usage: validate_dependency_ancestry.rb --draft|--final") unless %w[--draft --final].include?(mode)

root = File.expand_path("../../../..", __dir__)
handoff_path = File.join(root, "docs", "milestones", "v0.91.8", "handoff", "issue-5352-v092-consumption-handoff.md")
handoff = File.read(handoff_path)
baseline_match = handoff.match(/Integrated baseline: `origin\/main` at `([0-9a-f]{40})`/)
abort("handoff omits exact integrated baseline") unless baseline_match
baseline = baseline_match[1]
origin_main, status = Open3.capture2("git", "-C", root, "rev-parse", "origin/main")
abort("cannot resolve origin/main") unless status.success?
abort("recorded baseline #{baseline} differs from origin/main #{origin_main.strip}") unless origin_main.strip == baseline
required = {
  "WP-14A" => "72fbf30c74a5193ea41f042c76c5986a48e59d6c",
  "C-SDLC v2" => "fc75f4fc697262f89f99461679a406be0b4b3775",
  "Runtime v3" => "f7258b07e9da414bfee518f0c89a76071bc03ee8",
  "#4758" => "038f718c377549db21df3a1eb08402867beb2cd5",
  "#4759" => "471db0c35dc34c2497682993378948481bdfa213",
  "#4760" => "d3dbfb31ba4bd53f4166ee5e09da2a8b9f89968e",
  "#4761" => "97d4036e0b5c21786d13cd1301b33038d95e3b98",
  "#4762" => "021be8e33b486d9b66886ff299c20607ed8a071a",
  "#4763" => "d2b19b3aba092aff871b315d60590731e730cb4a",
  "#5007" => "1bd6f73b1c449ffd132ad9a34c739e16c39186c2",
  "#5107" => "b77d020c5c5274e7b64b6ef8f36eed888f34fb4c",
  "#5558 / PR #5749 merge" => "c34f0c9412495039a6374f7ce88fa39e34bb5042"
}

required.each do |name, revision|
  system("git", "merge-base", "--is-ancestor", revision, baseline, chdir: root)
  abort("missing ancestry: #{name} #{revision} -> #{baseline}") unless $?.success?
end

wp20 = File.read(File.join(root, "docs", "milestones", "v0.91.8", "WP_ISSUE_WAVE_v0.91.8.yaml"))
abort("WP-21 dependency on WP-20 is missing") unless wp20.match?(/wp: WP-21.*?depends_on: \["WP-20"\]/m)

observed_head = "033b28cffa6bdf191b1d013aa5a730ce7b10d9df"
system("git", "merge-base", "--is-ancestor", observed_head, baseline, chdir: root)
puts "INFO #5558 branch head #{observed_head} is #{ $?.success? ? '' : 'not ' }ancestral; PR #5749 merge commit is the publication gate"
puts "PASS dependency_ancestry mode=#{mode.delete_prefix('--')} required=#{required.length} baseline=#{baseline}"
