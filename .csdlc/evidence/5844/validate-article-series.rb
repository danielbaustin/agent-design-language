#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "pathname"

def local_markdown_links(body)
  body.scan(/\[[^\]]+\]\(([^)#]+)(?:#[^)]+)?\)/).flatten.reject do |link|
    link.match?(%r{\Ahttps?://})
  end
end

def broken_links(path, body)
  local_markdown_links(body).each_with_object([]) do |link, errors|
    target = File.expand_path(link, File.dirname(path))
    errors << "#{path}: #{link}" unless File.exist?(target)
  end
end

def source_packet_refs(body)
  body.scan(/^\|\s*`([^`]+)`\s*\|/).flatten
end

def article_repo_refs(root, path, body)
  section = body.split("## Repository Sources", 2)[1].to_s
  local_markdown_links(section).map do |link|
    target = Pathname.new(File.expand_path(link, File.dirname(path)))
    target.relative_path_from(Pathname.new(root)).to_s
  end
end

def safe_publication_disposition?(body)
  body.match?(/review-ready|operator-approved/i) &&
    !body.match?(/autonomously published|auto-published/i)
end

root = File.expand_path("../../..", __dir__)
articles = File.join(root, "docs/milestones/v0.92/publication/articles")
slugs = %w[
  01-what-is-adl
  02-adl-runtime-and-cognitive-spacetime-model
  03-godel-agents-and-godel-hadamard-bayes-algorithm
  04-the-freedom-gate
  05-uts-and-acc-making-agents-with-tools-safe
  06-codefriend-and-the-cognitive-sdlc
  07-continuous-adversarial-verification-for-continuous-security
  08-agent-economics
  09-adl-and-social-intelligence
  10-whats-next-for-adl
]
required = slugs.flat_map do |slug|
  %w[SOURCE_PACKET.md ARTICLE.md EDITORIAL_REVIEW.md].map do |name|
    File.join(articles, slug, name)
  end
end
required.concat(%w[SERIES_ARC_AND_CLAIM_MATRIX.md PUBLICATION_DISPOSITION.md].map { |name| File.join(articles, name) })

missing = required.reject { |path| File.file?(path) && !File.read(path).strip.empty? }
raise "missing or empty WP-24 artifacts: #{missing.join(', ')}" unless missing.empty?

contents = required.map { |path| [path, File.read(path)] }
forbidden = /\b(TODO|TBD|PLACEHOLDER|lorem ipsum)\b|\/Users\/|file:\/\//i
violations = contents.select { |_path, body| body.match?(forbidden) }.map(&:first)
raise "placeholder or private-path content: #{violations.join(', ')}" unless violations.empty?

source_headings = ["## Brief", "## Evidence", "## Claim Posture", "## Guardrails"]
review_headings = ["## Verdict", "## Evidence And Claim Review", "## Privacy And Publication Review", "## Findings And Disposition"]
slugs.each do |slug|
  directory = File.join(articles, slug)
  source = File.read(File.join(directory, "SOURCE_PACKET.md"))
  article = File.read(File.join(directory, "ARTICLE.md"))
  review = File.read(File.join(directory, "EDITORIAL_REVIEW.md"))
  missing_source = source_headings.reject { |heading| source.include?(heading) }
  missing_review = review_headings.reject { |heading| review.include?(heading) }
  raise "#{slug} source packet missing headings: #{missing_source.join(', ')}" unless missing_source.empty?
  raise "#{slug} editorial review missing headings: #{missing_review.join(', ')}" unless missing_review.empty?
  raise "#{slug} source packet lacks current claim posture" unless source.match?(/\bCurrent\b/)
  raise "#{slug} source packet lacks planned or proposed posture" unless source.match?(/\b(Planned|Proposed|planned|proposed)\b/)
  raise "#{slug} article is shorter than 650 words" if article.split.length < 650
  raise "#{slug} article lacks repository sources" unless article.include?("## Repository Sources")

  declared_refs = source_packet_refs(source)
  article_refs = article_repo_refs(root, File.join(directory, "ARTICLE.md"), article)
  missing_declared_files = declared_refs.reject { |ref| File.file?(File.join(root, ref)) }
  undeclared_refs = article_refs - declared_refs
  raise "#{slug} source packet references missing files: #{missing_declared_files.join(', ')}" unless missing_declared_files.empty?
  raise "#{slug} article cites sources absent from its source packet: #{undeclared_refs.join(', ')}" unless undeclared_refs.empty?
end

article_digests = slugs.map do |slug|
  path = File.join(articles, slug, "ARTICLE.md")
  Digest::SHA256.hexdigest(File.read(path))
end
raise "article drafts must be unique" unless article_digests.uniq.length == slugs.length

link_errors = contents.flat_map { |path, body| broken_links(path, body) }
raise "broken repository-relative links: #{link_errors.join(', ')}" unless link_errors.empty?

sensitive = /HOME\/keys|BEGIN (?:RSA |OPENSSH |EC )?PRIVATE KEY|Bearer\s+[A-Za-z0-9._-]{12,}|(?:api[_ -]?key|secret)\s*[:=]\s*[A-Za-z0-9._-]{12,}/i
sensitive_paths = contents.select { |_path, body| body.match?(sensitive) }.map(&:first)
raise "credential-like content in article packet: #{sensitive_paths.join(', ')}" unless sensitive_paths.empty?

if ARGV.include?("--negative")
  disposition = File.read(File.join(articles, "PUBLICATION_DISPOSITION.md"))
  raise "publication disposition must remain review-ready or operator-approved" unless safe_publication_disposition?(disposition)

  fixture_failures = []
  fixture_failures << "placeholder" unless "Draft TODO".match?(forbidden)
  fixture_failures << "private path" unless "See /Users/example/private.md".match?(forbidden)
  fixture_failures << "broken link" if broken_links(File.join(articles, "fixture.md"), "[missing](not-present.md)").empty?
  fixture_failures << "unlisted citation" unless (["docs/other.md"] - ["docs/declared.md"]).any?
  fixture_failures << "missing current posture" if "Planned only".match?(/\bCurrent\b/)
  fixture_failures << "unsafe publication" if safe_publication_disposition?("Autonomously published")
  fixture_failures << "duplicate digest" unless %w[same same].uniq.length < 2
  raise "negative fixtures failed to reject: #{fixture_failures.join(', ')}" unless fixture_failures.empty?
end

if ARGV.include?("--rollback")
  manifest_path = File.join(root, ".csdlc/evidence/5844/rollback-manifest.json")
  manifest = JSON.parse(File.read(manifest_path))
  raise "rollback issue mismatch" unless manifest["issue"] == 5844
  raise "rollback must not require external publication action" unless manifest["external_publication_action"] == false
  raise "rollback evidence root mismatch" unless manifest["retain_evidence_root"] == ".csdlc/evidence/5844"
  raise "rollback evidence root missing" unless File.directory?(File.join(root, manifest["retain_evidence_root"]))
  raise "rollback lifecycle record missing" unless File.directory?(File.join(root, ".csdlc/issues/5844"))

  remove = manifest.fetch("remove_paths")
  retain = manifest.fetch("retain_paths")
  restore = manifest.fetch("restore_paths")
  raise "rollback path sets overlap" unless (remove & retain).empty? && (remove & restore).empty?
  raise "rollback remove set must contain exactly ten article drafts" unless remove.length == 10 && remove.all? { |path| path.end_with?("/ARTICLE.md") }
  raise "rollback retain set must contain all source and editorial records" unless retain.length == 20
  raise "rollback restore set must contain matrix and disposition" unless restore.sort == %w[
    docs/milestones/v0.92/publication/articles/PUBLICATION_DISPOSITION.md
    docs/milestones/v0.92/publication/articles/SERIES_ARC_AND_CLAIM_MATRIX.md
  ]
  (remove + retain + restore).each do |path|
    raise "rollback path is outside WP-24: #{path}" unless path.start_with?("docs/milestones/v0.92/publication/articles/")
    raise "rollback path does not exist: #{path}" unless File.file?(File.join(root, path))
  end


  provider_evidence = %w[
    claude-final-01-05-result.json
    claude-final-06-10-result.json
    gemini-exact-closing-result.json
  ] + (1..10).map { |number| format("gemini-article-%02d-result.json", number) }
  missing_provider_evidence = provider_evidence.reject do |name|
    File.file?(File.join(root, manifest["retain_evidence_root"], name))
  end
  raise "rollback provider evidence missing: #{missing_provider_evidence.join(', ')}" unless missing_provider_evidence.empty?
end

puts "WP-24 article series contract passed"
