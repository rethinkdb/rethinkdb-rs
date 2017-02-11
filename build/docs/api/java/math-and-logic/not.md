---
layout: api-command
language: Java
permalink: api/java/not/
command: not
related_commands:
    eq: eq
    ne: ne/
---

# Command syntax #

{% apibody %}
bool.not() &rarr; bool
r.not(bool) &rarr; bool
{% endapibody %}

# Description #

Compute the logical inverse (not) of an expression.

`not` can be called either via method chaining, immediately after an expression that evaluates as a boolean value, or by passing the expression as a parameter to `not`. All values that are not `false` or `null` will be converted to `true`.

__Example:__ Not true is false.

```java
r(true).not().run(conn);
r.not(true).run(conn);
```

These evaluate to `false`.

__Example:__ Return all the users that do not have a "flag" field.

```java
r.table("users").filter(user -> user.hasFields("flag").not()).run(conn);
```

__Example:__ As above, but prefix-style.

```java
r.table("users").filter(user -> r.not(user.hasFields("flag")).run(conn);
```
