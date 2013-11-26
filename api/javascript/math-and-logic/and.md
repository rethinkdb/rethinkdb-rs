---
layout: api-command
language: JavaScript
permalink: api/javascript/and/
command: and
io:
    -   - bool
        - bool
related_commands:
    or: or/
    eq: eq/
    ne: ne/
---

# Command syntax #

{% apibody %}
bool.and(bool) &rarr; bool
{% endapibody %}

# Description #

Compute the logical and of two values.

__Example:__ True and false anded is false?

```js
r.expr(true).and(false).run(conn, callback)
```
