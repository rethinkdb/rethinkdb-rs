---
layout: api-command
language: Python
permalink: api/python/sub/
command: '-'
related_commands:
    '+': add/
    '*': mul/
    '/': div/
    '%': mod/
---

# Command syntax #

{% apibody %}
number - number &rarr; number
time - number &rarr; time
time - time &rarr; number
number.sub(number[, number, ...]) &rarr; number
time.sub(number[, number, ...]) &rarr; time
time.sub(time) &rarr; number
{% endapibody %}

# Description #

Subtract two numbers.

__Example:__ It's as easy as 2 - 2 = 0.

```py
(r.expr(2) - 2).run(conn)
```


__Example:__ Create a date one year ago today.

```py
r.now() - 365*24*60*60
```

__Example:__ Retrieve how many seconds elapsed between today and `date`.

```py
r.now() - date
```

