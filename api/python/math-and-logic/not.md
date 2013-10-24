---
layout: api-command 
language: Python
permalink: api/python/not/
command: '~'
related_commands:
    '==': eq/
    '!=': ne/
---

# Command syntax #

{% apibody %}
~bool &rarr; bool
{% endapibody %}

# Description #
Compute the logical inverse (not).

__Example:__ Not true is false.

```py
(~r.expr(True)).run(conn)
```
