---
layout: api-command
language: Java
permalink: api/java/or/
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

```java
boolean a = true;
boolean b = false;
r.expr(a).or(b).run(conn);

// Result:
true
```

__Example:__ Return whether any of `x`, `y` or `z` evaluate to true.

```java
boolean x = false;
boolean y = false;
boolean z = false;
r.or(x, y, z).run(conn);

// Result:
false
```

__Note:__ When using `or` inside a `filter` predicate to test the values of fields that may not exist on the documents being tested, you should use the `default` command with those fields so they explicitly return `false`.

```java
r.table("posts").filter(row ->
    row("category").default("foo").eq("article").
    or(row("genre").default("foo").eq("mystery"))
).run(conn);
```
