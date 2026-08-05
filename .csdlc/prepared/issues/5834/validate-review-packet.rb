#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "optparse"
require "pathname"

ROOT = Pathname.new(__dir__).join("../../../..").realpath
EXPECTED_WPS = %w[WP-08 WP-09 WP-10 WP-11 WP-12 WP-13 WP-13A WP-14 WP-15].freeze
EXPECTED_ISSUES = [5825, 5826, 5827, 5828, 5829, 5830, 5831, 5832, 5833].freeze
FORBIDDEN_PUBLIC_CLAIMS = /\b(personhood|consciousness|production citizenship|legal citizenship|governance authority)\b/i

def fail!(message)
  warn "validate-review-packet: #{message}"
  exit 1
end

def relative_repo_path!(value, label)
  path = Pathname.new(String(value))
  fail!("#{label} must be repo-relative: #{value}") if path.absolute? || path.each_filename.include?("..")
  resolved = ROOT.join(path).cleanpath
  fail!("#{label} escapes repository: #{value}") unless resolved.to_s.start_with?(ROOT.to_s + File::SEPARATOR)
  resolved
end

def load_json!(path, label)
  JSON.parse(path.read)
rescue Errno::ENOENT
  fail!("missing #{label}: #{path.relative_path_from(ROOT)}")
rescue JSON::ParserError => error
  fail!("invalid #{label}: #{error.message}")
end

def validate_packet!(packet_path, manifest_path, schema_path)
  packet = packet_path.read
  manifest = load_json!(manifest_path, "evidence manifest")
  schema = load_json!(schema_path, "packet schema")

  fail!("unsupported schema id") unless schema["$id"] == "adl.v092.first-birthday-review-packet.schema.v1"
  required = Array(schema["required"])
  expected_required = %w[schema digest_algorithm entries public_claims non_claims]
  missing_schema_keys = expected_required - required
  fail!("schema omits required keys: #{missing_schema_keys.join(', ')}") unless missing_schema_keys.empty?

  fail!("unsupported manifest schema") unless manifest["schema"] == "adl.v092.first-birthday-review-evidence.v1"
  fail!("digest algorithm must be sha256") unless manifest["digest_algorithm"] == "sha256"
  entries = Array(manifest["entries"])
  fail!("WP roster mismatch") unless entries.map { |entry| entry["wp"] } == EXPECTED_WPS
  fail!("issue roster mismatch") unless entries.map { |entry| entry["issue"] } == EXPECTED_ISSUES
  fail!("duplicate evidence digest") unless entries.map { |entry| entry["digest"] }.uniq.length == entries.length

  entries.each do |entry|
    fail!("#{entry['wp']} is not terminal") unless entry["terminal_state"] == "closed_out"
    fail!("#{entry['wp']} lacks approved exact-head review") unless entry["review_state"] == "approved"
    revision = String(entry["revision"])
    fail!("#{entry['wp']} revision is not a full Git SHA") unless revision.match?(/\A[0-9a-f]{40}\z/)
    source = relative_repo_path!(entry["path"], "#{entry['wp']} evidence path")
    fail!("missing #{entry['wp']} evidence path") unless source.file?
    actual_digest = Digest::SHA256.file(source).hexdigest
    fail!("#{entry['wp']} digest mismatch") unless actual_digest == entry["digest"]
    fail!("packet omits #{entry['wp']} evidence path") unless packet.include?(entry["path"])
  end

  public_claims = Array(manifest["public_claims"])
  fail!("public claims must be an array of bounded strings") unless public_claims.all? { |claim| claim.is_a?(String) && !claim.strip.empty? }
  forbidden = public_claims.grep(FORBIDDEN_PUBLIC_CLAIMS)
  fail!("forbidden public claim: #{forbidden.first}") unless forbidden.empty?
  non_claims = Array(manifest["non_claims"])
  fail!("non-claim boundary is incomplete") unless non_claims.length >= 5

  packet.scan(/`([^`]+)`/).flatten.each do |candidate|
    next unless candidate.include?("/")
    next if candidate.start_with?("http://", "https://")

    relative_repo_path!(candidate, "packet reference")
  end
end

options = {}
OptionParser.new do |parser|
  parser.on("--packet PATH") { |value| options[:packet] = value }
  parser.on("--manifest PATH") { |value| options[:manifest] = value }
  parser.on("--schema PATH") { |value| options[:schema] = value }
  parser.on("--negative-fixtures PATH") { |value| options[:negative_fixtures] = value }
end.parse!

if options[:negative_fixtures]
  root = relative_repo_path!(options[:negative_fixtures], "negative fixture root")
  cases = %w[stale-digest missing-roster private-path contradictory-status forbidden-public-claim]
  cases.each do |name|
    fixture = root.join(name)
    pid = Process.fork do
      validate_packet!(fixture.join("packet.md"), fixture.join("manifest.json"), fixture.join("schema.json"))
      exit 0
    end
    Process.wait(pid)
    fail!("negative fixture unexpectedly passed: #{name}") if $?.success?
  end
  puts JSON.generate(schema: "adl.v092.birthday-review-negative-proof.v1", cases: cases, outcome: "passed")
  exit 0
end

%i[packet manifest schema].each { |key| fail!("--#{key} is required") unless options[key] }
packet_path = relative_repo_path!(options[:packet], "packet")
manifest_path = relative_repo_path!(options[:manifest], "manifest")
schema_path = relative_repo_path!(options[:schema], "schema")
validate_packet!(packet_path, manifest_path, schema_path)
puts JSON.generate(schema: "adl.v092.birthday-review-packet-validation.v1", outcome: "passed")
