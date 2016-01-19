---
layout: api-command
language: Python
permalink: api/python/and/
command: '&, and_'
related_commands:
    '|, or_': or/
---

# Command syntax #

{% apibody %}
bool & bool &rarr; bool
bool.and_([bool, bool, ...]) &rarr; bool
r.and_([bool, bool, ...]) &rarr; bool
{% endapibody %}

# Description #

Compute the logical "and" of one or more values.

The `and_` command can be used as an infix operator after its first argument (`r.expr(True).and_(False)`) or given all of its arguments as parameters (`r.and_(True, False)`). The standard Python and operator, `&`, may also be used with ReQL.

Calling `and_` with zero arguments will return `True`.

__Example:__ Return whether both `a` and `b` evaluate to true.

```py
> a = True
> b = False
> (r.expr(a) & b).run(conn)

False
```
__Example:__ Return whether all of `x`, `y` and `z` evaluate to true.

```py
> x = True
> y = True
> z = True
> r.and_(x, y, z).run(conn)

True
```
