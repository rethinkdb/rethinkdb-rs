# RethinkDB Documentation #

This repository contains all the documentation available at [rethinkdb.com](http://rethinkdb.com/). Documentation is written in [Github Flavored Markdown](https://help.github.com/articles/github-flavored-markdown).

## Contributing ##

Check out our [contributing guidelines](https://github.com/rethinkdb/docs/blob/master/CONTRIBUTING.md).

## Documentation Layout ##

### YAML front-matter ###

We use [Jekyll](http://jekyllrb.com/) to build a static site. Each file starts with a [YAML front-matter block](http://jekyllrb.com/docs/frontmatter/), which defines variables used by Jekyll in the build process. The required variables are:

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

We use use [Redcarpet](https://github.com/vmg/redcarpet) to parse the Markdown files, so make sure you use the appropriate syntax. See this [handy guide](https://github.com/adam-p/markdown-here/wiki/Markdown-Cheatsheet) to get started with Markdown. Use Markdown as much as you can. Use HTML markup only if needed.

### Consistency for multi-version docs ###

Some documentation pages have multiple versions for different languages, client drivers, platforms, etc. All updates need to be reflected in each version of the page.

For example, if you add a recipe to the [Cookbook](http://rethinkdb.com/docs/cookbook/javascript/), you will have to add it to the JavaScript, Python and Ruby versions of the recipe. If you aren't familiar with one of the languages, we'll be more than happy to help you add all the versions.

### API docs ###

All the API files are in `/api`. Each language has its own directory, which means that ReQL command changes require updating three different files. The file `index.md` contains a short description of every command.

Each command has a dedicated Markdown file for each language. A [YAML](http://yaml.org/) header is used in each file for our build system, and has to contain:

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

### Custom Jekyll tags ###

__faqsection__: defines a FAQ section (e.g. the [Cookbook](http://rethinkdb.com/docs/cookbook/javascript/)), and creates links to jump to the relevant entry.

```
{% faqsection %}
<body>
{% endfaqsection %}
```

__apisection__: defines an API section as seen On the [API index](http://rethinkdb.com/api/javascript).

```
{% apisection %}
<body>
{% endapisection %}
```

__apibody__: defines the method signature of a ReQL command

```
{% apibody %}
<body>
{% endapibody %}
```

__infobox__: produces an info box -- the _info_ version of the infobox produces a blue box

```
{% infobox info %}
<content>
{% endinfobox %}
```

## License ##

This work is licensed under a [Creative Commons Attribution-ShareAlike 3.0 Unported License](http://creativecommons.org/licenses/by-sa/3.0/).
