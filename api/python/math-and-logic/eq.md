---
layout: api-command
language: Python
permalink: api/python/eq/
command: '==, eq'
related_commands:
    '&, and_': and/
    '|, or_': or/
    '!=, ne': ne/
---

# Command syntax #

{% apibody %}
value.eq(value[, value, ...]) &rarr; bool
value == value &rarr; bool
{% endapibody %}

# Description #

Test if two or more values are equal.

__Example:__ See if a user's `role` field is set to `administrator`. 

```py
r.table('users').get(1)['role'].eq('administrator').run(conn)
# alternative syntax
(r.table('users').get(1)['role'] == 'administrator').run(conn)
```

__Example:__ See if three variables contain equal values.

```py
r.eq(a, b, c).run(conn)
```
