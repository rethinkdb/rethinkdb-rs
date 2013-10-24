---
layout: api-command 
language: Python
permalink: api/python/lt/
command: '<'
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/math-and-logic/lt.md
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

```py
(r.expr(2) < 2).run(conn)
```

