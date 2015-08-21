---
layout: api-command
language: Python
permalink: api/python/ne/
command: '!=, ne'
related_commands:
    '&, and_': and/
    '|, or_': or/
    '==, eq': eq/
---

# Command syntax #

{% apibody %}
value.ne(value[, value, ...]) &rarr; bool
value != value &rarr; bool
{% endapibody %}

# Description #

Test if two or more values are not equal.

__Example:__ See if a user's `role` field is not set to `administrator`. 

```py
r.table('users').get(1)['role'].ne('administrator').run(conn)
# alternative syntax
(r.table('users').get(1)['role'] != 'administrator').run(conn)
```

__Example:__ See if three variables do not contain equal values.

```py
r.ne(a, b, c).run(conn)
```
