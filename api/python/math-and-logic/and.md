---
layout: api-command
language: Python
permalink: api/python/and/
command: '&, and_'
related_commands:
    '|, or_': or/
---

# Command syntax #

{% apibody %}
bool & bool &rarr; bool
r.and_(bool, bool) &rarr; bool
bool.and_(bool) &rarr; bool
{% endapibody %}

# Description #

Compute the logical and of two values.

__Example:__ True and false anded is false?

```py
(r.expr(True) & False).run(conn)
r.expr(True).and_(False).run(conn)
r.and_(True, False).run(conn)
```
