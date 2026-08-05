#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "open3"
require "yaml"

EXPECTED = (1..16).to_h { |number| [format("WP-04.%02d", number), 5862 + number] }.freeze
SHA = /\A[0-9a-f]{40}\z/
SHA256 = /\A[0-9a-f]{64}\z/
PREFLIGHT = ARGV.delete("--preflight")

def exact_owned_paths(design, wp)
  section = design[/## Owned Paths\n\n(.*?)\n\n## /m, 1]
  abort "#{wp} missing exact Owned Paths" unless section
  paths = section.scan(/`([^`]+)`/).flatten
  abort "#{wp} has no owned paths" if paths.empty?
  paths
end

def checked_digest(path, expected, label)
  abort "missing #{label}: #{path}" unless File.file?(path)
  abort "invalid #{label} digest" unless expected.to_s.match?(SHA256)
  abort "#{label} digest mismatch" unless Digest::SHA256.file(path).hexdigest == expected
end

wave = YAML.safe_load(File.read("docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml"), aliases: true)
canonical = Array(wave["work_packages"]).select { |row| EXPECTED.key?(row["wp"]) }.to_h { |row| [row["wp"], row["issue"]] }
abort "canonical WP-04 child denominator drift" unless canonical == EXPECTED

all_paths = {}
records = {}
EXPECTED.each do |wp, issue|
  index_path = ".csdlc/issues/#{issue}/index.json"
  abort "missing index for #{wp} ##{issue}" unless File.file?(index_path)
  index = JSON.parse(File.read(index_path))
  records[issue] = index
  abort "issue mismatch for #{wp}" unless index["issue"] == issue
  abort "#{wp} design not approved" unless index.dig("design_review", "approved", "revision").to_s.match?(SHA256)
  abort "#{wp} preparation claim remains active" unless index["claim"].nil?
  %w[sip stp spp vpp].each do |card|
    values = JSON.parse(File.read(".csdlc/issues/#{issue}/cards/#{card}.values.json"))
    abort "#{wp} #{card} not ready" unless values["status"] == "ready"
  end
  paths = exact_owned_paths(File.read(".csdlc/prepared/issues/#{issue}/design.md"), wp)
  paths.each do |path|
    abort "path collision #{path}: #{all_paths[path]} and #{wp}" if all_paths.key?(path)
    all_paths[path] = wp
  end
end

umbrella = JSON.parse(File.read(".csdlc/issues/5862/index.json"))
gate = File.read(".csdlc/prepared/issues/5821/design.md")
EXPECTED.each { |wp, issue| abort "gate mapping missing #{wp} ##{issue}" unless gate.include?("| #{wp} | ##{issue} |") }

if PREFLIGHT
  abort "umbrella preparation claim remains active" unless umbrella["claim"].nil?
  puts "PASS: WP-04-IMP preflight, sixteen approved claim-null children, #{all_paths.length} exact owned paths"
  exit 0
end

umbrella_claim = umbrella["claim"]
abort "terminal reconciliation requires the active WP-04-IMP execution claim" unless umbrella_claim.is_a?(Hash)
abort "terminal reconciliation claim does not own umbrella evidence" unless Array(umbrella_claim["protected_paths"]).include?(".csdlc/evidence/5862")

manifest_path = ".csdlc/evidence/5862/terminal-child-receipts.json"
abort "missing terminal reconciliation manifest" unless File.file?(manifest_path)
manifest = JSON.parse(File.read(manifest_path))
abort "wrong terminal manifest schema" unless manifest["schema"] == "adl.wp04.terminal_child_receipts.v1"
entries = Array(manifest["children"])
abort "terminal manifest denominator drift" unless entries.map { |entry| entry["issue"] }.sort == EXPECTED.values

git_common, git_status = Open3.capture2("git", "rev-parse", "--git-common-dir")
abort "cannot resolve Git common directory" unless git_status.success?
pr_binary = ENV.fetch("CSDLC_GITHUB_PR_BIN", File.join(File.expand_path("..", git_common.strip), ".adl/bin/csdlc-v2/csdlc-github-pr"))
abort "missing typed GitHub PR binary" unless File.executable?(pr_binary)
request_dir = ".csdlc/evidence/5862/pr-state-requests"
FileUtils.mkdir_p(request_dir)

entries.each do |entry|
  issue = entry.fetch("issue")
  index = records.fetch(issue)
  terminal = index.fetch("terminal")
  abort "issue ##{issue} is not terminal merged" unless terminal["disposition"] == "merged" && terminal["observed_state"] == "merged"
  pr = terminal.fetch("pull_request")
  child_head = terminal.fetch("observed_sha")
  abort "invalid child head for ##{issue}" unless child_head.match?(SHA)
  abort "manifest PR drift for ##{issue}" unless entry["pull_request"] == pr
  abort "manifest head drift for ##{issue}" unless entry["head_sha"] == child_head
  receipt_path = terminal.fetch("receipt_path")
  abort "manifest receipt drift for ##{issue}" unless entry["receipt_path"] == receipt_path
  checked_digest(receipt_path, entry.fetch("receipt_sha256"), "##{issue} terminal receipt")

  request_path = File.join(request_dir, "#{issue}.json")
  File.write(request_path, JSON.pretty_generate({repository: "danielbaustin/agent-design-language", pull_request: pr, required_checks: [], require_review: false, linked_issue: issue}) + "\n")
  stdout, stderr, status = Open3.capture3(pr_binary, "state", "--request", request_path)
  abort "typed PR read failed for ##{issue}: #{stderr} #{stdout}" unless status.success?
  packet = JSON.parse(stdout)
  abort "PR ##{pr} does not close ##{issue}" unless packet["linked_issue"] == issue
  abort "PR ##{pr} is not merged" unless packet["state"] == "closed" && packet["merged"] == true
  abort "PR head drift for ##{issue}" unless packet["head_sha"] == child_head
  merge_sha = packet["merge_commit_sha"]
  abort "invalid merge SHA for ##{issue}" unless merge_sha.to_s.match?(SHA)
  abort "manifest merge drift for ##{issue}" unless entry["merge_sha"] == merge_sha
  system("git", "merge-base", "--is-ancestor", merge_sha, "HEAD") or abort "merge for ##{issue} is not ancestral to candidate HEAD"
end

integrated = records.fetch(5878).fetch("terminal").fetch("observed_sha")
proof_path = ".csdlc/evidence/5878/execution-proof.json"
checked_digest(proof_path, manifest.fetch("wp04_16_execution_proof_sha256"), "WP-04.16 execution proof")
proof = JSON.parse(File.read(proof_path))
abort "WP-04.16 proof schema drift" unless proof["schema"] == "adl.wp04.execution_proof.v2"
abort "WP-04.16 proof is not exact child head" unless proof["source_revision"] == integrated
commands = Array(proof["commands"])
required = [["bash", "adl/tools/validate_v092_distributed_guardian.sh"], ["ruby", "adl/tools/validate_v092_distributed_native_receipts.rb"]]
required.each { |argv| abort "WP-04.16 missing #{argv.join(' ')}" unless commands.one? { |command| command["argv"] == argv && command["exit_code"] == 0 } }
puts "PASS: sixteen live merged child PRs, terminal receipts, exact heads, and WP-04.16 integrated proof authorize WP-14 handoff"
