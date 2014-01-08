---
layout: api-command
language: Ruby
permalink: api/ruby/or/
command: '|, or'
related_commands:
    '&, and': and/
---

# Command syntax #

{% apibody %}
bool | bool &rarr; bool
bool.or(bool) &rarr; bool
r.or(bool, bool) &rarr; bool
{% endapibody %}

# Description #

Compute the logical or of two values.

__Example:__ True or false ored is true?

```rb
(r.expr(True) | False).run(conn)
r.expr(True).or(False).run(conn)
r.or(True, False).run(conn)
```
