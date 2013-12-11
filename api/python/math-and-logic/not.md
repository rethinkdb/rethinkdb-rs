---
layout: api-command
language: Python
permalink: api/python/not/
command: '~, not_'
related_commands:
    '==': eq/
    '!=': ne/
---

# Command syntax #

{% apibody %}
~bool &rarr; bool
bool.not_() &rarr; bool
r.not_(bool) &rarr; bool
{% endapibody %}

# Description #
Compute the logical inverse (not).

__Example:__ Not true is false.

```py
r.not_(True).run(conn)
r.expr(True).not_().run(conn)
(~r.expr(True)).run(conn)
```

Note the parentheses around the last query. If you execute

```py
~r.expr(True).run(conn)
```

You will get back `-2` because the query executed is `r.expr(True)` which returns `True`,
and because `~True` evaluates to `-2` in Python.
