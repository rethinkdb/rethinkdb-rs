# RethinkDB Documentation #

This repository contains all the documentation available at [rethinkdb.com][1]. Documentation is written in [Github Flavored][2] [Markdown][3].

[1]: http://rethinkdb.com/
[2]: https://help.github.com/articles/github-flavored-markdown
[3]: https://daringfireball.net/projects/markdown/basics

## Contributing ##

Check out our [Contributing Guidelines][4].

[4]: https://github.com/rethinkdb/docs/blob/master/CONTRIBUTING.md

## Documentation Layout ##

### YAML front-matter ###

We use [Jekyll][5] to build a static site. Each file starts with a [YAML front-matter block][6], which defines variables used by Jekyll in the build process. The required variables are:

[5]: http://jekyllrb.com/
[6]: http://jekyllrb.com/docs/frontmatter/

```
---
layout: documentation                 # The layout we are going to use
title: Introduction to ReQL           # Title of the page
active: docs                          # The active link in the navbar at the top of the page
docs_active: introduction-to-reql     # The active link in the documentation index on the right
permalink: docs/introduction-to-reql/ # URL of the page
---
```

### Markdown parser ###

We use use [Redcarpet][7] to parse the Markdown files, so make sure you use the appropriate syntax. See this [handy guide][8] to get started with Markdown. Use Markdown as much as you can. Use HTML markup only if needed.

[7]: https://github.com/vmg/redcarpet
[8]: https://github.com/adam-p/markdown-here/wiki/Markdown-Cheatsheet

### Consistency for multi-version docs ###

Some documentation pages have multiple versions for different languages, client drivers, platforms, etc. All updates need to be reflected in each version of the page.

For example, if you add a recipe to the [Cookbook][9], you will have to add it to the JavaScript, Python and Ruby versions of the recipe. If you aren't familiar with one of the languages, we'll be more than happy to help you add all the versions.

[9]: http://rethinkdb.com/docs/cookbook/javascript/

### API docs ###

All the API files are in `/api`. Each language has its own directory, which means that ReQL command changes require updating three different files. The file `index.md` contains a short description of every command.

Each command has a dedicated Markdown file for each language. A [YAML][10] header is used in each file for our build system, and has to contain:

[10]: http://yaml.org/

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
io: [...]# Set of related commandsrelated_commands:    - <name>: <url_from_parent>    - <name>: <url_from_parent>---
```

### Custom Jekyll tags ###

__faqsection__: defines a FAQ section (e.g. the [Cookbook][11]), and creates links to jump to the relevant entry.

```
{% faqsection %} <body> {% endfaqsection %}
```

__apisection__: defines an API section as seen On the [API index][12].

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

[11]: http://rethinkdb.com/docs/cookbook/javascript/
[12]: http://rethinkdb.com/api/javascript

## License ##

This work is licensed under a [Creative Commons Attribution-ShareAlike 3.0 Unported License][13].

[13]: http://creativecommons.org/licenses/by-sa/3.0/
