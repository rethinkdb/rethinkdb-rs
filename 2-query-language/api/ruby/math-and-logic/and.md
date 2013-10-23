---
layout: api-command 
language: Ruby
permalink: api/ruby/and/
command: '&'
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/math-and-logic/and.md
related_commands:
    '|': or/
---

# Command syntax #

{% apibody %}
bool & bool &rarr; bool
{% endapibody %}

# Description #

Compute the logical and of two values.

__Example:__ True and false anded is false?

```rb
(r.expr(True) & False).run(conn)
```
