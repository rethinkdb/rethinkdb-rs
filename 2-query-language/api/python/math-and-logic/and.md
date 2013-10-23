---
layout: api-command 
language: Python
permalink: api/python/and/
command: '&'
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/math-and-logic/and.md
related_commands:
    '|': or/
---

{% apibody %}
bool & bool → bool
{% endapibody %}

Compute the logical and of two values.

__Example:__ True and false anded is false?

```py
(r.expr(True) & False).run(conn)
```
