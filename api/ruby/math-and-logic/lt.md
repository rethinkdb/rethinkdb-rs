---
layout: api-command
language: Ruby
permalink: api/ruby/lt/
command: <
related_commands:
    '>': gt/
    '>=': ge/
    '<=': le/
---

# Command syntax #

{% apibody %}
value < value &rarr; bool
{% endapibody %}

# Description #

Test if the first value is less than other.

__Example:__ Is 2 less than 2?

```rb
(r.expr(2) < 2).run(conn)
```


