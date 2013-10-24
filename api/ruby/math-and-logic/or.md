---
layout: api-command 
language: Ruby
permalink: api/ruby/or/
command: '|'
related_commands:
    '&': and/
---

# Command syntax #

{% apibody %}
bool | bool &rarr; bool
{% endapibody %}

# Description #

Compute the logical or of two values.

__Example:__ True or false ored is true?

```rb
(r.expr(True) | False).run(conn)
```


