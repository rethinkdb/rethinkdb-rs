---
layout: api-command 
language: Python
permalink: api/python/le/
command: '<='
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/math-and-logic/le.md
related_commands:
    '>': gt/
    '<': lt/
    '<=': le/
---

{% apibody %}
value <= value &rarr; bool
{% endapibody %}

Test if the first value is less than or equal to other.

__Example:__ Is 2 less than or equal to 2?

```py
(r.expr(2) <= 2).run(conn)
```


