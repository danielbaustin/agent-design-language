# frozen_string_literal: true

require "digest"
require "json"
require "open3"

module Wp04ProofReceiptContract
  module_function

  SHA256 = /\A[0-9a-f]{64}\z/

  def digest_file(path, expected, label, allow_empty: false)
    abort "#{label} path must be repository-relative" if path.to_s.empty? || path.start_with?("/") || path.split("/").include?("..")
    abort "missing #{label}: #{path}" unless File.file?(path)
    abort "empty #{label}: #{path}" if !allow_empty && File.zero?(path)
    abort "invalid #{label} digest" unless expected.to_s.match?(SHA256)
    abort "#{label} digest mismatch: #{path}" unless Digest::SHA256.file(path).hexdigest == expected
  end

  def validate_runner(runner, label)
    abort "#{label} runner missing" unless runner.is_a?(Hash)
    %w[provider run_id os arch].each do |field|
      abort "#{label} runner #{field} missing" if runner[field].to_s.strip.empty?
    end
    abort "#{label} runner identity hash invalid" unless runner["identity_sha256"].to_s.match?(SHA256)
  end

  def validate_command(command, label)
    abort "#{label} command missing" unless command.is_a?(Hash)
    argv = Array(command["argv"])
    abort "#{label} argv missing" if argv.empty? || argv.any? { |part| part.to_s.empty? }
    abort "#{label} command failed" unless command["exit_code"] == 0
    abort "#{label} start time missing" if command["started_at"].to_s.empty?
    abort "#{label} finish time missing" if command["finished_at"].to_s.empty?
    validate_runner(command["runner"], label)
    digest_file(command["stdout_path"], command["stdout_sha256"], "#{label} stdout")
    digest_file(command["stderr_path"], command["stderr_sha256"], "#{label} stderr", allow_empty: true)
    argv
  end

  def validate_artifacts(artifacts, issue, label)
    entries = Array(artifacts)
    abort "#{label} artifacts missing" if entries.empty?
    entries.each do |artifact|
      path = artifact.fetch("path")
      abort "#{label} artifact escapes issue evidence: #{path}" unless path.start_with?(".csdlc/evidence/#{issue}/")
      digest_file(path, artifact.fetch("sha256"), "#{label} artifact")
    end
  end

  def validate_negative_cases(cases, issue)
    entries = Array(cases)
    abort "negative cases missing" if entries.empty?
    entries.each do |entry|
      abort "negative case name missing" if entry["case"].to_s.empty?
      abort "negative case has no proving result" unless %w[denied rejected fenced recovered fail_closed].include?(entry["result"])
      digest_file(entry["evidence_path"], entry["evidence_sha256"], "negative case #{entry['case']}")
      abort "negative evidence escapes issue evidence" unless entry["evidence_path"].start_with?(".csdlc/evidence/#{issue}/")
    end
  end

  def validate(issue:, wp:, paths:, test:, platforms:, required_commands: [])
    evidence_path = ARGV.fetch(0, ".csdlc/evidence/#{issue}/execution-proof.json")
    abort "missing execution proof: #{evidence_path}" unless File.file?(evidence_path)
    proof = JSON.parse(File.read(evidence_path))
    abort "wrong schema" unless proof["schema"] == "adl.wp04.execution_proof.v2"
    abort "wrong issue" unless proof["issue"] == issue
    abort "wrong WP" unless proof["wp"] == wp
    head, status = Open3.capture2("git", "rev-parse", "HEAD")
    abort "cannot resolve HEAD" unless status.success?
    abort "stale source revision" unless proof["source_revision"] == head.strip
    abort "protected path drift" unless proof["protected_paths"] == paths

    commands = Array(proof["commands"])
    test_commands = commands.select do |command|
      argv = validate_command(command, "command")
      argv.include?(test) && argv.include?("--no-tests=fail") && command["selected_tests"].to_i.positive?
    end
    abort "missing one nonzero exact test command #{test}" unless test_commands.length == 1
    required_commands.each do |required|
      matches = commands.select { |command| Array(command["argv"]) == required }
      abort "missing exact proving command #{required.join(' ')}" unless matches.length == 1
    end
    validate_negative_cases(proof["negative_cases"], issue)
    validate_artifacts(proof["artifacts"], issue, "execution proof")

    receipts = Array(proof["native_receipts"])
    abort "unexpected native receipt denominator" unless receipts.map { |entry| entry["platform"] }.sort == platforms.sort
    receipts.each do |receipt|
      platform = receipt.fetch("platform")
      abort "stale native receipt for #{platform}" unless receipt["source_revision"] == head.strip
      validate_command(receipt["command"], "#{platform} native")
      validate_artifacts(receipt["artifacts"], issue, "#{platform} native")
    end
    run_ids = receipts.map { |receipt| receipt.dig("command", "runner", "run_id") }
    abort "native runner runs are not distinct" unless run_ids.uniq.length == run_ids.length
    puts "PASS: #{wp} exact-head logs, artifacts, negatives, and native receipts"
  end
end
