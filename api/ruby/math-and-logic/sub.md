---
layout: api-command
language: Ruby
permalink: api/ruby/sub/
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
time - time &rarr; number
time - number &rarr; time
{% endapibody %}

# Description #

Subtract two numbers.

__Example:__ It's as easy as 2 - 2 = 0.

```rb
(r.expr(2) - 2).run(conn)
```


__Example:__ Create a date one year ago today.

```rb
r.now() - 365*24*60*60
```


__Example:__ Retrieve how many seconds elapsed between today and date

```rb
r.now() - date
```

