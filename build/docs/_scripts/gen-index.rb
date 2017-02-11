#!/usr/bin/env ruby

require 'yaml'
require 'optparse'

# load individual file and return index fragment

def gen_fragment(input_file)
    in_header = in_apibody = in_desc = in_example = in_code = nil
    header = apibody = desc = example = last_line = ""
    blanks = 0
    sep_line = "<!-- break -->\n"
    stop_line = "<!-- stop -->\n"
    
    lines = IO.readlines(input_file)
    
    use_separator = lines.include? sep_line
    
    lines.each do |line|
        line = line.rstrip + "\n"
        # Skip multiple runs of blank lines
        if line == "\n" and last_line == line
            next
        else
            last_line = line
        end
        
        # detect YAML header
        if line == "---\n"
            in_header = (in_header == nil ? true : false)
            next
        end
    
        # collect YAML header
        if in_header
            header += line
            next
        end
    
        # collect API body
        in_apibody = true if line == "{% apibody %}\n" and in_apibody == nil
        apibody += line if in_apibody
        in_apibody = false if line == "{% endapibody %}\n"
    
        # collect description
        if line == "# Description #\n"
            in_desc = true
            next
        end
        if in_desc
            if line.start_with? "<img"
                blanks -= 1
                next
            end
            if use_separator
                if line == sep_line
                    in_desc = false
                else
                    desc += line
                end
                next
            end
            blanks += 1 if line == "\n"
            if blanks > 1
                in_desc = false
            else
                desc += line
            end
        end

        # collect first example
        if line.start_with? "__Example:" and in_example == nil
            in_example = true
            example += line
            next
        end
        if in_example
            if line.start_with? "__Example:" or line == stop_line
                in_example = false
            else
                example += line
            end
        end
    end
    example.strip!
    apibody.strip!
    desc.strip!
    header = YAML.load(header)
    permalink = header['permalink'].split("/")[-1] + "/"
    
    <<-EOS
## [#{header['command']}](#{permalink}) ##

#{apibody}

#{desc}

#{example}

[Read more about this command &rarr;](#{permalink})

EOS
end

# Output header

def index_header(language)
    langmap = { "java" => "Java", "python" => "Python", "ruby" => "Ruby",
                "javascript" => "JavaScript" }
    header = <<-EOS
---
layout: api
title: "ReQL command reference"
active: api
no_footer: true
permalink: api/#{language}/
language: #{langmap[language]}
EOS
    header += "alias: api/\n" if language == "javascript"
    header += "---\n\n"
end

# Parse options
def get_options
    options = {}
    op = OptionParser.new do |opts|
        opts.banner = "Usage: gen-index language [options]"
        opts.on("-d[dir]", "--dir=[dir]", "Specify source directory") do |opt|
            options[:dir] = opt
        end
        opts.on("-n[yaml]", "--nav=[yaml]", "Specify nav YAML file") do |opt|
            options[:nav] = opt
        end
        opts.on("-h", "--help", "Print help") do
            puts opts
            exit
        end
    end
    begin
        op.parse!
    rescue OptionParser::InvalidOption
        puts "Invalid option specified -- try '-h' for help"
        exit
    end
    [options, ARGV[0]]
end

# -----
# Start
# -----

options, language = get_options
language = "javascript" if language.nil?
src_dir = if options[:dir]
    options[:dir].split("/")
else
    # default: assume script is in docs/_scripts
    File.expand_path(File.dirname(__FILE__)).split("/")[0..-2]
end
nav_file = if options[:nav]
    options[:nav]
else
    File.join(src_dir, "_jekyll", "_data", "api_#{language}.yml")
end

# Create { permalink => path } hash
filedirs = File.join(src_dir, "api", language, "**", "*.md")
files = Hash[Dir.glob(filedirs).map { |f| [f.match(/([\w-]*).md/)[1], f] }]

# Parse navigation file
nav_yaml = YAML.load_file(nav_file)

# Write index file
puts index_header(language)

nav_yaml.each do |node|
    section = node["section"]
    puts "{% apisection #{section['name']} %}\n\n"
    section["commands"].each do |command|
        mdfile = files[command["permalink"]]
        if mdfile.nil?
            puts "<!-- NO FILE MATCH: #{command['permalink']} -->\n\n"
        else
            puts gen_fragment(mdfile)
        end
    end
    puts "{% endapisection %}\n\n"
end
