---
layout: api-command 
language: Ruby
permalink: api/ruby/gt/
command: '>'
related_commands:
    '>=': ge/
    '<' : lt/
    '<=': le/
---

# Command syntax #

{% apibody %}
value > value &rarr; bool
{% endapibody %}

# Description #

Test if the first value is greater than other.

__Example:__ Is 2 greater than 2?

```rb
(r.expr(2) > 2).run(conn)
```


