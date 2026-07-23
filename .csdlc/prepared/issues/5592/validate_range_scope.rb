#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"

BASE = "6d0f6115632a06619544b8ad4792792e741f1f31"
ALLOWED = [
  %r{\A\.csdlc/issues/5592/},
  %r{\A\.csdlc/prepared/issues/5592/},
  %r{\A\.csdlc/locks/5592\.lock\z}
].freeze
FORBIDDEN = [
  %r{\Aadl-runtime-kernel/},
  %r{\Aadl-runtime/},
  %r{\Aadl/},
  %r{runtime[_-]v2}i
].freeze

head = ARGV.fetch(0, "HEAD")

def capture!(*argv)
  stdout, stderr, status = Open3.capture3(*argv)
  abort "#{argv.join(' ')} failed: #{stderr.strip}" unless status.success?
  stdout
end

base = capture!("git", "rev-parse", "--verify", "#{BASE}^{commit}").strip
reviewed_head = capture!("git", "rev-parse", "--verify", "#{head}^{commit}").strip
capture!("git", "diff", "--check", "#{base}..#{reviewed_head}")
paths = capture!("git", "diff", "--name-only", "--diff-filter=ACDMRT", "#{base}..#{reviewed_head}").lines.map(&:strip).reject(&:empty?)
abort "base-to-reviewed-head inventory is empty" if paths.empty?
unexpected = paths.reject { |path| ALLOWED.any? { |pattern| pattern.match?(path) } }
abort "out-of-scope paths: #{unexpected.join(', ')}" unless unexpected.empty?
product = paths.select { |path| FORBIDDEN.any? { |pattern| pattern.match?(path) } }
abort "product or Runtime v2 paths changed: #{product.join(', ')}" unless product.empty?

puts JSON.pretty_generate(
  "schema" => "csdlc.base-to-reviewed-head-path-inventory.v1",
  "issue" => 5592,
  "base" => base,
  "reviewed_head" => reviewed_head,
  "diff_range" => "#{base}..#{reviewed_head}",
  "diff_check" => "pass",
  "product_scope" => "absent",
  "runtime_v2_scope" => "absent",
  "paths" => paths
)
