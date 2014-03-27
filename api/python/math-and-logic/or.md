---
layout: api-command
language: Python
permalink: api/python/or/
command: '|, or_'
related_commands:
    '&, and_': and/
---

# Command syntax #

{% apibody %}
bool | bool &rarr; bool
bool.or_(bool) &rarr; bool
r.or_(bool, bool) &rarr; bool
{% endapibody %}

# Description #

Compute the logical or of two values.

__Example:__ True or false ored is true?

```py
(r.expr(True) | False).run(conn)
r.expr(True).or_(False).run(conn)
r.or_(True, False).run(conn)
```
