# RethinkDB docs #

All the documentation available at [www.rethinkdb.com](http://www.rethinkdb.com).

## Architecture ##

### API ###
All the API files are in `2-query-language/api`.

Each language has its own directory, which means that a change for a method should be
done in three different files.

The file `index.md` contains a short description of every commands.

The other files in this directory are dedicated to one other command.
The yaml header has to contains:
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
# This method is not defined in python (valid keys are js, py, rb)
py: false
# This method is not defined in python (valid keys are js, py, rb)
rb: false
# Link to edit this document
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/accessing-rql/addListener.md
---
```



### Guide and cookbook ###
There are one file per language, so if you want to update a recipe, make sure you do it
for all the languages.


## Style guidelines ##
We use `redcarpet` to convert Markdown files.

### Markdown ###
Use Markdown as much as you can. Use HTML code only if needed.


### Special tags ###
Define a FAQ section (like in the cookbook), and creates some links to jump to the relevant faq.
```
{% faqsection %} <body> {% endfaqsection %}
```


Define an API section in the `index.md`.
```
{% apisection %} <body> {% endapisection %}
```

Define the body of a command
```
{% apibody %} <body> {% endapibody %}
```



The blue info box
```
{% infobox info %} <content> {% endinfobox %}
```



## Contribute ##

While the RethinkDB team will do its best to provide the best docu
Fork this repository, add your changes and submit a pull request :)

## License ##

This work is licensed under a [Creative Commons Attribution-ShareAlike 3.0 Unported License](http://creativecommons.org/licenses/by-sa/3.0/).




