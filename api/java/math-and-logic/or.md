---
layout: api-command
language: JavaScript
permalink: api/javascript/or/
command: or
related_commands:
    and: and/
    eq: eq/
    ne: ne/
---

# Command syntax #

{% apibody %}
bool.or(bool[, bool, ...]) &rarr; bool
r.or(bool[, bool, ...]) &rarr; bool
{% endapibody %}

# Description #

Compute the logical "or" of two or more values. The `or` command can be used as an infix operator after its first argument (`r.expr(true).or(false)`) or given all of its arguments as parameters (`r.or(true,false)`).

__Example:__ Return whether either `a` or `b` evaluate to true.

```js
var a = true, b = false;
r.expr(a).or(b).run(conn);
// result passed to callback
true
```

__Example:__ Return whether any of `x`, `y` or `z` evaluate to true.

```js
var x = false, y = false, z = false;
r.or(x, y, z).run(conn);
// result passed to callback
false
```

__Note:__ When using `or` inside a `filter` predicate to test the values of fields that may not exist on the documents being tested, you should use the `default` command with those fields so they explicitly return `false`.

```js
r.table('posts').filter(
    r.row('category').default('foo').eq('article').
    or(r.row('genre').default('foo').eq('mystery'))
).run(conn);
```
