---
layout: api-command
language: Python
permalink: api/python/ne/
command: '!=, ne'
related_commands:
    '&': and/
    '|': or/
    '==, eq': eq/
---

# Command syntax #

{% apibody %}
value != value &rarr; bool
value.ne(value) &rarr; bool
{% endapibody %}

# Description #

Test if two values are not equal.

__Example:__ Does 2 not equal 2?

```py
(r.expr(2) != 2).run(conn)
r.expr(2).ne(2).run(conn)
```


