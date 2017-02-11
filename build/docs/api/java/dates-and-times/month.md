---
layout: api-command
language: Java
permalink: api/java/month/
command: month
related_commands:
    now: now/
    time: time/
---


# Command syntax #

{% apibody %}
time.month() &rarr; number
{% endapibody %}

# Description #

Return the month of a time object as a number between 1 and 12. For your convenience, the terms `r.january`, `r.february`, etc. are defined and map to the appropriate integer.

__Example:__ Retrieve all the users who were born in November.

```java
r.table("users").filter(row -> row.g("birthdate").month().eq(11)).run(conn);
```


__Example:__ Retrieve all the users who were born in September.

```java
r.table("users").filter(
    row -> row.g("birthdate").month().eq(r.september())
).run(conn);
```

