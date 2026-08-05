#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "digest"
require "yaml"

git_common, git_status = Open3.capture2("git", "rev-parse", "--git-common-dir")
abort "cannot resolve Git common directory" unless git_status.success?
install_root = File.dirname(File.expand_path(git_common.strip))
BIN_DIR = File.join(install_root, ".adl/bin/csdlc-v2")
INSTALL = File.join(BIN_DIR, "csdlc-install")
DOCTOR = File.join(BIN_DIR, "csdlc-doctor")
INSTALL_RECEIPT = File.join(BIN_DIR, "install-receipt.json")
INVENTORY = "csdlc-v2/operator/coexistence.json"
SELECTOR = "csdlc-v2/operator/generation-selector.json"
OUTPUT = ".csdlc/evidence/5860/V092_TYPED_DOCTOR_REPORTS.json"
WAVE_PATH = "docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml"

wave = YAML.safe_load(File.read(WAVE_PATH), aliases: true)
ISSUES = (Array(wave["work_packages"]) + Array(wave["supporting_issues"]))
  .map { |row| row["issue"] if row["issue"].is_a?(Integer) && row["issue"] != 5817 }.compact
  .uniq.sort.freeze

abort "missing installed csdlc-install" unless File.executable?(INSTALL)
abort "missing installed csdlc-doctor" unless File.executable?(DOCTOR)
abort "missing install receipt" unless File.file?(INSTALL_RECEIPT)

resolve_stdout, resolve_stderr, resolve_status = Open3.capture3(INSTALL, "resolve", "--repo", ".", "--issue", "5860")
abort "generation resolution failed: #{resolve_stderr}" unless resolve_status.success?
generation = JSON.parse(resolve_stdout)
abort "installed generation is not v2" unless generation == "v2"

receipt = JSON.parse(File.read(INSTALL_RECEIPT))
doctor_receipt = Array(receipt["binaries"]).find { |entry| entry["name"] == "csdlc-doctor" }
abort "install receipt does not identify csdlc-doctor" unless doctor_receipt
installed_revision = receipt.fetch("source_revision").delete_prefix("git:")
abort "invalid installed source revision" unless installed_revision.match?(/\A[0-9a-f]{40}\z/)
installed_tree, installed_tree_status = Open3.capture2("git", "rev-parse", "#{installed_revision}:csdlc-v2")
candidate_tree, candidate_tree_status = Open3.capture2("git", "rev-parse", "HEAD:csdlc-v2")
abort "cannot resolve installed C-SDLC v2 source tree" unless installed_tree_status.success?
abort "cannot resolve candidate C-SDLC v2 source tree" unless candidate_tree_status.success?
abort "installed doctor source differs from the candidate C-SDLC v2 source tree" unless installed_tree.strip == candidate_tree.strip

reports = ISSUES.map do |issue|
  stdout, stderr, status = Open3.capture3(DOCTOR, "--repo", ".", "--issue", issue.to_s)
  abort "doctor failed for ##{issue}: #{stderr}" unless [0, 2].include?(status.exitstatus)

  report = JSON.parse(stdout)
  expected_finding = [{ "code" => "claim_dormant", "message" => "nonterminal issue has no active writer claim" }]
  abort "##{issue}: wrong report identity" unless report["issue"] == issue
  abort "##{issue}: wrong handoff status" unless report["status"] == "block" && report["ready"] == false
  abort "##{issue}: wrong phase" unless report["phase"] == "bound"
  abort "##{issue}: unexpected doctor findings #{report['findings'].inspect}" unless report["findings"] == expected_finding
  abort "##{issue}: wrong next operation" unless report["next_operation"] == "reacquire_claim"
  report
end

payload = {
  "schema" => "adl.v092.child-doctor-reports.v1",
  "interpretation" => "design-ready handoff; execution remains dependency-gated and requires just-in-time claim reacquisition",
  "toolchain" => {
    "generation" => generation,
    "selector" => SELECTOR,
    "inventory" => INVENTORY,
    "install_source_revision" => receipt.fetch("source_revision"),
    "installed_source_tree" => installed_tree.strip,
    "candidate_source_tree" => candidate_tree.strip,
    "install_receipt_sha256" => Digest::SHA256.file(INSTALL_RECEIPT).hexdigest,
    "doctor_binary" => ".adl/bin/csdlc-v2/csdlc-doctor",
    "doctor_binary_sha256" => Digest::SHA256.file(DOCTOR).hexdigest,
    "doctor_binary_blake3" => doctor_receipt.fetch("blake3")
  },
  "reports" => reports
}
rendered = JSON.pretty_generate(payload) + "\n"
if ARGV.include?("--write")
  File.write(OUTPUT, rendered)
else
  abort "missing pinned doctor evidence; run with --write explicitly" unless File.file?(OUTPUT)
  expected = File.read(OUTPUT)
  abort "typed doctor evidence drift: expected #{Digest::SHA256.hexdigest(expected)}, live #{Digest::SHA256.hexdigest(rendered)}" unless expected == rendered
end
puts "v0.92 typed doctors: PASS (#{reports.length} claim-null handoffs)"
