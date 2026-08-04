#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

ROOT = File.expand_path("../../../..", __dir__)
REPOSITORY = "danielbaustin/agent-design-language"
CARDS = %w[sip stp spp vpp srp sor].freeze
VERIFIER = File.join(__dir__, "receipt-verifier/Cargo.toml")

def fail!(message)
  warn("terminal receipt verification blocked: #{message}")
  exit(1)
end

issue = Integer(ARGV.fetch(0))
expected_sha = ARGV[1]
common, status = Open3.capture2("git", "-C", ROOT, "rev-parse", "--git-common-dir")
fail!("cannot resolve shared Git directory") unless status.success?
common = File.expand_path(common.strip, ROOT)
receipt_path = File.join(common, "csdlc-v2/closeout/#{issue}.json")
record_path = File.join(ROOT, ".csdlc/issues/#{issue}/index.json")
fail!("missing retained receipt for ##{issue}") unless File.file?(receipt_path)
fail!("missing integrated terminal record for ##{issue}") unless File.file?(record_path)

receipt = JSON.parse(File.read(receipt_path))
record = JSON.parse(File.read(record_path))
fail!("##{issue} receipt identity mismatch") unless receipt.values_at("schema", "issue", "repository", "receipt_ref") == ["csdlc.terminal_receipt.v1", issue, REPOSITORY, "csdlc-v2/closeout/#{issue}.json"]
fail!("##{issue} receipt is not the integrated record") unless receipt.fetch("record") == record
fail!("##{issue} is not typed closed_out") unless record["phase"] == "closed_out" && record["claim"].nil?
readiness = record.fetch("readiness")
review = record.fetch("review")
fail!("##{issue} was not accepted at readiness") unless readiness["ready"] == true && readiness["blockers"] == [] && readiness["post_publication_findings"] == []
fail!("##{issue} required checks are not green") unless readiness.fetch("checks").select { |check| check["requirement"] == "required" }.all? { |check| check["conclusion"] == "success" }
fail!("##{issue} exact review has findings") unless review.is_a?(Hash) && review["findings"] == [] && review["reviewed_revision"].to_s.match?(/\Agit-blake3:[0-9a-f]{40}:[0-9a-f]{64}\z/)

CARDS.each do |card|
  values_path = File.join(ROOT, ".csdlc/issues/#{issue}/cards/#{card}.values.json")
  fail!("##{issue} missing #{card} values") unless File.file?(values_path)
  fail!("##{issue} #{card} receipt projection differs") unless receipt.fetch("cards").fetch(card) == JSON.parse(File.read(values_path))
end
receipt.fetch("authored_artifacts").each do |relative, content|
  path = File.join(ROOT, relative)
  fail!("##{issue} authored artifact missing: #{relative}") unless File.file?(path)
  fail!("##{issue} authored artifact differs: #{relative}") unless File.read(path) == content
end

primary = File.dirname(common)
doctor = File.join(primary, ".adl/bin/csdlc-v2/csdlc-doctor")
fail!("stable typed doctor missing") unless File.executable?(doctor)
doctor_out, doctor_err, doctor_status = Open3.capture3(doctor, "--repo", ROOT, "--issue", issue.to_s)
fail!("typed doctor rejected ##{issue}: #{doctor_err.lines.first}") unless doctor_status.success?
doctor_report = JSON.parse(doctor_out)
fail!("typed doctor did not prove closed_out ##{issue}") unless doctor_report["status"] == "pass" && doctor_report["phase"] == "closed_out" && doctor_report["findings"] == []

_digest_out, digest_err, digest_status = Open3.capture3(
  "cargo", "run", "--quiet", "--offline", "--locked", "--manifest-path", VERIFIER, "--", receipt_path,
  chdir: ROOT,
  "CARGO_TARGET_DIR" => ENV.fetch("CARGO_TARGET_DIR", File.join(ROOT, "target/wp13-receipt-verifier"))
)
fail!("##{issue} receipt digest invalid: #{digest_err.lines.first}") unless digest_status.success?

terminal = record.fetch("terminal")
publication = record.fetch("publication")
sha = terminal.fetch("observed_sha")
fail!("##{issue} readiness SHA differs from terminal SHA") unless readiness["head_sha"] == sha
fail!("##{issue} terminal/publication state is not merged") unless terminal["disposition"] == "merged" && terminal["observed_state"] == "merged" && publication["observed_state"] == "merged"
fail!("##{issue} terminal receipt path mismatch") unless terminal["receipt_path"] == "csdlc-v2/closeout/#{issue}.json"
fail!("##{issue} publication/terminal revision mismatch") unless publication.fetch("revision").include?(sha)
fail!("##{issue} unexpected terminal SHA") if expected_sha && sha != expected_sha
_out, ancestry = Open3.capture2("git", "-C", ROOT, "merge-base", "--is-ancestor", sha, "HEAD")
fail!("##{issue} terminal SHA is not ancestral") unless ancestry.success?

puts(JSON.generate(schema: "adl.wp13.terminal_receipt_verification.v1", issue: issue, sha: sha, status: "pass"))
