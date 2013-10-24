---
layout: api-command 
language: Python
permalink: api/python/and/
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

```py
(r.expr(True) & False).run(conn)
```
