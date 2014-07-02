---
layout: api-command
language: Python
permalink: api/python/not/
command: '~, not_'
related_commands:
    '==, eq': eq/
    '!=, ne': ne/
---

# Command syntax #

{% apibody %}
bool.not_() &rarr; bool
not_(bool) &rarr; bool
(~bool) &rarr; bool
{% endapibody %}

# Description #
Compute the logical inverse (not) of an expression.

`not_` can be called either via method chaining, immediately after an expression that evaluates as a boolean value, or by passing the expression as a parameter to `not_`.  All values that are not `False` or `None` will be converted to `True`.

You may also use `~` as a shorthand operator.

__Example:__ Not true is false.

```py
r.not_(True).run(conn)
r.expr(True).not_().run(conn)
(~r.expr(True)).run(conn)
```

These evaluate to `false`.

Note that when using `~` the expression is wrapped in parentheses. Without this, Python will evaluate `r.expr(True)` *first* rather than using the ReQL operator and return an incorrect value. (`~True` evaluates to &minus;2 in Python.)

__Example:__ Return all the users that do not have a "flag" field.

```py
r.table('users').filter(
    lambda users: (~users.has_fields('flag'))
).run(conn)
```

__Example:__ As above, but prefix-style.

```py
r.table('users').filter(
    lambda users: r.not_(users.has_fields('flag'))
).run(conn)
```
