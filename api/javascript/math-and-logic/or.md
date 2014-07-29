---
layout: api-command
language: JavaScript
permalink: api/javascript/or/
command: or
io:
    -   - bool
        - bool
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
r.expr(a).or(b).run(conn, callback);
// result passed to callback
true
```

__Example:__ Return whether any of `x`, `y` or `z` evaluate to true.

```js
var x = false, y = false, z = false;
r.or(x, y, z).run(conn, callback);
// result passed to callback
false
```
