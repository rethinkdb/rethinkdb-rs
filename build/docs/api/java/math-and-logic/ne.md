---
layout: api-command
language: Java
permalink: api/java/ne/
command: ne
related_commands:
    and: and/
    or: or/
    eq: eq/
---

# Command syntax #

{% apibody %}
value.ne(value[, value, ...]) &rarr; bool
{% endapibody %}

# Description #

Test if two or more values are not equal.

__Example:__ See if a user's `role` field is not set to `administrator`. 

```java
r.table("users").get(1).g("role").ne("administrator").run(conn);
```

__Example:__ See if three variables do not contain equal values.

```java
r.ne(a, b, c).run(conn);
```
