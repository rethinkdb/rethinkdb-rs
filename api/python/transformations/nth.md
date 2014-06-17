---
layout: api-command
language: Python
permalink: api/python/nth/
command: 'nth, []'
related_commands:
    order_by: order_by/
    skip: skip/
    limit: limit/
    'slice, []': slice/
---

# Command syntax #

{% apibody %}
sequence.nth(index) &rarr; object
selection.nth(index) &rarr; selection&lt;object&gt;
{% endapibody %}

# Description #

Get the *nth* element of a sequence.

In Python, you can use `[]` with an integer as a shorthand for `nth`.

__Example:__ Select the second element in the array.

```py
r.expr([1,2,3]).nth(1).run(conn)
r.expr([1,2,3])[1].run(conn)
```

**Example:** Select the bronze medalist from the competitors.

```py
r.table('players').order_by(index=r.desc('score')).nth(3).run(conn)
```
