---
layout: api-command
language: Java
permalink: api/java/round/
command: round
related_commands:
    ceil: ceil/
    floor: floor/
---
# Command syntax #

{% apibody %}
r.round(number) &rarr; number
number.round() &rarr; number
{% endapibody %}

# Description #

Rounds the given value to the nearest whole integer. For example, values of 1.0 up to but not including 1.5 will return 1.0, similar to [floor][]; values of 1.5 up to 2.0 will return 2.0, similar to [ceil][].

[floor]: /api/java/floor/
[ceil]:  /api/java/ceil/

__Example:__ Round 12.345 to the nearest integer.

```java
r.round(12.345).run(conn);

// Result:
12.0
```

The `round` command can also be chained after an expression.

__Example:__ Round -12.345 to the nearest integer.

```java
r.expr(-12.345).round().run(conn);

// Result:
-12.0
```

__Example:__ Return Iron Man's weight, rounded to the nearest integer.

```java
r.table("superheroes").get("ironman")("weight").round().run(conn);
```
