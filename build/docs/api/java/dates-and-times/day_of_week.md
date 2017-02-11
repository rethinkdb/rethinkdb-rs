---
layout: api-command
language: Java
permalink: api/java/day_of_week/
command: dayOfWeek
related_commands:
    now: now/
    time: time/
---

# Command syntax #

{% apibody %}
time.dayOfWeek() &rarr; number
{% endapibody %}

# Description #

Return the day of week of a time object as a number between 1 and 7 (following ISO 8601 standard). For your convenience, the terms r.monday, r.tuesday, etc. are defined and map to the appropriate integer.

__Example:__ Return today's day of week.

```java
r.now().dayOfWeek().run(conn);
```

__Example:__ Retrieve all the users who were born on a Tuesday.

```java
r.table("users").filter(
    row -> row.g("birthdate").dayOfWeek().eq(r.tuesday())
).run(conn);
```

