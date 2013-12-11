---
layout: api-command
language: Python
permalink: api/python/eq/
command: '=='
related_commands:
    '&': and/
    '|': or/
    '!=': ne/
---

# Command syntax #

{% apibody %}
value == value &rarr; bool
{% endapibody %}

Test if two values are equal.

__Example:__ Does 2 equal 2?

```py
(r.expr(2) == 2).run(conn)
```
