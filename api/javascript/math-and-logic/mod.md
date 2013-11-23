---
layout: api-command
language: JavaScript
permalink: api/javascript/mod/
command: mod
io:
    -   - number
        - number
related_commands:
    add: add/
    sub: sub/
    mul: mul/
    div: div/
---

# Command syntax #

{% apibody %}
number.div(number) &rarr; number
{% endapibody %}

# Description #

Divide two numbers.

__Example:__ It's as easy as 2 / 2 = 1.

```js
r.expr(2).div(2).run(conn, callback)
```

