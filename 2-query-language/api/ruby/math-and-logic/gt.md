---
layout: api-command 
language: Ruby
permalink: api/ruby/gt/
command: '>'
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/math-and-logic/gt.md
related_commands:
    '>=': ge/
    '<' : lt/
    '<=': le/
---

# Command syntax #

{% apibody %}
value > value &rarr; bool
{% endapibody %}

# Description #

Test if the first value is greater than other.

__Example:__ Is 2 greater than 2?

```rb
(r.expr(2) > 2).run(conn)
```


