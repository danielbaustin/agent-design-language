#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"
require "yaml"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
ISSUE = 5804
MILESTONE = ROOT.join("docs/milestones/v0.91.8")
HANDOFF = MILESTONE.join("review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md")
EXPECTED_OPEN_ISSUES = [5348, 5355, 5357, 5359, 5362, 5363, 5595, 5804].freeze
LOCAL_PATH_PATTERNS = [
  %r{/Volumes/FastWork},
  %r{/private/tmp},
  %r{/var/folders/},
  %r{/Users/[^/]+/}
].freeze

def assert(condition, message)
  raise message unless condition
end

def git(*args)
  output, status = Open3.capture2e("git", "-C", ROOT.to_s, *args)
  assert(status.success?, "git #{args.join(' ')} failed: #{output.strip}")
  output.strip
end

def validate_local_links(path)
  path.read.scan(/!?\[[^\]]*\]\(([^)]+)\)/).flatten.each do |raw|
    target = raw.strip.sub(/\A</, "").sub(/>\z/, "").split(/\s+[\"']/, 2).first
    next if target.empty? || target.start_with?("#", "http://", "https://", "mailto:", "data:")

    relative = target.split("#", 2).first
    next if relative.empty?

    resolved = path.dirname.join(relative).cleanpath
    assert(resolved.exist?, "broken local link in #{path.relative_path_from(ROOT)}: #{target}")
  end
end

index = JSON.parse(ROOT.join(".csdlc/issues/#{ISSUE}/index.json").read)
assert(index.fetch("issue") == ISSUE, "wrong issue")
assert(%w[bound implemented reviewed].include?(index.fetch("phase")), "unexpected lifecycle phase")
assert(git("branch", "--show-current") == "codex/5804-v0918-external-review-doc-readiness", "wrong branch")
assert(git("branch", "--show-current") != "main", "cannot repair the corpus on main")
claim = index.fetch("claim")
assert(claim.fetch("id") == "claim-5804-v0918-external-review-doc-readiness", "wrong claim")
required_claim_paths = [
  ".csdlc/prepared/issues/5594/feature_decisions_5594.rb",
  ".csdlc/prepared/issues/5594/validate_feature_crosswalk.rb",
  "docs/milestones/v0.91.8/feature_preservation_crosswalk_5594.v1.json",
  "docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md"
]
assert((required_claim_paths - claim.fetch("protected_paths")).empty?, "claim omits required repair paths")

handoff = HANDOFF.read
[
  "Packet status: `ready_to_freeze_not_sent`",
  "Review performed: false",
  "Release approval claimed: false",
  "v0.92 activation claimed: false",
  "Implementation And Proof Manifest",
  "Live GitHub truth refreshed on 2026-08-04"
].each { |text| assert(handoff.include?(text), "handoff omits required truth: #{text}") }
open_issue_block = handoff[/Live GitHub truth refreshed on 2026-08-04:(.*?)(?:\n\n|\z)/m, 1]
assert(open_issue_block, "handoff lacks dated open-issue inventory")
documented_open_issues = open_issue_block.scan(/#(\d+)/).flatten.map(&:to_i)
assert(documented_open_issues == EXPECTED_OPEN_ISSUES, "dated open-issue inventory mismatch: #{documented_open_issues.inspect}")

manifest = handoff[/## Implementation And Proof Manifest(.*?)### WP-16/m, 1]
assert(manifest, "handoff lacks implementation manifest")
manifest_paths = manifest.scan(/`([^`]+)`/).flatten.select do |value|
  value.start_with?(".csdlc/", "adl-v2/", "adl-runtime/", "adl-runtime-kernel/", "csdlc-v2/", "infra/", "demos/")
end
assert(!manifest_paths.empty?, "implementation manifest contains no repository paths")
manifest_paths.each do |relative|
  normalized = relative.sub(%r{/\z}, "")
  assert(ROOT.join(normalized).exist?, "manifest path does not exist: #{relative}")
end

corpus = git("ls-files", "docs/milestones/v0.91.8").lines.map(&:strip)
  .select { |path| path.match?(/\.(?:md|ya?ml|json)\z/) }
assert(corpus.length == 75, "canonical v0.91.8 document corpus count changed: #{corpus.length}")
corpus.each do |relative|
  path = ROOT.join(relative)
  bytes = path.binread
  unless relative.include?("/evidence/")
    assert(bytes.end_with?("\n"), "#{relative} lacks final newline")
    assert(bytes.lines.none? { |line| line.match?(/[ \t]+\r?\n\z/) }, "#{relative} has trailing whitespace")
  end
  LOCAL_PATH_PATTERNS.each { |pattern| assert(!bytes.match?(pattern), "#{relative} contains machine-local path #{pattern.inspect}") }
  JSON.parse(bytes) if relative.end_with?(".json")
  YAML.safe_load(bytes, aliases: true) if relative.match?(/\.ya?ml\z/)
  validate_local_links(path) if relative.end_with?(".md")
end

assert(system("ruby", ROOT.join(".csdlc/prepared/issues/5594/validate_feature_crosswalk.rb").to_s, chdir: ROOT.to_s), "feature crosswalk validation failed")
assert(system("ruby", ROOT.join(".csdlc/prepared/issues/5594/validate_links.rb").to_s, chdir: ROOT.to_s), "milestone link validation failed")
assert(system("git", "-C", ROOT.to_s, "diff", "--check"), "git diff hygiene failed")
assert(system("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", "1b1ba9990bee81cf74ea449f09c52373aeb7e16c", "HEAD"), "merged #5791 is not ancestral")

if ENV["ADL_VERIFY_LIVE_GITHUB"] == "1"
  output, status = Open3.capture2e(
    "gh", "issue", "list", "--state", "open", "--label", "version:v0.91.8",
    "--limit", "100", "--json", "number"
  )
  assert(status.success?, "live GitHub issue query failed: #{output.strip}")
  live_open_issues = JSON.parse(output).map { |entry| entry.fetch("number") }.sort
  assert(live_open_issues == EXPECTED_OPEN_ISSUES, "live open-issue inventory mismatch: #{live_open_issues.inspect}")
  puts "live GitHub open-issue inventory: PASS"
end

puts "review corpus validation: PASS (#{corpus.length} documents)"
