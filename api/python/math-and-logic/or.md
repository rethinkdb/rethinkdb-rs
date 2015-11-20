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
bool.or_([bool, bool, ...]) &rarr; bool
r.or_([bool, bool, ...]) &rarr; bool
{% endapibody %}

# Description #

Compute the logical "or" of one or more values. The `or_` command can be used as an infix operator after its first argument (`r.expr(True).or_(False)`) or given all of its arguments as parameters (`r.or_(True, False)`). The standard Python or operator, `|`, may also be used with ReQL.

Calling `or_` with zero arguments will return `False`.

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

__Note:__ When using `or` inside a `filter` predicate to test the values of fields that may not exist on the documents being tested, you should use the `default` command with those fields so they explicitly return `False`.

```py
r.table('posts').filter(lambda post:
    post['category'].default('foo').eq('article').or(
        post['genre'].default('foo').eq('mystery'))
).run(conn)
```
