---
layout: api-command 
language: Ruby
permalink: api/ruby/ge/
command: '>='
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/math-and-logic/ge.md
related_commands:
    '>': gt/
    '<' : lt/
    '<=': le/
---

# Command syntax #

{% apibody %}
value >= value &rarr; bool
{% endapibody %}

# Description #

Test if the first value is greater than or equal to other.

__Example:__ Is 2 greater than or equal to 2?

```rb
(r.expr(2) >= 2).run(conn)
```


