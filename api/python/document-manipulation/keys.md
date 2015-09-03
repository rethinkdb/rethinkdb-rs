---
layout: api-command
language: Python
permalink: api/python/keys/
command: keys
related_commands:
    values: values/
---

# Command syntax #

{% apibody %}
singleSelection.keys() &rarr; array
object.keys() &rarr; array
{% endapibody %}

# Description #

Return an array containing all of an object's keys.

__Example:__ Get all the keys from a table row.

```py
# row: { "id": 1, "name": "fred", "email": "fred@example.com" }

r.table('users').get(1).keys().run(conn)

> [ "id", "name", "email" ]
```
