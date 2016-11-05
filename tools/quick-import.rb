#!/usr/bin/env ruby

if ARGV.length != 2
  puts "usage: ./quick-import.rb <database-file.sqlite> <lang.yml>"
  exit 1
end

begin
  require "sqlite3"
rescue LoadError
  puts "This script needs sqlite3 to work. You can install it by typing `gem install sqlite3`."
  exit 1
end

require "yaml"

if !File.exists?(ARGV[0])
  puts "#{ARGV[0]} is not a file, aborting."
  exit 1
end

db = SQLite3::Database.new(ARGV[0])

begin
  db.execute("SELECT COUNT(*) FROM translations")
rescue SQLite3::NotADatabaseException
  puts "#{ARGV[0]} is encrypted or is not a SQLite3 database, aborting."
  exit 1
end

begin
  yaml = YAML.load_file(ARGV[1])
rescue Psych::SyntaxError => e
  puts "#{ARGV[1]} is not a YAML file or have a problem, aborting.\n#{e.message}"
  exit 1
end

top_level_keys = yaml.keys

if top_level_keys.length > 1
  puts "Top-level keys in #{ARGV[1]} are more than one (#{top_level_keys.length})."
  puts "Make sure that there is only one top key, and that it is the locale"
  puts "of the translations to import."
  exit 1
end

translations_added = 0

yaml.map do |key, values|
  lang = key

  nested_translation_groups = values

  def flatten_hash(my_hash, parent = [])
    my_hash.flat_map do |key, value|
      case value
      when Array then
        numerated_hash = value.each_with_index.map { |v, k| [k.to_s, v] }.compact.flatten
        flatten_hash( Hash[*numerated_hash], parent + [key] )
      when Hash then flatten_hash( value, parent + [key] )
      else [(parent + [key]).join('.'), value.to_s]
      end
    end
  end

  translation_groups = Hash[*flatten_hash(nested_translation_groups)]

  sql = <<-SQL
    INSERT INTO translations (key, locale, content) VALUES (?, ?, ?);
  SQL

  translation_groups.each do |key, content|
    db.execute(sql, [key, lang, content])
    translations_added += 1
  end
end

puts "Done successfully! #{translations_added} translations inserted."
