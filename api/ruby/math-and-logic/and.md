---
layout: api-command 
language: Ruby
permalink: api/ruby/and/
command: '&'
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
