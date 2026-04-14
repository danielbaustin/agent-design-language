#!/usr/bin/env bash
set -euo pipefail

if [[ "$#" -lt 1 ]]; then
  echo "usage: validate_skill_frontmatter.sh <SKILL.md> [<SKILL.md> ...]" >&2
  exit 1
fi

ruby - "$@" <<'RUBY'
require "yaml"
require "psych"

def extract_frontmatter(path)
  text = File.read(path)
  match = text.match(/\A---\n(.*?)\n---\n/m)
  raise "#{path}: missing YAML frontmatter block" unless match
  match[1]
end

def check_duplicate_keys(node, path)
  return unless node.is_a?(Psych::Nodes::Node)

  if node.is_a?(Psych::Nodes::Mapping)
    seen = {}
    node.children.each_slice(2) do |key_node, value_node|
      key = if key_node.respond_to?(:value)
        key_node.value
      else
        key_node.to_s
      end
      raise "#{path}: duplicate frontmatter key #{key.inspect}" if seen[key]
      seen[key] = true
      check_duplicate_keys(value_node, path)
    end
  elsif node.respond_to?(:children) && node.children
    node.children.each do |child|
      check_duplicate_keys(child, path)
    end
  end
end

ARGV.each do |path|
  begin
    frontmatter = extract_frontmatter(path)
    ast = Psych.parse(frontmatter)
    raise "#{path}: invalid YAML frontmatter" if ast.nil?
    check_duplicate_keys(ast, path)
    YAML.safe_load(frontmatter)
  rescue StandardError => e
    warn "validate_skill_frontmatter.sh: #{path}: #{e.message}"
    exit 1
  end
end
RUBY
