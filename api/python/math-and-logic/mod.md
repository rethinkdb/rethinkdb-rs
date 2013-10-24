---
layout: api-command 
language: Python
permalink: api/python/mod/
command: '%'
related_commands:
    '+': add/
    '-': sub/
    '*': mul/
    '/': div/
---

# Command syntax #

{% apibody %}
number % number &rarr; number
{% endapibody %}

# Description #

Find the remainder when dividing two numbers.

__Example:__ It's as easy as 2 % 2 = 0.

```py
(r.expr(2) % 2).run(conn)
```

`
