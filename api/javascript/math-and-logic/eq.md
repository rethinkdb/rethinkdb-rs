---
layout: api-command
language: JavaScript
permalink: api/javascript/eq/
command: eq
io:
    -   - value
        - bool
related_commands:
    and: and/
    or: or/
    ne: ne/
---

# Command syntax #

{% apibody %}
value.eq(value) &rarr; bool
{% endapibody %}

# Description #

Test if two values are equal.

__Example:__ Does 2 equal 2?

```js
r.expr(2).eq(2).run(conn, callback)
```


