---
layout: api-command 
language: JavaScript
permalink: api/javascript/not/
command: not 
io:
    -   - bool
        - bool
related_commands:
    eq: eq
    ne: ne/
---

# Command syntax #

{% apibody %}
bool.not() &rarr; bool
{% endapibody %}

# Description #
Compute the logical inverse (not).

__Example:__ Not true is false.

```js
r.expr(true).not().run(conn, callback)
```
