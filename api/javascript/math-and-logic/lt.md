---
layout: api-command
language: JavaScript
permalink: api/javascript/lt/
command: lt
io:
    -   - value
        - bool
related_commands:
    eq: eq/
    ne: ne/
    gt: gt/
    ge: ge/
    le: le/
---

# Command syntax #

{% apibody %}
value.lt(value) &rarr; bool
{% endapibody %}

# Description #

Test if the first value is less than other.

__Example:__ Is 2 less than 2?

```js
r.expr(2).lt(2).run(conn, callback)
```
