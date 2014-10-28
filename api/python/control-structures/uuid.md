---
layout: api-command
language: Python
permalink: api/python/uuid/
command: uuid
---

# Command syntax #

{% apibody %}
r.uuid() &rarr; string
{% endapibody %}

# Description #

Return a UUID (universally unique identifier), a string that can be used as a unique ID.

__Example:__ Generate a UUID.

```py
> r.uuid().run(conn)

27961a0e-f4e8-4eb3-bf95-c5203e1d87b9
```
