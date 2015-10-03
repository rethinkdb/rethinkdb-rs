---
layout: api-command
language: Java
permalink: api/javascript/uuid/
command: uuid
io:
    -   - r
        - string
---

# Command syntax #

{% apibody %}
r.uuid() &rarr; string
{% endapibody %}

# Description #

Return a UUID (universally unique identifier), a string that can be used as a unique ID.

__Example:__ Generate a UUID.

```js
> r.uuid().run(conn)
// result returned to callback
"27961a0e-f4e8-4eb3-bf95-c5203e1d87b9"
```
