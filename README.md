# RethinkDB docs #

This repository contains all of the documentation available at [rethinkdb.com](http://www.rethinkdb.com). 
We use [Jekyll](http://jekyllrb.com/) to build the site, so documentation is written in [Markdown](http://whatismarkdown.com/).

## Architecture ##

### API docs ###
All the API files are in `/api`. Each language has its own directory, which means that ReQL command changes require updating three different files. The file `index.md` contains a short description of every command.

Each command has a dedicated Markdown file for each language. A [YAML](http://yaml.org/)
header is used in each file for our [Jekyll](http://jekyllrb.com/) build system, and has to contain:
```yaml
---
# The layout of the document
layout: api-command
# The language, valid values are JavaScript, Python, Ruby
language: JavaScript
# The permalink
permalink: api/javascript/add_listener/
# The name of the command (used in the title)
command: addListener
# This method is not defined in a language, in this case, JavaScript -- (valid keys are js, py, rb) -- optional
js: false
# Defines the input and output of the command
io: [...]
# Set of related commands
related_commands:
    - <name>: <url_from_parent>
    - <name>: <url_from_parent>
---
```

### Multi-language docs ###

When there is a documentation page that differs between languages (e.g. the guides and the cookbook), one Markdown 
file is used per language. If, for example, a recipe is updated in the cookbook, make sure to update each version per-language.

## Style guidelines ##
We use [redcarpet](https://github.com/vmg/redcarpet) to process Markdown files.


### Markdown vs HTML ###
Use Markdown as much as you can. Use HTML markup only if needed.

### Custom Jekyll tags ###
__faqsection__: defines a FAQ section (e.g. the [Cookbook](http://rethinkdb.com/docs/cookbook/javascript/)),
and creates links to jump to the relevant entry.
```
{% faqsection %} <body> {% endfaqsection %}
```


__apisection__: defines an API section as seen On the [API index](http://rethinkdb.com/api/javascript).
```
{% apisection %} <body> {% endapisection %}
```

__apibody__: defines the method signature of a ReQL command
```
{% apibody %} <body> {% endapibody %}
```

__infobox__: produces an info box -- the _info_ version of the infobox produces a blue box
```
{% infobox info %} <content> {% endinfobox %}
```

## Contribute ##

While the RethinkDB team will do its best to provide the best documentation possible, 
please fork this repository, add your changes and submit pull requests. :)

## License ##

This work is licensed under a [Creative Commons Attribution-ShareAlike 3.0 Unported License](http://creativecommons.org/licenses/by-sa/3.0/).




