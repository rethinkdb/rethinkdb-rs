---
layout: api-command
language: Python
permalink: api/python/le/
command: '<=, le'
related_commands:
    '>, gt': gt/
    '<, lt': lt/
    '<=, le': le/
---

# Command syntax #

{% apibody %}
value <= value &rarr; bool
value.le(value) &rarr; bool
{% endapibody %}

# Description #

Test if the first value is less than or equal to other.

__Example:__ Is 2 less than or equal to 2?

```py
(r.expr(2) <= 2).run(conn)
r.expr(2).le(2).run(conn)
```


