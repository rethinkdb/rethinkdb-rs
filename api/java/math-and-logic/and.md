---
layout: api-command
language: Java
permalink: api/java/and/
command: and
related_commands:
    or: or/
    eq: eq/
    ne: ne/
---

# Command syntax #

{% apibody %}
bool.and(bool[, bool, ...]) &rarr; bool
r.and(bool[, bool, ...]) &rarr; bool
{% endapibody %}

# Description #

Compute the logical "and" of two or more values. The `and` command can be used as an infix operator after its first argument (`r.expr(true).and(false)`) or given all of its arguments as parameters (`r.and(true,false)`).

__Example:__ Return whether both `a` and `b` evaluate to true.

```java
var a = true, b = false;
r.expr(a).and(b).run(conn);
// result passed to callback
false
```

__Example:__ Return whether all of `x`, `y` and `z` evaluate to true.

```java
var x = true, y = true, z = true;
r.and(x, y, z).run(conn);
// result passed to callback
true
```
