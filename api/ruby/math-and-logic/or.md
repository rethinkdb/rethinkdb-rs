---
layout: api-command
language: Ruby
permalink: api/ruby/or/
command: '|, or'
related_commands:
    '&, and': and/
---

# Command syntax #

{% apibody %}
bool | bool &rarr; bool
bool.or(bool[, bool, ...]) &rarr; bool
r.or(bool, bool[, bool, ...]) &rarr; bool
{% endapibody %}

# Description #

Compute the logical "or" of two or more values. The `or` command can be used as an infix operator after its first argument (`r.expr(true).or(false)`) or given all of its arguments as parameters (`r.or(true, false)`). The standard Ruby or operator, `|`, may also be used with ReQL.

__Example:__ Return whether either `a` or `b` evaluate to true.

```rb
> a = true
> b = false
> (r.expr(a) | b).run(conn)

true
```

__Example:__ Return whether any of `x`, `y` or `z` evaluate to true.

```rb
> x = false
> y = false
> z = false
> r.or(x, y, z).run(conn)

false
```

__Note:__ When using `or` inside a `filter` predicate to test the values of fields that may not exist on the documents being tested, you should use the `default` command with those fields so they explicitly return `false`.

```rb
r.table('posts').filter { |post|
    post['category'].default('foo').eq('article').
    or(post['genre'].default('foo').eq('mystery'))
}.run(conn)
```
