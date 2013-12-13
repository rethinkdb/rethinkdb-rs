---
layout: api-command
language: Python
permalink: api/python/eq/
command: '==, eq'
related_commands:
    '&, and_': and/
    '|, or_': or/
    '!=, ne': ne/
---

# Command syntax #

{% apibody %}
value == value &rarr; bool
value.eq(value) &rarr; bool
{% endapibody %}

Test if two values are equal.

__Example:__ Does 2 equal 2?

```py
(r.expr(2) == 2).run(conn)
r.expr(2).eq(2).run(conn)
```
