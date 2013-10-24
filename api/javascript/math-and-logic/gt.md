---
layout: api-command 
language: JavaScript
permalink: api/javascript/gt/
command: gt 
io:
    -   - value
        - bool
related_commands:
    eq: eq
    ne: ne/
    ge: ge/
    lt: lt/
    le: le/
---

# Command syntax #

{% apibody %}
value.gt(value) &rarr; bool
{% endapibody %}

# Description #

Test if the first value is greater than other.

__Example:__ Is 2 greater than 2?

```js
r.expr(2).gt(2).run(conn, callback)
```

