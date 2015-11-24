---
layout: api-command
language: Java
permalink: api/java/lt/
command: lt
related_commands:
    eq: eq/
    ne: ne/
    gt: gt/
    ge: ge/
    le: le/
---

# Command syntax #

{% apibody %}
value.lt(value[, value, ...]) &rarr; bool
{% endapibody %}

# Description #

Compare values, testing if the left-hand value is less than the right-hand.

__Example:__ Test if a player has scored less than 10 points.

```java
r.table("players").get(1).g("score").lt(10).run(conn);
```

__Example:__ Test if variables are ordered from highest to lowest, with no values being equal to one another.

```java
int a = 20;
int b = 10;
int c = 15;
r.lt(a, b, c).run(conn);
```

This is the equivalent of the following:

```java
r.lt(a, b).and(r.lt(b, c)).run(conn);
```
