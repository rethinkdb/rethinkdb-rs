---
layout: api-command
language: JavaScript
permalink: api/javascript/and/
command: and
io:
    -   - bool
        - bool
related_commands:
    or: or/
    eq: eq/
    ne: ne/
---

# Command syntax #

{% apibody %}
bool.and([bool, bool, ...]) &rarr; bool
r.and([bool, bool, ...]) &rarr; bool
{% endapibody %}

# Description #

Compute the logical "and" of one or more values.

The `and` command can be used as an infix operator after its first argument (`r.expr(true).and(false)`) or given all of its arguments as parameters (`r.and(true,false)`).

Calling `and` with zero arguments will return `true`.

__Example:__ Return whether both `a` and `b` evaluate to true.

```js
var a = true, b = false;
r.expr(a).and(b).run(conn, callback);
// result passed to callback
false
```

__Example:__ Return whether all of `x`, `y` and `z` evaluate to true.

```js
var x = true, y = true, z = true;
r.and(x, y, z).run(conn, callback);
// result passed to callback
true
```
