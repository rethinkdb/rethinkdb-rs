---
layout: api-command
language: Java
permalink: api/java/div/
command: div
related_commands:
    add: add/
    sub: sub/
    mul: mul/
    mod: mod/
---

# Command syntax #

{% apibody %}
number.div(number[, number ...]) &rarr; number
{% endapibody %}

# Description #

Divide two numbers.

__Example:__ It's as easy as 2 / 2 = 1.

```java
r.expr(2).div(2).run(conn);
```

