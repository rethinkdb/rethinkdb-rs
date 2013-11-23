---
layout: api-command
language: JavaScript
permalink: api/javascript/ge/
command: ge
io:
    -   - value
        - bool
related_commands:
    eq: eq/
    ne: ne/
    gt: gt/
    lt: lt/
    le: le/
---

# Command syntax #

{% apibody %}
value.ge(value) &rarr; bool
{% endapibody %}

# Description #

Test if the first value is greater than or equal to other.

__Example:__ Is 2 greater than or equal to 2?

```js
r.expr(2).ge(2).run(conn, callback)
```
