---
layout: api-command
language: Python
permalink: api/python/or/
command: '|, or_'
related_commands:
    '&, and_': and/
---

# Command syntax #

{% apibody %}
bool | bool &rarr; bool
bool.or_(bool[, bool, ...]) &rarr; bool
r.or_(bool, bool) &rarr; bool
{% endapibody %}

# Description #

Compute the logical "or" of two or more values. The `or_` command can be used as an infix operator after its first argument (`r.expr(True).or_(False)`) or given all of its arguments as parameters (`r.or_(True, False)`). The standard Python or operator, `|`, may also be used with ReQL.

__Example:__ Return whether either `a` or `b` evaluate to true.

```py
> a = True
> b = False
> (r.expr(a) | b).run(conn)

True
```

__Example:__ Return whether any of `x`, `y` or `z` evaluate to true.

```py
> x = False
> y = False
> z = False
> r.or_(x, y, z).run(conn)

False
```
