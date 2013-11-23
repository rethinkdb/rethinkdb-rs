---
layout: api-command
language: JavaScript
permalink: api/javascript/ne/
command: ne
io:
    -   - value
        - bool
related_commands:
    and: and/
    or: or/
    eq: eq/
---

# Command syntax #

{% apibody %}
value.ne(value) &rarr; bool
{% endapibody %}

# Description #

Test if two values are not equal.

__Example:__ Does 2 not equal 2?

```js
r.expr(2).ne(2).run(conn, callback)
```
