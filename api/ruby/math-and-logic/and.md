---
layout: api-command
language: Ruby
permalink: api/ruby/and/
command: '&, and'
related_commands:
    '|, or': or/
---

# Command syntax #

{% apibody %}
bool & bool &rarr; bool
bool.and(bool) &rarr; bool
r.and(bool, bool) &rarr; bool
{% endapibody %}

# Description #

Compute the logical "and" of two or more values. The `and` command can be used as an infix operator after its first argument (`r.expr(true).and(false)`) or given all of its arguments as parameters (`r.and(true, false)`). The standard Ruby or operator, `|`, may also be used with ReQL.

__Example:__ Return whether both `a` and `b` evaluate to true.

```rb
> a = true
> b = false
> (r.expr(a) & b).run(conn)

false
```

__Example:__ Return whether all of `x`, `y` and `z` evaluate to true.

```rb
> x = true
> y = true
> z = true
> r.and(x, y, z).run(conn)

true
```
