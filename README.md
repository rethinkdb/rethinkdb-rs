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
layout: api-command                       # The layout of the document
language: JavaScript                      # The language, valid values are JavaScript, Python, Ruby
permalink: api/javascript/add_listener/   # The permaling
command: addListener                      # The name of the command (used in the title)
py: false                                 # This method is not defined in python (valid keys are js, py, rb)
rb: false                                 # This method is not defined in python (valid keys are js, py, rb)
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/accessing-rql/addListener.md
# Link to edit this document
---

```



### Guide and cookbook ###
There are one file per language, so if you want to update a recipe, make sure you do it
for all the languages.


## Special tags ##
We use `redcarpet` to convert Markdown files.

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

Fork this repository, add your changes and submit a pull request.

## License ##

Copyright 2010-2013 RethinkDB

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this product except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

