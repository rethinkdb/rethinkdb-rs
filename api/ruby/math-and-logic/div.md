---
layout: api-command
language: Ruby
permalink: api/ruby/div/
command: '/'
related_commands:
    '+': add/
    '-': sub/
    '*': mul/
    '%': mod/
---

# Command syntax #

{% apibody %}
number / number &rarr; number
{% endapibody %}

# Description #

Divide two numbers.

__Example:__ It's as easy as 2 / 2 = 1.

```rb
(r.expr(2) / 2).run(conn)
```


