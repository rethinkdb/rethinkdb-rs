---
layout: api-command
language: Java
permalink: api/javascript/mul/
command: mul
related_commands:
    add: add/
    sub: sub/
    div: div/
    mod: mod/
---

# Command syntax #

{% apibody %}
number.mul(number[, number, ...]) &rarr; number
array.mul(number[, number, ...]) &rarr; array
{% endapibody %}

# Description #

Multiply two numbers, or make a periodic array.

__Example:__ It's as easy as 2 * 2 = 4.

```js
r.expr(2).mul(2).run(conn)
```

__Example:__ Arrays can be multiplied by numbers as well.

```js
r.expr(["This", "is", "the", "song", "that", "never", "ends."]).mul(100).run(conn)
```

