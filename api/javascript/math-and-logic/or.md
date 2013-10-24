---
layout: api-command 
language: JavaScript
permalink: api/javascript/or/
command: or 
io:
    -   - bool
        - bool
related_commands:
    and: and/
    eq: eq/
    ne: ne/
---

# Command syntax #

{% apibody %}
bool.or(bool) &rarr; bool
{% endapibody %}

# Description #

Compute the logical or of two values.

__Example:__ True or false ored is true?

```js
r.expr(true).or(false).run(conn, callback)
```
