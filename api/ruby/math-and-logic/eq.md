---
layout: api-command 
language: Ruby
permalink: api/ruby/eq/
command: eq
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/math-and-logic/eq.md
related_commands:
    '&': and/
    '|': or/
    ne: ne/
---

# Command syntax #

{% apibody %}
value.eq(value) &rarr; bool
{% endapibody %}

# Description #

Test if two values are equal.

__Example:__ Does 2 equal 2?

```rb
r.expr(2).eq(2).run(conn)
```
