---
layout: api-command
language: Python
permalink: api/python/gt/
command: '>'
related_commands:
    '>=': ge/
    '<': lt/
    '<=': le/
---

# Command syntax #

{% apibody %}
value > value &rarr; bool
{% endapibody %}

Test if the first value is greater than other.

__Example:__ Is 2 greater than 2?

```py
(r.expr(2) > 2).run(conn)
```

