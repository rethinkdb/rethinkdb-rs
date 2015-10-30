---
layout: api-command
language: Java
permalink: api/java/nth/
command: nth
related_commands:
    skip: skip/
    limit: limit/
    bracket: bracket/
    slice: slice/
---

# Command syntax #

{% apibody %}
sequence.nth(index) &rarr; object
selection.nth(index) &rarr; selection&lt;object&gt;
{% endapibody %}

# Description #

Get the *nth* element of a sequence, counting from zero. If the argument is negative, count from the last element.

__Example:__ Select the second element in the array.

```java
r.expr([1,2,3]).nth(1).run(conn);
```

__Example:__ Select the bronze medalist from the competitors.

```java
r.table("players").orderBy().optArg("index", r.desc("score")).nth(3).run(conn);
```

__Example:__ Select the last place competitor.

```java
r.table("players").orderBy().optArg("index", r.desc("score")).nth(-1).run(conn);
```
