---
layout: api-command 
language: Python
permalink: api/python/eq/
command: '=='
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/math-and-logic/eq.md
related_commands:
    '&': and/
    '|': or/
    '!=': ne/
---

# Command syntax #

{% apibody %}
bool & bool &rarr; bool
{% endapibody %}

# Description #

Compute the logical and of two values.

__Example:__ True and false anded is false?

```py
(r.expr(True) & False).run(conn)
```
