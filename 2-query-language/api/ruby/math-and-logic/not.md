---
layout: api-command 
language: Ruby
permalink: api/ruby/not/
command: not
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/math-and-logic/not.md
related_commands:
    'eq': eq/
    'ne': ne/
---

# Command syntax #

{% apibody %}
bool.not() &rarr; bool
{% endapibody %}

# Description #
Compute the logical inverse (not).

__Example:__ Not true is false.

```rb
r(true).not.run(conn)
```
