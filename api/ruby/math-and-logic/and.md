---
layout: api-command
language: Ruby
permalink: api/ruby/and/
command: '&, and'
related_commands:
    '|, or': or/
---

# Command syntax #

{% apibody %}
bool & bool &rarr; bool
bool.and(bool) &rarr; bool
r.and(bool, bool) &rarr; bool
{% endapibody %}

# Description #

Compute the logical and of two values.

__Example:__ True and false anded is false?

```rb
(r.expr(True) & False).run(conn)
r.expr(True).and(False).run(conn)
r.and(True, False).run(conn)
```
