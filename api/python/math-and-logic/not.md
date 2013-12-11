---
layout: api-command
language: Python
permalink: api/python/not/
command: '~'
related_commands:
    '==, eq': eq/
    '!=, ne': ne/
---

# Command syntax #

{% apibody %}
~bool &rarr; bool
bool.not_() &rarr; bool
{% endapibody %}

# Description #
Compute the logical inverse (not).

__Example:__ Not true is false.

```py
(~r.expr(True)).run(conn)
```

Note the parentheses around the query. If you execute

```py
~r.expr(True).run(conn)
```

You will get back `-2` because the query executed is `r.expr(True)` which returns `True`,
and because `~True` evaluates to `-2` in Python.

__Example:__ The previous query can be rewritten with `not_`

```py
r.expr(True).not_().run(conn)
```
