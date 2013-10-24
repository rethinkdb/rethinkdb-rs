---
layout: api-command 
language: Ruby
permalink: api/ruby/mul/
command: '*'
related_commands:
    '+': add/
    '-': sub/
    '/': div/
    '%': mod/
---

# Command syntax #

{% apibody %}
number * number &rarr; number
array * number &rarr; array
{% endapibody %}

# Description #

Multiply two numbers, or make a periodic array.

__Example:__ It's as easy as 2 * 2 = 4.

```rb
(r.expr(2) * 2).run(conn)
```

__Example:__ Arrays can be multiplied by numbers as well.

```rb
(r.expr(["This", "is", "the", "song", "that", "never", "ends."]) * 100).run(conn)
```

