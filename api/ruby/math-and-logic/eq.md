---
layout: api-command
language: Ruby
permalink: api/ruby/eq/
command: eq
related_commands:
    '&, and': and/
    '|, or': or/
    ne: ne/
---

# Command syntax #

{% apibody %}
value.eq(value) &rarr; bool
{% endapibody %}

# Description #

Test if two values are equal.

__Example:__ Does 2 equal 2?

```rb
r.expr(2).eq(2).run(conn)
```
