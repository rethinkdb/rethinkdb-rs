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
number.mod(number) &rarr; number
{% endapibody %}

Find the remainder when dividing two numbers.

__Example:__ It's as easy as 2 % 2 = 0.

```js
r.expr(2).mod(2).run(conn)
```

