---
layout: api-command
language: Python
permalink: api/python/ne/
command: '!='
related_commands:
    '&': and/
    '|': or/
    '==': eq/
---

# Command syntax #

{% apibody %}
value != value &rarr; bool
{% endapibody %}

# Description #

Test if two values are not equal.

__Example:__ Does 2 not equal 2?

```py
(r.expr(2) != 2).run(conn)
```


