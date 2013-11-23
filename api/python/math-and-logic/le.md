---
layout: api-command
language: Python
permalink: api/python/le/
command: '<='
related_commands:
    '>': gt/
    '<': lt/
    '<=': le/
---

# Command syntax #

{% apibody %}
value <= value &rarr; bool
{% endapibody %}

# Description #

Test if the first value is less than or equal to other.

__Example:__ Is 2 less than or equal to 2?

```py
(r.expr(2) <= 2).run(conn)
```


