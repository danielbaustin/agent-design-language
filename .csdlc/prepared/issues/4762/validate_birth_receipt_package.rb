#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "set"

ROOT = File.expand_path("../../../../", __dir__)

SCHEMA_PATH = ".csdlc/prepared/issues/4762/birth-witness-receipt-schema.v1.json"
NEGATIVE_PATH = ".csdlc/prepared/issues/4762/birth-witness-receipt-negative-cases.v1.json"
REGISTER_PATH = "docs/milestones/v0.91.8/review/v092_handoff_4762/birth-witness-register-4762.v1.json"
RECEIPT_PATH = "docs/milestones/v0.91.8/review/v092_handoff_4762/birth-receipt-4762.v1.json"
SUMMARY_PATH = "docs/milestones/v0.91.8/review/v092_handoff_4762/BIRTH_WITNESSES_AND_RECEIPT_PACKAGE_4762.md"

def read_json(path)
  JSON.parse(File.read(File.join(ROOT, path)))
end

def fail_with(message)
  warn "#4762 birth witness receipt package: FAIL: #{message}"
  exit 1
end

def require_file(path)
  full = File.join(ROOT, path)
  fail_with("missing required path #{path}") unless File.file?(full)
end

[SCHEMA_PATH, NEGATIVE_PATH, REGISTER_PATH, RECEIPT_PATH, SUMMARY_PATH].each do |path|
  require_file(path)
end

schema = read_json(SCHEMA_PATH)
negative = read_json(NEGATIVE_PATH)
register = read_json(REGISTER_PATH)
receipt = read_json(RECEIPT_PATH)

fail_with("schema issue mismatch") unless schema["issue"] == 4762
fail_with("register issue mismatch") unless register["issue"] == 4762
fail_with("receipt issue mismatch") unless receipt["issue"] == 4762

[schema, negative, register, receipt].each do |artifact|
  fail_with("#{artifact["schema"]} claims a birthday event") unless artifact["birth_event_status"] == "not_claimed"
end

schema["required_register_fields"].each do |field|
  fail_with("register missing #{field}") unless register.key?(field)
end

schema["required_receipt_fields"].each do |field|
  fail_with("receipt missing #{field}") unless receipt.key?(field)
end

required_witnesses = schema["required_witness_ids"].to_set
actual_witnesses = register.fetch("witnesses").map { |witness| witness.fetch("id") }.to_set
missing_witnesses = required_witnesses - actual_witnesses
fail_with("missing witnesses #{missing_witnesses.to_a.join(", ")}") unless missing_witnesses.empty?

required_cases = schema["required_negative_case_ids"].to_set
actual_cases = negative.fetch("cases").map { |entry| entry.fetch("id") }.to_set
missing_cases = required_cases - actual_cases
fail_with("missing negative cases #{missing_cases.to_a.join(", ")}") unless missing_cases.empty?

negative.fetch("cases").each do |entry|
  fail_with("negative case #{entry["id"]} is not rejected") unless entry["disposition"] == "rejected"
  fail_with("negative case #{entry["id"]} has no missing evidence") if entry.fetch("missing_evidence").empty?
end

schema["required_source_paths"].each do |path|
  require_file(path)
end

register.fetch("source_evidence").each do |entry|
  require_file(entry.fetch("path"))
  fail_with("source evidence #{entry["id"]} lacks text") if entry.fetch("evidence").strip.empty?
end

register.fetch("witnesses").each do |witness|
  fail_with("witness #{witness["id"]} lacks attestation") if witness.fetch("attestation").strip.empty?
  witness.fetch("evidence_refs").each { |path| require_file(path) }
end

fail_with("receipt witness ref mismatch") unless receipt["witness_register_ref"] == REGISTER_PATH
fail_with("receipt negative ref mismatch") unless receipt["negative_case_ref"] == NEGATIVE_PATH
fail_with("register negative ref mismatch") unless register["negative_case_ref"] == NEGATIVE_PATH

receipt.fetch("handoff_consumers").each do |consumer|
  require_file(consumer.fetch("path"))
end

joined_boundaries = receipt.fetch("claim_boundaries").join("\n")
schema.fetch("forbidden_claims").each do |claim|
  fail_with("claim boundary does not mention forbidden claim #{claim}") unless joined_boundaries.include?(claim)
end

summary = File.read(File.join(ROOT, SUMMARY_PATH))
[
  REGISTER_PATH,
  RECEIPT_PATH,
  NEGATIVE_PATH,
  "not a birthday occurrence",
  "first true Godel-agent birthday has happened"
].each do |needle|
  fail_with("summary missing #{needle}") unless summary.include?(needle)
end

puts "#4762 birth witness receipt package: PASS"
