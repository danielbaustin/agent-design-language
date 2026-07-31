#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").realpath
ENVELOPE_PATH = ROOT.join(".csdlc/evidence/4761/capability-envelope/envelope.v1.json")
INPUTS_PATH = ROOT.join(".csdlc/evidence/4761/capability-envelope/inputs.v1.json")
NON_CLAIMS_PATH = ROOT.join(".csdlc/evidence/4761/capability-envelope/non-claims.v1.md")
FORBIDDEN_TEMP_PATH = ["/private", "tmp"].join("/")

def fail_closed(message)
  warn("FAIL: #{message}")
  exit(1)
end

def read_json(path)
  JSON.parse(path.read)
rescue JSON::ParserError => e
  fail_closed("#{path.relative_path_from(ROOT)} is not valid JSON: #{e.message}")
end

def source_ref_ids(inputs)
  inputs.fetch("sources").map { |source| source.fetch("id") }
end

[ENVELOPE_PATH, INPUTS_PATH, NON_CLAIMS_PATH].each do |path|
  fail_closed("missing #{path.relative_path_from(ROOT)}") unless path.file?
end

envelope = read_json(ENVELOPE_PATH)
inputs = read_json(INPUTS_PATH)
non_claims = NON_CLAIMS_PATH.read

fail_closed("unexpected envelope schema") unless envelope["schema"] == "adl.v092.capability_envelope.v1"
fail_closed("unexpected inputs schema") unless inputs["schema"] == "adl.v092.capability_envelope.inputs.v1"
fail_closed("issue mismatch") unless envelope["issue"] == 4761 && inputs["issue"] == 4761

required_categories = %w[provider model tool skill authority limit]
observed_categories = envelope.fetch("capability_categories").map { |entry| entry.fetch("category") }
missing_categories = required_categories - observed_categories
fail_closed("missing capability categories: #{missing_categories.join(", ")}") unless missing_categories.empty?

accepted_products = envelope.fetch("accepted_products")
expected_products = {
  "C-SDLC v2" => ["e048230245b1ad101c8056678123a2747faa4b60", "fc75f4fc697262f89f99461679a406be0b4b3775"],
  "Runtime v3" => ["f7fc71421f4bcf70039b910c9b88b538bb111400", "f7258b07e9da414bfee518f0c89a76071bc03ee8"],
  "ADL v2 soak and rollback" => ["141dfa20ccc3753060687259ad933397331df9c7", "d4825d4be9ed14ed6060dd33cbdafe5eaa5efcd2"],
  "ADL v2 reversible default" => ["e4bbc988cad682cbb2ff8d24085e1a99bccec1ce", "e1b6a34e4763a79d1c40c641e64c0c061a0aa96c"]
}
fail_closed("accepted product count mismatch") unless accepted_products.length == expected_products.length
accepted_products.each do |product|
  expected = expected_products[product.fetch("product")]
  fail_closed("unexpected accepted product #{product.fetch("product")}") unless expected
  fail_closed("PR head mismatch for #{product.fetch("product")}") unless product.fetch("pr_head") == expected[0]
  fail_closed("accepted merge mismatch for #{product.fetch("product")}") unless product.fetch("accepted_merge") == expected[1]
  fail_closed("missing evidence refs for #{product.fetch("product")}") if product.fetch("evidence_refs").empty?
end

ids = source_ref_ids(inputs)
envelope.fetch("capability_categories").each do |category|
  fail_closed("#{category.fetch("category")} has no supported claims") if category.fetch("supported_claims").empty?
  fail_closed("#{category.fetch("category")} has no limits") if category.fetch("limits").empty?
  category.fetch("evidence_refs").each do |ref|
    fail_closed("unknown evidence ref #{ref}") unless ids.include?(ref)
  end
end
accepted_products.each do |product|
  product.fetch("evidence_refs").each do |ref|
    fail_closed("unknown accepted-product evidence ref #{ref}") unless ids.include?(ref)
  end
end

unsupported_ids = envelope.fetch("unsupported_claim_ids")
fail_closed("unsupported claim list is empty") if unsupported_ids.empty?
unsupported_ids.each do |claim_id|
  fail_closed("unsupported claim #{claim_id} missing from non-claims doc") unless non_claims.include?("`#{claim_id}`")
end

inputs.fetch("sources").each do |source|
  rel = source.fetch("path")
  fail_closed("absolute or temp source path recorded: #{rel}") if rel.start_with?("/") || rel.include?(FORBIDDEN_TEMP_PATH)
  path = ROOT.join(rel)
  fail_closed("missing source #{rel}") unless path.file?
  actual = Digest::SHA256.file(path).hexdigest
  fail_closed("sha256 mismatch for #{rel}: expected #{source.fetch("sha256")} got #{actual}") unless actual == source.fetch("sha256")
  fail_closed("source #{rel} lacks claim class") if source.fetch("claim_class").strip.empty?
  fail_closed("source #{rel} lacks proof status") if source.fetch("proof_status").strip.empty?
end

consumer_surfaces = envelope.fetch("birth_packet_consumption").fetch("consumer_surfaces")
consumer_surfaces.each do |rel|
  fail_closed("consumer surface missing #{rel}") unless ROOT.join(rel).file?
end

scan_paths = [ENVELOPE_PATH, INPUTS_PATH, NON_CLAIMS_PATH]
scan_paths.each do |path|
  fail_closed("forbidden temp path in #{path.relative_path_from(ROOT)}") if path.read.include?(FORBIDDEN_TEMP_PATH)
end

puts JSON.pretty_generate(
  {
    status: "pass",
    issue: 4761,
    accepted_products: accepted_products.length,
    capability_categories: observed_categories,
    source_count: inputs.fetch("sources").length,
    unsupported_claims: unsupported_ids.length
  }
)
