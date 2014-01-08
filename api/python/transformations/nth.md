---
layout: api-command
language: Python
permalink: api/python/nth/
command: '[] (nth)'
related_commands:
    order_by: order_by/
    skip: skip/
    limit: limit/
    '[] (slice)': slice/
---

# Command syntax #

{% apibody %}
sequence[index] &rarr; object
sequence.nth(index) &rarr; object
{% endapibody %}

# Description #

Get the nth element of a sequence.

__Example:__ Select the second element in the array.

```py
r.expr([1,2,3])[1].run(conn)
r.expr([1,2,3]).nth(1).run(conn)
```
