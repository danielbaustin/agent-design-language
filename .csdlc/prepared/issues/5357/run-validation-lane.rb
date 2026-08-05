#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").expand_path
PREP = ROOT.join(".csdlc/prepared/issues/5357")
EVIDENCE = ROOT.join(".csdlc/evidence/5357")
SEVERITY = {"P0" => 0, "P1" => 1, "P2" => 2, "P3" => 3}.freeze
HEX_40 = /\A[0-9a-f]{40}\z/
HEX_64 = /\A[0-9a-f]{64}\z/

def assert(condition, message)
  raise message unless condition
end

def load_json(path)
  assert(path.file?, "missing retained artifact #{path.relative_path_from(ROOT)}")
  JSON.parse(path.read)
rescue JSON::ParserError => e
  raise "invalid JSON #{path.relative_path_from(ROOT)}: #{e.message}"
end

def digest(path)
  Digest::SHA256.file(path).hexdigest
end

def git(*args)
  out, status = Open3.capture2e("git", "-C", ROOT.to_s, *args)
  assert(status.success?, "git #{args.join(' ')} failed: #{out.strip}")
  out
end

def safe_relative_path(value)
  value.is_a?(String) && !value.empty? && !value.start_with?("/") && !value.split("/").include?("..")
end

def canonical_records(entries)
  entries.map do |entry|
    {
      "mode" => entry.fetch("mode"),
      "object_type" => entry.fetch("object_type"),
      "object_hash" => entry.fetch("object_hash"),
      "path" => entry.fetch("path")
    }
  end
end

def assert_redacted(value, context)
  case value
  when Hash
    value.each { |key, child| assert_redacted(child, "#{context}.#{key}") }
  when Array
    value.each_with_index { |child, index| assert_redacted(child, "#{context}[#{index}]") }
  when String
    forbidden = ["/Users/", "/Volumes/", "/private/tmp/", "AKIA", "BEGIN PRIVATE KEY", "github_pat_", "sk-"]
    assert(forbidden.none? { |marker| value.include?(marker) }, "unredacted value in #{context}")
  end
end

def validate_corpus
  path = EVIDENCE.join("corpus/manifest.json")
  corpus = load_json(path)
  assert(corpus["schema"] == "adl.v0918.external_review_corpus.v1", "wrong corpus schema")
  assert(corpus["status"] == "frozen", "corpus is not frozen")
  assert(corpus["target_sha"].to_s.match?(HEX_40), "invalid corpus target")
  assert(corpus["wp18_terminal_sha"].to_s.match?(HEX_40), "invalid WP-18 identity")
  assert(corpus["repository"] == "danielbaustin/agent-design-language", "wrong corpus repository")
  assert(corpus["base_branch"] == "main", "wrong corpus base branch")
  assert(corpus["head_branch"].is_a?(String) && !corpus["head_branch"].empty?, "missing corpus head branch")
  assert(corpus["canonical_handoff"] == "docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md", "canonical handoff path changed")
  assert(corpus["record_encoding"] == "canonical-json-utf8-v1", "unknown corpus record encoding")
  ancestry = Open3.capture2e("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", corpus.fetch("wp18_terminal_sha"), corpus.fetch("target_sha")).last
  assert(ancestry.success?, "target is not descended from WP-18")
  entries = corpus.fetch("entries")
  assert(entries.length == corpus.fetch("entry_count"), "corpus entry count mismatch")
  paths = entries.map { |entry| entry.fetch("path") }
  assert(paths == paths.sort && paths.uniq == paths, "corpus paths are not sorted and unique")
  assert(paths.all? { |item| safe_relative_path(item) }, "unsafe corpus path")
  assert(paths.none? { |item| item.start_with?(".csdlc/evidence/5357/") }, "corpus includes its own evidence")
  entries.each do |entry|
    assert(%w[blob].include?(entry.fetch("object_type")), "unsupported corpus object type")
    assert(entry.fetch("object_hash").to_s.match?(HEX_40), "invalid corpus object hash")
    assert(entry.fetch("mode").to_s.match?(/\A[0-7]{6}\z/), "invalid corpus mode")
    tree = git("ls-tree", corpus.fetch("target_sha"), "--", entry.fetch("path")).strip
    match = tree.match(/\A([0-7]{6}) (\w+) ([0-9a-f]{40})\t(.+)\z/)
    assert(match && [match[1], match[2], match[3], match[4]] == [entry["mode"], entry["object_type"], entry["object_hash"], entry["path"]], "corpus entry differs from target Git tree")
  end
  records_digest = Digest::SHA256.hexdigest(JSON.generate(canonical_records(entries)))
  assert(corpus["object_records_sha256"] == records_digest, "corpus records digest mismatch")
  handoff_bytes = git("show", "#{corpus.fetch('target_sha')}:#{corpus.fetch('canonical_handoff')}")
  assert(corpus["canonical_handoff_sha256"] == Digest::SHA256.hexdigest(handoff_bytes), "canonical handoff digest mismatch")
  assert_redacted(corpus, "corpus")
  [path, corpus]
end

def validate_receipt(corpus_path, corpus)
  path = EVIDENCE.join("dispatch/receipt.json")
  receipt = load_json(path)
  assert(receipt["schema"] == "adl.v0918.external_review_dispatch.v1", "wrong receipt schema")
  assert(%w[ready_to_dispatch completed failed].include?(receipt["status"]), "invalid receipt status")
  assert(receipt["target_sha"] == corpus["target_sha"], "receipt target differs from corpus")
  assert(receipt["base_branch"] == corpus["base_branch"] && receipt["head_branch"] == corpus["head_branch"], "receipt branch identity differs from corpus")
  assert(receipt["head_sha"] == corpus["target_sha"], "receipt head differs from corpus target")
  base_ancestry = Open3.capture2e("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", receipt.fetch("base_sha"), receipt.fetch("target_sha")).last
  assert(receipt["base_sha"].to_s.match?(HEX_40) && base_ancestry.success?, "receipt base is invalid or not ancestral")
  assert(receipt["corpus_sha256"] == digest(corpus_path), "receipt does not bind corpus bytes")
  assert(receipt["handoff_sha256"] == corpus["canonical_handoff_sha256"], "receipt handoff digest differs from corpus")
  assert(safe_relative_path(receipt["prompt_path"]), "unsafe prompt path")
  assert(corpus.fetch("entries").any? { |entry| entry.fetch("path") == receipt.fetch("prompt_path") }, "prompt is outside frozen corpus")
  prompt_bytes = git("show", "#{corpus.fetch('target_sha')}:#{receipt.fetch('prompt_path')}")
  assert(receipt["prompt_sha256"] == Digest::SHA256.hexdigest(prompt_bytes), "prompt digest mismatch")
  %w[corpus_selector_identity prompt_author_identity prompt_selector_identity reviewer_identity provider_selector_identity process_owner_identity funder_identity retry_controller_identity].each do |key|
    assert(receipt[key].is_a?(String) && !receipt[key].empty?, "receipt missing #{key}")
  end
  project_controls = %w[corpus_selector_identity prompt_author_identity prompt_selector_identity provider_selector_identity process_owner_identity funder_identity retry_controller_identity].map { |key| receipt.fetch(key) }
  assert(project_controls.none? { |identity| identity == receipt.fetch("reviewer_identity") }, "reviewer identity is not separate from project control")
  %w[provider model independence_statement].each { |key| assert(receipt[key].is_a?(String) && !receipt[key].empty?, "receipt missing #{key}") }
  attempts = receipt.fetch("attempts")
  assert(attempts.map { |attempt| attempt.fetch("ordinal") } == (1..attempts.length).to_a, "attempt ordinals are not contiguous")
  required = %w[attempt_id ordinal started_at completed_at outcome request_sha256 response_sha256 error_class]
  attempts.each do |attempt|
    assert(required.all? { |key| attempt.key?(key) }, "attempt is incomplete")
    assert(attempt["request_sha256"].to_s.match?(HEX_64), "invalid attempt request digest")
    response = attempt["response_sha256"]
    assert(response.nil? || response.to_s.match?(HEX_64), "invalid attempt response digest")
  end
  canonical_attempts = JSON.generate(attempts)
  assert(receipt["attempts_sha256"] == Digest::SHA256.hexdigest(canonical_attempts), "attempt digest mismatch")
  previous_path = receipt["supersedes_receipt_path"]
  previous_digest = receipt["supersedes_receipt_sha256"]
  assert(previous_path.nil? == previous_digest.nil?, "supersession path and digest must appear together")
  if previous_path
    assert(previous_path.match?(/\A\.csdlc\/evidence\/5357\/dispatch\/history\/[0-9]+-[0-9a-f]{64}\.json\z/), "unsafe supersession path")
    previous_file = ROOT.join(previous_path)
    previous = load_json(previous_file)
    assert(digest(previous_file) == previous_digest, "superseded receipt digest mismatch")
    prior_attempts = previous.fetch("attempts")
    assert(attempts.length > prior_attempts.length && attempts.first(prior_attempts.length) == prior_attempts, "retry overwrote prior attempt history")
  end
  assert(receipt["review_performed"] == (receipt["status"] == "completed"), "review_performed contradicts status")
  if receipt["status"] == "completed"
    assert(!attempts.empty? && attempts.last.fetch("outcome") == "success", "completed review lacks a successful terminal attempt")
    terminal = receipt.fetch("terminal_attempt")
    assert(terminal == attempts.last.slice("attempt_id", "ordinal", "outcome"), "terminal attempt does not identify the final successful attempt")
  end
  assert(receipt["release_approval_claimed"] == false && receipt["lifecycle_authority"] == false, "receipt claims forbidden authority")
  assert_redacted(receipt, "receipt")
  [path, receipt]
end

def validate_review_output(receipt_path, receipt, corpus)
  path = EVIDENCE.join("review/output.json")
  output = load_json(path)
  assert(output.keys.index("findings") < output.keys.index("residual_risks"), "findings must precede residual risks")
  assert(output.keys.index("findings") < output.keys.index("open_author_decisions"), "findings must precede author decisions")
  assert(output["schema"] == "adl.v0918.external_review_output.v1", "wrong review output schema")
  assert(output["target_sha"] == receipt["target_sha"], "review target differs from receipt")
  assert(output["dispatch_receipt_sha256"] == digest(receipt_path), "review does not bind receipt bytes")
  assert(%w[blocked deferred findings_returned no_findings].include?(output["outcome"]), "invalid review outcome")
  findings = output.fetch("findings")
  ranks = findings.map { |finding| SEVERITY.fetch(finding.fetch("severity")) }
  assert(ranks == ranks.sort, "findings are not ordered P0 through P3")
  ids = findings.map { |finding| finding.fetch("id") }
  assert(ids.uniq == ids, "finding ids are not unique")
  findings.each do |finding|
    %w[summary impact invariant failure_mode remediation].each do |key|
      assert(finding[key].is_a?(String) && !finding[key].empty?, "finding #{finding['id']} missing #{key}")
    end
    evidence = finding.fetch("observed_evidence")
    assert(!evidence.empty?, "finding #{finding['id']} has no observed evidence")
    evidence.each do |item|
      assert(safe_relative_path(item.fetch("path")), "evidence path must be repository-relative")
      assert(corpus.fetch("entries").any? { |entry| entry.fetch("path") == item.fetch("path") }, "evidence path is outside frozen corpus")
      assert(item.fetch("line_start").is_a?(Integer) && item.fetch("line_start") > 0, "invalid evidence start line")
      assert(item.fetch("line_end").is_a?(Integer) && item.fetch("line_end") >= item.fetch("line_start"), "invalid evidence end line")
      assert(item.fetch("statement").is_a?(String) && !item["statement"].empty?, "evidence statement missing")
      source = git("show", "#{corpus.fetch('target_sha')}:#{item.fetch('path')}").lines
      assert(item.fetch("line_end") <= source.length, "evidence line exceeds target file")
      excerpt = source[(item.fetch("line_start") - 1)..(item.fetch("line_end") - 1)].join
      assert(item.fetch("excerpt_sha256") == Digest::SHA256.hexdigest(excerpt), "evidence excerpt digest mismatch")
    end
    assert(finding.key?("inference") && finding.key?("open_author_decision"), "finding statement classes incomplete")
    %w[inference open_author_decision].each do |key|
      value = finding[key]
      assert(value.nil? || (value.is_a?(String) && !value.strip.empty?), "finding #{finding['id']} has empty #{key}")
    end
  end
  assert(output["corpus_sha256"] == receipt["corpus_sha256"], "review corpus differs from receipt")
  assert(receipt["output_path"] == ".csdlc/evidence/5357/review/output.json", "receipt output path is not canonical")
  assert(receipt["output_bytes"] == path.size, "receipt output byte count mismatch")
  assert(receipt["output_sha256"] == digest(path), "receipt output digest mismatch")
  assert_redacted(output, "review_output")
  [path, output]
end

lane = ARGV.fetch(0) { abort("usage: run-validation-lane.rb LANE") }
allowed = %w[corpus-dispatch-preflight review-output-contract complete post-merge-exact].freeze
abort("unknown validation lane: #{lane}") unless allowed.include?(lane)

corpus_path, corpus = validate_corpus
receipt_path, receipt = validate_receipt(corpus_path, corpus)
output_path, output = validate_review_output(receipt_path, receipt, corpus) unless lane == "corpus-dispatch-preflight"

if %w[complete post-merge-exact].include?(lane)
  gate_out, gate_status = Open3.capture2e("ruby", PREP.join("check-dependencies.rb").to_s, chdir: ROOT.to_s)
  assert(gate_status.success?, "WP-18 terminal gate failed: #{gate_out.strip}")
  assert(receipt["status"] == "completed" && receipt["provider_outcome"] == "success", "review dispatch is not complete and successful")
  assert(output["outcome"] != "deferred", "required external review may not be deferred at completion")
end

if lane == "post-merge-exact"
  post = load_json(EVIDENCE.join("post-merge.json"))
  assert(post["schema"] == "adl.v0918.external_review_post_merge.v1", "wrong post-merge schema")
  assert(post["target_sha"] == corpus["target_sha"], "post-merge target mismatch")
  assert(post["corpus_sha256"] == digest(corpus_path), "post-merge corpus digest mismatch")
  assert(post["dispatch_receipt_sha256"] == digest(receipt_path), "post-merge receipt digest mismatch")
  assert(post["review_output_sha256"] == digest(output_path), "post-merge output digest mismatch")
  assert(post["merge_sha"].to_s.match?(HEX_40), "invalid merge SHA")
  merged = Open3.capture2e("git", "-C", ROOT.to_s, "merge-base", "--is-ancestor", corpus.fetch("target_sha"), post.fetch("merge_sha")).last
  assert(merged.success?, "target is not ancestral to merge SHA")
  checks = post.fetch("required_checks")
  assert(!checks.empty? && checks.values.all? { |value| value == "success" }, "post-merge required checks are not green")
  assert(post["typed_synthesis_recorded"] == true, "typed synthesis is not recorded")
  assert(post["wp20_release_authorized"] == false, "#5357 cannot authorize WP-20 release")
  assert_redacted(post, "post_merge")
end

puts JSON.generate(status: "pass", issue: 5357, lane: lane, target_sha: corpus.fetch("target_sha"))
