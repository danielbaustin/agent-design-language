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
REQUIRED_MANIFEST_PATHS = %w[
  adl-v2/crates/adl-language
  adl-v2/crates/adl-compiler
  csdlc-v2/src
  csdlc-v2/tests
  adl-runtime-kernel/src
  adl-runtime-kernel/tests
  .csdlc/evidence/5501
  .csdlc/evidence/5351/csdlc-v2-all-targets.log
  .csdlc/issues/5778
  .csdlc/issues/5779
  .csdlc/issues/5780
].freeze
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
%w[#5348 #5355 #5357 #5359 #5362 #5363 #5595].each do |issue|
  assert(handoff.include?("`#{issue}`"), "handoff omits open issue #{issue}")
end
REQUIRED_MANIFEST_PATHS.each do |relative|
  assert(ROOT.join(relative).exist?, "manifest path does not exist: #{relative}")
  assert(handoff.include?(relative), "handoff manifest omits: #{relative}")
end

corpus = git("ls-files", "docs/milestones/v0.91.8").lines.map(&:strip)
  .select { |path| path.match?(/\.(?:md|ya?ml|json)\z/) }
assert(corpus.length >= 70, "canonical v0.91.8 document corpus is unexpectedly small")
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

puts "review corpus validation: PASS (#{corpus.length} documents)"
